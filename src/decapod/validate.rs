use serde::{Deserialize, Serialize};
use std::process::Stdio;
use tokio::process::Command;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationResult {
    pub passed: bool,
    pub gate: String,
    pub details: Vec<ValidationDetail>,
    pub errors: Vec<ValidationError>,
    pub warnings: Vec<ValidationWarning>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationDetail {
    pub check: String,
    pub status: String,
    pub message: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationError {
    pub check: String,
    pub message: String,
    pub remediation: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationWarning {
    pub check: String,
    pub message: String,
}

#[derive(Debug, Clone)]
pub struct Validator {
    strict: bool,
}

impl Validator {
    pub fn new() -> Self {
        Self { strict: false }
    }

    pub fn strict(mut self) -> Self {
        self.strict = true;
        self
    }

    pub async fn run(&self) -> anyhow::Result<ValidationResult> {
        let output = Command::new("decapod")
            .args(["validate"])
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .await?;

        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();

        if stdout.trim().is_empty() && stderr.trim().is_empty() {
            return Err(anyhow::anyhow!("validate produced no output"));
        }

        let output_str = if !stdout.trim().is_empty() { stdout } else { stderr };

        if let Ok(result) = serde_json::from_str(&output_str) {
            return Ok(result);
        }

        for line in output_str.lines() {
            if let Ok(result) = serde_json::from_str(line) {
                return Ok(result);
            }
        }

        Ok(ValidationResult {
            passed: output.status.success(),
            gate: "unknown".to_string(),
            details: Self::parse_text_output(&output_str),
            errors: Vec::new(),
            warnings: Vec::new(),
        })
    }

    fn parse_text_output(output: &str) -> Vec<ValidationDetail> {
        output
            .lines()
            .filter(|l| !l.is_empty())
            .map(|line| ValidationDetail {
                check: line.trim().to_string(),
                status: "unknown".to_string(),
                message: None,
            })
            .collect()
    }

    pub async fn run_gate(&self, gate: &str) -> anyhow::Result<ValidationResult> {
        let output = Command::new("decapod")
            .args(["validate", "--gate", gate])
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .await?;

        let output_str = String::from_utf8_lossy(&output.stdout)
            .to_string();

        if let Ok(result) = serde_json::from_str(&output_str) {
            return Ok(result);
        }

        Ok(ValidationResult {
            passed: output.status.success(),
            gate: gate.to_string(),
            details: Vec::new(),
            errors: Vec::new(),
            warnings: Vec::new(),
        })
    }

    pub fn validate_or_panic(result: &ValidationResult) {
        if !result.passed {
            panic!(
                "Validation failed: {:?}",
                result.errors.iter().map(|e| e.message.clone()).collect::<Vec<_>>()
            );
        }
    }

    pub fn require_validation(result: &ValidationResult) -> anyhow::Result<()> {
        if !result.passed {
            let errors: Vec<String> = result.errors.iter().map(|e| e.message.clone()).collect();
            Err(anyhow::anyhow!("validation failed: {}", errors.join("; ")))
        } else {
            Ok(())
        }
    }
}

impl Default for Validator {
    fn default() -> Self {
        Self::new()
    }
}

pub async fn validate_project() -> anyhow::Result<ValidationResult> {
    Validator::new().run().await
}

pub async fn validate_strict() -> anyhow::Result<ValidationResult> {
    Validator::new().strict().run().await
}

pub async fn validate_gate(gate: &str) -> anyhow::Result<ValidationResult> {
    Validator::new().run_gate(gate).await
}

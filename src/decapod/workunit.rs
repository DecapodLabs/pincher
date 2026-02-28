use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::process::Stdio;
use tokio::process::Command;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkUnit {
    pub id: String,
    pub task_id: String,
    pub intent_ref: String,
    pub status: WorkUnitStatus,
    pub state: WorkUnitState,
    pub acceptance_criteria: Vec<String>,
    pub constraints: Vec<String>,
    pub proofs: Vec<Proof>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum WorkUnitStatus {
    Pending,
    Active,
    Blocked,
    Completed,
    Failed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkUnitState {
    pub intent: String,
    pub plan: Option<String>,
    pub patches: Vec<Patch>,
    pub approvals: Vec<Approval>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Patch {
    pub id: String,
    pub path: String,
    pub operation: String,
    pub content: Option<String>,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Approval {
    pub approver: String,
    pub approved_at: String,
    pub scope: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Proof {
    pub id: String,
    pub proof_type: String,
    pub criteria: String,
    pub evidence: HashMap<String, serde_json::Value>,
    pub passed: bool,
    pub verified_at: Option<String>,
}

pub struct WorkUnitManager {
    binary_path: String,
    session_token: Option<String>,
}

impl WorkUnitManager {
    pub fn new() -> Self {
        Self {
            binary_path: "decapod".to_string(),
            session_token: None,
        }
    }

    pub fn with_session(mut self, token: impl Into<String>) -> Self {
        self.session_token = Some(token.into());
        self
    }

    async fn run_command(&self, args: &[&str]) -> anyhow::Result<String> {
        let mut cmd = Command::new(&self.binary_path);
        cmd.args(args)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());

        if let Some(ref token) = self.session_token {
            cmd.env("DECAPOD_SESSION_PASSWORD", token);
        }

        let output = cmd.output().await?;

        if output.status.success() {
            Ok(String::from_utf8_lossy(&output.stdout).to_string())
        } else {
            Err(anyhow::anyhow!(
                "workunit command failed: {}",
                String::from_utf8_lossy(&output.stderr)
            ))
        }
    }

    pub async fn init(&self, task_id: &str, intent_ref: &str) -> anyhow::Result<WorkUnit> {
        let args = vec!["workunit", "init", "--task-id", task_id, "--intent-ref", intent_ref];
        let output = self.run_command(&args).await?;
        
        if let Ok(wu) = serde_json::from_str(&output) {
            return Ok(wu);
        }

        Err(anyhow::anyhow!("failed to init workunit"))
    }

    pub async fn get(&self, workunit_id: &str) -> anyhow::Result<WorkUnit> {
        let args = vec!["workunit", "get", "--id", workunit_id];
        let output = self.run_command(&args).await?;
        
        if let Ok(wu) = serde_json::from_str(&output) {
            return Ok(wu);
        }

        Err(anyhow::anyhow!("failed to get workunit"))
    }

    pub async fn list(&self, task_id: Option<&str>, status: Option<&str>) -> anyhow::Result<Vec<WorkUnit>> {
        let mut args = vec!["workunit", "list"];
        
        if let Some(t) = task_id {
            args.push("--task-id");
            args.push(t);
        }
        
        if let Some(s) = status {
            args.push("--status");
            args.push(s);
        }

        let output = self.run_command(&args).await?;
        
        if let Ok(workunits) = serde_json::from_str(&output) {
            return Ok(workunits);
        }

        let mut workunits = Vec::new();
        for line in output.lines() {
            if let Ok(wu) = serde_json::from_str(line) {
                workunits.push(wu);
            }
        }

        Ok(workunits)
    }

    pub async fn update_state(&self, workunit_id: &str, intent: Option<&str>, plan: Option<&str>) -> anyhow::Result<WorkUnit> {
        let mut args = vec!["workunit", "update", "--id", workunit_id];
        
        if let Some(i) = intent {
            args.push("--intent");
            args.push(i);
        }
        
        if let Some(p) = plan {
            args.push("--plan");
            args.push(p);
        }

        let output = self.run_command(&args).await?;
        
        if let Ok(wu) = serde_json::from_str(&output) {
            return Ok(wu);
        }

        Err(anyhow::anyhow!("failed to update workunit state"))
    }

    pub async fn add_patch(&self, workunit_id: &str, path: &str, operation: &str, content: Option<&str>) -> anyhow::Result<WorkUnit> {
        let mut args = vec!["workunit", "patch", "--id", workunit_id, "--path", path, "--op", operation];
        
        if let Some(c) = content {
            args.push("--content");
            args.push(c);
        }

        let output = self.run_command(&args).await?;
        
        if let Ok(wu) = serde_json::from_str(&output) {
            return Ok(wu);
        }

        Err(anyhow::anyhow!("failed to add patch"))
    }

    pub async fn request_approval(&self, workunit_id: &str, scope: Vec<&str>) -> anyhow::Result<WorkUnit> {
        let mut args = vec!["workunit", "approve", "--id", workunit_id];
        
        for s in scope {
            args.push("--scope");
            args.push(s);
        }

        let output = self.run_command(&args).await?;
        
        if let Ok(wu) = serde_json::from_str(&output) {
            return Ok(wu);
        }

        Err(anyhow::anyhow!("failed to request approval"))
    }

    pub async fn record_proof(&self, workunit_id: &str, proof_type: &str, criteria: &str, _evidence: HashMap<String, serde_json::Value>) -> anyhow::Result<WorkUnit> {
        let args = vec!["workunit", "proof", "--id", workunit_id, "--type", proof_type, "--criteria", criteria];
        
        let output = self.run_command(&args).await?;
        
        if let Ok(wu) = serde_json::from_str(&output) {
            return Ok(wu);
        }

        Err(anyhow::anyhow!("failed to record proof"))
    }

    pub async fn complete(&self, workunit_id: &str) -> anyhow::Result<WorkUnit> {
        let args = vec!["workunit", "complete", "--id", workunit_id];
        let output = self.run_command(&args).await?;
        
        if let Ok(wu) = serde_json::from_str(&output) {
            return Ok(wu);
        }

        Err(anyhow::anyhow!("failed to complete workunit"))
    }

    pub async fn fail(&self, workunit_id: &str, reason: &str) -> anyhow::Result<WorkUnit> {
        let args = vec!["workunit", "fail", "--id", workunit_id, "--reason", reason];
        let output = self.run_command(&args).await?;
        
        if let Ok(wu) = serde_json::from_str(&output) {
            return Ok(wu);
        }

        Err(anyhow::anyhow!("failed to mark workunit as failed"))
    }
}

impl Default for WorkUnitManager {
    fn default() -> Self {
        Self::new()
    }
}

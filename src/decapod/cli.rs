use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct DecapodCli {
    pub binary_path: String,
    pub project_root: Option<String>,
}

impl DecapodCli {
    pub fn new() -> anyhow::Result<Self> {
        Ok(Self {
            binary_path: "decapod".to_string(),
            project_root: None,
        })
    }

    pub fn with_project_root(mut self, root: impl Into<String>) -> Self {
        self.project_root = Some(root.into());
        self
    }

    pub fn validate(&self) -> CommandBuilder {
        CommandBuilder::new(self.binary_path.clone()).arg("validate")
    }

    pub fn docs(&self) -> CommandBuilder {
        CommandBuilder::new(self.binary_path.clone()).arg("docs")
    }

    pub fn session(&self) -> CommandBuilder {
        CommandBuilder::new(self.binary_path.clone()).arg("session")
    }

    pub fn rpc(&self) -> CommandBuilder {
        CommandBuilder::new(self.binary_path.clone()).arg("rpc")
    }

    pub fn todo(&self) -> CommandBuilder {
        CommandBuilder::new(self.binary_path.clone()).arg("todo")
    }

    pub fn workspace(&self) -> CommandBuilder {
        CommandBuilder::new(self.binary_path.clone()).arg("workspace")
    }

    pub fn capabilities(&self) -> CommandBuilder {
        CommandBuilder::new(self.binary_path.clone()).arg("capabilities")
    }

    pub fn data(&self) -> CommandBuilder {
        CommandBuilder::new(self.binary_path.clone()).arg("data")
    }

    pub fn govern(&self) -> CommandBuilder {
        CommandBuilder::new(self.binary_path.clone()).arg("govern")
    }

    pub fn workunit(&self) -> CommandBuilder {
        CommandBuilder::new(self.binary_path.clone()).arg("workunit")
    }

    pub fn eval(&self) -> CommandBuilder {
        CommandBuilder::new(self.binary_path.clone()).arg("eval")
    }

    pub fn flight_recorder(&self) -> CommandBuilder {
        CommandBuilder::new(self.binary_path.clone()).arg("flight-recorder")
    }

    pub fn doctor(&self) -> CommandBuilder {
        CommandBuilder::new(self.binary_path.clone()).arg("doctor")
    }

    pub fn init(&self) -> CommandBuilder {
        CommandBuilder::new(self.binary_path.clone()).arg("init")
    }

    pub fn context(&self) -> CommandBuilder {
        CommandBuilder::new(self.binary_path.clone()).arg("context")
    }
}

#[derive(Debug, Clone)]
pub struct CommandBuilder {
    binary: String,
    args: Vec<String>,
}

impl CommandBuilder {
    pub fn new(binary: String) -> Self {
        Self {
            binary,
            args: Vec::new(),
        }
    }

    pub fn arg(mut self, arg: impl Into<String>) -> Self {
        self.args.push(arg.into());
        self
    }

    pub fn args(mut self, args: &[&str]) -> Self {
        for arg in args {
            self.args.push(arg.to_string());
        }
        self
    }

    pub fn build(&self) -> Vec<&str> {
        let mut cmd = vec![self.binary.as_str()];
        cmd.extend(self.args.iter().map(|s| s.as_str()));
        cmd
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecapodResponse<T: Default> {
    pub id: Option<String>,
    pub success: bool,
    #[serde(default)]
    pub receipt: Option<Receipt>,
    #[serde(default)]
    pub context_capsule: Option<ContextCapsule>,
    #[serde(default)]
    pub allowed_next_ops: Vec<String>,
    #[serde(default)]
    pub blocked_by: Vec<Interlock>,
    #[serde(default)]
    pub interlock: Option<Interlock>,
    #[serde(default)]
    pub advisory: Option<Advisory>,
    #[serde(default)]
    pub attestation: Option<Attestation>,
    #[serde(default)]
    pub data: Option<T>,
    #[serde(default)]
    pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Receipt {
    pub timestamp: String,
    pub operation: String,
    pub session_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextCapsule {
    pub scope: String,
    pub query: String,
    pub fragments: Vec<ContextFragment>,
    pub hash: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextFragment {
    pub path: String,
    pub content: String,
    pub relevance_score: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Interlock {
    pub policy: String,
    pub reason: String,
    pub blocking: bool,
    pub required_approval: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Advisory {
    pub message: String,
    pub suggestions: Vec<String>,
    pub priority: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Attestation {
    pub proof_id: String,
    pub criteria: String,
    pub passed: bool,
    pub evidence: HashMap<String, serde_json::Value>,
}

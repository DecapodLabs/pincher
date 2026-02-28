use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::process::Stdio;
use tokio::process::Command;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Capabilities {
    pub version: String,
    pub commands: Vec<CommandCapability>,
    pub plugins: Vec<PluginCapability>,
    pub rpc_operations: Vec<RpcOperation>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandCapability {
    pub name: String,
    pub alias: Option<String>,
    pub description: String,
    pub subcommands: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginCapability {
    pub name: String,
    pub enabled: bool,
    pub version: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RpcOperation {
    pub name: String,
    pub description: Option<String>,
    pub params: HashMap<String, serde_json::Value>,
}

pub struct CapabilitiesManager {
    binary_path: String,
}

impl CapabilitiesManager {
    pub fn new() -> Self {
        Self {
            binary_path: "decapod".to_string(),
        }
    }

    pub async fn discover(&self, format: Option<&str>) -> anyhow::Result<Capabilities> {
        let mut args = vec!["capabilities"];
        
        if let Some(f) = format {
            args.push("--format");
            args.push(f);
        }

        let output = Command::new(&self.binary_path)
            .args(&args)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .await?;

        if !output.status.success() {
            return Err(anyhow::anyhow!(
                "capabilities failed: {}",
                String::from_utf8_lossy(&output.stderr)
            ));
        }

        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        
        if let Ok(caps) = serde_json::from_str(&stdout) {
            return Ok(caps);
        }

        Err(anyhow::anyhow!("failed to parse capabilities"))
    }

    pub async fn discover_json(&self) -> anyhow::Result<Capabilities> {
        self.discover(Some("json")).await
    }

    pub async fn schema(&self, deterministic: bool) -> anyhow::Result<SchemaInfo> {
        let mut args = vec!["data", "schema"];
        
        if deterministic {
            args.push("--deterministic");
        }

        let output = Command::new(&self.binary_path)
            .args(&args)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .await?;

        if !output.status.success() {
            return Err(anyhow::anyhow!(
                "schema failed: {}",
                String::from_utf8_lossy(&output.stderr)
            ));
        }

        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        
        if let Ok(schema) = serde_json::from_str(&stdout) {
            return Ok(schema);
        }

        Err(anyhow::anyhow!("failed to parse schema"))
    }
}

impl Default for CapabilitiesManager {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SchemaInfo {
    pub entities: Vec<EntitySchema>,
    pub version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntitySchema {
    pub name: String,
    pub fields: Vec<FieldSchema>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FieldSchema {
    pub name: String,
    pub field_type: String,
    pub optional: bool,
}

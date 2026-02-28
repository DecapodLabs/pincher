use serde::{Deserialize, Serialize};
use std::process::Stdio;
use tokio::process::Command;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Workspace {
    pub name: String,
    pub branch: String,
    pub path: String,
    pub status: WorkspaceStatus,
    pub created_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum WorkspaceStatus {
    Active,
    Suspended,
    Archived,
}

pub struct WorkspaceManager {
    binary_path: String,
    session_token: Option<String>,
}

impl WorkspaceManager {
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
                "workspace command failed: {}",
                String::from_utf8_lossy(&output.stderr)
            ))
        }
    }

    pub async fn ensure(&self, name: Option<&str>) -> anyhow::Result<Workspace> {
        let mut args = vec!["workspace", "ensure"];
        
        if let Some(n) = name {
            args.push("--name");
            args.push(n);
        }

        let output = self.run_command(&args).await?;
        
        if let Ok(ws) = serde_json::from_str(&output) {
            return Ok(ws);
        }

        Err(anyhow::anyhow!("failed to ensure workspace"))
    }

    pub async fn status(&self) -> anyhow::Result<WorkspaceStatusResponse> {
        let args = vec!["workspace", "status"];
        let output = self.run_command(&args).await?;
        
        if let Ok(status) = serde_json::from_str(&output) {
            return Ok(status);
        }

        Ok(WorkspaceStatusResponse {
            current: None,
            available: Vec::new(),
            default_branch: "main".to_string(),
        })
    }

    pub async fn list(&self) -> anyhow::Result<Vec<Workspace>> {
        let args = vec!["workspace", "list"];
        let output = self.run_command(&args).await?;
        
        if let Ok(workspaces) = serde_json::from_str(&output) {
            return Ok(workspaces);
        }

        let mut workspaces = Vec::new();
        for line in output.lines() {
            if let Ok(ws) = serde_json::from_str(line) {
                workspaces.push(ws);
            }
        }

        Ok(workspaces)
    }

    pub async fn enter(&self, name: &str) -> anyhow::Result<Workspace> {
        let args = vec!["workspace", "enter", "--name", name];
        let output = self.run_command(&args).await?;
        
        if let Ok(ws) = serde_json::from_str(&output) {
            return Ok(ws);
        }

        Err(anyhow::anyhow!("failed to enter workspace"))
    }

    pub async fn suspend(&self, name: &str) -> anyhow::Result<()> {
        let args = vec!["workspace", "suspend", "--name", name];
        self.run_command(&args).await?;
        Ok(())
    }

    pub async fn archive(&self, name: &str) -> anyhow::Result<()> {
        let args = vec!["workspace", "archive", "--name", name];
        self.run_command(&args).await?;
        Ok(())
    }

    pub async fn delete(&self, name: &str, force: bool) -> anyhow::Result<()> {
        let mut args = vec!["workspace", "delete", "--name", name];
        
        if force {
            args.push("--force");
        }

        self.run_command(&args).await?;
        Ok(())
    }

    pub async fn get_path(&self, name: &str) -> anyhow::Result<String> {
        let args = vec!["workspace", "path", "--name", name];
        let output = self.run_command(&args).await?;
        
        Ok(output.trim().to_string())
    }
}

impl Default for WorkspaceManager {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkspaceStatusResponse {
    pub current: Option<Workspace>,
    pub available: Vec<Workspace>,
    pub default_branch: String,
}

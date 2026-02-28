use serde::{Deserialize, Serialize};
use std::process::Stdio;
use tokio::process::Command;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: String,
    pub content: String,
    pub status: TaskStatus,
    pub priority: Option<String>,
    pub owner: Option<String>,
    pub created_at: String,
    pub updated_at: String,
    pub claimed_at: Option<String>,
    pub completed_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[serde(rename_all = "lowercase")]
pub enum TaskStatus {
    #[default]
    Pending,
    Claimed,
    InProgress,
    Completed,
    Cancelled,
}

pub struct TodoManager {
    binary_path: String,
    session_token: Option<String>,
}

impl TodoManager {
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
                "todo command failed: {}",
                String::from_utf8_lossy(&output.stderr)
            ))
        }
    }

    async fn run_command_strings(&self, args: &[String]) -> anyhow::Result<String> {
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
                "todo command failed: {}",
                String::from_utf8_lossy(&output.stderr)
            ))
        }
    }

    pub async fn add(&self, content: &str, priority: Option<&str>, tags: Option<Vec<&str>>) -> anyhow::Result<Task> {
        let mut args = vec!["todo", "add", content];
        
        if let Some(p) = priority {
            args.push("--priority");
            args.push(p);
        }
        
        if let Some(ref tags) = tags {
            for tag in tags {
                args.push("--tag");
                args.push(tag);
            }
        }

        let output = self.run_command(&args).await?;
        
        if let Ok(task) = serde_json::from_str(&output) {
            return Ok(task);
        }

        for line in output.lines() {
            if let Ok(task) = serde_json::from_str(line) {
                return Ok(task);
            }
        }

        Err(anyhow::anyhow!("failed to parse task response"))
    }

    pub async fn claim(&self, task_id: &str) -> anyhow::Result<Task> {
        let args = vec!["todo", "claim", "--id", task_id];
        let output = self.run_command(&args).await?;
        
        if let Ok(task) = serde_json::from_str(&output) {
            return Ok(task);
        }

        for line in output.lines() {
            if let Ok(task) = serde_json::from_str(line) {
                return Ok(task);
            }
        }

        Err(anyhow::anyhow!("failed to claim task"))
    }

    pub async fn release(&self, task_id: &str) -> anyhow::Result<Task> {
        let args = vec!["todo", "release", "--id", task_id];
        let output = self.run_command(&args).await?;
        
        if let Ok(task) = serde_json::from_str(&output) {
            return Ok(task);
        }

        Err(anyhow::anyhow!("failed to release task"))
    }

    pub async fn complete(&self, task_id: &str, resolution: Option<&str>) -> anyhow::Result<Task> {
        let mut args = vec!["todo", "complete", "--id", task_id];
        
        if let Some(r) = resolution {
            args.push("--resolution");
            args.push(r);
        }

        let output = self.run_command(&args).await?;
        
        if let Ok(task) = serde_json::from_str(&output) {
            return Ok(task);
        }

        Err(anyhow::anyhow!("failed to complete task"))
    }

    pub async fn list(&self, status: Option<&str>, owner: Option<&str>, limit: Option<usize>) -> anyhow::Result<Vec<Task>> {
        let mut args: Vec<String> = vec!["todo".to_string(), "list".to_string()];
        
        if let Some(s) = status {
            args.push("--status".to_string());
            args.push(s.to_string());
        }
        
        if let Some(o) = owner {
            args.push("--owner".to_string());
            args.push(o.to_string());
        }

        if let Some(l) = limit {
            args.push("--limit".to_string());
            args.push(l.to_string());
        }

        let output = self.run_command_strings(&args).await?;
        
        if let Ok(tasks) = serde_json::from_str(&output) {
            return Ok(tasks);
        }

        let mut tasks = Vec::new();
        for line in output.lines() {
            if let Ok(task) = serde_json::from_str(line) {
                tasks.push(task);
            }
        }

        Ok(tasks)
    }

    pub async fn get(&self, task_id: &str) -> anyhow::Result<Task> {
        let args = vec!["todo", "get", "--id", task_id];
        let output = self.run_command(&args).await?;
        
        if let Ok(task) = serde_json::from_str(&output) {
            return Ok(task);
        }

        Err(anyhow::anyhow!("failed to get task"))
    }

    pub async fn handoff(&self, task_id: &str, to_agent: &str) -> anyhow::Result<Task> {
        let args = vec!["todo", "handoff", "--id", task_id, "--to", to_agent];
        let output = self.run_command(&args).await?;
        
        if let Ok(task) = serde_json::from_str(&output) {
            return Ok(task);
        }

        Err(anyhow::anyhow!("failed to handoff task"))
    }

    pub async fn update(&self, task_id: &str, content: Option<&str>, priority: Option<&str>) -> anyhow::Result<Task> {
        let mut args = vec!["todo", "update", "--id", task_id];
        
        if let Some(c) = content {
            args.push("--content");
            args.push(c);
        }
        
        if let Some(p) = priority {
            args.push("--priority");
            args.push(p);
        }

        let output = self.run_command(&args).await?;
        
        if let Ok(task) = serde_json::from_str(&output) {
            return Ok(task);
        }

        Err(anyhow::anyhow!("failed to update task"))
    }

    pub async fn archive(&self, task_id: &str) -> anyhow::Result<()> {
        let args = vec!["todo", "archive", "--id", task_id];
        self.run_command(&args).await?;
        Ok(())
    }

    pub async fn blocks(&self, task_id: &str, blocked_by: Vec<&str>) -> anyhow::Result<Task> {
        let mut args = vec!["todo", "blocks", "--id", task_id];
        
        for blocked in blocked_by {
            args.push("--blocked-by");
            args.push(blocked);
        }

        let output = self.run_command(&args).await?;
        
        if let Ok(task) = serde_json::from_str(&output) {
            return Ok(task);
        }

        Err(anyhow::anyhow!("failed to set blocks"))
    }
}

impl Default for TodoManager {
    fn default() -> Self {
        Self::new()
    }
}

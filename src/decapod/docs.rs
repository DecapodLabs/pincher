use serde::{Deserialize, Serialize};
use std::process::Stdio;
use tokio::process::Command;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocFragment {
    pub path: String,
    pub content: String,
    pub relevance_score: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocSearchResult {
    pub fragments: Vec<DocFragment>,
    pub total_matches: usize,
    pub query: String,
}

pub struct DocsManager {
    binary_path: String,
}

impl DocsManager {
    pub fn new() -> Self {
        Self {
            binary_path: "decapod".to_string(),
        }
    }

    pub async fn ingest(&self) -> anyhow::Result<IngestResult> {
        let output = Command::new(&self.binary_path)
            .args(["docs", "ingest"])
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .await?;

        if !output.status.success() {
            return Err(anyhow::anyhow!(
                "docs ingest failed: {}",
                String::from_utf8_lossy(&output.stderr)
            ));
        }

        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        
        if let Ok(result) = serde_json::from_str(&stdout) {
            return Ok(result);
        }

        Ok(IngestResult {
            success: true,
            documents_ingested: 0,
            errors: Vec::new(),
        })
    }

    pub async fn show(&self, path: &str) -> anyhow::Result<String> {
        let output = Command::new(&self.binary_path)
            .args(["docs", "show", path])
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .await?;

        if !output.status.success() {
            return Err(anyhow::anyhow!(
                "docs show failed: {}",
                String::from_utf8_lossy(&output.stderr)
            ));
        }

        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }

    pub async fn search(&self, query: &str, op: Option<&str>, path: Option<&str>, tag: Option<&str>) -> anyhow::Result<DocSearchResult> {
        let mut args = vec!["docs", "search", "--query", query];
        
        if let Some(op) = op {
            args.push("--op");
            args.push(op);
        }
        if let Some(path) = path {
            args.push("--path");
            args.push(path);
        }
        if let Some(tag) = tag {
            args.push("--tag");
            args.push(tag);
        }

        let output = Command::new(&self.binary_path)
            .args(&args)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .await?;

        if !output.status.success() {
            return Err(anyhow::anyhow!(
                "docs search failed: {}",
                String::from_utf8_lossy(&output.stderr)
            ));
        }

        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        
        if let Ok(result) = serde_json::from_str(&stdout) {
            return Ok(result);
        }

        Ok(DocSearchResult {
            fragments: Vec::new(),
            total_matches: 0,
            query: query.to_string(),
        })
    }

    pub async fn list(&self, path: Option<&str>) -> anyhow::Result<Vec<DocEntry>> {
        let mut args = vec!["docs", "list"];
        
        if let Some(path) = path {
            args.push(path);
        }

        let output = Command::new(&self.binary_path)
            .args(&args)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .await?;

        if !output.status.success() {
            return Err(anyhow::anyhow!(
                "docs list failed: {}",
                String::from_utf8_lossy(&output.stderr)
            ));
        }

        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        
        if let Ok(result) = serde_json::from_str(&stdout) {
            return Ok(result);
        }

        Ok(Vec::new())
    }
}

impl Default for DocsManager {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IngestResult {
    pub success: bool,
    pub documents_ingested: usize,
    pub errors: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocEntry {
    pub path: String,
    pub title: Option<String>,
    pub modified: Option<String>,
}

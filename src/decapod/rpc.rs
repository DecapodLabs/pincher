use serde::Deserialize;
use serde_json::{json, Value};
use std::process::Stdio;
use tokio::process::Command;
use crate::decapod::cli::{DecapodResponse, Interlock, Advisory, Attestation, ContextCapsule};

#[derive(Debug, Clone)]
pub struct RpcClient {
    session_token: Option<String>,
}

impl RpcClient {
    pub fn new() -> Self {
        Self {
            session_token: None,
        }
    }

    pub fn with_session(mut self, token: impl Into<String>) -> Self {
        self.session_token = Some(token.into());
        self
    }

    pub fn session_token(&self) -> Option<&str> {
        self.session_token.as_deref()
    }

    pub async fn call<T: for<'de> Deserialize<'de> + Default>(
        &self,
        operation: &str,
        params: Option<Value>,
    ) -> anyhow::Result<DecapodResponse<T>> {
        let mut cmd = Command::new("decapod");
        cmd.arg("rpc")
            .arg("--op")
            .arg(operation);

        if let Some(token) = &self.session_token {
            cmd.env("DECAPOD_SESSION_PASSWORD", token);
        }

        if let Some(p) = params {
            let params_str = serde_json::to_string(&p)?;
            cmd.arg("--params").arg(params_str);
        }

        let output = cmd
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .await?;

        if !output.status.success() {
            return Err(anyhow::anyhow!(
                "RPC call failed: {}",
                String::from_utf8_lossy(&output.stderr)
            ));
        }

        let stdout = String::from_utf8_lossy(&output.stdout);
        
        if let Ok(response) = serde_json::from_str(&stdout) {
            return Ok(response);
        }

        for line in stdout.lines() {
            if let Ok(response) = serde_json::from_str(line) {
                return Ok(response);
            }
        }

        Err(anyhow::anyhow!("failed to parse RPC response"))
    }

    pub async fn agent_init(&self, agent_id: &str) -> anyhow::Result<RpcResponse> {
        self.call("agent.init", Some(json!({ "agent_id": agent_id })))
            .await
            .map(|r| r.into())
    }

    pub async fn context_resolve(&self, scopes: Vec<&str>) -> anyhow::Result<RpcResponse> {
        self.call("context.resolve", Some(json!({ "scopes": scopes })))
            .await
            .map(|r| r.into())
    }

    pub async fn context_scope(&self, query: &str, limit: Option<usize>) -> anyhow::Result<RpcResponse> {
        let mut params = json!({ "query": query });
        if let Some(l) = limit {
            params["limit"] = json!(l);
        }
        self.call("context.scope", Some(params))
            .await
            .map(|r| r.into())
    }

    pub async fn store_upsert(&self, entity_type: &str, key: &str, value: Value) -> anyhow::Result<RpcResponse> {
        self.call("store.upsert", Some(json!({
            "entity_type": entity_type,
            "key": key,
            "value": value
        })))
        .await
        .map(|r| r.into())
    }

    pub async fn store_query(&self, entity_type: &str, query: Value) -> anyhow::Result<RpcResponse> {
        self.call("store.query", Some(json!({
            "entity_type": entity_type,
            "query": query
        })))
        .await
        .map(|r| r.into())
    }

    pub async fn validate_run(&self) -> anyhow::Result<RpcResponse> {
        self.call("validate.run", None)
            .await
            .map(|r| r.into())
    }

    pub async fn workspace_ensure(&self) -> anyhow::Result<RpcResponse> {
        self.call("workspace.ensure", None)
            .await
            .map(|r| r.into())
    }

    pub async fn workspace_status(&self) -> anyhow::Result<RpcResponse> {
        self.call("workspace.status", None)
            .await
            .map(|r| r.into())
    }

    pub async fn todo_add(&self, content: &str, priority: Option<&str>) -> anyhow::Result<RpcResponse> {
        let mut params = json!({ "content": content });
        if let Some(p) = priority {
            params["priority"] = json!(p);
        }
        self.call("todo.add", Some(params))
            .await
            .map(|r| r.into())
    }

    pub async fn todo_claim(&self, task_id: &str) -> anyhow::Result<RpcResponse> {
        self.call("todo.claim", Some(json!({ "task_id": task_id })))
            .await
            .map(|r| r.into())
    }

    pub async fn todo_list(&self, status: Option<&str>) -> anyhow::Result<RpcResponse> {
        let params = status.map(|s| json!({ "status": s }));
        self.call("todo.list", params)
            .await
            .map(|r| r.into())
    }

    pub async fn workunit_init(&self, task_id: &str, intent_ref: &str) -> anyhow::Result<RpcResponse> {
        self.call("workunit.init", Some(json!({
            "task_id": task_id,
            "intent_ref": intent_ref
        })))
        .await
        .map(|r| r.into())
    }

    pub async fn capsule_query(&self, topic: &str, scope: &str, task_id: Option<&str>) -> anyhow::Result<RpcResponse> {
        let mut params = json!({ "topic": topic, "scope": scope });
        if let Some(t) = task_id {
            params["task_id"] = json!(t);
        }
        self.call("capsule.query", Some(params))
            .await
            .map(|r| r.into())
    }
}

impl Default for RpcClient {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
pub struct RpcResponse {
    pub success: bool,
    pub id: Option<String>,
    pub receipt: Option<crate::decapod::cli::Receipt>,
    pub context_capsule: Option<ContextCapsule>,
    pub allowed_next_ops: Vec<String>,
    pub blocked_by: Vec<Interlock>,
    pub interlock: Option<Interlock>,
    pub advisory: Option<Advisory>,
    pub attestation: Option<Attestation>,
    pub error: Option<String>,
}

impl From<DecapodResponse<Value>> for RpcResponse {
    fn from(r: DecapodResponse<Value>) -> Self {
        Self {
            success: r.success,
            id: r.id,
            receipt: r.receipt,
            context_capsule: r.context_capsule,
            allowed_next_ops: r.allowed_next_ops,
            blocked_by: r.blocked_by,
            interlock: r.interlock,
            advisory: r.advisory,
            attestation: r.attestation,
            error: r.error,
        }
    }
}

impl RpcResponse {
    pub fn is_blocked(&self) -> bool {
        !self.blocked_by.is_empty() || self.interlock.as_ref().map(|i| i.blocking).unwrap_or(false)
    }

    pub fn get_blocking_policies(&self) -> Vec<&Interlock> {
        self.blocked_by.iter()
            .filter(|i| i.blocking)
            .collect()
    }

    pub fn has_advisory(&self) -> bool {
        self.advisory.is_some()
    }

    pub fn has_attestation(&self) -> bool {
        self.attestation.is_some()
    }

    pub fn is_attested(&self) -> bool {
        self.attestation.as_ref().map(|a| a.passed).unwrap_or(false)
    }
}

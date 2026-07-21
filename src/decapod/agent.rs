//! Deprecated pre-contract agent facade.
//!
//! Hosts must use [`crate::governed_run::GovernedRunEngine`].  The former
//! `execute(&str)` API was intentionally disabled because an intent string is
//! not sufficient custody, context, validation, or proof.

#![allow(deprecated)]

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[deprecated(note = "construct a governed_run::RunRequest instead")]
pub struct AgentConfig {
    pub agent_id: String,
    pub agent_type: AgentKind,
    pub model: ModelConfig,
    pub governance: GovernanceConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[deprecated(note = "provider configuration is deferred; use governed_run::ProviderTurn")]
pub struct ModelConfig {
    pub provider: String,
    pub model: String,
    /// Secret material is never part of a serialized host contract.
    #[serde(skip_serializing, skip_deserializing)]
    pub api_key: Option<String>,
    pub base_url: Option<String>,
    pub max_tokens: Option<usize>,
    pub temperature: Option<f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[deprecated(note = "governed_run owns mandatory control-plane sequencing")]
pub struct GovernanceConfig {
    pub validate_before_prompt: bool,
    pub validate_after_response: bool,
    pub require_context_resolution: bool,
    pub enforce_workspace_isolation: bool,
    pub block_on_interlock: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[serde(rename_all = "lowercase")]
#[deprecated(note = "agent kind is not part of the governed-run contract")]
pub enum AgentKind {
    #[default]
    Coordinator,
    Executor,
    Researcher,
    Reviewer,
    Specialist,
    #[serde(other)]
    Generic,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[deprecated(note = "use governed_run::GovernedInferenceRequest")]
pub struct PromptContext {
    pub intent: String,
    pub scope: Vec<String>,
    pub task_id: Option<String>,
    pub workunit_id: Option<String>,
    pub resolved_context: Vec<ContextFragment>,
    pub governance_state: GovernanceState,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[deprecated(note = "raw context is not a host-facing event or snapshot field")]
pub struct ContextFragment {
    pub source: String,
    pub content: String,
    pub relevance_score: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[deprecated(note = "use typed governed_run evidence references")]
pub struct GovernanceState {
    pub interlock: bool,
    pub advisories: Vec<String>,
    pub blocked: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[deprecated(note = "use governed_run::RunOutcome")]
pub struct AgentResponse {
    pub response: String,
    pub tool_calls: Vec<ToolCall>,
    pub governance: GovernanceState,
    pub context_used: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[deprecated(note = "tool execution is deferred from governed-run v1")]
pub struct ToolCall {
    pub tool: String,
    pub arguments: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone)]
#[deprecated(note = "use governed_run::GovernedRunEngine")]
pub struct AgentEngine {
    config: AgentConfig,
    session: Option<crate::decapod::session::Session>,
}

impl AgentEngine {
    pub fn new(config: AgentConfig) -> Self {
        Self {
            config,
            session: None,
        }
    }

    pub async fn initialize(
        &mut self,
        session: crate::decapod::session::Session,
    ) -> anyhow::Result<()> {
        self.session = Some(session);
        Ok(())
    }

    /// Disabled compatibility boundary: the former API could infer without
    /// explicit custody and authoritative context, validation, and proof.
    pub async fn execute(&self, _intent: &str) -> anyhow::Result<AgentResponse> {
        Err(anyhow::anyhow!(
            "legacy AgentEngine::execute is disabled; use GovernedRunEngine::run with a v1 RunRequest"
        ))
    }

    pub fn agent_id(&self) -> &str {
        &self.config.agent_id
    }

    pub fn agent_kind(&self) -> &AgentKind {
        &self.config.agent_type
    }
}

pub mod prelude {
    pub use super::{
        AgentConfig, AgentEngine, AgentKind, AgentResponse, ContextFragment, GovernanceConfig,
        GovernanceState, ModelConfig, PromptContext, ToolCall,
    };
}

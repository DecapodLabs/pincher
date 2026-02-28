use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub mod model;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentConfig {
    pub agent_id: String,
    pub agent_type: AgentKind,
    pub model: ModelConfig,
    pub governance: GovernanceConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelConfig {
    pub provider: String,
    pub model: String,
    pub api_key: Option<String>,
    pub base_url: Option<String>,
    pub max_tokens: Option<usize>,
    pub temperature: Option<f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GovernanceConfig {
    pub validate_before_prompt: bool,
    pub validate_after_response: bool,
    pub require_context_resolution: bool,
    pub enforce_workspace_isolation: bool,
    pub block_on_interlock: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[serde(rename_all = "lowercase")]
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
pub struct PromptContext {
    pub intent: String,
    pub scope: Vec<String>,
    pub task_id: Option<String>,
    pub workunit_id: Option<String>,
    pub resolved_context: Vec<ContextFragment>,
    pub governance_state: GovernanceState,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextFragment {
    pub source: String,
    pub content: String,
    pub relevance_score: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GovernanceState {
    pub validated: bool,
    pub interlock: bool,
    pub advisories: Vec<String>,
    pub attested: bool,
    pub blocked: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentResponse {
    pub response: String,
    pub tool_calls: Vec<ToolCall>,
    pub governance: GovernanceState,
    pub context_used: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolCall {
    pub tool: String,
    pub arguments: HashMap<String, serde_json::Value>,
}

pub struct AgentEngine {
    config: AgentConfig,
    session: Option<crate::decapod::session::Session>,
    context_cache: HashMap<String, Vec<ContextFragment>>,
}

impl AgentEngine {
    pub fn new(config: AgentConfig) -> Self {
        Self {
            config,
            session: None,
            context_cache: HashMap::new(),
        }
    }

    pub async fn initialize(&mut self, session: crate::decapod::session::Session) -> anyhow::Result<()> {
        self.session = Some(session.clone());
        
        let rpc = crate::decapod::rpc::RpcClient::new()
            .with_session(session.token().to_string());
        
        rpc.agent_init(&self.config.agent_id).await?;
        
        if self.config.governance.require_context_resolution {
            self.preload_context(&rpc).await?;
        }
        
        tracing::info!("Agent {} initialized", self.config.agent_id);
        Ok(())
    }

    async fn preload_context(&mut self, rpc: &crate::decapod::rpc::RpcClient) -> anyhow::Result<()> {
        let scopes = vec!["core", "interfaces", "methodology"];
        
        for scope in scopes {
            let response = rpc.context_scope(scope, Some(5)).await?;
            
            if let Some(capsule) = response.context_capsule {
                self.context_cache.insert(
                    scope.to_string(),
                    capsule.fragments.into_iter().map(|f| {
                        ContextFragment {
                            source: f.path,
                            content: f.content,
                            relevance_score: f.relevance_score,
                        }
                    }).collect()
                );
            }
        }
        
        Ok(())
    }

    pub async fn execute(&self, intent: &str) -> anyhow::Result<AgentResponse> {
        let session = self.session.as_ref()
            .ok_or_else(|| anyhow::anyhow!("agent not initialized"))?;

        let rpc = crate::decapod::rpc::RpcClient::new()
            .with_session(session.token().to_string());

        let governance_state = if self.config.governance.validate_before_prompt {
            self.governance_check(&rpc, intent).await?
        } else {
            GovernanceState::default()
        };

        let resolved_context = self.resolve_context(&rpc, intent).await?;

        let prompt = self.build_governed_prompt(intent, &resolved_context, &governance_state);

        let model_response = self.call_model(&prompt).await?;

        let final_governance = if self.config.governance.validate_after_response {
            self.governance_check(&rpc, &model_response).await?
        } else {
            governance_state
        };

        Ok(AgentResponse {
            response: model_response,
            tool_calls: Vec::new(),
            governance: final_governance,
            context_used: resolved_context.iter().map(|c| c.source.clone()).collect(),
        })
    }

    async fn governance_check(&self, rpc: &crate::decapod::rpc::RpcClient, input: &str) -> anyhow::Result<GovernanceState> {
        let scope_response = rpc.context_scope(input, Some(8)).await?;

        let mut state = GovernanceState {
            validated: true,
            ..Default::default()
        };

        if let Some(ref interlock) = scope_response.interlock {
            state.interlock = interlock.blocking;
            state.blocked = interlock.blocking;
        }

        if let Some(ref advisory) = scope_response.advisory {
            state.advisories = vec![advisory.message.clone()];
        }

        state.attested = scope_response.attestation
            .as_ref()
            .map(|a| a.passed)
            .unwrap_or(false);

        if self.config.governance.block_on_interlock && state.blocked {
            return Err(anyhow::anyhow!(
                "BLOCKED: governance interlock - {:?}",
                state.advisories
            ));
        }

        Ok(state)
    }

    async fn resolve_context(&self, rpc: &crate::decapod::rpc::RpcClient, intent: &str) -> anyhow::Result<Vec<ContextFragment>> {
        let response = rpc.context_scope(intent, Some(10)).await?;

        if let Some(capsule) = response.context_capsule {
            Ok(capsule.fragments.into_iter().map(|f| {
                ContextFragment {
                    source: f.path,
                    content: f.content,
                    relevance_score: f.relevance_score,
                }
            }).collect())
        } else {
            Ok(Vec::new())
        }
    }

    fn build_governed_prompt(&self, intent: &str, context: &[ContextFragment], governance: &GovernanceState) -> String {
        let mut prompt = String::new();
        
        prompt.push_str("## Governance Context\n");
        prompt.push_str(&format!("- Validated: {}\n", governance.validated));
        
        if governance.attested {
            prompt.push_str("- Attestation: PASSED\n");
        }
        
        if !governance.advisories.is_empty() {
            prompt.push_str("- Advisories:\n");
            for adv in &governance.advisories {
                prompt.push_str(&format!("  - {}\n", adv));
            }
        }
        
        if governance.blocked {
            prompt.push_str("- ⚠️ BLOCKED: Cannot proceed without approval\n");
        }
        
        prompt.push_str("\n## Relevant Context\n");
        for fragment in context.iter().take(5) {
            prompt.push_str(&format!("\n### {}\n{}\n", fragment.source, fragment.content));
        }
        
        prompt.push_str(&format!("\n## Intent\n{}\n", intent));
        
        prompt
    }

    async fn call_model(&self, prompt: &str) -> anyhow::Result<String> {
        tracing::debug!("Calling model with governed prompt ({} chars)", prompt.len());
        
        Ok(format!("[Model response would go here - prompt was {} chars]", prompt.len()))
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
        AgentConfig, AgentEngine, AgentKind, AgentResponse, ContextFragment,
        GovernanceConfig, GovernanceState, ModelConfig, PromptContext, ToolCall,
    };
}

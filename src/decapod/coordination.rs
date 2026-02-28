use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Agent {
    pub id: String,
    pub agent_type: AgentType,
    pub status: AgentStatus,
    pub capabilities: Vec<String>,
    pub current_task: Option<String>,
    pub workspace: Option<String>,
    pub metadata: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum AgentType {
    Coordinator,
    Executor,
    Reviewer,
    Researcher,
    Specialist,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum AgentStatus {
    Idle,
    Active,
    Blocked,
    Waiting,
    Completed,
    Failed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentMessage {
    pub id: String,
    pub from_agent: String,
    pub to_agent: String,
    pub message_type: MessageType,
    pub payload: HashMap<String, serde_json::Value>,
    pub timestamp: String,
    pub correlation_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MessageType {
    Request,
    Response,
    Delegate,
    Coordinate,
    StatusUpdate,
    Blocked,
    Unblocked,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoordinationPlan {
    pub id: String,
    pub root_agent: String,
    pub sub_agents: Vec<SubAgentPlan>,
    pub dependencies: Vec<Dependency>,
    pub execution_order: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubAgentPlan {
    pub agent_id: String,
    pub agent_type: AgentType,
    pub task: String,
    pub scope: Vec<String>,
    pub constraints: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dependency {
    pub from: String,
    pub to: String,
    pub dependency_type: DependencyType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DependencyType {
    Requires,
    Blocks,
    Conflicts,
    Coordinates,
}

pub struct CoordinationManager {
    self_agent_id: String,
    self_agent_type: AgentType,
}

impl CoordinationManager {
    pub fn new(agent_id: &str, agent_type: AgentType) -> Self {
        Self {
            self_agent_id: agent_id.to_string(),
            self_agent_type: agent_type,
        }
    }

    pub fn create_coordination_plan(&self, tasks: Vec<(&str, &str)>) -> CoordinationPlan {
        let plan_id = ulid::Ulid::new().to_string();
        let mut sub_agents = Vec::new();
        let mut execution_order = Vec::new();

        for (i, (task, agent_type)) in tasks.iter().enumerate() {
            let agent_id = format!("{}-sub-{}", plan_id, i);
            let agent_type = match *agent_type {
                "coordinator" => AgentType::Coordinator,
                "executor" => AgentType::Executor,
                "reviewer" => AgentType::Reviewer,
                "researcher" => AgentType::Researcher,
                _ => AgentType::Specialist,
            };

            sub_agents.push(SubAgentPlan {
                agent_id: agent_id.clone(),
                agent_type,
                task: task.to_string(),
                scope: Vec::new(),
                constraints: Vec::new(),
            });

            execution_order.push(agent_id);
        }

        CoordinationPlan {
            id: plan_id,
            root_agent: self.self_agent_id.clone(),
            sub_agents,
            dependencies: Vec::new(),
            execution_order,
        }
    }

    pub fn delegate_task(
        &self,
        to_agent: &str,
        task: &str,
        context: HashMap<String, serde_json::Value>,
    ) -> AgentMessage {
        AgentMessage {
            id: ulid::Ulid::new().to_string(),
            from_agent: self.self_agent_id.clone(),
            to_agent: to_agent.to_string(),
            message_type: MessageType::Delegate,
            payload: {
                let mut p = context;
                p.insert("task".to_string(), serde_json::json!(task));
                p
            },
            timestamp: chrono::Utc::now().to_rfc3339(),
            correlation_id: None,
        }
    }

    pub fn request_coordination(&self, with_agent: &str, topic: &str) -> AgentMessage {
        AgentMessage {
            id: ulid::Ulid::new().to_string(),
            from_agent: self.self_agent_id.clone(),
            to_agent: with_agent.to_string(),
            message_type: MessageType::Coordinate,
            payload: {
                let mut p = HashMap::new();
                p.insert("topic".to_string(), serde_json::json!(topic));
                p
            },
            timestamp: chrono::Utc::now().to_rfc3339(),
            correlation_id: None,
        }
    }

    pub fn send_status_update(
        &self,
        to_agent: &str,
        status: &AgentStatus,
        task_id: Option<&str>,
    ) -> AgentMessage {
        AgentMessage {
            id: ulid::Ulid::new().to_string(),
            from_agent: self.self_agent_id.clone(),
            to_agent: to_agent.to_string(),
            message_type: MessageType::StatusUpdate,
            payload: {
                let mut p = HashMap::new();
                p.insert("status".to_string(), serde_json::json!(status));
                if let Some(t) = task_id {
                    p.insert("task_id".to_string(), serde_json::json!(t));
                }
                p
            },
            timestamp: chrono::Utc::now().to_rfc3339(),
            correlation_id: None,
        }
    }

    pub fn signal_blocked(
        &self,
        to_agent: &str,
        reason: &str,
        blocked_by: Option<&str>,
    ) -> AgentMessage {
        AgentMessage {
            id: ulid::Ulid::new().to_string(),
            from_agent: self.self_agent_id.clone(),
            to_agent: to_agent.to_string(),
            message_type: MessageType::Blocked,
            payload: {
                let mut p = HashMap::new();
                p.insert("reason".to_string(), serde_json::json!(reason));
                if let Some(b) = blocked_by {
                    p.insert("blocked_by".to_string(), serde_json::json!(b));
                }
                p
            },
            timestamp: chrono::Utc::now().to_rfc3339(),
            correlation_id: None,
        }
    }

    pub fn signal_unblocked(&self, to_agent: &str, resolution: &str) -> AgentMessage {
        AgentMessage {
            id: ulid::Ulid::new().to_string(),
            from_agent: self.self_agent_id.clone(),
            to_agent: to_agent.to_string(),
            message_type: MessageType::Unblocked,
            payload: {
                let mut p = HashMap::new();
                p.insert("resolution".to_string(), serde_json::json!(resolution));
                p
            },
            timestamp: chrono::Utc::now().to_rfc3339(),
            correlation_id: None,
        }
    }

    pub fn is_coordinator(&self) -> bool {
        matches!(self.self_agent_type, AgentType::Coordinator)
    }

    pub fn can_delegate(&self) -> bool {
        matches!(
            self.self_agent_type,
            AgentType::Coordinator | AgentType::Executor
        )
    }
}

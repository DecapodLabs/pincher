use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
    pub id: String,
    pub event_type: EventType,
    pub timestamp: DateTime<Utc>,
    pub source: EventSource,
    pub payload: HashMap<String, serde_json::Value>,
    pub session_id: Option<String>,
    pub task_id: Option<String>,
    pub workunit_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EventType {
    AgentStarted,
    AgentStopped,
    TaskCreated,
    TaskClaimed,
    TaskCompleted,
    TaskFailed,
    WorkUnitCreated,
    WorkUnitUpdated,
    WorkUnitCompleted,
    WorkUnitFailed,
    PatchProposed,
    PatchApplied,
    ApprovalRequested,
    ApprovalGranted,
    ApprovalDenied,
    ProofRecorded,
    ValidationPassed,
    ValidationFailed,
    InterlockEncountered,
    AdvisoryIssued,
    AttestationRecorded,
    ContextResolved,
    WorkspaceEntered,
    WorkspaceLeft,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventSource {
    pub agent_id: String,
    pub agent_type: String,
    pub host: String,
    pub version: String,
}

impl Event {
    pub fn new(agent_id: &str, agent_type: &str, event_type: EventType) -> Self {
        Self {
            id: ulid::Ulid::new().to_string(),
            event_type,
            timestamp: Utc::now(),
            source: EventSource {
                agent_id: agent_id.to_string(),
                agent_type: agent_type.to_string(),
                host: hostname::get()
                    .map(|h| h.to_string_lossy().to_string())
                    .unwrap_or_else(|_| "unknown".to_string()),
                version: env!("CARGO_PKG_VERSION").to_string(),
            },
            payload: HashMap::new(),
            session_id: None,
            task_id: None,
            workunit_id: None,
        }
    }

    pub fn with_payload(mut self, key: impl Into<String>, value: serde_json::Value) -> Self {
        self.payload.insert(key.into(), value);
        self
    }

    pub fn with_session(mut self, session_id: impl Into<String>) -> Self {
        self.session_id = Some(session_id.into());
        self
    }

    pub fn with_task(mut self, task_id: impl Into<String>) -> Self {
        self.task_id = Some(task_id.into());
        self
    }

    pub fn with_workunit(mut self, workunit_id: impl Into<String>) -> Self {
        self.workunit_id = Some(workunit_id.into());
        self
    }

    pub fn emit(&self) -> anyhow::Result<String> {
        let json = serde_json::to_string(self)?;
        println!("{}", json);
        Ok(json)
    }
}

pub struct EventEmitter {
    agent_id: String,
    agent_type: String,
    session_id: Option<String>,
}

impl EventEmitter {
    pub fn new(agent_id: &str, agent_type: &str) -> Self {
        Self {
            agent_id: agent_id.to_string(),
            agent_type: agent_type.to_string(),
            session_id: None,
        }
    }

    pub fn with_session(mut self, session_id: impl Into<String>) -> Self {
        self.session_id = Some(session_id.into());
        self
    }

    pub fn emit(&self, event_type: EventType) -> EventBuilder {
        EventBuilder::new(&self.agent_id, &self.agent_type, event_type)
            .with_session(self.session_id.as_deref().unwrap_or(""))
    }

    pub fn agent_started(&self) -> Event {
        self.emit(EventType::AgentStarted)
            .with_payload("timestamp", serde_json::json!(Utc::now().to_rfc3339()))
            .build()
    }

    pub fn agent_stopped(&self, reason: Option<&str>) -> Event {
        let mut event = self
            .emit(EventType::AgentStopped)
            .with_payload("timestamp", serde_json::json!(Utc::now().to_rfc3339()))
            .build();

        if let Some(r) = reason {
            event
                .payload
                .insert("reason".to_string(), serde_json::json!(r));
        }

        event
    }

    pub fn task_created(&self, task_id: &str, content: &str) -> Event {
        self.emit(EventType::TaskCreated)
            .with_task(task_id)
            .with_payload("content", serde_json::json!(content))
            .build()
    }

    pub fn task_claimed(&self, task_id: &str) -> Event {
        self.emit(EventType::TaskClaimed).with_task(task_id).build()
    }

    pub fn task_completed(&self, task_id: &str, resolution: Option<&str>) -> Event {
        let mut event = self
            .emit(EventType::TaskCompleted)
            .with_task(task_id)
            .build();

        if let Some(r) = resolution {
            event
                .payload
                .insert("resolution".to_string(), serde_json::json!(r));
        }

        event
    }

    pub fn workunit_created(&self, workunit_id: &str, task_id: &str, intent_ref: &str) -> Event {
        self.emit(EventType::WorkUnitCreated)
            .with_workunit(workunit_id)
            .with_task(task_id)
            .with_payload("intent_ref", serde_json::json!(intent_ref))
            .build()
    }

    pub fn patch_proposed(&self, workunit_id: &str, path: &str, operation: &str) -> Event {
        self.emit(EventType::PatchProposed)
            .with_workunit(workunit_id)
            .with_payload("path", serde_json::json!(path))
            .with_payload("operation", serde_json::json!(operation))
            .build()
    }

    pub fn validation_passed(&self, gate: &str) -> Event {
        self.emit(EventType::ValidationPassed)
            .with_payload("gate", serde_json::json!(gate))
            .build()
    }

    pub fn validation_failed(&self, gate: &str, errors: Vec<&str>) -> Event {
        self.emit(EventType::ValidationFailed)
            .with_payload("gate", serde_json::json!(gate))
            .with_payload("errors", serde_json::json!(errors))
            .build()
    }

    pub fn interlock_encountered(&self, policy: &str, reason: &str, blocking: bool) -> Event {
        self.emit(EventType::InterlockEncountered)
            .with_payload("policy", serde_json::json!(policy))
            .with_payload("reason", serde_json::json!(reason))
            .with_payload("blocking", serde_json::json!(blocking))
            .build()
    }
}

pub struct EventBuilder {
    event: Event,
}

impl EventBuilder {
    fn new(agent_id: &str, agent_type: &str, event_type: EventType) -> Self {
        Self {
            event: Event::new(agent_id, agent_type, event_type),
        }
    }

    pub fn with_session(mut self, session_id: &str) -> Self {
        self.event.session_id = Some(session_id.to_string());
        self
    }

    pub fn with_task(mut self, task_id: &str) -> Self {
        self.event.task_id = Some(task_id.to_string());
        self
    }

    pub fn with_workunit(mut self, workunit_id: &str) -> Self {
        self.event.workunit_id = Some(workunit_id.to_string());
        self
    }

    pub fn with_payload(mut self, key: impl Into<String>, value: serde_json::Value) -> Self {
        self.event.payload.insert(key.into(), value);
        self
    }

    pub fn build(self) -> Event {
        self.event
    }
}

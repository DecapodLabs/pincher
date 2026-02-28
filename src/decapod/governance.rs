use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Interlock {
    pub policy: String,
    pub reason: String,
    pub blocking: bool,
    pub required_approval: Option<String>,
    pub approver_scope: Vec<String>,
    pub escalation_path: Option<String>,
}

impl Interlock {
    pub fn is_blocking(&self) -> bool {
        self.blocking
    }

    pub fn requires_approval(&self) -> bool {
        self.required_approval.is_some()
    }

    pub fn approval_scope(&self) -> &[String] {
        &self.approver_scope
    }

    pub fn can_proceed(&self) -> bool {
        !self.blocking
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Advisory {
    pub message: String,
    pub suggestions: Vec<String>,
    pub priority: AdvisoryPriority,
    pub category: Option<String>,
    pub related_policies: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AdvisoryPriority {
    Info,
    Warning,
    Critical,
}

impl Advisory {
    pub fn is_warning(&self) -> bool {
        matches!(self.priority, AdvisoryPriority::Warning)
    }

    pub fn is_critical(&self) -> bool {
        matches!(self.priority, AdvisoryPriority::Critical)
    }

    pub fn should_pause(&self) -> bool {
        self.is_critical()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Attestation {
    pub proof_id: String,
    pub criteria: String,
    pub passed: bool,
    pub evidence: HashMap<String, serde_json::Value>,
    pub verified_by: Option<String>,
    pub verified_at: Option<String>,
    pub expiration: Option<String>,
}

impl Attestation {
    pub fn is_valid(&self) -> bool {
        self.passed
    }

    pub fn has_expired(&self) -> bool {
        if let Some(exp) = &self.expiration {
            if let Ok(exp_time) = chrono::DateTime::parse_from_rfc3339(exp) {
                return exp_time < chrono::Utc::now();
            }
        }
        false
    }

    pub fn get_proof_id(&self) -> &str {
        &self.proof_id
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GovernanceResponse {
    pub interlock: Option<Interlock>,
    pub advisories: Vec<Advisory>,
    pub attestation: Option<Attestation>,
    pub allowed_next_ops: Vec<String>,
    pub blocked_by: Vec<String>,
}

impl GovernanceResponse {
    pub fn is_blocked(&self) -> bool {
        self.interlock.as_ref().map(|i| i.blocking).unwrap_or(false)
    }

    pub fn has_warnings(&self) -> bool {
        self.advisories.iter().any(|a| a.is_warning())
    }

    pub fn has_critical(&self) -> bool {
        self.advisories.iter().any(|a| a.is_critical())
    }

    pub fn is_attested(&self) -> bool {
        self.attestation.as_ref().map(|a| a.passed).unwrap_or(false)
    }

    pub fn can_proceed(&self) -> bool {
        !self.is_blocked() && !self.has_critical()
    }

    pub fn get_blocking_reasons(&self) -> Vec<String> {
        let mut reasons = Vec::new();

        if let Some(ref interlock) = self.interlock {
            if interlock.blocking {
                reasons.push(interlock.reason.clone());
            }
        }

        reasons.extend(self.blocked_by.clone());

        reasons
    }

    pub fn get_suggestions(&self) -> Vec<String> {
        self.advisories
            .iter()
            .flat_map(|a| a.suggestions.clone())
            .collect()
    }
}

pub struct GovernanceEngine;

impl GovernanceEngine {
    pub fn evaluate(response: &GovernanceResponse) -> GovernanceDecision {
        if response.is_blocked() {
            return GovernanceDecision::Blocked {
                reasons: response.get_blocking_reasons(),
                required_approval: response
                    .interlock
                    .as_ref()
                    .and_then(|i| i.required_approval.clone()),
            };
        }

        if response.has_critical() {
            return GovernanceDecision::CriticalAdvisory {
                advisories: response
                    .advisories
                    .iter()
                    .filter(|a| a.is_critical())
                    .cloned()
                    .collect(),
            };
        }

        if response.has_warnings() {
            return GovernanceDecision::Warning {
                advisories: response
                    .advisories
                    .iter()
                    .filter(|a| a.is_warning())
                    .cloned()
                    .collect(),
            };
        }

        if response.is_attested() {
            return GovernanceDecision::Proceed {
                allowed_ops: response.allowed_next_ops.clone(),
                attestation: response.attestation.clone(),
            };
        }

        GovernanceDecision::Proceed {
            allowed_ops: response.allowed_next_ops.clone(),
            attestation: None,
        }
    }

    pub fn must_validate(response: &GovernanceResponse) -> bool {
        response.is_blocked() || response.has_critical()
    }

    pub fn extract_approval_requirements(
        response: &GovernanceResponse,
    ) -> Vec<ApprovalRequirement> {
        let mut requirements = Vec::new();

        if let Some(ref interlock) = response.interlock {
            if let Some(ref approval) = interlock.required_approval {
                requirements.push(ApprovalRequirement {
                    scope: interlock.approver_scope.clone(),
                    reason: interlock.reason.clone(),
                    approval_type: approval.clone(),
                });
            }
        }

        requirements
    }
}

#[derive(Debug, Clone)]
pub enum GovernanceDecision {
    Proceed {
        allowed_ops: Vec<String>,
        attestation: Option<Attestation>,
    },
    Blocked {
        reasons: Vec<String>,
        required_approval: Option<String>,
    },
    Warning {
        advisories: Vec<Advisory>,
    },
    CriticalAdvisory {
        advisories: Vec<Advisory>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApprovalRequirement {
    pub scope: Vec<String>,
    pub reason: String,
    pub approval_type: String,
}

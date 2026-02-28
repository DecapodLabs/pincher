use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StateCommitment {
    pub id: String,
    pub previous_commitment: Option<String>,
    pub state_hash: String,
    pub commitments: Vec<CommitmentEntry>,
    pub timestamp: String,
    pub agent_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommitmentEntry {
    pub entry_type: EntryType,
    pub key: String,
    pub value_hash: String,
    pub metadata: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EntryType {
    Intent,
    Plan,
    Patch,
    Approval,
    Proof,
    Validation,
    Event,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProofSurface {
    pub id: String,
    pub commitment_id: String,
    pub proof_type: ProofType,
    pub criteria: String,
    pub evidence: HashMap<String, serde_json::Value>,
    pub verified: bool,
    pub verified_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProofType {
    Validation,
    Test,
    Review,
    Audit,
    External,
}

pub struct StateCommitmentManager {
    agent_id: String,
    current_commitment_id: Option<String>,
}

impl StateCommitmentManager {
    pub fn new(agent_id: &str) -> Self {
        Self {
            agent_id: agent_id.to_string(),
            current_commitment_id: None,
        }
    }

    pub fn create_commitment(&mut self, entries: Vec<CommitmentEntry>) -> StateCommitment {
        let commitment_id = ulid::Ulid::new().to_string();

        let state_hash = self.compute_state_hash(&entries);

        let commitment = StateCommitment {
            id: commitment_id.clone(),
            previous_commitment: self.current_commitment_id.clone(),
            state_hash,
            commitments: entries,
            timestamp: chrono::Utc::now().to_rfc3339(),
            agent_id: self.agent_id.clone(),
        };

        self.current_commitment_id = Some(commitment_id);

        commitment
    }

    pub fn add_intent(&self, intent: &str) -> CommitmentEntry {
        CommitmentEntry {
            entry_type: EntryType::Intent,
            key: format!("intent:{}", ulid::Ulid::new()),
            value_hash: self.hash_value(intent),
            metadata: {
                let mut m = HashMap::new();
                m.insert("content".to_string(), serde_json::json!(intent));
                m
            },
        }
    }

    pub fn add_plan(&self, plan: &str, task_id: &str) -> CommitmentEntry {
        CommitmentEntry {
            entry_type: EntryType::Plan,
            key: format!("plan:{}", task_id),
            value_hash: self.hash_value(plan),
            metadata: {
                let mut m = HashMap::new();
                m.insert("content".to_string(), serde_json::json!(plan));
                m.insert("task_id".to_string(), serde_json::json!(task_id));
                m
            },
        }
    }

    pub fn add_patch(&self, path: &str, operation: &str, content: Option<&str>) -> CommitmentEntry {
        CommitmentEntry {
            entry_type: EntryType::Patch,
            key: format!("patch:{}:{}", path, operation),
            value_hash: self.hash_value(content.unwrap_or("")),
            metadata: {
                let mut m = HashMap::new();
                m.insert("path".to_string(), serde_json::json!(path));
                m.insert("operation".to_string(), serde_json::json!(operation));
                if let Some(c) = content {
                    m.insert("content".to_string(), serde_json::json!(c));
                }
                m
            },
        }
    }

    pub fn add_approval(&self, approver: &str, scope: &[&str], task_id: &str) -> CommitmentEntry {
        CommitmentEntry {
            entry_type: EntryType::Approval,
            key: format!("approval:{}", task_id),
            value_hash: self.hash_value(&approver),
            metadata: {
                let mut m = HashMap::new();
                m.insert("approver".to_string(), serde_json::json!(approver));
                m.insert("scope".to_string(), serde_json::json!(scope));
                m.insert("task_id".to_string(), serde_json::json!(task_id));
                m
            },
        }
    }

    pub fn add_proof(
        &self,
        proof_type: ProofType,
        criteria: &str,
        evidence: HashMap<String, serde_json::Value>,
    ) -> CommitmentEntry {
        CommitmentEntry {
            entry_type: EntryType::Proof,
            key: format!("proof:{}", ulid::Ulid::new()),
            value_hash: self.hash_value(criteria),
            metadata: {
                let mut m = evidence;
                m.insert("proof_type".to_string(), serde_json::json!(proof_type));
                m.insert("criteria".to_string(), serde_json::json!(criteria));
                m
            },
        }
    }

    pub fn create_proof_surface(
        &self,
        commitment_id: &str,
        proof_type: ProofType,
        criteria: &str,
        evidence: HashMap<String, serde_json::Value>,
    ) -> ProofSurface {
        ProofSurface {
            id: ulid::Ulid::new().to_string(),
            commitment_id: commitment_id.to_string(),
            proof_type,
            criteria: criteria.to_string(),
            evidence,
            verified: false,
            verified_at: None,
        }
    }

    pub fn verify_proof(&mut self, proof: &mut ProofSurface) -> bool {
        proof.verified = true;
        proof.verified_at = Some(chrono::Utc::now().to_rfc3339());
        proof.verified
    }

    fn compute_state_hash(&self, entries: &[CommitmentEntry]) -> String {
        let mut hasher = Sha256::new();

        if let Some(ref prev) = self.current_commitment_id {
            hasher.update(prev.as_bytes());
        }

        for entry in entries {
            hasher.update(entry.value_hash.as_bytes());
        }

        format!("{:x}", hasher.finalize())
    }

    fn hash_value(&self, value: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(value.as_bytes());
        format!("{:x}", hasher.finalize())
    }

    pub fn get_current_commitment_id(&self) -> Option<&str> {
        self.current_commitment_id.as_deref()
    }

    pub fn verify_chain(&self, commitment: &StateCommitment) -> bool {
        if let Some(ref prev_id) = commitment.previous_commitment {
            if Some(prev_id) != self.current_commitment_id.as_ref() {
                return false;
            }
        }

        let computed_hash = self.compute_state_hash(&commitment.commitments);
        computed_hash == commitment.state_hash
    }
}

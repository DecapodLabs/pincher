//! Versioned, host-facing contract for one Decapod-governed Pincher run.
//!
//! This module deliberately contains no provider, transport, persistence, or UI
//! implementation.  It is the narrow boundary between a host such as Amnion,
//! the Pincher loop, and the Decapod control plane.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fmt;
use thiserror::Error;

/// Stable identifier for the first host contract.
pub const GOVERNED_RUN_CONTRACT_ID: &str = "pincher.governed-run";
/// Version of [`GOVERNED_RUN_CONTRACT_ID`].
pub const GOVERNED_RUN_CONTRACT_VERSION: &str = "1.0.0";

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ContractIdentity {
    pub id: String,
    pub version: String,
}

impl ContractIdentity {
    pub fn v1() -> Self {
        Self {
            id: GOVERNED_RUN_CONTRACT_ID.to_string(),
            version: GOVERNED_RUN_CONTRACT_VERSION.to_string(),
        }
    }

    fn is_supported(&self) -> bool {
        self.id == GOVERNED_RUN_CONTRACT_ID && self.version == GOVERNED_RUN_CONTRACT_VERSION
    }
}

#[derive(Debug, Error, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ContractError {
    #[error("{field} reference must not be empty")]
    EmptyReference { field: &'static str },
}

macro_rules! public_reference {
    ($name:ident, $field:literal) => {
        #[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
        #[serde(transparent)]
        pub struct $name(String);

        impl $name {
            pub fn new(value: impl Into<String>) -> Result<Self, ContractError> {
                let value = value.into();
                if value.trim().is_empty() {
                    return Err(ContractError::EmptyReference { field: $field });
                }
                Ok(Self(value))
            }

            pub fn as_str(&self) -> &str {
                &self.0
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str(&self.0)
            }
        }
    };
}

public_reference!(RunId, "run_id");
public_reference!(IntentId, "intent_id");
public_reference!(SessionRef, "session");
public_reference!(TaskRef, "task");
public_reference!(WorkUnitRef, "work_unit");
public_reference!(RepositoryRef, "repository");
public_reference!(WorkspaceRef, "workspace");
public_reference!(CustodyReceiptRef, "custody_receipt");
public_reference!(ContextEvidenceRef, "context_evidence");
public_reference!(ApprovalInterlockRef, "approval_interlock");
public_reference!(ApprovalEvidenceRef, "approval_evidence");
public_reference!(ValidationEvidenceRef, "validation_evidence");
public_reference!(ProofEvidenceRef, "proof_evidence");
public_reference!(ProviderProposalRef, "provider_proposal");
public_reference!(CorrelationId, "correlation_id");
public_reference!(IdempotencyKey, "idempotency_key");
public_reference!(EventId, "event_id");

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct CustodyBinding {
    pub session: Option<SessionRef>,
    pub task: Option<TaskRef>,
    pub work_unit: Option<WorkUnitRef>,
    pub repository: Option<RepositoryRef>,
    pub workspace: Option<WorkspaceRef>,
}

impl CustodyBinding {
    pub fn complete(
        session: SessionRef,
        task: TaskRef,
        work_unit: WorkUnitRef,
        repository: RepositoryRef,
        workspace: WorkspaceRef,
    ) -> Self {
        Self {
            session: Some(session),
            task: Some(task),
            work_unit: Some(work_unit),
            repository: Some(repository),
            workspace: Some(workspace),
        }
    }

    fn missing_fields(&self) -> Vec<CustodyField> {
        let mut missing = Vec::new();
        if self.session.is_none() {
            missing.push(CustodyField::Session);
        }
        if self.task.is_none() {
            missing.push(CustodyField::Task);
        }
        if self.work_unit.is_none() {
            missing.push(CustodyField::WorkUnit);
        }
        if self.repository.is_none() {
            missing.push(CustodyField::Repository);
        }
        if self.workspace.is_none() {
            missing.push(CustodyField::Workspace);
        }
        missing
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CustodyField {
    Session,
    Task,
    WorkUnit,
    Repository,
    Workspace,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RunRequest {
    pub contract: ContractIdentity,
    pub run_id: RunId,
    pub intent_id: IntentId,
    pub correlation_id: CorrelationId,
    pub idempotency_key: IdempotencyKey,
    pub custody: CustodyBinding,
}

impl RunRequest {
    pub fn v1(
        run_id: RunId,
        intent_id: IntentId,
        correlation_id: CorrelationId,
        idempotency_key: IdempotencyKey,
        custody: CustodyBinding,
    ) -> Self {
        Self {
            contract: ContractIdentity::v1(),
            run_id,
            intent_id,
            correlation_id,
            idempotency_key,
            custody,
        }
    }

    pub fn validate(&self) -> Result<(), RunFailure> {
        if !self.contract.is_supported() {
            return Err(RunFailure::InvalidRequest {
                reason: InvalidRequestReason::UnsupportedContract {
                    id: self.contract.id.clone(),
                    version: self.contract.version.clone(),
                },
                remediation: Some(Remediation::new(
                    "use pincher.governed-run contract version 1.0.0",
                )),
            });
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CustodyEvidence {
    pub session: SessionRef,
    pub task: TaskRef,
    pub work_unit: WorkUnitRef,
    pub repository: RepositoryRef,
    pub workspace: WorkspaceRef,
    pub receipt: CustodyReceiptRef,
    pub workspace_allowed: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ContextEvidence {
    pub reference: ContextEvidenceRef,
    pub resolved: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ValidationEvidence {
    pub reference: ValidationEvidenceRef,
    pub passed: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProofEvidence {
    pub reference: ProofEvidenceRef,
    pub backed: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ApprovalEvidence {
    pub reference: ApprovalEvidenceRef,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AdvisoryEvidence {
    pub reference: Option<ApprovalInterlockRef>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum InterlockDecision {
    Allow {
        advisory: Option<AdvisoryEvidence>,
    },
    Block {
        reference: ApprovalInterlockRef,
        remediation: Remediation,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum ApprovalStatus {
    #[default]
    NotRequired,
    Granted {
        evidence: ApprovalEvidence,
    },
    Pending {
        reference: ApprovalInterlockRef,
        remediation: Remediation,
    },
    Denied {
        reference: ApprovalInterlockRef,
        remediation: Remediation,
    },
}

/// The only Decapod operations the core engine requires for one run.
///
/// Implementations must return authoritative references from Decapod.  They
/// must not manufacture validation, approval, or proof success locally.
pub trait DecapodControlPlane {
    fn validate_custody(
        &self,
        binding: &CustodyBinding,
    ) -> Result<CustodyEvidence, DecapodPortError>;

    fn resolve_context(
        &self,
        custody: &CustodyEvidence,
        intent: &IntentId,
    ) -> Result<ContextEvidence, DecapodPortError>;

    fn evaluate_interlocks(
        &self,
        custody: &CustodyEvidence,
        context: &ContextEvidence,
    ) -> Result<InterlockDecision, DecapodPortError>;

    fn approval_status(
        &self,
        custody: &CustodyEvidence,
        context: &ContextEvidence,
    ) -> Result<ApprovalStatus, DecapodPortError>;

    fn validate(
        &self,
        custody: &CustodyEvidence,
        context: &ContextEvidence,
        proposal: &ProviderProposal,
    ) -> Result<ValidationEvidence, DecapodPortError>;

    fn obtain_proof(
        &self,
        custody: &CustodyEvidence,
        validation: &ValidationEvidence,
    ) -> Result<ProofEvidence, DecapodPortError>;
}

#[derive(Debug, Error, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DecapodPortError {
    #[error("Decapod operation is unsupported: {operation}")]
    Unsupported {
        operation: String,
        remediation: Remediation,
    },
    #[error("Decapod rejected custody: {reason:?}")]
    CustodyRejected { reason: CustodyFailure },
    #[error("Decapod context resolution failed: {reason}")]
    ContextUnavailable { reason: String },
    #[error("Decapod operation {operation} is incomplete: {reason}")]
    Incomplete { operation: String, reason: String },
}

/// Explicit adapter placeholder.  It fails closed until the concrete Decapod
/// CLI/RPC envelope is proven by the follow-up adapter-conformance issue.
#[derive(Debug, Default, Clone, Copy)]
pub struct UnsupportedDecapodControlPlane;

impl UnsupportedDecapodControlPlane {
    fn unsupported(operation: &str) -> DecapodPortError {
        DecapodPortError::Unsupported {
            operation: operation.to_string(),
            remediation: Remediation::new("implement the Decapod adapter conformance contract"),
        }
    }
}

impl DecapodControlPlane for UnsupportedDecapodControlPlane {
    fn validate_custody(
        &self,
        _binding: &CustodyBinding,
    ) -> Result<CustodyEvidence, DecapodPortError> {
        Err(Self::unsupported("validate_custody"))
    }

    fn resolve_context(
        &self,
        _custody: &CustodyEvidence,
        _intent: &IntentId,
    ) -> Result<ContextEvidence, DecapodPortError> {
        Err(Self::unsupported("resolve_context"))
    }

    fn evaluate_interlocks(
        &self,
        _custody: &CustodyEvidence,
        _context: &ContextEvidence,
    ) -> Result<InterlockDecision, DecapodPortError> {
        Err(Self::unsupported("evaluate_interlocks"))
    }

    fn approval_status(
        &self,
        _custody: &CustodyEvidence,
        _context: &ContextEvidence,
    ) -> Result<ApprovalStatus, DecapodPortError> {
        Err(Self::unsupported("approval_status"))
    }

    fn validate(
        &self,
        _custody: &CustodyEvidence,
        _context: &ContextEvidence,
        _proposal: &ProviderProposal,
    ) -> Result<ValidationEvidence, DecapodPortError> {
        Err(Self::unsupported("validate"))
    }

    fn obtain_proof(
        &self,
        _custody: &CustodyEvidence,
        _validation: &ValidationEvidence,
    ) -> Result<ProofEvidence, DecapodPortError> {
        Err(Self::unsupported("obtain_proof"))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GovernedInferenceRequest {
    pub contract: ContractIdentity,
    pub run_id: RunId,
    pub intent_id: IntentId,
    pub correlation_id: CorrelationId,
    pub custody: CustodyEvidence,
    pub context: ContextEvidence,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProviderProposal {
    pub reference: ProviderProposalRef,
    pub output_digest: String,
}

pub trait ProviderTurn {
    fn infer(&self, request: GovernedInferenceRequest) -> Result<ProviderProposal, ProviderError>;
}

#[derive(Debug, Error, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ProviderError {
    #[error("provider is unavailable: {reason}")]
    Unavailable { reason: String },
    #[error("provider rejected governed request: {reason}")]
    Rejected { reason: String },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct EventKind(String);

impl EventKind {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    fn state(name: &str) -> Self {
        Self::new(format!("run.state.{name}"))
    }

    fn activity(name: &str) -> Self {
        Self::new(format!("run.activity.{name}"))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EventCustody {
    pub session: SessionRef,
    pub task: TaskRef,
    pub work_unit: WorkUnitRef,
    pub repository: RepositoryRef,
    pub workspace: WorkspaceRef,
}

impl From<&CustodyEvidence> for EventCustody {
    fn from(evidence: &CustodyEvidence) -> Self {
        Self {
            session: evidence.session.clone(),
            task: evidence.task.clone(),
            work_unit: evidence.work_unit.clone(),
            repository: evidence.repository.clone(),
            workspace: evidence.workspace.clone(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RunEvent {
    pub contract: ContractIdentity,
    pub event_id: EventId,
    pub run_id: RunId,
    pub correlation_id: CorrelationId,
    pub sequence: u64,
    pub occurred_at: DateTime<Utc>,
    pub source: String,
    pub kind: EventKind,
    pub state: Option<RunState>,
    pub custody: Option<EventCustody>,
    pub advisory_ref: Option<ApprovalInterlockRef>,
    pub approval_ref: Option<ApprovalInterlockRef>,
    pub approval_evidence_ref: Option<ApprovalEvidenceRef>,
    pub validation_ref: Option<ValidationEvidenceRef>,
    pub proof_ref: Option<ProofEvidenceRef>,
    pub failure: Option<FailureCode>,
    pub payload: serde_json::Value,
}

pub trait EventSink {
    fn publish(&mut self, event: RunEvent) -> Result<(), EventSinkError>;
}

#[derive(Debug, Error, Clone, PartialEq, Eq)]
#[error("event sink rejected event: {reason}")]
pub struct EventSinkError {
    pub reason: String,
}

#[derive(Debug, Default, Clone)]
pub struct InMemoryEventSink {
    events: Vec<RunEvent>,
}

impl InMemoryEventSink {
    pub fn events(&self) -> &[RunEvent] {
        &self.events
    }
}

impl EventSink for InMemoryEventSink {
    fn publish(&mut self, event: RunEvent) -> Result<(), EventSinkError> {
        self.events.push(event);
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RunState {
    Prepared,
    ContextResolved,
    Executing,
    AwaitingApproval,
    Verifying,
    Ready,
    Blocked,
    Failed,
    HandedOff,
}

impl RunState {
    fn can_transition_to(self, next: Self) -> bool {
        matches!(
            (self, next),
            (Self::Prepared, Self::ContextResolved | Self::Failed)
                | (
                    Self::ContextResolved,
                    Self::Executing | Self::AwaitingApproval | Self::Failed
                )
                | (
                    Self::AwaitingApproval,
                    Self::Executing | Self::Blocked | Self::Failed
                )
                | (Self::Executing, Self::Verifying | Self::Failed)
                | (Self::Verifying, Self::Ready | Self::Failed)
                | (Self::Ready, Self::HandedOff)
                | (Self::Blocked, Self::HandedOff)
                | (Self::Failed, Self::HandedOff)
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StateTransition {
    pub from: RunState,
    pub to: RunState,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Remediation {
    pub action: String,
}

impl Remediation {
    pub fn new(action: impl Into<String>) -> Self {
        Self {
            action: action.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CustodyFailure {
    Missing { fields: Vec<CustodyField> },
    WorkspaceNotAllowed { workspace: WorkspaceRef },
    Rejected { reason: String },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum InvalidRequestReason {
    UnsupportedContract { id: String, version: String },
    MissingCustody { fields: Vec<CustodyField> },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ValidationFailure {
    DecapodRejected,
    EvidenceMissing,
    ControlPlane { source: DecapodPortError },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ProofFailure {
    DecapodRejected,
    EvidenceMissing,
    ControlPlane { source: DecapodPortError },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum FailureCode {
    InvalidRequest,
    Custody,
    Context,
    Provider,
    Validation,
    Proof,
    ControlPlane,
    EventSink,
    IllegalTransition,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RunFailure {
    InvalidRequest {
        reason: InvalidRequestReason,
        remediation: Option<Remediation>,
    },
    Custody {
        reason: CustodyFailure,
        receipt: Option<CustodyReceiptRef>,
        remediation: Option<Remediation>,
    },
    Context {
        reason: String,
        remediation: Option<Remediation>,
    },
    Provider {
        reason: ProviderError,
        remediation: Option<Remediation>,
    },
    Validation {
        reason: ValidationFailure,
        evidence: Option<ValidationEvidenceRef>,
        remediation: Option<Remediation>,
    },
    Proof {
        reason: ProofFailure,
        evidence: Option<ProofEvidenceRef>,
        remediation: Option<Remediation>,
    },
}

impl RunFailure {
    fn code(&self) -> FailureCode {
        match self {
            Self::InvalidRequest { .. } => FailureCode::InvalidRequest,
            Self::Custody { .. } => FailureCode::Custody,
            Self::Context { .. } => FailureCode::Context,
            Self::Provider { .. } => FailureCode::Provider,
            Self::Validation { .. } => FailureCode::Validation,
            Self::Proof { .. } => FailureCode::Proof,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum BlockedReason {
    Interlock {
        reference: ApprovalInterlockRef,
        remediation: Remediation,
    },
    ApprovalPending {
        reference: ApprovalInterlockRef,
        remediation: Remediation,
    },
    ApprovalDenied {
        reference: ApprovalInterlockRef,
        remediation: Remediation,
    },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RunSnapshot {
    pub contract: ContractIdentity,
    pub request: RunRequest,
    pub state: RunState,
    pub custody: Option<CustodyEvidence>,
    pub context: Option<ContextEvidence>,
    pub advisory: Option<AdvisoryEvidence>,
    pub approval: Option<ApprovalEvidence>,
    pub validation: Option<ValidationEvidence>,
    pub proof: Option<ProofEvidence>,
    pub blocked: Option<BlockedReason>,
    pub failure: Option<RunFailure>,
    pub transitions: Vec<StateTransition>,
    pub event_count: u64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum RunOutcome {
    Ready(RunSnapshot),
    Blocked(RunSnapshot),
    Failed(RunSnapshot),
    HandedOff {
        terminal_state: RunState,
        snapshot: RunSnapshot,
    },
}

impl RunOutcome {
    pub fn snapshot(&self) -> &RunSnapshot {
        match self {
            Self::Ready(snapshot)
            | Self::Blocked(snapshot)
            | Self::Failed(snapshot)
            | Self::HandedOff { snapshot, .. } => snapshot,
        }
    }

    pub fn handoff(mut self) -> Result<Self, RunError> {
        let (terminal_state, snapshot) = match &mut self {
            Self::Ready(snapshot) => (RunState::Ready, snapshot),
            Self::Blocked(snapshot) => (RunState::Blocked, snapshot),
            Self::Failed(snapshot) => (RunState::Failed, snapshot),
            Self::HandedOff { .. } => {
                return Err(RunError::IllegalTransition {
                    from: RunState::HandedOff,
                    to: RunState::HandedOff,
                });
            }
        };

        if !snapshot.state.can_transition_to(RunState::HandedOff) {
            return Err(RunError::IllegalTransition {
                from: snapshot.state,
                to: RunState::HandedOff,
            });
        }
        snapshot.transitions.push(StateTransition {
            from: snapshot.state,
            to: RunState::HandedOff,
        });
        snapshot.state = RunState::HandedOff;
        Ok(Self::HandedOff {
            terminal_state,
            snapshot: snapshot.clone(),
        })
    }
}

#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum RunError {
    #[error("illegal run transition from {from:?} to {to:?}")]
    IllegalTransition { from: RunState, to: RunState },
    #[error(transparent)]
    EventSink(#[from] EventSinkError),
}

pub struct GovernedRunEngine<C, P, S> {
    control_plane: C,
    provider: P,
    event_sink: S,
    source: String,
}

impl<C, P, S> GovernedRunEngine<C, P, S>
where
    C: DecapodControlPlane,
    P: ProviderTurn,
    S: EventSink,
{
    pub fn new(control_plane: C, provider: P, event_sink: S) -> Self {
        Self {
            control_plane,
            provider,
            event_sink,
            source: "pincher.governed-run".to_string(),
        }
    }

    pub fn run(&mut self, request: RunRequest) -> Result<RunOutcome, RunError> {
        if let Err(failure) = request.validate() {
            return self.failed_without_started_run(request, failure);
        }

        let mut session = RunSession::new(request, &self.source, &mut self.event_sink);
        session.emit_state(RunState::Prepared)?;

        let missing = session.snapshot.request.custody.missing_fields();
        if !missing.is_empty() {
            return session.finish_failure(RunFailure::InvalidRequest {
                reason: InvalidRequestReason::MissingCustody { fields: missing },
                remediation: Some(Remediation::new(
                    "bind the run to an active session, task, work unit, repository, and isolated workspace",
                )),
            });
        }

        let custody = match self
            .control_plane
            .validate_custody(&session.snapshot.request.custody)
        {
            Ok(custody) => custody,
            Err(error) => {
                return session.finish_failure(RunFailure::Custody {
                    reason: CustodyFailure::Rejected {
                        reason: error.to_string(),
                    },
                    receipt: None,
                    remediation: Some(Remediation::new(
                        "resolve the Decapod custody rejection before retrying",
                    )),
                });
            }
        };

        if !custody.workspace_allowed {
            return session.finish_failure(RunFailure::Custody {
                reason: CustodyFailure::WorkspaceNotAllowed {
                    workspace: custody.workspace.clone(),
                },
                receipt: Some(custody.receipt.clone()),
                remediation: Some(Remediation::new(
                    "enter the Decapod-approved isolated workspace",
                )),
            });
        }

        let context = match self
            .control_plane
            .resolve_context(&custody, &session.snapshot.request.intent_id)
        {
            Ok(context) if context.resolved => context,
            Ok(_) => {
                return session.finish_failure(RunFailure::Context {
                    reason: "Decapod returned no resolved context evidence".to_string(),
                    remediation: Some(Remediation::new(
                        "resolve governed context through Decapod before inference",
                    )),
                });
            }
            Err(error) => {
                return session.finish_failure(RunFailure::Context {
                    reason: error.to_string(),
                    remediation: Some(Remediation::new(
                        "resolve governed context through Decapod before inference",
                    )),
                });
            }
        };

        session.snapshot.custody = Some(custody.clone());
        session.snapshot.context = Some(context.clone());
        session.transition(RunState::ContextResolved)?;
        session.emit_state(RunState::ContextResolved)?;

        let interlocks = match self.control_plane.evaluate_interlocks(&custody, &context) {
            Ok(decision) => decision,
            Err(error) => {
                return session.finish_failure(RunFailure::Context {
                    reason: error.to_string(),
                    remediation: Some(Remediation::new(
                        "obtain authoritative Decapod interlock status",
                    )),
                });
            }
        };

        if let InterlockDecision::Block {
            reference,
            remediation,
        } = &interlocks
        {
            return session.finish_blocked(BlockedReason::Interlock {
                reference: reference.clone(),
                remediation: remediation.clone(),
            });
        }

        if let InterlockDecision::Allow { advisory } = interlocks {
            session.snapshot.advisory = advisory;
            if session.snapshot.advisory.is_some() {
                session.emit_activity(EventKind::activity("advisory"), serde_json::Value::Null)?;
            }
        }

        let approval = match self.control_plane.approval_status(&custody, &context) {
            Ok(status) => status,
            Err(error) => {
                return session.finish_failure(RunFailure::Context {
                    reason: error.to_string(),
                    remediation: Some(Remediation::new(
                        "obtain authoritative Decapod approval status",
                    )),
                });
            }
        };

        match approval {
            ApprovalStatus::NotRequired => {}
            ApprovalStatus::Granted { evidence } => {
                session.snapshot.approval = Some(evidence);
            }
            ApprovalStatus::Pending {
                reference,
                remediation,
            } => {
                return session.finish_blocked(BlockedReason::ApprovalPending {
                    reference,
                    remediation,
                });
            }
            ApprovalStatus::Denied {
                reference,
                remediation,
            } => {
                return session.finish_blocked(BlockedReason::ApprovalDenied {
                    reference,
                    remediation,
                });
            }
        }

        session.transition(RunState::Executing)?;
        session.emit_state(RunState::Executing)?;

        let inference_request = GovernedInferenceRequest {
            contract: session.snapshot.contract.clone(),
            run_id: session.snapshot.request.run_id.clone(),
            intent_id: session.snapshot.request.intent_id.clone(),
            correlation_id: session.snapshot.request.correlation_id.clone(),
            custody: custody.clone(),
            context: context.clone(),
        };
        let proposal = match self.provider.infer(inference_request) {
            Ok(proposal) => proposal,
            Err(error) => {
                return session.finish_failure(RunFailure::Provider {
                    reason: error,
                    remediation: Some(Remediation::new(
                        "inspect the typed provider failure before retrying",
                    )),
                });
            }
        };

        session.emit_activity(
            EventKind::activity("proposal_received"),
            serde_json::json!({ "proposal_ref": proposal.reference }),
        )?;
        session.transition(RunState::Verifying)?;
        session.emit_state(RunState::Verifying)?;

        let validation = match self.control_plane.validate(&custody, &context, &proposal) {
            Ok(validation) => validation,
            Err(error) => {
                return session.finish_failure(RunFailure::Validation {
                    reason: ValidationFailure::ControlPlane { source: error },
                    evidence: None,
                    remediation: Some(Remediation::new(
                        "obtain an authoritative Decapod validation receipt",
                    )),
                });
            }
        };
        session.snapshot.validation = Some(validation.clone());
        if !validation.passed {
            return session.finish_failure(RunFailure::Validation {
                reason: ValidationFailure::DecapodRejected,
                evidence: Some(validation.reference),
                remediation: Some(Remediation::new(
                    "resolve the failed Decapod validation gate",
                )),
            });
        }

        let proof = match self.control_plane.obtain_proof(&custody, &validation) {
            Ok(proof) => proof,
            Err(error) => {
                return session.finish_failure(RunFailure::Proof {
                    reason: ProofFailure::ControlPlane { source: error },
                    evidence: None,
                    remediation: Some(Remediation::new(
                        "obtain authoritative Decapod proof evidence",
                    )),
                });
            }
        };
        session.snapshot.proof = Some(proof.clone());
        if !proof.backed {
            return session.finish_failure(RunFailure::Proof {
                reason: ProofFailure::DecapodRejected,
                evidence: Some(proof.reference),
                remediation: Some(Remediation::new(
                    "resolve the missing or failed Decapod proof gate",
                )),
            });
        }

        session.transition(RunState::Ready)?;
        session.emit_state(RunState::Ready)?;
        Ok(RunOutcome::Ready(session.snapshot))
    }

    pub fn handoff(&mut self, outcome: RunOutcome) -> Result<RunOutcome, RunError> {
        let mut outcome = outcome.handoff()?;
        let snapshot = outcome.snapshot().clone();
        let event = RunEvent {
            contract: snapshot.contract.clone(),
            event_id: EventId::new(format!("{}-handoff", snapshot.request.run_id)).map_err(
                |_| RunError::IllegalTransition {
                    from: RunState::HandedOff,
                    to: RunState::HandedOff,
                },
            )?,
            run_id: snapshot.request.run_id.clone(),
            correlation_id: snapshot.request.correlation_id.clone(),
            sequence: snapshot.event_count + 1,
            occurred_at: Utc::now(),
            source: self.source.clone(),
            kind: EventKind::state("handed_off"),
            state: Some(RunState::HandedOff),
            custody: snapshot.custody.as_ref().map(EventCustody::from),
            advisory_ref: snapshot
                .advisory
                .as_ref()
                .and_then(|evidence| evidence.reference.clone()),
            approval_ref: snapshot.blocked.as_ref().map(|reason| match reason {
                BlockedReason::Interlock { reference, .. }
                | BlockedReason::ApprovalPending { reference, .. }
                | BlockedReason::ApprovalDenied { reference, .. } => reference.clone(),
            }),
            approval_evidence_ref: snapshot
                .approval
                .as_ref()
                .map(|evidence| evidence.reference.clone()),
            validation_ref: snapshot.validation.as_ref().map(|e| e.reference.clone()),
            proof_ref: snapshot.proof.as_ref().map(|e| e.reference.clone()),
            failure: snapshot.failure.as_ref().map(RunFailure::code),
            payload: serde_json::Value::Null,
        };
        self.event_sink.publish(event)?;
        if let RunOutcome::HandedOff { snapshot, .. } = &mut outcome {
            snapshot.event_count += 1;
        }
        Ok(outcome)
    }

    fn failed_without_started_run(
        &mut self,
        request: RunRequest,
        failure: RunFailure,
    ) -> Result<RunOutcome, RunError> {
        let snapshot = RunSnapshot {
            contract: request.contract.clone(),
            request,
            state: RunState::Failed,
            custody: None,
            context: None,
            advisory: None,
            approval: None,
            validation: None,
            proof: None,
            blocked: None,
            failure: Some(failure),
            transitions: vec![StateTransition {
                from: RunState::Prepared,
                to: RunState::Failed,
            }],
            event_count: 0,
        };
        Ok(RunOutcome::Failed(snapshot))
    }
}

struct RunSession<'a> {
    snapshot: RunSnapshot,
    sequence: u64,
    source: &'a str,
    sink: &'a mut dyn EventSink,
}

impl<'a> RunSession<'a> {
    fn new(request: RunRequest, source: &'a str, sink: &'a mut dyn EventSink) -> Self {
        Self {
            snapshot: RunSnapshot {
                contract: request.contract.clone(),
                request,
                state: RunState::Prepared,
                custody: None,
                context: None,
                advisory: None,
                approval: None,
                validation: None,
                proof: None,
                blocked: None,
                failure: None,
                transitions: Vec::new(),
                event_count: 0,
            },
            sequence: 0,
            source,
            sink,
        }
    }

    fn transition(&mut self, next: RunState) -> Result<(), RunError> {
        if !self.snapshot.state.can_transition_to(next) {
            return Err(RunError::IllegalTransition {
                from: self.snapshot.state,
                to: next,
            });
        }
        self.snapshot.transitions.push(StateTransition {
            from: self.snapshot.state,
            to: next,
        });
        self.snapshot.state = next;
        Ok(())
    }

    fn emit_state(&mut self, state: RunState) -> Result<(), RunError> {
        self.emit(
            EventKind::state(&format!("{state:?}").to_lowercase()),
            Some(state),
            serde_json::Value::Null,
            None,
        )
    }

    fn emit_activity(
        &mut self,
        kind: EventKind,
        payload: serde_json::Value,
    ) -> Result<(), RunError> {
        self.emit(kind, Some(self.snapshot.state), payload, None)
    }

    fn emit(
        &mut self,
        kind: EventKind,
        state: Option<RunState>,
        payload: serde_json::Value,
        failure: Option<FailureCode>,
    ) -> Result<(), RunError> {
        self.sequence += 1;
        self.snapshot.event_count = self.sequence;
        self.sink.publish(RunEvent {
            contract: self.snapshot.contract.clone(),
            event_id: EventId::new(format!(
                "{}-{}",
                self.snapshot.request.run_id, self.sequence
            ))
            .map_err(|_| RunError::IllegalTransition {
                from: self.snapshot.state,
                to: self.snapshot.state,
            })?,
            run_id: self.snapshot.request.run_id.clone(),
            correlation_id: self.snapshot.request.correlation_id.clone(),
            sequence: self.sequence,
            occurred_at: Utc::now(),
            source: self.source.to_string(),
            kind,
            state,
            custody: self.snapshot.custody.as_ref().map(EventCustody::from),
            advisory_ref: self
                .snapshot
                .advisory
                .as_ref()
                .and_then(|evidence| evidence.reference.clone()),
            approval_ref: self.snapshot.blocked.as_ref().map(|reason| match reason {
                BlockedReason::Interlock { reference, .. }
                | BlockedReason::ApprovalPending { reference, .. }
                | BlockedReason::ApprovalDenied { reference, .. } => reference.clone(),
            }),
            approval_evidence_ref: self
                .snapshot
                .approval
                .as_ref()
                .map(|evidence| evidence.reference.clone()),
            validation_ref: self
                .snapshot
                .validation
                .as_ref()
                .map(|e| e.reference.clone()),
            proof_ref: self.snapshot.proof.as_ref().map(|e| e.reference.clone()),
            failure,
            payload,
        })?;
        Ok(())
    }

    fn finish_failure(mut self, failure: RunFailure) -> Result<RunOutcome, RunError> {
        self.snapshot.failure = Some(failure.clone());
        self.transition(RunState::Failed)?;
        self.emit(
            EventKind::state("failed"),
            Some(RunState::Failed),
            serde_json::Value::Null,
            Some(failure.code()),
        )?;
        Ok(RunOutcome::Failed(self.snapshot))
    }

    fn finish_blocked(mut self, reason: BlockedReason) -> Result<RunOutcome, RunError> {
        self.snapshot.blocked = Some(reason.clone());
        self.transition(RunState::AwaitingApproval)?;
        self.emit_state(RunState::AwaitingApproval)?;
        self.transition(RunState::Blocked)?;
        self.emit_state(RunState::Blocked)?;
        Ok(RunOutcome::Blocked(self.snapshot))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    fn reference<T: FromRef>(value: &str) -> T {
        T::from_ref(value)
    }

    trait FromRef: Sized {
        fn from_ref(value: &str) -> Self;
    }

    macro_rules! impl_from_ref {
        ($($type:ty),+ $(,)?) => {
            $(impl FromRef for $type {
                fn from_ref(value: &str) -> Self {
                    <$type>::new(value).expect("test reference")
                }
            })+
        };
    }

    impl_from_ref!(
        RunId,
        IntentId,
        SessionRef,
        TaskRef,
        WorkUnitRef,
        RepositoryRef,
        WorkspaceRef,
        CustodyReceiptRef,
        ContextEvidenceRef,
        ValidationEvidenceRef,
        ProofEvidenceRef,
        ProviderProposalRef,
        CorrelationId,
        IdempotencyKey,
    );

    fn request(custody: CustodyBinding) -> RunRequest {
        RunRequest::v1(
            reference("run-1"),
            reference("intent-1"),
            reference("corr-1"),
            reference("idem-1"),
            custody,
        )
    }

    fn complete_custody() -> CustodyBinding {
        CustodyBinding::complete(
            reference("session-1"),
            reference("task-1"),
            reference("work-unit-1"),
            reference("repo-1"),
            reference("workspace-1"),
        )
    }

    #[derive(Default)]
    struct FakeControlPlane {
        calls: RefCell<Vec<&'static str>>,
        interlocks: InterlockDecision,
        approval: ApprovalStatus,
        validation: ValidationEvidence,
        proof: ProofEvidence,
    }

    impl Default for InterlockDecision {
        fn default() -> Self {
            Self::Allow { advisory: None }
        }
    }

    impl Default for ValidationEvidence {
        fn default() -> Self {
            Self {
                reference: reference("validation-1"),
                passed: true,
            }
        }
    }

    impl Default for ProofEvidence {
        fn default() -> Self {
            Self {
                reference: reference("proof-1"),
                backed: true,
            }
        }
    }

    impl DecapodControlPlane for FakeControlPlane {
        fn validate_custody(
            &self,
            binding: &CustodyBinding,
        ) -> Result<CustodyEvidence, DecapodPortError> {
            self.calls.borrow_mut().push("custody");
            Ok(CustodyEvidence {
                session: binding.session.clone().unwrap(),
                task: binding.task.clone().unwrap(),
                work_unit: binding.work_unit.clone().unwrap(),
                repository: binding.repository.clone().unwrap(),
                workspace: binding.workspace.clone().unwrap(),
                receipt: reference("custody-1"),
                workspace_allowed: true,
            })
        }

        fn resolve_context(
            &self,
            _custody: &CustodyEvidence,
            _intent: &IntentId,
        ) -> Result<ContextEvidence, DecapodPortError> {
            self.calls.borrow_mut().push("context");
            Ok(ContextEvidence {
                reference: reference("context-1"),
                resolved: true,
            })
        }

        fn evaluate_interlocks(
            &self,
            _custody: &CustodyEvidence,
            _context: &ContextEvidence,
        ) -> Result<InterlockDecision, DecapodPortError> {
            self.calls.borrow_mut().push("interlocks");
            Ok(self.interlocks.clone())
        }

        fn approval_status(
            &self,
            _custody: &CustodyEvidence,
            _context: &ContextEvidence,
        ) -> Result<ApprovalStatus, DecapodPortError> {
            self.calls.borrow_mut().push("approval");
            Ok(self.approval.clone())
        }

        fn validate(
            &self,
            _custody: &CustodyEvidence,
            _context: &ContextEvidence,
            _proposal: &ProviderProposal,
        ) -> Result<ValidationEvidence, DecapodPortError> {
            self.calls.borrow_mut().push("validation");
            Ok(self.validation.clone())
        }

        fn obtain_proof(
            &self,
            _custody: &CustodyEvidence,
            _validation: &ValidationEvidence,
        ) -> Result<ProofEvidence, DecapodPortError> {
            self.calls.borrow_mut().push("proof");
            Ok(self.proof.clone())
        }
    }

    struct FakeProvider {
        calls: RefCell<Vec<&'static str>>,
    }

    impl ProviderTurn for FakeProvider {
        fn infer(
            &self,
            _request: GovernedInferenceRequest,
        ) -> Result<ProviderProposal, ProviderError> {
            self.calls.borrow_mut().push("provider");
            Ok(ProviderProposal {
                reference: reference("proposal-1"),
                output_digest: "digest".to_string(),
            })
        }
    }

    #[test]
    fn governed_happy_path_orders_custody_context_provider_validation_and_proof() {
        let control = FakeControlPlane::default();
        let provider = FakeProvider {
            calls: RefCell::new(Vec::new()),
        };
        let mut engine = GovernedRunEngine::new(control, provider, InMemoryEventSink::default());
        let outcome = engine.run(request(complete_custody())).unwrap();
        assert!(matches!(outcome, RunOutcome::Ready(_)));
        assert_eq!(outcome.snapshot().state, RunState::Ready);
        assert!(outcome.snapshot().validation.as_ref().unwrap().passed);
        assert!(outcome.snapshot().proof.as_ref().unwrap().backed);
    }
}

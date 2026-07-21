//! Pincher is the Rust governed inference loop owned by the Pincher project.
//!
//! Amnion owns the soft terminal UI/UX. Decapod owns authoritative repository
//! custody, governed context, approvals, validation, proof, and promotion
//! state. Pincher does not replace either boundary.
//!
//! The canonical host boundary is [`governed_run::GovernedRunEngine`]. Its
//! `RunRequest` requires explicit session, task, work-unit, repository, and
//! workspace references. The provider port cannot be reached until Decapod
//! custody and context evidence have been resolved. A run becomes `Ready` only
//! after Decapod supplies successful validation and proof evidence references.
//! Provider output is an untrusted proposal and ordinary events contain no raw
//! prompts, resolved context, or credentials.
//!
//! Real provider adapters, tool and patch execution, transport, persistence,
//! retries/recovery, and multi-agent operation are deferred from governed-run
//! contract version 1.0.0. Deterministic fake ports in the integration tests
//! prove the boundary without credentials or a live provider.

pub mod decapod;
pub mod governed_run;

pub use governed_run::{
    AdvisoryEvidence, ApprovalEvidence, ApprovalInterlockRef, ApprovalStatus, BlockedReason,
    ContextEvidence, ContextEvidenceRef, ContractError, ContractIdentity, CorrelationId,
    CustodyBinding, CustodyEvidence, CustodyFailure, CustodyField, CustodyReceiptRef,
    DecapodControlPlane, DecapodPortError, EventCustody, EventId, EventKind, EventSink,
    EventSinkError, FailureCode, GOVERNED_RUN_CONTRACT_ID,
    GOVERNED_RUN_CONTRACT_VERSION, GovernedInferenceRequest, GovernedRunEngine, IdempotencyKey,
    InMemoryEventSink, IntentId, InterlockDecision, InvalidRequestReason, ProofEvidence,
    ProofEvidenceRef, ProofFailure, ProviderError, ProviderProposal, ProviderProposalRef,
    ProviderTurn, Remediation, RepositoryRef, RunError, RunEvent, RunFailure, RunId, RunOutcome,
    RunRequest, RunSnapshot, RunState, SessionRef, StateTransition, TaskRef,
    UnsupportedDecapodControlPlane, ValidationEvidence, ValidationEvidenceRef, ValidationFailure,
    WorkUnitRef, WorkspaceRef,
};

pub use decapod::{
    Decapod, DecapodError,
    broker::{Event, EventEmitter, EventSource, EventType},
    capabilities::{Capabilities, CapabilitiesManager, SchemaInfo},
    cli::{Advisory, Attestation, ContextCapsule, DecapodCli, Interlock, Receipt},
    commitment::{
        CommitmentEntry, ProofSurface, ProofVerification, StateCommitment, StateCommitmentManager,
    },
    coordination::{
        Agent, AgentMessage, AgentStatus, AgentType, CoordinationManager, CoordinationPlan,
        Dependency, DependencyType, MessageType, SubAgentPlan,
    },
    docs::{DocsManager, IngestResult},
    governance::{
        AdvisoryPriority, ApprovalRequirement, GovernanceDecision, GovernanceEngine,
        GovernanceResponse,
    },
    rpc::{RpcClient, RpcResponse},
    session::{Session, SessionConfig, get_session_password},
    todo::{Task, TaskStatus, TodoManager},
    validate::{ValidationDetail, ValidationError, ValidationResult, Validator},
    workspace::{Workspace, WorkspaceManager, WorkspaceStatus, WorkspaceStatusResponse},
    workunit::{Approval, Patch, Proof, WorkUnit, WorkUnitManager, WorkUnitState, WorkUnitStatus},
};

pub use anyhow::Result;

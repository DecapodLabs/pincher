//! Pincher - The Decapod-native governed loop engine
//!
//! Pincher is a Rust-first loop engine that integrates with Decapod to enforce
//! governed context exposure, approvals, workspace custody, and proof-backed
//! quality inside explicitly allowed repos.
//!
//! Pincher owns execution state and typed events. A host such as Amnion owns
//! presentation and human interaction; Pincher does not contain UI policy.
//!
//! ## Key Features
//!
//! - **Session Management** - Acquire/validate Decapod sessions with token handling
//! - **Governed loop execution** - Prepare context, execute a turn, and stop at
//!   approval or proof boundaries.
//! - **Decapod coordination** - Sessions, todos, workspaces, work units, and
//!   validation remain delegated to the Decapod control plane.
//! - **Typed runtime events** - Hosts can render state without owning engine
//!   semantics.
//! - **Multi-agent coordination** - Delegate and coordinate work units without
//!   coupling the engine to a particular UI.
//! - **State commitment** - Preserve evidence and proof surfaces for handoff.
//!
//! ## Quick Start
//!
//! ```rust,no_run
//! use pincher::{Decapod, Result, RpcClient, Validator, Session};
//!
//! #[tokio::main]
//! async fn main() -> Result<()> {
//!     // Initialize Decapod connection
//!     let _decapod = Decapod::new()?;
//!
//!     // Acquire session (requires DECAPOD_SESSION_PASSWORD env var)
//!     let session = Session::acquire("your-password").await?;
//!
//!     // Create RPC client with session
//!     let rpc = RpcClient::new().with_session(session.token());
//!
//!     // Initialize agent
//!     let _response = rpc.agent_init("my-agent").await?;
//!
//!     // Run validation gates
//!     let validation = Validator::new().run().await?;
//!     if !validation.passed {
//!         return Err(anyhow::anyhow!("validation failed"));
//!     }
//!
//!     Ok(())
//! }
//! ```

pub mod decapod;

pub use decapod::{
    capabilities::{Capabilities, CapabilitiesManager, SchemaInfo},
    cli::{Advisory, Attestation, ContextCapsule, DecapodCli, Interlock, Receipt},
    commitment::{CommitmentEntry, ProofSurface, StateCommitment, StateCommitmentManager},
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
    session::{get_session_password, Session, SessionConfig},
    todo::{Task, TaskStatus, TodoManager},
    validate::{ValidationDetail, ValidationError, ValidationResult, Validator},
    workspace::{Workspace, WorkspaceManager, WorkspaceStatus, WorkspaceStatusResponse},
    workunit::{Approval, Patch, Proof, WorkUnit, WorkUnitManager, WorkUnitState, WorkUnitStatus},
    broker::{Event, EventEmitter, EventSource, EventType},
    Decapod, DecapodError,
};

pub use anyhow::Result;

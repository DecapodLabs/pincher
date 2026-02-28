//! Pincher - The Decapod-native agent engine
//!
//! Pincher is a Rust-first agent engine that integrates pristinely with Decapod
//! to enforce governance, approvals, and proof-backed quality inside explicitly
//! allowed repos.
//!
//! ## Key Features
//!
//! - **Session Management** - Acquire/validate Decapod sessions with token handling
//! - **RPC Client** - Full JSON-RPC interface to Decapod governance plane
//! - **Validation Gates** - Execute governance validation before operations
//! - **Task Management** - Full todo lifecycle (add, claim, complete, handoff)
//! - **Workspace Isolation** - Git worktree-based isolated workspaces
//! - **WorkUnit Governance** - Intent→Plan→Patches→Approvals→Proofs workflow
//! - **Governance Engine** - Interlock/Advisory/Attestation response handling
//! - **Event Emission** - Emit structured events to Decapod broker
//! - **Multi-Agent Coordination** - Delegate, coordinate, status updates between agents
//! - **State Commitment** - Cryptographic state commitment with proof surfaces
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

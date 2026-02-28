# Pincher

A Rust-first agent engine that integrates pristinely with [Decapod](https://github.com/DecapodLabs/decapod) to enforce governance, approvals, and proof-backed quality inside explicitly allowed repos.

## Overview

Pincher is designed to be the core execution spine that host processes (TUI, webapp, SaaS) integrate to run agent work with consistent behavior and guarantees. It owns:

- Model API orchestration
- Tool/function calling
- Patch planning and application
- Typed state + event emission
- Deterministic request/retry/idempotency envelopes
- Multi-agent concurrency (spawning and coordinating sub-agents)

## Features

| Feature | Description |
|---------|-------------|
| **Session Management** | Acquire/validate Decapod sessions with token handling |
| **RPC Client** | Full JSON-RPC interface to Decapod governance plane |
| **Validation Gates** | Execute governance validation before operations |
| **Task Management** | Full todo lifecycle (add, claim, complete, handoff) |
| **Workspace Isolation** | Git worktree-based isolated workspaces |
| **WorkUnit Governance** | Intent→Plan→Patches→Approvals→Proofs workflow |
| **Governance Engine** | Interlock/Advisory/Attestation response handling |
| **Event Emission** | Emit structured events to Decapod broker |
| **Multi-Agent Coordination** | Delegate, coordinate, status updates between agents |
| **State Commitment** | Cryptographic state commitment with proof surfaces |

## Installation

```bash
cargo add pincher
```

## Quick Start

```rust,no_run
use pincher::{
    decapod::{Decapod, Session, RpcClient, Validator},
    Result,
};

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize Decapod connection
    let decapod = Decapod::new()?;

    // Acquire session (requires DECAPOD_SESSION_PASSWORD env var)
    let session = Session::acquire("your-password").await?;

    // Create RPC client with session
    let rpc = RpcClient::new().with_session(session.token());

    // Initialize agent
    let response = rpc.agent_init("my-agent").await?;

    // Run validation gates
    let validation = Validator::new().run().await?;
    if !validation.passed {
        return Err(anyhow::anyhow!("validation failed"));
    }

    Ok(())
}
```

## Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                        Pincher                              │
│  (Agent Engine - embedded library)                         │
├─────────────────────────────────────────────────────────────┤
│  CLI Wrapper  │  RPC Client  │  Governance  │  Coordination │
├───────────────┴──────────────┴──────────────┴──────────────┤
│                    Decapod Binary                          │
│  (Governance Control Plane - invoked on-demand)            │
├─────────────────────────────────────────────────────────────┤
│  Sessions │ Validation │ Todos │ Workunits │ Workspaces    │
└─────────────────────────────────────────────────────────────┘
```

## Decapod Integration

Pincher is designed specifically for Decapod-managed repositories. The integration follows the control-plane-first pattern:

```bash
# Initialize in a Decapod repo
decapod validate
decapod docs ingest  
decapod session acquire

# Use Pincher to run agent work
pincher run --agent-type coordinator
```

## Governance Flow

```
User Intent
    │
    ▼
Decapod Intent Capsule
    │
    ▼
Pincher: Intent → Plan → Patches
    │
    ├──▶ Decapod: Validate (gate)
    │         │
    │         ▼
    │    Interlock? ──▶ Request Approval
    │
    ▼
Decapod: Proof Surfaces
    │
    ▼
Promotion Gate
```

## Cargo Features

Default features include full Decapod integration. Minimal feature set available for non-Decapod usage.

## Requirements

- Rust 1.90+
- Decapod binary installed (`cargo install decapod`)
- Decapod-managed repository

## License

MIT

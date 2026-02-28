# AGENTS.md — Universal Agent Contract

This is a Decapod-managed repository. **Strict Dependency: You are strictly bound to the Decapod control plane.**

This contract applies equally to Claude, Codex, Gemini, and any other agent operating here.

## This Repository

**Pincher** is the Rust agent engine library that integrates with Decapod. This is NOT a Decapod-managed project in the traditional sense - it's the **engine that powers agents** to work WITH Decapod.

## For Agents Developing Pincher

```bash
cargo install decapod

decapod validate
decapod docs ingest
decapod session acquire
decapod rpc --op agent.init
decapod rpc --op context.resolve
decapod todo add "<task>"
decapod todo claim --id <task-id>
decapod workspace ensure
```

## For Agents Using Pincher (as a library)

When building agent systems that use Pincher to integrate with Decapod:

```rust
use pincher::{
    decapod::{
        Session, RpcClient, Validator, TodoManager,
        WorkspaceManager, WorkUnitManager, EventEmitter,
    },
    Result,
};

#[tokio::main]
async fn main() -> Result<()> {
    // Acquire Decapod session
    let session = Session::acquire(&password).await?;
    
    // Create RPC client
    let rpc = RpcClient::new().with_session(session.token());
    
    // Initialize agent
    rpc.agent_init("my-agent").await?;
    
    // Validate governance gates
    let validation = Validator::new().run().await?;
    
    // Emit events
    let emitter = EventEmitter::new("my-agent", "executor");
    emitter.task_created(&task_id, "description");
    
    Ok(())
}
```

## Control-Plane First Loop

```bash
# Discover what this binary actually supports in this repo
decapod capabilities --format json
decapod data schema --deterministic

# Resolve scoped governance context before implementation
decapod docs search --query "<problem>" --op <op> --path <path> --tag <tag>
decapod rpc --op context.scope --params '{"query":"<problem>","limit":8}'

# Convergence/proof surfaces (call when relevant)
decapod workunit init --task-id <task-id> --intent-ref <intent>
decapod govern capsule query --topic "<topic>" --scope interfaces --task-id <task-id>
decapod eval plan --task-set-id <id> --task-ref <task-id> --model-id <model> --prompt-hash <hash> --judge-model-id <judge> --judge-prompt-hash <hash>
```

## Golden Rules (Non-Negotiable)

1. Always refine intent with the user before inference-heavy work.
2. Never work on main/master. Use `.decapod/workspaces/*`.
3. `.decapod files are accessed only via decapod CLI`.
4. Never claim done without `decapod validate` passing.
5. Never invent capabilities that are not exposed by the binary.
6. Stop if requirements conflict, intent is ambiguous, or policy boundaries are unclear.
7. Respect the Interface abstraction boundary.

## Pincher Library Usage

When modifying Pincher (the library):

| Component | Test Command |
|-----------|--------------|
| Full build | `cargo build --all-features` |
| Tests | `cargo test --all-features` |
| Doc tests | `cargo test --doc` |
| Examples | `cargo run --example agent_workflow` |
| Clippy | `cargo clippy --all-features -- -D warnings` |

## Safety Invariants

- ✅ Router pointer: `core/DECAPOD.md`
- ✅ Validation gate: `decapod validate`
- ✅ Constitution ingestion gate: `decapod docs ingest`
- ✅ Claim-before-work gate: `decapod todo claim --id <task-id>`
- ✅ Session auth gate: `DECAPOD_SESSION_PASSWORD`
- ✅ Workspace gate: Docker git workspaces
- ✅ Privilege gate: request elevated permissions before Docker/container workspace commands

## Operating Notes

- Use `decapod docs show core/DECAPOD.md` and `decapod docs show core/INTERFACES.md` for binding contracts.
- Use `decapod capabilities --format json` as the authority surface for available operations.
- Use Decapod shared aptitude memory for human-taught preferences that must persist across sessions and agents: `decapod data memory add|get` (aliases: `decapod data aptitude`).
- Use `decapod docs search --query \"<problem>\" --op <op> --path <path> --tag <tag>` or `decapod rpc --op context.scope --params '{\"query\":\"...\"}'` for scoped just-in-time constitution context.
- Use `decapod todo handoff --id <id> --to <agent>` for cross-agent ownership transfer.
- Treat lock/contention failures (including `VALIDATE_TIMEOUT_OR_LOCK`) as blocking until resolved.

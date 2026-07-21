# Pincher

Pincher is the Rust-first governed loop engine for Decapod-managed agent work. It prepares context, runs provider turns, coordinates work units, and records the custody and proof state that a host can render.

Pincher is a library/runtime boundary. Amnion owns the human-facing terminal UI/UX; other hosts may consume the same engine contract. Pincher does not own screens, terminal layout, conversation presentation, or product interaction flow.

## Ownership boundary

| Concern | Owner |
| --- | --- |
| Intent preparation, context resolution, and governed execution | Pincher |
| Provider/tool adapters and retry/idempotency behavior | Pincher |
| Sessions, todos, workspaces, approvals, validation, and proof custody | Decapod, coordinated by Pincher |
| Typed runtime events and execution state for hosts | Pincher |
| Terminal cockpit, workspace view, conversation UI, status views, and human attention flow | [Amnion](https://github.com/DecapodLabs/amnion) |
| Governance source of truth and promotion gates | Decapod |

## Execution model

```text
Host request
    |
    v
Pincher loop engine
    |-- resolve governed context
    |-- prepare a provider turn
    |-- propose tool/patch/work-unit work
    |-- stop at Decapod approval interlocks
    |-- emit typed state and events
    |-- run validation and record proof
    v
Host rendering / handoff (Amnion)
```

Each run stays bound to an explicit Decapod session, task/work unit, allowed workspace, and proof surface. Pincher may report `ready`, `blocked`, `failed`, or `handed_off`; it does not turn those states into UI policy.

## Current library surfaces

- `AgentEngine` prepares governed context and executes a provider turn.
- `RpcClient`, `Session`, `TodoManager`, `WorkspaceManager`, and `WorkUnitManager` coordinate Decapod state.
- `Event` and `EventEmitter` provide host-readable execution events.
- `StateCommitmentManager` and proof types preserve evidence for handoff and promotion.

The provider implementation and host transport remain explicit extension points. The current model call is a deterministic placeholder, so provider behavior is not represented as shipped capability yet.

## Quick start

```rust,no_run
use pincher::{AgentEngine, Decapod, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let _decapod = Decapod::new()?;
    // Construct an AgentEngine, initialize it with a Decapod session,
    // then expose its state/events to a host such as Amnion.
    Ok(())
}
```

## Development

```bash
cargo fmt --check
cargo test
cargo clippy -- -D warnings
decapod validate
```

## Requirements

- Rust 1.90+
- A Decapod installation for governed repository operations
- A Decapod-managed repository and an isolated workspace for mutations

## License

MIT

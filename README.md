# Pincher

Pincher is a Rust library/runtime for one Decapod-governed inference and
execution loop. It is not a TUI, chat application, provider launcher, or
governance store.

The ownership boundary is deliberate:

| System | Owns |
| --- | --- |
| Amnion | Soft terminal UI/UX and host presentation |
| Pincher | Governed run sequencing, provider port, typed state, and host events |
| Decapod | Authoritative custody, context, approvals, validation, proof, and promotion |

## Governed-run v1

The public contract is `pincher.governed-run` version `1.0.0`. A host submits a
`RunRequest` containing stable run, intent, correlation, idempotency, and
explicit Decapod custody references. The `GovernedRunEngine` accepts a provider
turn only after the control-plane port has confirmed all of these references:

```text
Prepared
  -> Decapod custody
  -> Decapod context resolution
  -> interlock and approval decision
  -> ContextResolved
  -> Executing / provider proposal
  -> Verifying
  -> authoritative validation
  -> authoritative proof evidence
  -> Ready
```

No provider output, local hash, host action, process exit, or Pincher event can
create approval, validation, proof, or readiness. `Ready` requires both a
successful Decapod validation evidence reference and a successful Decapod proof
evidence reference. Interlocks and pending/denied approvals produce a typed
`Blocked` outcome; custody, provider, validation, and proof failures produce a
typed `Failed` outcome with remediation information. Either terminal outcome
can be handed off without erasing its evidence.

### Minimal host request

The request is serializable and contains references rather than credentials or
raw resolved context:

```rust
use pincher::governed_run::{
    CorrelationId, CustodyBinding, IdempotencyKey, IntentId, RepositoryRef, RunId,
    RunRequest, SessionRef, TaskRef, WorkUnitRef, WorkspaceRef,
};

let request = RunRequest::v1(
    RunId::new("run-1")?,
    IntentId::new("intent-1")?,
    CorrelationId::new("correlation-1")?,
    IdempotencyKey::new("idempotency-1")?,
    CustodyBinding::complete(
        SessionRef::new("session-1")?,
        TaskRef::new("task-1")?,
        WorkUnitRef::new("work-unit-1")?,
        RepositoryRef::new("repository-1")?,
        WorkspaceRef::new("workspace-1")?,
    ),
);
```

`GovernedRunEngine<C, P, S>` takes three explicit ports:

- `DecapodControlPlane` returns custody, context, interlock/approval, validation,
  and proof evidence from Decapod.
- `ProviderTurn` receives only `GovernedInferenceRequest`, which already
  contains successful custody and context evidence.
- `EventSink` receives versioned envelopes with stable sequence numbers. The
  core never prints events to stdout; `InMemoryEventSink` is provided for
  deterministic tests.

The checked-in `tests/governed_run_contract.rs` supplies deterministic fake
ports and proves the happy, blocked, and failed paths without credentials or a
live provider. The current concrete Decapod CLI/RPC adapter remains explicitly
unsupported until its actual structured envelopes are proven in the adapter
conformance follow-up issue.

## Deferred from v1

This slice does not claim a real model provider, tool execution, patch
application, multi-turn autonomy, multi-agent delegation, transport/daemon
support, Pincher-owned governance persistence, recovery/replay, metrics, or
promotion/merge behavior. Those boundaries are tracked as follow-up issues.

## Development

```bash
cargo fmt --check
cargo test
cargo clippy -- -D warnings
decapod validate --refresh-specs
decapod validate
```

The repository is Decapod-managed. Implementations must run inside a claimed
isolated Decapod workspace, and generated specs must be refreshed through
Decapod rather than edited directly.

## License

MIT

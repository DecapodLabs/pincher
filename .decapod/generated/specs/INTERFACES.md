# Interfaces

<!-- decapod:capability-overlay:public-api:start -->

## Public API Capability Overlay

### API Contract Requirements
- All public endpoints MUST define explicit request/response schemas
- Versioning strategy MUST be documented (URL path or header-based)
- All public endpoints MUST implement idempotency for mutating operations
- Rate limiting and pagination MUST be implemented for list endpoints

### Compatibility Guarantees
- Backward-compatible changes ONLY within a version
- Breaking changes require new version (v1, v2, etc.)
- Deprecation and removal policy MUST be selected for this project and proven against its consumers

### Security Requirements
- All public endpoints MUST implement authentication
- Abuse-control enforcement point MUST be a documented project decision
- Input validation MUST reject malformed requests with typed errors
<!-- decapod:capability-overlay:public-api:end -->

## Contract boundary

The host contract is a typed Rust/event boundary. Pincher produces execution
state and events; Amnion consumes them to render and control a run. Decapod is
the producer and owner of governance records.

## Inbound Contracts

Hosts provide intent and run configuration through Pincher's Rust library boundary.

## Outbound Dependencies

Pincher invokes Decapod CLI/RPC operations and provider/tool adapters. It does
not write Decapod state through a parallel persistence path.

| Contract | Producer | Consumer | State owner | Proof |
| --- | --- | --- | --- | --- |
| `AgentConfig` / `AgentResponse` | Pincher | Host | Pincher run | Rust serialization/tests |
| `Event` / `EventEmitter` | Pincher | Host/event sink | Pincher + Decapod refs | JSON serialization |
| RPC/CLI adapter calls | Decapod | Pincher | Decapod | command/RPC result |
| Approval, validation, proof records | Decapod | Pincher and host | Decapod | Decapod receipt/gate |

## Event identity and lifecycle fields

Every emitted event carries an event id, timestamp, source, event type, and
optional session/task/work-unit identifiers. Payload fields are typed JSON
values. Host projections must preserve unknown event types and must not infer a
successful promotion from a local event alone.

## Data Ownership

Pincher owns ephemeral loop state and event serialization. Decapod owns durable
governance entities. Hosts own only projections and local view state.

The lifecycle vocabulary includes `agent_started`, `context_resolved`,
`work_unit_created`, `patch_proposed`, `approval_requested`,
`validation_passed`/`validation_failed`, `proof_recorded`, and terminal
`agent_stopped`, `work_unit_completed`, or `work_unit_failed` events.

## Decapod adapter contract

Pincher invokes Decapod on demand through its supported CLI/RPC surface. The
adapter must preserve the operation name, request/correlation context, typed
interlock/advisory/attestation result, and receipt/error. It must not invent a
second governance protocol or write `.decapod` state directly.

## Failure Semantics

| Failure | Retry | Host-visible result |
| --- | --- | --- |
| Invalid intent/configuration | No | Typed failure with correction detail |
| Provider timeout/transient dependency failure | Bounded retry | Retryable failure and retained cause |
| Decapod blocking interlock | No automatic bypass | `blocked`, required approval reference |
| Workspace/task conflict | No unsafe retry | `blocked` or `failed` with custody reference |
| Validation/proof failure | No promotion | `failed-with-cause`, evidence references |

## Compatibility

Current compatibility is the Rust crate's public types plus serialized event
shape. A separately transported host protocol is deferred. When introduced it
must define a contract id/version, producer/consumer, lifecycle, typed outcome
and failure, idempotency/correlation fields, and migration/removal evidence.

<!-- decapod:codebase-attestation:start -->
## Codebase Attestation

- Repository signal fingerprint: `4662065c21bacd9fd48af88524e80aa78796a654d6aa58642b9f7fb3da842383`
- Significant implementation surfaces: `.github/` (1 files), `Cargo.lock/` (1 files), `Cargo.toml/` (1 files), `README.md/` (1 files), `src/` (18 files)
- Refreshed from the current codebase by `decapod specs.refresh`
<!-- decapod:codebase-attestation:end -->

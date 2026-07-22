# Architecture

## Direction
Pincher is the Rust-first prompt actor and complete agent runtime between Amnion and Decapod. Amnion is the user-facing host: it accepts chat intent and presents Pincher events. Pincher owns prompt and context construction, model API orchestration, inference, tool and function calling, the agent loop, patch planning and application, typed state and event emission, deterministic request/retry/idempotency envelopes, multi-agent coordination, and automatic Decapod trigger and RPC integration. Decapod is a governed control-plane dependency that validates, authorizes, and records Pincher actions; Pincher is not merely a Decapod client. Keep Amnion’s UI and transport concerns thin while keeping governance decisions in Decapod.

## What This Project Is
pincher is a service_or_library project built using Rust.
Pincher is the Rust-first prompt actor and complete agent runtime between Amnion and Decapod. Amnion is the user-facing host: it accepts chat intent and presents Pincher events. Pincher owns prompt and context construction, model API orchestration, inference, tool and function calling, the agent loop, patch planning and application, typed state and event emission, deterministic request/retry/idempotency envelopes, multi-agent coordination, and automatic Decapod trigger and RPC integration. Decapod is a governed control-plane dependency that validates, authorizes, and records Pincher actions; Pincher is not merely a Decapod client. Keep Amnion’s UI and transport concerns thin while keeping governance decisions in Decapod.

Architectural principles:
- **Simplicity**: Keep components focused and reusable.
- **Modularity**: Clearly defined interface boundaries and dependency separation.
- **Reliability**: Graceful failure handling and thorough verification.

## Current Facts
- Runtime/languages: Rust
- Detected surfaces/framework hints: Amnion, Decapod, Pincher
- Product type: service_or_library

## Architecture Map
This project's architecture consists of the following key layers/directories:
- `src/`: Main source directory containing primary logic.
- `tests/`: Integration and unit test suite.

## Data Flows
- Inbound request/command parses and validates at the entrypoint.
- Core runtime handles business logic and initiates queries or state changes.
- Storage adapter reads or writes data to the underlying persistence layers.

## Strongest Existing Primitives
- Define the strongest existing primitives in the codebase (e.g., helper utilities, base controllers, data access layers).

## Topology
```text
Host Application -> Library API -> Domain Core -> Adapters (Store / Network)
```

## Store Boundaries
```mermaid
flowchart LR
  I[Inbound Requests] --> C[Core Logic]
  C --> W[(Write Store)]
  C --> R[(Read Store)]
```

## Happy Path Sequence
```text
Client request -> API validation -> domain execution -> persistence -> response with trace id
```

## Error Path
```mermaid
sequenceDiagram
  participant Client
  participant Service
  participant Store
  Client->>Service: Request
  Service->>Store: Database Query
  Store--xService: Error/Timeout
  Service-->>Client: Typed Error / Recovery Instructions
```

## Execution Path
- Ingress parse + validation:
- Policy/interlock checks:
- Core execution + persistence:
- Verification and artifact emission:

## Concurrency and Runtime Model
- Execution model:
- Isolation boundaries:
- Backpressure strategy:
- Shared state synchronization:

## Deployment Topology
- Runtime units:
- Region/zone model:
- Rollout strategy (blue/green/canary):
- Rollback trigger and blast-radius scope:

## Data and Contracts
- Inbound contracts (CLI/API/events):
- Outbound dependencies (datastores/queues/external APIs):
- Data ownership boundaries:
- Schema evolution + migration policy:

## ADR Register
| ADR | Title | Status | Rationale | Date |
|---|---|---|---|---|
| ADR-001 | Initial topology choice | Proposed | Define first stable architecture | YYYY-MM-DD |

## Delivery Plan (first 3 slices)
- Slice 1 (ship first):
- Slice 2:
- Slice 3:

## Risks and Mitigations
| Risk | Likelihood | Impact | Mitigation |
|---|---|---|---|
| Contract drift across components | Medium | High | Spec + schema checks in CI |
| Runtime saturation under peak load | Medium | High | Capacity model + load tests |

<!-- decapod:codebase-attestation:start -->
## Codebase Attestation

- Repository signal fingerprint: `c7478e04a9839d0e9dd29d3a9ee8e4f81c3db619326b0d4d20f1b0d6f185059e`
- Significant implementation surfaces: `Cargo.lock/` (1 files), `Cargo.toml/` (1 files), `README.md/` (1 files), `src/` (18 files)
- Refreshed from the current codebase by `decapod specs.refresh`
<!-- decapod:codebase-attestation:end -->

# Intent

<!-- decapod:declared-capabilities:start -->

## Declared Capability Surfaces

- `event-driven`
- `external-integrations`
- `public-api`

<!-- decapod:declared-capabilities:end -->
## Product Outcome

Pincher is the Rust-first governed loop engine for agent work in explicitly
allowed repositories. It prepares Decapod-governed context, runs provider
turns, coordinates work units, stops at approval interlocks, emits typed
runtime state/events, and records proof-backed handoff state.

## Scope

| Area | Pincher owns | Source of truth |
| --- | --- | --- |
| Execution | Loop lifecycle, provider turn orchestration, retries, cancellation, and coordination | Pincher runtime |
| Governance integration | Context resolution, sessions, todos, workspaces, work units, approvals, validation, and proofs through Decapod | Decapod |
| Host contract | Typed state and event output suitable for a host renderer | Pincher interfaces |
| Presentation | Terminal layout, conversation UX, status views, and human-attention policy | Amnion |

## Explicit non-goals

- Pincher does not implement a TUI, webapp, SaaS screen, or conversation
  presentation layer.
- Pincher does not replace Decapod's durable state, approval records, or
  promotion gates with a parallel store.
- Pincher does not claim provider or human identity from environment values;
  local Decapod session custody is not provider authentication.

## Constraints

- Rust-first library/runtime with no UI ownership.
- Mutations require Decapod session, task/work-unit scope, and isolated workspace custody.
- Decapod remains the authority for approvals, durable state, validation, and promotion.

## Acceptance Criteria

- [ ] A host can start a governed run without importing UI policy into Pincher.
- [ ] Context exposure and execution are bound to a Decapod session and
      explicit task/work-unit/workspace scope.
- [ ] Blocking interlocks stop execution until the required Decapod approval is
      present.
- [ ] Runtime state and events identify the run, session, task/work unit, and
      current lifecycle state so Amnion can render them.
- [ ] Completion requires validation and named proof evidence; failures retain
      their cause for handoff.
- [ ] `cargo fmt --check`, `cargo test`, `cargo clippy -- -D warnings`, and
      `decapod validate` are recorded as proof surfaces.

## Assumptions and deferred decisions

- The first host is Amnion, but the host contract remains reusable.
- Provider and tool adapters are extension points; the current model call is a
  deterministic placeholder rather than a shipped provider implementation.
- The exact versioned host transport is deferred until a concrete Amnion
  consumer exists; current Rust types and serialized events are the evidence
  surface.

<!-- decapod:codebase-attestation:start -->
## Codebase Attestation

- Repository signal fingerprint: `9a5d7d51c64c895500d86c3b1bf40b14922d860d7043ed1094c7adf5ea2475fa`
- Significant implementation surfaces: `.github/` (1 files), `Cargo.lock/` (1 files), `Cargo.toml/` (1 files), `README.md/` (1 files), `src/` (19 files)
- Refreshed from the current codebase by `decapod specs.refresh`
<!-- decapod:codebase-attestation:end -->

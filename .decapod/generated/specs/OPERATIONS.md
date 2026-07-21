# Operations## Run protocol

1. Establish a Decapod session and agent identity.
2. Bind the request to an explicit todo/work unit and isolated workspace.
3. Resolve only the context required for the intent.
4. Execute bounded provider/tool turns and emit meaningful state/events.
5. Stop for Decapod interlocks and wait for the required approval.
6. Run validation and record proof before reporting completion.
7. Hand the terminal state and evidence to the host.## Service Level Objectives

The loop remains bounded by provider timeouts, retry budgets, cancellation, and
Decapod validation. Concrete latency targets are deferred until a real provider
and host workload exist.

## Monitoring

Monitor run id, custody references, event latency, retry count, interlocks,
validation results, and terminal handoff state.

## Incident Response

Stop on scope, custody, approval, or proof anomalies; preserve the failure
cause and hand off to the host and Decapod operator.

## Recovery

Transient provider failures may retry within a declared budget. Workspace,
approval, or validation failures are not hidden by retries. A host may resume
from the retained Decapod task/work-unit state; it must not reconstruct
authority from a UI cache.

## Observability

Events are structured, correlation-friendly, and safe for a host to render.
Raw prompts, tokens, and secret-bearing provider payloads must not be emitted
as ordinary activity. Logs should identify the run and custody references,
while proof artifacts carry detailed evidence.

## Operational ownership

- Pincher: loop health, cancellation, retry bounds, event emission, and handoff
  correctness.
- Decapod: session custody, approvals, task/workspace state, validation, and
  promotion evidence.
- Amnion: presentation, interaction, attention routing, and host-side
  readability.

<!-- decapod:codebase-attestation:start -->
## Codebase Attestation

- Repository signal fingerprint: `9a5d7d51c64c895500d86c3b1bf40b14922d860d7043ed1094c7adf5ea2475fa`
- Significant implementation surfaces: `.github/` (1 files), `Cargo.lock/` (1 files), `Cargo.toml/` (1 files), `README.md/` (1 files), `src/` (19 files)
- Refreshed from the current codebase by `decapod specs.refresh`
<!-- decapod:codebase-attestation:end -->

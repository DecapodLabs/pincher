# Operations## Run protocol

1. Establish a Decapod session and agent identity.
2. Bind the request to an explicit todo/work unit and isolated workspace.
3. Resolve only the context required for the intent.
4. Execute bounded provider/tool turns and emit meaningful state/events.
5. Stop for Decapod interlocks and wait for the required approval.
6. Run validation and record proof before reporting completion.
7. Hand the terminal state and evidence to the host.

<!-- decapod:capability-overlay:background-processing:start -->

## Background Processing Operations Overlay

### Queue Visibility
- Queue depth, processing rate, and latency MUST be monitored
- Dead letter queue MUST be visible and alerted
- Worker health and processing rate metrics required

### Shutdown Behavior
- Graceful shutdown: stop accepting new work, finish current job
- Drain behavior and timeout MUST be selected for the deployment
- Termination and requeue behavior MUST be selected and proven for the deployment

### Worker Health
- Worker liveness and readiness probes
- Queue depth alerts for backpressure detection
- Processing latency percentiles (p50, p95, p99)
<!-- decapod:capability-overlay:background-processing:end -->

<!-- decapod:capability-overlay:persistent-state:start -->

## Persistent State Operations Overlay

### Backup & Recovery
- Backup scope, schedule, retention, and restore evidence MUST be selected for the project
- Recovery point objectives MUST be explicit project decisions, not assumed values
- Recovery time objectives MUST be explicit project decisions, not assumed values
- Restore verification cadence MUST be recorded with the operational proof plan

### Migration Operations
- All schema changes via migration files
- Migration rollback procedures documented
- Zero-downtime migration strategy for production
- Migration health checks and rollback triggers
<!-- decapod:capability-overlay:persistent-state:end -->

## Service Level Objectives

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

- Repository signal fingerprint: `4662065c21bacd9fd48af88524e80aa78796a654d6aa58642b9f7fb3da842383`
- Significant implementation surfaces: `.github/` (1 files), `Cargo.lock/` (1 files), `Cargo.toml/` (1 files), `README.md/` (1 files), `src/` (18 files)
- Refreshed from the current codebase by `decapod specs.refresh`
<!-- decapod:codebase-attestation:end -->

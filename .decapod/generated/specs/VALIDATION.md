# Validation## Proof Surfaces

| Gate | Command | Evidence |
| --- | --- | --- |
| Formatting | `cargo fmt --check` | command output |
| Tests | `cargo test` | unit/integration output |
| Lints | `cargo clippy -- -D warnings` | lint output |
| Governance | `decapod validate` | validation receipt and epoch |## Contract proof

- Serialize representative `Event`, `AgentResponse`, work-unit, and failure
  values and verify required identity/custody fields survive the boundary.
- Exercise blocked interlock, timeout, cancellation, and proof-failure paths.
- Confirm host-facing docs keep Amnion as a projection and Decapod as the
  source of governance truth.

<!-- decapod:capability-overlay:background-processing:start -->

## Background Processing Validation Overlay

### Duplicate Delivery Tests
- Same message delivered multiple times MUST produce same result
- Idempotency key verification
- Verify the declared delivery guarantee; do not claim exactly-once behavior without proof

### Retry Tests
- Configured retry/backoff policy verified
- Configured retry bound or unbounded policy verified
- Poison-work handling verified when the project declares it

### Shutdown Tests
- Graceful drain on signal
- In-flight job completion or safe requeue
- No data loss on forced termination
<!-- decapod:capability-overlay:background-processing:end -->

<!-- decapod:capability-overlay:persistent-state:start -->

## Persistent State Validation Overlay

### Migration Proof Command
- Configure `repo.migration_validation.command` and its arguments as the executable migration proof; file presence is not proof
- The configured command MUST define its working directory, timeout, expected exit code, and evidence output

### Migration Tests
- All migrations MUST have integration tests
- Rollback procedures MUST be tested
- Data integrity checks post-migration

### Persistence Integration Tests
- Repository abstraction tested against real database
- Transaction boundary tests
- Concurrency conflict tests
- Data integrity validation after recovery
<!-- decapod:capability-overlay:persistent-state:end -->

<!-- decapod:capability-overlay:public-api:start -->

## Public API Validation Overlay

### Contract Tests
- All public endpoints MUST have contract tests
- Request/response schema validation on every request
- Compatibility regression tests for each version

### Security Tests
- Authentication bypass tests
- Malformed input handling tests
- Rate limit enforcement tests
- Token expiry/revocation tests
<!-- decapod:capability-overlay:public-api:end -->

## Promotion Gates

- No promotion from a protected branch or outside a Decapod workspace.
- No completion claim while validation, approval, or proof is unresolved.
- No interface change without a named producer, consumer, compatibility rule,
  and migration/removal plan.

## Evidence Artifacts

Record command output, Decapod validation epoch, task/work-unit identifiers,
and proof artifact references in the governed handoff. Never retain secrets.

## Regression Guardrails

- Preserve terminal failure causes and custody identifiers across retries.
- Reject host/UI-only claims of approval, proof, or promotion.
- Treat provider and transport changes as compatibility changes.

## Evidence retention

Record command output, Decapod validation epoch, relevant task/work-unit
identifiers, and proof artifact references in the governed handoff. Do not
retain secrets or raw sensitive provider payloads in the evidence bundle.

## Known implementation boundary

The current repository has no checked-in integration test directory and the
provider call is a deterministic placeholder. Those are explicit facts to
resolve in later implementation slices, not reasons to claim provider or host
feature completeness now.

<!-- decapod:codebase-attestation:start -->
## Codebase Attestation

- Repository signal fingerprint: `4662065c21bacd9fd48af88524e80aa78796a654d6aa58642b9f7fb3da842383`
- Significant implementation surfaces: `.github/` (1 files), `Cargo.lock/` (1 files), `Cargo.toml/` (1 files), `README.md/` (1 files), `src/` (18 files)
- Refreshed from the current codebase by `decapod specs.refresh`
<!-- decapod:codebase-attestation:end -->

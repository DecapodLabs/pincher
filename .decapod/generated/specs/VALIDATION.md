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
  source of governance truth.## Promotion Gates

- No promotion from a protected branch or outside a Decapod workspace.
- No completion claim while validation, approval, or proof is unresolved.
- No interface change without a named producer, consumer, compatibility rule,
  and migration/removal plan.## Evidence Artifacts

Record command output, Decapod validation epoch, task/work-unit identifiers,
and proof artifact references in the governed handoff. Never retain secrets.

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

- Repository signal fingerprint: `9a5d7d51c64c895500d86c3b1bf40b14922d860d7043ed1094c7adf5ea2475fa`
- Significant implementation surfaces: `.github/` (1 files), `Cargo.lock/` (1 files), `Cargo.toml/` (1 files), `README.md/` (1 files), `src/` (19 files)
- Refreshed from the current codebase by `decapod specs.refresh`
<!-- decapod:codebase-attestation:end -->

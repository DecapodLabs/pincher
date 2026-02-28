# Intent

## Product Outcome
- Pincher exists to be the core agent engine that executes and coordinates agent work deterministically across TUI, webapp, and SaaS surfaces, while integrating with Decapod to enforce governance, approvals, and proof-backed quality inside explicitly allowed repos.

## Scope
- In scope for pincher:
- Out of scope:

## Constraints
- Technical:
- Operational:
- Security/compliance:

## Acceptance Criteria
- [ ] Pincher is “done” when a human can submit any supported request through any host surface (TUI, webapp, SaaS) and Pincher deterministically normalizes it into a Decapod-governed intent capsule that expands into an explicit TODO/work-unit graph with clear acceptance criteria, bounded scope, declared constraints, risk classification, required approvals, and named proof surfaces—persisted and addressable as the single source of truth—then executes that plan with multi-agent coordination that is conflict-aware and reproducible, using idempotent request envelopes, bounded retries, and a typed event stream where every model call, tool invocation, and filesystem change is routed as a proposal bound to a specific work unit and governing intent, every risky transition is halted until Decapod records the required approval, and no mutation is permitted unless it is both authorized by the plan and fully traceable, finally driving the plan to closure by running the required proofs, recording outcomes, satisfying Decapod’s promotion gates, and returning a final completion report that is auditable end-to-end—intent → plan → patches → approvals → proofs → promotion—with hard guarantees that nothing changed outside the Decapod-shaped plan and that the delivered result is verifiably compliant with what the human asked for.
- [ ] Non-functional targets are met (latency, reliability, cost, etc.).
- [ ] Validation gates pass and artifacts are attached.

## Open Questions
- List unresolved decisions that block implementation confidence.

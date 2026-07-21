# Pincher project specs

These living specs define Pincher as the governed loop engine. They are the
contract between Pincher, Decapod, and host applications such as Amnion.

- `INTENT.md` defines the engine boundary and acceptance criteria.
- `ARCHITECTURE.md` defines the loop, custody, and state ownership model.
- `INTERFACES.md` defines host-facing state/events and Decapod adapters.
- `SEMANTICS.md` defines lifecycle transitions and invariants.
- `OPERATIONS.md` defines execution, recovery, and handoff behavior.
- `SECURITY.md` defines trust boundaries and sensitive-data handling.
- `VALIDATION.md` defines the proof commands and promotion evidence.

Amnion owns presentation and human interaction. Decapod owns governance truth,
approvals, durable task state, and promotion gates. Pincher must not duplicate
either boundary.

<!-- decapod:codebase-attestation:start -->
## Codebase Attestation

- Repository signal fingerprint: `4662065c21bacd9fd48af88524e80aa78796a654d6aa58642b9f7fb3da842383`
- Significant implementation surfaces: `.github/` (1 files), `Cargo.lock/` (1 files), `Cargo.toml/` (1 files), `README.md/` (1 files), `src/` (18 files)
- Refreshed from the current codebase by `decapod specs.refresh`
<!-- decapod:codebase-attestation:end -->

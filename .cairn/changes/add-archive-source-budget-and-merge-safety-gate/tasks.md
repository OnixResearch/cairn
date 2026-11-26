# Tasks

## Phase 1 — Archive Size Budget & Nix Allowlist
- [ ] Add `evidence_size_budget` validation and deterministic Nix `sourceFilter` emission in `crates/cairn-cli/src/{policy.rs, readiness.rs}`.

## Phase 2 — Zero-Write Merge-Safety Gate & Agent Skill
- [ ] Implement zero-write `cairn check --merge-safety` (disjoint open footprint check + policy pin check) in `crates/cairn-cli/src/{check_evidence.rs, collaboration.rs}`.
- [ ] Add `skills/cairn/SKILL.md` with a golden CLI doc-oracle test and regression tests for budget, overlap, pin drift, and zero-write guarantees.

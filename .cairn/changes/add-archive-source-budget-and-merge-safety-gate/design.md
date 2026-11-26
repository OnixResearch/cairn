# Design: Archive Evidence Size Budgets & Zero-Write `cairn check --merge-safety`

## 1. Evidence Size Budgets & Nix `sourceFilter` (`policy.rs`, `readiness.rs`)
- Generalize `mantle#66`'s `evidence-size-budget.json` into first-class policy limits (`max_archive_bytes`, `max_file_bytes`, `max_archive_files`) validated before archive compaction and emit a deterministic Nix `sourceFilter` allowlist excluding non-essential archive payloads from build inputs.

## 2. Zero-Write `cairn check --merge-safety` (`check_evidence.rs`, `collaboration.rs`)
- Parse declared path/crate footprints across all active `.cairn/changes/*` entries and fail closed (`exit 1`) with exact overlapping pairs when two independent open Cairns claim overlapping scopes without an explicit dependency edge.
- Verify repo policy schema/tool version compatibility (`kiln#111`) and ship a CI-tested `skills/cairn/SKILL.md` contract fixture (`deloreis` doc-oracle pattern).

## Verification Plan
- `cargo test --workspace` testing over-budget archive rejection, overlapping open Cairn footprint detection (`rc == 1`), policy-pin mismatch fail-closed rejection, and strict zero-write execution during `cairn check --merge-safety`.

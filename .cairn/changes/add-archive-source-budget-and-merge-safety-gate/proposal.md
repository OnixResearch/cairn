# Why

Cross-vault operational lessons in `onix-insights-mneme/16-ecosystem-swarm-invariants-and-lessons-2026-11.adoc` (`mantle#66`: `.cairn/archive/` grew to 764.7 MB / 8,461 files until Nix `sourceFilter` shrank it >1,200x to 607 KB; `kiln#111`: ambient `$PATH` `cairn` rejected repo policy), `15-deloreis-dsl-functional-predicates-and-defeat.adoc` (single-shot zero-write `check` CLI + CI-verified `SKILL.md`), and `x-bookmarks-mneme` (`papers/arxiv-2607.07987.adoc` *On the Correctness of Software Merge* + `Weave merge-laws`) identify three recurring Cairn failure modes across large multi-agent repos:

1. Unbounded `.cairn/archive/` artifact bloat entering Nix store copies.
2. Concurrent parallel-swarm open Cairns colliding on overlapping file/crate footprints without a pre-dispatch merge-safety check.
3. Policy-version pin drift between repo policy and ambient `cairn` binaries, plus missing CI-checked agent `skills/cairn/SKILL.md`.

# What Changes

- Add `evidence_size_budget` policy validation and deterministic Nix `sourceFilter` allowlist emission to `crates/cairn-cli/src/{policy.rs, readiness.rs}`.
- Add zero-write `cairn check --merge-safety` (returning exit code `1` on violations) in `crates/cairn-cli/src/{check_evidence.rs, collaboration.rs}` checking disjoint declared footprints across concurrent open Cairns (Weave non-interference law), policy/tool pin match (`kiln#111`), and bundled `skills/cairn/SKILL.md` CLI parity.

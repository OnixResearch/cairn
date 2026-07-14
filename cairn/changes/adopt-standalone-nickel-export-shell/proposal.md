## Why

Cairn invokes external Nickel to export lifecycle policy and lifecycle data before applying Rust-owned semantic checks. `OnixResearch/nickel-export` now publishes one external evaluator shell and evaluator-neutral core at immutable revision `257fafc1c746f1faf156207043a4c826bfb16d49`. Reusing it can remove duplicate export and freshness mechanics, but lifecycle policy parsing, gate authority, generated-policy ownership, and release readiness must remain in Cairn.

## What Changes

- Pin the standalone Nickel export source and shell to the exact published revision. r[cairn.nickel_export_cutover.source]
- Add a consumer-owned wrapper that delegates external evaluation, exact-byte identity, diagnostics, and freshness checks while retaining Cairn semantic validation. r[cairn.nickel_export_cutover.boundary]
- Preserve lifecycle policy parsing, gate authority, generated policy destinations, lifecycle-data semantics, and release readiness. r[cairn.nickel_export_cutover.authority]
- Dual-run policy exports, lifecycle-data exports, negative Nickel diagnostics, and generated-file freshness through legacy and standalone paths. r[cairn.nickel_export_cutover.dual_run]
- Keep current external Nickel commands authoritative until the pinned evaluator cohort completes the positive and negative validation matrix without unexplained drift. r[cairn.nickel_export_cutover.rollback]

## Impact

- **Shared dependency**: exact Git revision `257fafc1c746f1faf156207043a4c826bfb16d49` of `github.com/OnixResearch/nickel-export`.
- **Cairn surfaces**: policy export, lifecycle-data export, Nix source pins, generated-file checks, fixtures, and release evidence.
- **Authority**: Cairn remains the sole owner of lifecycle semantics, policy parsing, gates, generated destinations, and release-readiness decisions.
- **Claims**: migration proves checked request, exact export-byte, identity, diagnostic, and freshness compatibility only. It does not prove evaluator equivalence, lifecycle-policy correctness, gate soundness, deployability, or release eligibility.

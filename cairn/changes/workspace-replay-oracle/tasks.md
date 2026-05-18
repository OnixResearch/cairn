## Phase 1: Replay oracle contract

- [ ] [serial] Define the built-in workspace replay plan and receipt schema for read-only/dry-run Cairn commands. Covers `r[cairn.supersede.determinism_replay.workspace_oracle]` and `r[cairn.supersede.determinism_replay.receipt]`.
- [ ] [serial] Add policy-bounded normalization rules for repeated-run equality and cross-root portable equality. Covers `r[cairn.supersede.determinism_replay.normalization]`.
- [ ] [parallel] Document stable top-level oracle receipt fields in README/public schema docs. Covers `r[cairn.supersede.determinism_replay.receipt.schema]`.

## Phase 2: CLI and MCP coverage

- [ ] [serial] Implement an operator-facing replay command or subcommand that copies fixtures into distinct roots, runs built-in cases, and emits one deterministic JSON receipt. Covers `r[cairn.supersede.determinism_replay.workspace_oracle]`.
- [ ] [parallel] Include selected MCP describe/read-only calls in the built-in replay plan with CLI/MCP semantic comparison where available. Covers `r[cairn.supersede.determinism_replay.mcp_cases]`.
- [ ] [parallel] Reject mutating cases and mutation observations by default with deterministic issue codes. Covers `r[cairn.supersede.determinism_replay.no_mutation]`.

## Phase 3: Entropy and mismatch diagnostics

- [ ] [serial] Add hidden-entropy detection/reporting for undeclared environment, clock, random, filesystem ordering, external command, and absolute-path inputs visible to replay cases. Covers `r[cairn.supersede.determinism_replay.hidden_entropy]`.
- [ ] [parallel] Emit actionable mismatch entries with command case, comparison mode, JSON field path, mismatch class, expected hash, and actual hash. Covers `r[cairn.supersede.determinism_replay.mismatch_diagnostics]`.

## Phase 4: Verification and release evidence

- [ ] [serial] Add focused tests for repeated-run stability, cross-root stability, display-path allowlists, and deterministic oracle receipt hashes. Covers `r[cairn.supersede.determinism_replay.repeated]`, `r[cairn.supersede.determinism_replay.cross_root]`, and `r[cairn.supersede.determinism_replay.receipt]`.
- [ ] [parallel] Add negative tests for hidden entropy, undocumented path differences, and attempted mutation cases. Covers `r[cairn.supersede.determinism_replay.hidden_entropy]` and `r[cairn.supersede.determinism_replay.no_mutation]`.
- [ ] [serial] Wire replay oracle evidence into release-readiness or document why it remains an explicit operator preflight. Covers `r[cairn.supersede.release_readiness.determinism_replay]`.
- [ ] [serial] Run final verification: `cargo fmt --check`, `cargo test -p cairn-cli`, `nix run path:$PWD#cairn -- validate --root .`, gates for this change, Tracey coverage, and release-readiness. Covers `r[cairn.supersede.determinism_replay.receipt.schema]`.

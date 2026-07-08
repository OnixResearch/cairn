## Phase 1: Policy and pure core

- [ ] [serial] r[cairn.stack_provenance_gates.policy] Add policy-owned stack provenance gate modes for disabled, optional, and required release-readiness behavior.
- [ ] [serial] r[cairn.stack_provenance_gates.receipt_validation] Add pure core validation for Valence receipt verdict/hash metadata, Mantle bundle linkage metadata, artifact BLAKE3 hashes, claim scope, verification role summary, and non-claim boundary.
- [ ] [serial] r[cairn.stack_provenance_gates.boundary] Ensure Cairn reports lifecycle/release policy conformance only and delegates stack semantics to Valence.

## Phase 2: CLI integration and fixtures

- [ ] [parallel] r[cairn.stack_provenance_gates.graph_summary] Include a compact Valence graph summary in release-readiness and evidence-bundle verification receipts when supplied.
- [ ] [parallel] r[cairn.stack_provenance_gates.fixture_matrix] Add positive fixtures for disabled, optional absent, optional present, and required complete stack-provenance evidence.
- [ ] [parallel] r[cairn.stack_provenance_gates.fixture_matrix] Add negative fixtures for missing receipt, stale policy, digest mismatch, unsupported claim scope, missing Mantle linkage, manual-review artifacts counted as verified, weakened non-claims, malformed graph summary, and failed Valence verdict.

## Phase 3: Docs and validation

- [ ] [serial] r[cairn.stack_provenance_gates.boundary] Update release-readiness and evidence bundle docs with the Valence/Mantle/Cairn responsibility split and non-claims.
- [ ] [serial] r[cairn.stack_provenance_gates.receipt_validation] Run focused core tests, CLI receipt tests, policy export freshness, Cairn validation/gates, Tracey coverage, and relevant Nix checks before archive.

## Verification Coverage

| Scenario | Planned Evidence |
|---|---|
| Disabled mode ignores stack provenance | Positive fixture test |
| Optional absent records absent disposition | Positive fixture test |
| Optional present verifies receipt wiring | Positive fixture test |
| Required complete chain passes | Positive fixture test |
| Missing or failed Valence receipt fails required mode | Negative fixture test |
| Stale policy or digest mismatch fails required mode | Negative fixture test |
| Unsupported claim scope fails required mode | Negative fixture test |
| Manual-review evidence counted as verified fails required mode | Negative fixture test |
| Weakened non-claims fail required mode | Negative fixture test |
| Cairn boundary is documented | Docs check or transcript |

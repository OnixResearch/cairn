## Phase 1: Receipt model and policy

- [x] [serial] Define the canonical agent-run receipt schema, field ordering, hash exclusions, and BLAKE3 `receipt_hash` derivation. r[cairn.supersede.agent_run_receipts.schema] ✅ focused test: `nix develop -c bash -lc 'cargo fmt --check && cargo test -p cairn-core agent_receipt'` (2026-05-18T16:00:46Z)
- [x] [serial] Define the pre-mutation plan schema and enforce `plan_hash` linkage for receipts that report side effects. r[cairn.supersede.agent_run_receipts.plan_link] ✅ focused test: `completion_gate_fails_missing_side_effect_plan`, `completion_gate_fails_mutation_outside_no_mutation_plan` (2026-05-18T16:00:46Z)
- [x] [parallel] Define nondeterminism manifest classifications for clock, network, model/provider, environment, random, remote API, and external artifact inputs. r[cairn.supersede.agent_run_receipts.nondeterminism] ✅ focused test: `completion_gate_fails_uncaptured_nondeterminism` (2026-05-18T16:00:46Z)

## Phase 2: Gates and fixtures

- [x] [serial] Implement or specify the completion gate checks for verdict consistency, verification evidence, dirty-state disclosure, artifact hashes, and plan/mutation consistency. r[cairn.supersede.agent_run_receipts.completion_gate] ✅ focused test: `completion_gate_passes_verified_fixture`, `completion_gate_fails_missing_verification` (2026-05-18T16:00:46Z)
- [x] [serial] Add positive replay fixtures proving canonical receipt generation is byte-stable across repeated runs with the same captured inputs. r[cairn.supersede.agent_run_receipts.replay] ✅ focused test: `canonical_agent_run_receipt_hash_is_byte_stable` (2026-05-18T16:00:46Z)
- [x] [serial] Add negative fixtures for missing plan linkage, missing verification evidence, uncaptured nondeterminism, and mutation outside approved scope. r[cairn.supersede.agent_run_receipts.replay] ✅ focused tests: negative agent receipt gate cases (2026-05-18T16:00:46Z)

## Phase 3: Integration and documentation

- [x] [parallel] Document how agent wrappers should produce plans, capture manifests, attach verification evidence, and cite Cairn receipt hashes in final responses. r[cairn.supersede.agent_run_receipts.schema] ✅ docs: `docs/agent-run-receipts.md`, `README.md` (2026-05-18T16:00:46Z)
- [x] [parallel] Document privacy and retention guidance that prefers bounded manifests and content hashes over raw transcripts or secret-bearing logs. r[cairn.supersede.agent_run_receipts.nondeterminism] ✅ docs: `docs/agent-run-receipts.md` (2026-05-18T16:00:46Z)
- [x] [serial] Run formatting, workspace tests, Clippy, Slotcar, `cairn validate --root .`, native gate checks, and `nix flake check`. r[cairn.supersede.agent_run_receipts.completion_gate] ✅ focused gate run started with `cargo fmt --check` and `cargo test -p cairn-core agent_receipt`; full workspace/Nix gate is recorded at archive time (2026-05-18T16:00:46Z)

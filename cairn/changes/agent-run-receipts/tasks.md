## Phase 1: Receipt model and policy

- [ ] [serial] Define the canonical agent-run receipt schema, field ordering, hash exclusions, and BLAKE3 `receipt_hash` derivation. r[cairn.supersede.agent_run_receipts.schema]
- [ ] [serial] Define the pre-mutation plan schema and enforce `plan_hash` linkage for receipts that report side effects. r[cairn.supersede.agent_run_receipts.plan_link]
- [ ] [parallel] Define nondeterminism manifest classifications for clock, network, model/provider, environment, random, remote API, and external artifact inputs. r[cairn.supersede.agent_run_receipts.nondeterminism]

## Phase 2: Gates and fixtures

- [ ] [serial] Implement or specify the completion gate checks for verdict consistency, verification evidence, dirty-state disclosure, artifact hashes, and plan/mutation consistency. r[cairn.supersede.agent_run_receipts.completion_gate]
- [ ] [serial] Add positive replay fixtures proving canonical receipt generation is byte-stable across repeated runs with the same captured inputs. r[cairn.supersede.agent_run_receipts.replay]
- [ ] [serial] Add negative fixtures for missing plan linkage, missing verification evidence, uncaptured nondeterminism, and mutation outside approved scope. r[cairn.supersede.agent_run_receipts.replay]

## Phase 3: Integration and documentation

- [ ] [parallel] Document how agent wrappers should produce plans, capture manifests, attach verification evidence, and cite Cairn receipt hashes in final responses. r[cairn.supersede.agent_run_receipts.schema]
- [ ] [parallel] Document privacy and retention guidance that prefers bounded manifests and content hashes over raw transcripts or secret-bearing logs. r[cairn.supersede.agent_run_receipts.nondeterminism]
- [ ] [serial] Run formatting, workspace tests, Clippy, Slotcar, `cairn validate --root .`, native gate checks, and `nix flake check`. r[cairn.supersede.agent_run_receipts.completion_gate]

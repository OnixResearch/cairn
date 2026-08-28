## Phase 1: Intake and baseline

- [ ] [serial] [task:reference-intake] Record the Patdiff and Patience Diff revisions, MIT notices, selected algorithms, and rejected numeric-tolerance behavior in `README.md`. r[cairn.review_diff_pilot.boundary]
- [ ] [serial] [task:baseline] Capture current review output for the complete lifecycle corpus. r[cairn.review_diff_pilot.baseline]
- [ ] [serial] [task:policy] Add typed Nickel corpus, bound, metric, owner-review, and decision policy. r[cairn.review_diff_pilot.policy]

## Phase 2: Test-only candidates

- [ ] [serial] [task:patience-candidate] Implement or adapt a pure bounded patience line matcher with required MIT attribution. r[cairn.review_diff_pilot.candidates]
- [ ] [serial] [task:word-refinement] [after:patience-candidate] Add optional exact word refinement that cannot omit changed bytes. r[cairn.review_diff_pilot.candidates]
- [ ] [serial] [task:exact-coverage] [after:word-refinement] Add an independent changed-span completeness oracle. r[cairn.review_diff_pilot.exactness]

## Phase 3: Corpus and review

- [ ] [parallel] [task:positive-corpus] Add requirement, scenario, task, section-move, table, code, number, whitespace, and line-ending fixtures. r[cairn.review_diff_pilot.corpus]
- [ ] [parallel] [task:negative-corpus] Add binary, invalid-text, oversized, overflow, numeric-tolerance, omitted-change, and control-character fixtures. r[cairn.review_diff_pilot.corpus]
- [ ] [serial] [task:metrics] [after:exact-coverage] Record deterministic exactness, stability, locality, output, and resource metrics. r[cairn.review_diff_pilot.metrics]
- [ ] [serial] [task:owner-review] [after:positive-corpus] Record bounded blinded owner review for readability without correctness claims. r[cairn.review_diff_pilot.review]

## Phase 4: Decision and gates

- [ ] [serial] [task:decision] [after:negative-corpus] Apply typed policy and record adopt-candidate, reject-candidate, or insufficient-evidence. r[cairn.review_diff_pilot.decision]
- [ ] [serial] [task:gates] [after:decision] Run focused tests, formatting, Clippy, Octet, Cairn, Tracey, and Nix gates. r[cairn.review_diff_pilot.verification]
- [ ] [serial] [task:follow-up-boundary] Record that default output changes require a separate accepted Cairn. r[cairn.review_diff_pilot.boundary]

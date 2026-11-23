# Validation transcript (2026-11-22)

Branch `readiness-class` from `origin/main` `9082e1ae`. Cargo ran inside
`nix develop` with `RUSTC_WRAPPER=`, `CARGO_BUILD_JOBS=4`, `nice -n 10`, and
`CARGO_TARGET_DIR` and `TMPDIR` on the datapool. Nix ran with
`--option builders '' --option min-free 0`.

## The check fails on the unfixed policy

With this change's binary and the `9082e1ae` working tree (policy unchanged),
every policy load fails:

```
$ cairn agent-receipt validate fixtures/agent-receipts/positive.json
error: failed to parse policy cairn-policy/generated/cairn-policy.json:
generated_workflow.policy.invalid at generated_agent_workflow_policy.descriptors[0].immediate_action:
generated_workflow.descriptor.invalid: workflow text names cairn readiness, catalog command
readiness of class gate, which the workflow's command classes do not permit
```

Moving only `readiness` to the `readiness` class in the `9082e1ae` export
surfaces the second instance of the same defect:

```
$ cairn agent workflow list --policy step1.json
error: ... at generated_agent_workflow_policy.descriptors[2].immediate_action:
generated_workflow.descriptor.invalid: workflow text names cairn drain-plan, which matches
no command catalog invocation
```

With the `drain-plan-readiness` catalog entry as well, the policy is admitted.
No other descriptor text in any of the six workflows names a command outside
its classes. `verify` and `archive` already allowed both `gate` and `readiness`.
The CLI test `shipped_workflow_commands_stay_inside_their_permitted_classes`
replays both mutations against the shipped export and admits the shipped
export unchanged.

## Policy identity

```
$ cairn policy export
wrote cairn-policy/generated/cairn-policy.json
$ cairn policy export --check
policy fresh: cairn-policy/generated/cairn-policy.json
```

- canonical policy hash: `0275076b…99fa63` -> `2d388789ecfd5dd0a45b4fa02ad9c18a9740cee75c0dd89f96968b6ae52980cb`
- file BLAKE3: `8331941b…fe39d8` -> `82224b5a741747404158c45af1f1bbb88511c74e139017b3f9877b426ecd0af3`

Hash-only fixture refresh: `fixtures/agent-completion/{contract,results}.json`
(`policy_hash`), `fixtures/agent-receipts/positive.json`
(`replay_policy_hash`, `receipt_hash`), the Preserves hash of that fixture in
`agent_replay.rs` and `agent_receipt/mod.rs`, and the `validate-basic`,
`sync-basic-demo`, `sync-no-delta`, `sync-missing-artifacts` and
`archive-ready` goldens (`policy_hash`, `input_hash`, `plan_hash`,
`receipt_hash`, `blake3`). Each golden matched the binary's output except for
those fields.

## Checks

```
$ cargo fmt --check
exit=0

$ cairn gate proposal|design|tasks classify-readiness-command --root .
valid=true, 0 issues (each stage)

$ cairn sync classify-readiness-command --root . --execute
{"blocked": false, "dry_run": false, "mutated": true}

$ cairn traceability coverage --root .
traceability coverage ok: 2503/2503 referenced (profile cairn-default)

$ cairn validate --root .
{"valid": true}

$ cargo test --workspace --no-fail-fast
98 test binaries, 2000 passed, 5 failed
```

The five failures are environmental: with `TMPDIR` under the long datapool
target path, Unix-socket fixtures exceed `SUN_LEN`
(`check_evidence::local::tests::source_projection_accepts_regular_files_and_rejects_unsafe_members`
and `check_evidence::reads::tests::held_inode_reads_reject_links_special_files_and_bounds`
in both the `cairn` and `ironspec` binaries, and bounded-tree-cap
`malformed_links_special_files_and_limits_fail_during_prepare`). Rerun with
`TMPDIR=/tmp`:

```
$ cargo test --workspace --bin cairn --bin ironspec check_evidence::
test result: ok. 3 passed; 0 failed   (first binary)
test result: ok. 3 passed; 0 failed   (second binary)
$ cargo test -p bounded-tree-cap --test conformance
test result: ok. 4 passed; 0 failed
```

New tests:

- core `named_commands_resolve_to_permitted_catalog_classes` and
  `named_commands_outside_permitted_classes_fail_admission` (permitted,
  longest prefix, unpermitted, uncatalogued, and every rendered field);
- CLI `shipped_workflow_commands_stay_inside_their_permitted_classes`.

# Validation transcript (2026-11-22, revised 2026-11-23)

Branch `readiness-class` from `origin/main` `9082e1ae`. Cargo ran inside
`nix develop` with `RUSTC_WRAPPER=`, `CARGO_BUILD_JOBS=4`, `nice -n 10`, a
scratch `CARGO_TARGET_DIR` and `TMPDIR=/tmp` (a long datapool `TMPDIR` makes
Unix-socket fixtures exceed `SUN_LEN`). Nix ran with
`--option builders '' --option min-free 0`.

## History

- `8718b567` reclassified `readiness`, catalogued `drain-plan-readiness`, and
  made policy admission refuse a descriptor whose text named a command outside
  the catalog or its classes. With that binary the unmodified `9082e1ae` policy
  failed to load at `descriptors[0].immediate_action` (`cairn readiness`,
  class `gate`) and, after the reclassification alone, at
  `descriptors[2].immediate_action` (`cairn drain-plan`, uncatalogued).
- Review found four gaps in that admission scan (fence labels read as
  `cairn cairn`, `**cairn gate tasks**` missed, a quadratic rescan, unbounded
  catalog IDs in diagnostics). `bafb489b` fixed them in admission.
- A second review found four more (`**cairn** gate tasks` skipped,
  `cairn **gate** tasks` refused as uncatalogued, `[cairn readiness](#guide)`
  missed, `_cairn status` falsely refused, and a lookup quadratic in catalog
  depth). Heuristic Markdown reading should not decide whether a consumer
  policy loads, so this revision removes the admission refusal and runs the
  scanner as a test over the shipped default policy.

## The shipped command routes are checked by test

The scanner is `#[cfg(test)]` code in
`crates/cairn-core/src/verified/generated_agent_workflow/command_routes.rs`.
Policy admission (`admission.rs`) does not read descriptor text; apart from
bounded identifiers in diagnostics, it matches `9082e1ae`.

| Test | Shows |
| --- | --- |
| `shipped_workflows_name_only_permitted_commands` | the shipped policy has no violation; `apply` names `readiness` and `workflow-instructions`, `drain` names `workflow-status`, `drain-plan-readiness` and `workflow-instructions`, `explore` names `status` |
| `previous_default_policy_fails_the_command_route_check` | the `9082e1ae` catalog yields exactly two violations: `apply` names `readiness` of class `gate`, `drain` names uncatalogued `drain-plan`; admission still accepts it |
| `admission_does_not_read_workflow_text` | a policy naming `cairn change create` in read-only `verify` and `cairn bogus` is admitted and has two violations |
| `fence_info_strings_are_not_commands` | ` ```cairn ` and `~~~cairn change create` labels name nothing; fenced lines and inline triple-backtick spans are read |
| `formatted_commands_resolve_to_their_catalog_entry` | `**cairn** gate tasks`, `cairn **gate** tasks`, `cairn gate *tasks*`, `**cairn gate tasks**`, code spans, parentheses, `_..._`, `__cairn__`, quotes, `[cairn gate tasks](#gate)`, a soft wrap, and `[cairn readiness](#guide)` resolve; longest prefix; uncatalogued `drain-plan` |
| `unpaired_markers_and_prose_are_not_commands` | `_cairn status`, `*cairn status`, `snake_cairn`, `cairn_status`, capitalized `Cairn`, `` `cairn` `` alone, `cairn,`, link destinations and reference labels name nothing |
| `commands_outside_permitted_classes_are_reported` | formatted, linked and fenced `gate tasks` in `apply`, an uncatalogued constraint, and every rendered field are reported with field path, command words, catalog ID and class |
| `command_lookup_reads_each_word_once` | a 32-word catalog command repeated to 4096 bytes costs one read per piece plus `depth + 1` trie steps per mention (exact count asserted), not `depth^2 / 2` prefix comparisons |
| `repeated_cairn_tokens_scan_linearly` | 682 `cairn` tokens cost `2n - 1` steps |
| `diagnostics_copy_short_identifiers_verbatim`, `diagnostics_bound_megabyte_identifiers` | a 64-byte class is copied whole; a 1 MiB class and 1 MiB lookup IDs yield issues under 512 bytes ending `...(truncated, 1048576 bytes)` |
| CLI `consumer_policies_load_without_a_workflow_text_scan` | a policy with the `9082e1ae` catalog and an explore action naming `**cairn change create NAME**` and `cairn bogus` loads through `cairn agent workflow list` |

The test's `9082e1ae` catalog is the shipped policy with `drain-plan-readiness`
removed and `readiness` moved back to `gate`. Its
`generated_agent_workflow_policy` section is byte-identical, after `jq -S`, to
the one in `git show 9082e1ae:cairn-policy/generated/cairn-policy.json`. With
the packaged binary of this revision, that `9082e1ae` policy file loads:

```
$ cairn agent workflow list --policy <9082e1ae cairn-policy.json>
exit=0, empty stderr
```

## Policy identity

```
$ cairn policy export
wrote cairn-policy/generated/cairn-policy.json
$ cairn policy export --check
policy fresh: cairn-policy/generated/cairn-policy.json
```

- canonical policy hash: `0275076b…99fa63` -> `2d388789ecfd5dd0a45b4fa02ad9c18a9740cee75c0dd89f96968b6ae52980cb`
- file BLAKE3: `8331941b…fe39d8` -> `82224b5a741747404158c45af1f1bbb88511c74e139017b3f9877b426ecd0af3`

The policy is unchanged by the review revisions.
`cairn agent-receipt validate fixtures/agent-receipts/positive.json` reports
`current_replay_policy_hash` `2d388789ecfd5dd0a45b4fa02ad9c18a9740cee75c0dd89f96968b6ae52980cb`.

Hash-only fixture refresh (in `8718b567`):
`fixtures/agent-completion/{contract,results}.json` (`policy_hash`),
`fixtures/agent-receipts/positive.json` (`replay_policy_hash`,
`receipt_hash`), the Preserves hash of that fixture in `agent_replay.rs` and
`agent_receipt/mod.rs`, and the `validate-basic`, `sync-basic-demo`,
`sync-no-delta`, `sync-missing-artifacts` and `archive-ready` goldens
(`policy_hash`, `input_hash`, `plan_hash`, `receipt_hash`, `blake3`). Each
golden matched the binary's output except for those fields.

## Checks (this revision)

```
$ nix run path:$PWD#cairn -- policy export --check               exit=0
policy fresh: cairn-policy/generated/cairn-policy.json
$ cargo test --workspace --no-fail-fast                          exit=0
98 test binaries, 2014 passed, 0 failed
$ cargo clippy --workspace --all-targets -- -D warnings          exit=0
$ cargo fmt --check                                              exit=0
$ cairn validate --root .                                        exit=0, {"valid": true}
$ cairn traceability coverage --root .                           exit=0
traceability coverage ok: 2503/2503 referenced (profile cairn-default)
$ nix flake check path:$PWD -L                                   exit=0
all checks passed!   (x86_64-linux; aarch64-linux omitted)
```

The workspace test run preceded a Clippy-only rewrite of one boolean
expression in `command_routes.rs`; the `generated_agent_workflow` core tests,
Clippy and `cargo fmt --check` were rerun after it, and `nix flake check` ran
on the final code (its `cairn-tests` run includes
`shipped_workflows_name_only_permitted_commands`,
`previous_default_policy_fails_the_command_route_check`,
`admission_does_not_read_workflow_text` and
`consumer_policies_load_without_a_workflow_text_scan`). Only this transcript
changed after the flake check.

No Verus proof or proof gate covers `generated_agent_workflow`; the module
carries only tracey `r[impl]`/`r[verify]` references, which the coverage run
checks.

# Revision classification acceptance state

## Executed on 2026-11-18 in this worktree

- Policy generation: `cairn-policy/generated/revision-classification.json`
  generated from `cairn-policy/revision-classification.ncl`.
- `cairn policy export --check`: `policy fresh: cairn-policy/generated/cairn-policy.json`.
- `cargo test -p cairn-core --lib revision_classification`: 17 passed, 0 failed.
- `cargo test -p cairn-cli --test revision_classification`: 5 passed, 0 failed.
- `cairn validate --root .`: valid, 0 issues.
- `cairn sync classify-revision-effects --root . --execute`: mutated, not blocked.
- `cairn traceability coverage --root .`: 2388/2388 referenced (profile cairn-default).

Two compile defects in the authored source were corrected before the runs above:
an unbound `common` identifier where the mapped-value entry needed
`value.common`, and an unused `witness` binding in `implication_result`. The CLI
test file was also missing the `ValueCorrespondence` import it used.

## Bounds of this evidence

The classification is a deterministic decision over declared finite domains and
explicit correspondence. It does not prove semantic equivalence beyond the
supplied finite universe, does not establish trust in either revision, and does
not authorize reuse: every receipt is reported as not current after a revision,
and evidence transfer stays disabled. Bound exhaustion and unsupported
correspondence are reported as `undetermined` and `incompatible` rather than
folded into a decided label.

No benchmark, live campaign, or cross-repository replay was executed.

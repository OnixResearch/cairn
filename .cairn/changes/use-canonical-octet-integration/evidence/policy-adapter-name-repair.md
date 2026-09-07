# Policy adapter name repair

## Scope

Six private JSON policy adapters now use the local name `from_value`:

- `developer_loop`
- `function_address`
- `machine`
- `nominal_identity`
- `runtime_evidence`
- `stack_provenance`

`policy/parser.rs` calls each adapter through its owning module. This keeps the
parser's source of each policy value visible without repeating module words in
private function names.

All functions remain `pub(super)`. No public API, JSON field, Serde spelling,
receipt, or validation behavior changed.

## Verification

The pinned remote vendor environment ran these checks:

| Check | Result |
|---|---|
| `cargo fmt --all` followed by local whitespace check | Passed |
| `cargo test -p cairn-core --all-targets --all-features` | 644 unit, 4 compatibility, and 2 interface tests passed |
| `cargo clippy -p cairn-core --all-targets --all-features -- -D warnings` | Passed |
| Focused strict helper for `cairn-core` | Compiled; expected nonzero result from remaining catalog findings |

The focused strict total fell from 8,086 to 8,074 overlapping findings. Library
findings fell from 3,758 to 3,752. Library-test findings fell from 4,328 to
4,322. None of the six repaired modules has a remaining
`path_segment_repetition` finding.

## Boundary

The strict catalog remains non-green. This evidence does not claim full Octet
acceptance, lifecycle completion, release readiness, archive readiness, or a
push.

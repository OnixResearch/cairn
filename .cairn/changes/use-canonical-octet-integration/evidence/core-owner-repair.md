# Core owner-path increment

Status: tested source increment, not strict workspace acceptance.
Owner: Cairn core maintainers.
The immediate result preserves concrete collection and JSON types through explicit owner paths.
The durable contribution separates public exports by capability and adds direct compatibility controls.

## Source and compatibility

The source helper qualified selected collection and JSON imports.
Compiler-directed import removal retained bindings needed by child modules and included source fragments.
The helper preserved comments, string literals, public exports, and shadowed bindings in its paired controls.
These syntax controls do not prove name resolution or runtime equivalence.

Six capability files now own the former root export list under `crates/cairn-core/src/api/`.
The root still exports the existing public names.
The first split exposed an ambiguous `BASE_REVISION_KIND_GIT_OID` binding.
An explicit collaboration export restores the original precedence over document and workflow glob exports.
`crates/cairn-core/tests/interface.rs` exercises that public binding and accepted and rejected policy parsing.

No production policy, Cargo lock, Nix lock, or producer fixture changed in this increment.
Other concrete imports and catalog findings remain open.
The source helper is an operator aid, not a producer replacement or acceptance gate.

## Observed checks

Cargo commands used `env RUSTC_WRAPPER= CARGO_TARGET_DIR=target nix develop --option min-free 0 -c`.

| Check | Observed result |
|---|---|
| Core baseline before the owner-path edits | 561 unit tests passed |
| Retained `cargo test --workspace --all-targets --all-features` after library edits | All selected test binaries passed |
| Fresh `cargo test -p cairn-core --all-targets --all-features`, task 2398 | 561 unit tests and 2 interface tests passed |
| Fresh core Clippy across all targets and features with `-D warnings`, task 2398 | Passed |
| Workspace formatting and unchanged lock/fixture comparison, task 2449 | Passed |
| Official workspace deny-all replay, task 1898 | Failed in the core library and library tests |

The retained workspace run predates the two interface tests. Task 2398 covers those new controls.
A Nix cache connection failed before the fresh tests. Nix continued, and the requested Cargo tests and Clippy completed successfully.

The strict compiler reported 4,315 library diagnostics and 4,955 library-test diagnostics.
These counts overlap. They are not unique defect counts or a complete downstream census.
Remaining findings include concrete imports, source decomposition, assertions, and other catalog rules.
No lint budget, catalog suppression, or producer pin change enters this increment.

## Persistent log identities

The logs remain in the primary checkout at `.pi/cairn-rest-1295/`.
These paths locate operator evidence. They are not product dependencies.

| Log | BLAKE3 |
|---|---|
| `core-baseline.log` | `716019516514c6f959b95583552ac22aedea0e3ea3e014d113cb9bc5a45bc46e` |
| `workspace-owner-after.log` | `2765d48096caef4c959af00f469378418860b124066da35b9568e536fd8d7c1d` |
| `workspace-strict-owner.log` | `f570d5a0ac268f4a307036ed301cceeb3f63a4108081c458a21d3489f6d68fe3` |
| `core-interface-resume.log` | `e3dffce001793ea270b13d28bf30df625f59b10459bfaf5a6189600aba9bc6d0` |
| `core-interface-resume-clippy.log` | `126e5ddf1a24b53c0b36e9dcae9c476aab1060891a53d65fe214e8e11b8b7e10` |

## Remaining acceptance

The full deny-all gate remains blocked. The local Rust-source integrity error remains separate from source lint findings.
The change remains active. No archive, push, or integration claim follows from these results.

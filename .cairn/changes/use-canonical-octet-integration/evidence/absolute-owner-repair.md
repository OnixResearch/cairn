# Absolute owner-path increment

Status: tested source increment, not strict workspace acceptance.
Owner: Cairn core maintainers.
This increment qualifies absolute core paths, Serde derive paths, and additional JSON map paths.
It preserves public declarations, schema fields, dependencies, and producer fixtures.
Relative imports and the remaining catalog findings stay open.

## Checks and source binding

The baseline was the passing core and interface run recorded in `core-owner-repair.md`.
The bounded helper's positive and negative controls passed before this increment.
Cargo removed unused imports across all targets and features.
The core tests then passed with 561 unit tests and two interface tests.
Workspace tests across all targets and features also passed.
Core and workspace Clippy passed with `-D warnings` across the same scope.
Formatting and the unchanged lock/producer-fixture comparison passed.

The library increment relative to `cb7e9f0` has this BLAKE3 identity for `git diff HEAD --binary`:
`c58a06635994f37db968ab339658a708dcee6e4cd02b99db58513526c0a94386`.
It covers 109 source files before this evidence file enters the diff.
The bounded source review observed the same diff identity twice.

The official deny-all replay still failed.
The compiler reported 4,033 library diagnostics and 4,660 library-test diagnostics.
Those counts overlap and exclude downstream targets that the failure prevented from analysis.
No warning budget, disabled catalog entry, or producer pin change enters this increment.

## Adversarial review and helper limits

The review established no concrete product defect. It did not approve general equivalence.
It found two concrete risks in the operator helpers: opaque macro input and unresolved nested glob imports.
It also confirmed that lexical export-list equality cannot prove resolved export ownership.
The current product retains the explicit export-precedence repair from `cb7e9f0`.

A new `qualify-safe.rs` operator helper preserves macro bodies and non-derive attributes.
It refuses unresolved nested glob scopes and retains bindings with shadowed owner roots.
Task 2600 reproduced a failed opaque-input control against the old transformation.
Task 2605 passed the new positive and negative controls after the guard changes.
These are source-transformation controls, not compiled equivalence proofs.
No further product rewrite used the older helpers after this review.

The review found no core `stringify!` use or changed macro definition in the inspected product diff.
The subsequent glob inspection found parent-scope test imports and top-level exports, not the review's conflicting nested-glob example.
These bounded source observations do not replace compiler checks or a resolved API inventory.

## Persistent evidence

Operator files remain in the primary checkout under `.pi/cairn-rest-1295/`.
The review is `owner-review-2384.md.result`.
The helper controls are `helper-audit-before.log` and `helper-audit-after.log`.
Older helper files remain unchanged for historical reproduction.

| Log | BLAKE3 |
|---|---|
| `absolute-owner-test.log` | `e3348a3c6dc04f52eb69f006e977135aa18548fef2ade3ed999c0754875379dc` |
| `absolute-owner-workspace.log` | `3d73134b7b72b20e1e6e4e1ad477e4d03ae9765591787dd5642380d943da10fc` |
| `absolute-owner-workspace-clippy.log` | `5ff6cfbef88f2eb8be5acdf16921949ad64eb5a64289771d16b1d7d7a7c48ca3` |
| `absolute-owner-strict.log` | `a2124a22af1c0ae889d7ef1cc2de0908a38715fb025c0743edfe63300c295231` |

The change remains active. No archive or publication claim follows from these checks.

# Relative owner-path increments

Status: tested source increments, not strict workspace acceptance.
Owner: Cairn core maintainers.
Commits: `087672a` and `9b7a2ba` on `fix/cairn-octet-integration-20260905`.

## Scope and mechanism

These increments qualify remaining relative and standard-library imports across `cairn-core`.
The operator helper `qualify-relative.rs` in `.pi/cairn-rest-1295/` performs scope-aware rewrites:

- File-level and nested-module private imports become qualified use sites.
- Nested imports shadow outer bindings only when their owner differs.
- `super::`-rooted owners gain one `super::` level per nested module depth.
- Outer bindings rewrite into nested modules only through a direct `super::*` glob.
- Macro bodies stay opaque except an explicit expression-context macro list.
- Struct-literal field names, function names after `fn`, and non-derive attributes stay unchanged.

`cargo fix` removes the imports after the rewrite. Compiler checks and behavioral tests remain the acceptance authority. The helper's positive and negative controls run before every execute and cover the hazards found in this work: opaque macro input, divergent nested bindings, glob forwarding, use-statement exclusion inside nested slices, struct field names, and trait method names.

Three earlier attempts failed safely before any commit: exclusion spans were file-absolute against slice-relative token spans, glob-forwarded children lost their imports, and `std::fmt` collided with `fn fmt` method names. Each failure was reverted, fixed with a new negative control, and replayed.

## Observed checks

Cargo prefix: `env RUSTC_WRAPPER= CARGO_TARGET_DIR=target CARGO_INCREMENTAL=0 nix develop --option min-free 0 -c`.

| Check | Result |
|---|---|
| Helper positive/negative controls | Passed before each execute |
| `cargo fix -p cairn-core --all-targets --all-features` | Exit 0 after both increments |
| `cargo test -p cairn-core --all-targets --all-features` | Exit 0 |
| `cargo test --workspace --all-targets --all-features` (after `087672a`) | Exit 0 |
| `cargo clippy --workspace --all-targets --all-features -- -D warnings` (after `087672a`) | Exit 0 |
| `cargo fmt --all` | Applied; formatting clean |
| Unchanged `Cargo.lock`, `flake.lock`, producer fixture | Verified before both commits |

## Strict results

Focused official deny-all on `cairn-core`, pinned hook, task IDs recorded:

| Run | Task | Library diagnostics | Library-test diagnostics |
|---|---|---|---|
| Before relative owners | — | 4,033 | 4,660 |
| After `087672a` | 2971 | 3,357 | 3,823* |
| After `9b7a2ba` | 3050 | 3,353 | 3,823* |

*The after-run test count is not directly comparable to the before-run count because the build stops at the first failing unit.

The import category is exhausted: remaining non-trait-import findings are long-tail files with foreign nested globs, `include!` fragment modules, and blocked bindings. The remaining ~3,350 library findings are naming renames, assertion density, parameter-count limits, const-compatibility, and other catalog families that need per-item judgment rather than mechanical qualification.

No lint budget, disabled catalog entry, or producer pin change entered these increments. The full workspace deny-all gate remains blocked. The change remains active.

## Persistent evidence

Logs live under `.pi/cairn-rest-1295/`: `relative-owner-fix5.log`, `relative-owner-test5.log`, `relative-owner-clippy5.log`, `relative-owner-workspace5.log`, `relative-owner-workspace-clippy5.log`, `relative-owner-strict5.log`, `finish2-2746-core-strict.log`, and the `finish2-2746-stdlib-*` series.

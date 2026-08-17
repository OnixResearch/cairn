# Nickel 1.17 cohort implementation evidence

## Result

Cairn now selects `nickel-lang-cli 1.17.0` from `nickel-lang/nickel` revision `1320a983e6c3d1e2fb53dd2464b084b4903b1426`.

Nix generated the lockfile update. Policy, package wrappers, fixtures, tests, self-dogfood, and the development shell use this input instead of ambient `pkgs.nickel`.

The policy export receipt descriptor records:

- identity: `github:nickel-lang/nickel@1320a983e6c3d1e2fb53dd2464b084b4903b1426`
- version: `nickel-lang-cli-1.17.0`
- evaluator source revision: `1320a983e6c3d1e2fb53dd2464b084b4903b1426`
- exporter source revision: `257fafc1c746f1faf156207043a4c826bfb16d49`

## Compatibility and rejection evidence

`nix build path:$PWD#checks.x86_64-linux.nickel-cohort-exact` passed. It checks the resolved executable, full reported version, source revision, and absence of `pkgs.nickel` references.

`nix build path:$PWD#checks.x86_64-linux.nickel-toolchain-fixtures` passed. Positive policy, contract, merge, import, and export fixtures evaluate and export. Malformed, unknown-field, missing-import, contract-violation, type-mismatch, and stale-output fixtures fail closed.

`nix run path:$PWD#cairn -- policy export --check` reported:

```text
policy fresh: cairn-policy/generated/cairn-policy.json
```

A non-checking export rewrote the checked path with identical bytes.

## Runtime boundary

`nix build path:$PWD#checks.x86_64-linux.runtime-generated-policy-only` passed.

The check runs the unwrapped Cairn binary with a poison `nickel` executable. A representative runtime status command reads generated policy, succeeds, and does not start the poison evaluator.

This proves only the tested command and inputs. It does not prove that all future runtime paths are evaluator-free.

## Rust and repository checks

- Focused evaluator tests: ten passed across the `cairn` and `ironspec` binaries.
- Packaged runtime documentation test: one passed.
- `cargo fmt --check`: passed.
- Focused `cargo clippy -p cairn-cli --all-targets --no-deps -- -D warnings`: passed.
- `nix build path:$PWD#checks.x86_64-linux.self-dogfood`: passed.
- Cairn validation, proposal gate, and design gate: passed.
- After spec sync, traceability coverage passed: 2264 of 2264 referenced.
- `nix flake check path:$PWD -L --option builders '' --option secret-key-files ''`: `all checks passed!`

The first full Nix run occurred before spec sync. It correctly rejected the five new evidence markers as dangling. Sync added the accepted requirements, and the final full run passed.

A direct newer-nightly broad Clippy attempt reached six inherited `unnecessary_sort_by` findings in unchanged `crates/cairn-radicle-cob` files. The focused changed-crate gate and the canonical full Nix Clippy gate pass.

The Octet consumer check emits inherited findings as warning output. This change does not cite that warning-only output as acceptance evidence.

## Baseline infrastructure note

The initial default-builder baseline did not reach Nickel because the configured remote builder rejected a copied Rust source path by hash. Post-change Nix checks disabled remote builders and completed locally.

## Non-claims

The cohort and fixture checks prove selected evaluator identity, tested compatibility, tested rejection, and tested runtime non-execution only.

They do not prove Nickel correctness, Cairn policy correctness, all-path runtime freedom, gate soundness, evidence truth, or release eligibility.

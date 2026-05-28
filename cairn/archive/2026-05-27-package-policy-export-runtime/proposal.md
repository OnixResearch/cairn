# Package Nickel for policy export runtime

## Why

The packaged `cairn` binary can execute most commands from `nix run path:$PWD#cairn`, but `policy export --check` shells out to `nickel`. The release gate exposed that the built package does not carry Nickel on its runtime `PATH`, so a direct packaged policy freshness check fails with `No such file or directory` outside a dev shell.

## What changes

- Wrap packaged `cairn` and `ironspec` binaries so `nickel` is available when policy export evaluates Nickel policy.
- Add a flake check that runs packaged `cairn policy export --check` in a stripped environment without ambient Nickel.
- Update operator check docs to use the packaged CLI path consistently.

## Non-goals

- Do not change policy schema, generated policy JSON, or policy semantic validation.
- Do not make normal validation, status, or lifecycle commands depend on Nickel.
- Do not alter remote builder configuration or cross-system coverage policy.

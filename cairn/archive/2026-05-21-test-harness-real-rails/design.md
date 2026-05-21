# Design: Test Harness Real Rails

## Approach

`cairn test-harness` keeps the existing synthetic `--case` path and adds an explicit `--rail <id>` path. A rail id maps to a narrow, reviewed command vector. The harness executes the selected command with `std::process::Command`, captures stdout/stderr, and emits a receipt with stable rail metadata, replay command, exit status, output hashes, and the existing failure taxonomy.

## Supported initial rails

- `validate` -> current executable, `validate --root <root>`
- `tracey-coverage` -> current executable, `tracey coverage --root <root>`
- `release-readiness` -> current executable, `release-readiness --root <root>`
- `policy-export-check` -> current executable, `policy export --root <root> --check`

## Determinism

Receipts hash captured stdout/stderr bytes instead of embedding full volatile output. Human summaries keep command ids and diagnostics stable. The replay command is root-relative and uses the native Cairn CLI.

## Failure classes

- Successful launch + zero exit: `passed` / `none`.
- Successful launch + nonzero exit: `failed` / `product`.
- Launch failure: `failed` / `environment`.
- Unknown rail id: `failed` / `harness`.

## Why

`evidence registry fetch-execute` is now a public deterministic receipt-producing consumer command, but policy and operator documentation still primarily describe the earlier `fetch-plan` boundary. Downstream agents need the new execution receipt to be discoverable in the same policy/schema and docs surfaces as registry build, verify, select, mirror-verify, and fetch-plan.

## What Changes

- Publish the `evidence registry fetch-execute` receipt shape in the Nickel-authored policy schema catalog and generated policy JSON.
- Add the command to the deterministic surface inventory so coverage audits can see the public receipt surface.
- Update README, agent-run receipt guidance, and the repo-maintained Hermes skill source with the local/offline materialization flow, BLAKE3 verification guarantees, stable receipt fields, and fail-closed cases.
- Add focused regression coverage that the command's emitted receipt matches the policy schema catalog.

## Impact

- **Files**: `cairn-policy/default.ncl`, generated policy JSON, registry CLI tests, README/docs/Hermes workflow docs, and this Cairn change package.
- **Testing**: policy export/check, focused registry CLI test, Cairn validate, Tracey coverage, workspace tests/clippy/fmt, and flake check before archive/commit.

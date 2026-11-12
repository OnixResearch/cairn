# aarch64 release support boundary

The v0.2.0 release gate that was run for Cairn passed on the current x86_64 Linux
system. A package build and lifecycle smoke also passed on a native
`aarch64-linux` GitHub runner.

## Current claim

- The v0.2.0 release workflow built `.#cairn` and passed the lifecycle smoke on
  x86_64 Linux.
- The `aarch64 validation` workflow (`ubuntu-24.04-arm`) built `.#cairn` and ran
  `cairn --version`, `cairn init`, and `cairn validate` from a foreign working
  directory. Run `43262828123` passed.
- Full `nix flake check --all-systems` remains unclaimed. The arm64 job does not
  run the complete check cohort.

## Validation history

| Release | Platform | Scope | Result |
|---|---|---|---|
| v0.1.0 | x86_64-linux | Local `nix flake check -L` | pass |
| v0.2.0 | x86_64-linux | Release package build and lifecycle smoke | pass |
| v0.2.0 | aarch64-linux | Native package build and lifecycle smoke | pass |

## Future validation path

When a suitable aarch64 builder/evaluator is available, run:

```bash
nix flake check --all-systems -L
```

Record the resulting receipt/log before claiming full aarch64 check coverage.
The package-build and smoke result above is not a full flake-check claim.

r[verify release_evidence.aarch64_support_boundary.docs]
r[verify release_evidence.aarch64_support_boundary.non_claims]
r[verify release_evidence.aarch64_support_boundary.validation]
r[verify release_evidence.aarch64_support_boundary.priority]

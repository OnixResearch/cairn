# Support local producer applicability

## Why

ChaosControl is a current downstream consumer of Cairn completion evidence.
Its required checks include real Nix builds and pinned dependency inputs.
The local producer currently rejects every nonempty dependency identity set and cannot admit a passing Nix payload.
An exit-zero evaluation is not a successful build, and hand-authored completion rows are not an acceptable workaround.

## What Changes

- Observe bounded, reviewed dependency-file projections and recheck their BLAKE3 identities around execution and publication.
- Replace the process-only Nix evaluation grammar with a real offline, no-link JSON build of the consumer-owned `check.nix` expression.
- Admit the native build payload separately from process disposition, retain build-only meaning, and re-admit it during manifest verification.
- Exercise positive, stale, malformed, failed, and incomplete evidence through the original completion consumer.
- Preserve the exact required-check selection, existing execution/publication bounds, historical receipts, and fail-closed behavior.

## Impact

Cairn producer-adapter maintainers own implementation and maintenance.
ChaosControl adopts a verified immutable cohort through its existing vendored-policy update workflow.
The durable contribution is a supported producer path, not downstream fixture copying, a readiness exemption, or a universal execution claim.
Affected surfaces are the local producer shell, pure Nix payload admission, producer contracts, maintained conformance fixtures, and operator documentation.
Required verification is focused producer regressions and real CLI execution followed by the existing Cargo, policy, lifecycle, and Nix checks.

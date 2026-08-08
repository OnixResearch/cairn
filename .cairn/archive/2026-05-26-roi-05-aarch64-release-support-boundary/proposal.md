# ROI 05: aarch64 release support boundary

## Why

`nix flake check -L` passed on x86_64 and reported that aarch64 was omitted as incompatible. A clear support boundary prevents overclaiming v0.1.0 portability while preserving a future verification path.

## Scope

- Document what v0.1.0 did and did not prove for aarch64.
- Provide the future command/operator path for builder-backed validation.
- Avoid treating unavailable builder infrastructure as a current release blocker.

## Non-goals

- No cross-compilation implementation, remote-builder configuration, or claim that aarch64 is currently release-validated.

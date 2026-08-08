# ROI 01: Post-tag remote install smoke

## Why

Cairn v0.1.0 is tagged after a passing local release gate, but operators still need proof that the published GitHub flake/tag can be consumed from outside the working checkout.

## Scope

- Run the v0.1.0 package from the remote GitHub flake reference.
- Exercise a minimal lifecycle path in an isolated temporary root: `init`, `status`, and `validate`.
- Record a compact, secret-free smoke report in docs.

## Non-goals

- No tag movement, release publication, credential validation, or remote mirror trust claim.

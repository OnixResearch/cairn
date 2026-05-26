# aarch64 release support boundary

The v0.1.0 release gate that was run for Cairn passed on the current x86_64 Linux system. Nix reported that incompatible `aarch64-linux` checks were omitted. That means v0.1.0 local release evidence must not be cited as a completed aarch64 release validation.

## Current claim

- x86_64 local `nix flake check -L` passed before the v0.1.0 tag.
- aarch64 release support remains unclaimed until builder-backed validation is run.
- The unreachable remote-builder warning seen during local work was infrastructure noise, not product-local proof or product-local failure.

## Future validation path

When a suitable aarch64 builder/evaluator is available, run one of:

```bash
nix flake check --all-systems -L
nix build .#packages.aarch64-linux.cairn -L
```

Record the resulting receipt/log before claiming aarch64 release support.

r[verify release_evidence.aarch64_support_boundary.docs]
r[verify release_evidence.aarch64_support_boundary.non_claims]
r[verify release_evidence.aarch64_support_boundary.validation]
r[verify release_evidence.aarch64_support_boundary.priority]

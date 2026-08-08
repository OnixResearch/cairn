## Why

Rust/Nix/Cairn/Octet toolchain expectations vary across repos. Without a profile matrix, downstream failures can come from incompatible devshells, stale lockfiles, or mismatched validation lanes rather than code changes.

## What Changes

- Add a workspace toolchain/profile matrix contract.
- Record per-repo dev, CI, release, proof, and compatibility profiles.
- Bind each profile to commands, toolchain pins, expected outputs, and non-claims.

## Impact

- Operators can see which profile a receipt used.
- Cross-repo validation can reject stale or missing profile declarations.
- Release evidence can distinguish devshell checks from release/proof lanes.

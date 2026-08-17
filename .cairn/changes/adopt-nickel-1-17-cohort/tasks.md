# Tasks: Adopt Nickel 1.17 for lifecycle policy

## Pin and policy generation

- [x] [serial] Add an exact Nickel `1.17.0` source input at commit `1320a983e6c3d1e2fb53dd2464b084b4903b1426`. r[cairn.nickel_toolchain.cohort]
- [x] [serial] Replace policy, fixture, and self-dogfood uses of the older package-set Nickel. r[cairn.nickel_toolchain.cohort]
- [x] [serial] Regenerate `flake.lock` only with a Nix lock command. r[cairn.nickel_toolchain.lockfile]
- [x] [parallel] Add a guard for older, floating, ambient, or mixed evaluators. r[cairn.nickel_toolchain.cohort]
- [x] [serial] Regenerate checked-in policy JSON and run policy freshness checks. r[cairn.nickel_toolchain.compatibility]

## Runtime boundary and compatibility

- [x] [parallel] Prove runtime commands consume generated policy without starting Nickel. r[cairn.nickel_toolchain.runtime_boundary]
- [x] [parallel] Run valid policy, contract, merge, import, and export fixtures. r[cairn.nickel_toolchain.compatibility]
- [x] [parallel] Add malformed, stale, unknown, import, contract, and type-mismatch negative fixtures. r[cairn.nickel_toolchain.compatibility]

## Validation

- [x] [serial] Record the exact evaluator identity in relevant policy evidence. r[cairn.nickel_toolchain.validation]
- [ ] [serial] Run policy freshness, self-dogfood, focused tests, formatting, Clippy, Cairn gates, and `nix flake check -L`. r[cairn.nickel_toolchain.validation]

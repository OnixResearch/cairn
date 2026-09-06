# Bootstrap dependency admission evidence

The reviewed Cap Root and Durable File Publication dependencies now use published, immutable Git sources in Cargo and Nix.
`docs/bootstrap-dependencies.md` records source identities, licenses, notices, owner authorization, and consumer boundaries.
Cairn maintainers own the admission gate and consumer tests.

## Acquisition and lock generation

Published Git clones into the disposable worktree target directory resolved to:

- Cap Root: `8991e2572578c16e590c2f1c0915db6c46c5d9cb`.
- Durable File Publication: `951c27f59003cea9bfdb40ed4d89653d50fada1f`.

`nix flake lock --option min-free 0` generated two new input nodes and their root references.
Cargo generated two new package records and two CLI dependency edges during `cargo check -p cairn-cli`.
No existing package versions, producer pins, or Nix input identities changed.
No lockfile was edited by hand. No producer implementation was copied into Cairn.

## Observed verdicts

Cargo commands used `env RUSTC_WRAPPER= CARGO_TARGET_DIR=target nix develop --option min-free 0 -c`.

| Command | Verdict |
|---|---|
| `cargo check -p cairn-cli` | Passed |
| `cargo test -p cairn-cli --test bootstrap_publication_contract --test consumer_bootstrap_controls` | 3 mechanism tests and 2 controls passed |
| `cargo clippy -p cairn-cli --all-targets -- -D warnings` | Passed |
| `cargo test -p cairn-cli --bin cairn nickel_export_boundary` | 5 existing export-boundary tests passed |
| `nix build path:$PWD#checks.x86_64-linux.bootstrap-source-pin --option min-free 0 --no-link -L` | Passed, including negative admission mutations |
| `nix build path:$PWD#packages.x86_64-linux.cairn.cargoDeps --option min-free 0 --no-link -L` | Passed with fresh published fetchgit sources and matching fixed-output hashes |

The source gate binds reviewed revisions, source URLs, Cargo lock records, Nix lock records, resolved inputs, NAR identities, and Cargo source hashes.
Negative cases reject changed revisions, floating branches, sibling paths, missing packages, and changed hashes.

The publication tests retain synchronized completion, no-replace destination conflicts, symlink-target preservation, interrupted writes, failed cleanup, and visible-but-unknown durability.
These are direct producer-contract observations. They are not an installation transaction, recovery receipt, or fresh setup result.
The existing Nickel Export tests reject byte drift, diagnostic drift, unsafe paths, and symlink inputs.

The dependency-admission task is complete. The installation shell task remains open.

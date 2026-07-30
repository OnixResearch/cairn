## Why

Cairn lifecycle work can span several repositories or begin before a code repository exists. `--root` selects a path, but it provides no stable shared name for a standalone lifecycle repository.

Repositories also lack bounded declarations for read-only lifecycle references. Operators lack personal worksets for opening related planning and code repositories together.

The OpenSpec stores beta guide provides useful interaction patterns. Cairn needs an adaptation that preserves native layout, explicit state, preview-first effects, deterministic receipts, and claim boundaries.

## What Changes

- Add standalone lifecycle stores with native `cairn/` content and typed store identity configuration.
- Add a local store locator with preview-first registration and atomic BLAKE3-bound updates.
- Add `--lifecycle-store ID` without reusing the existing `--store PATH` flag.
- Add deterministic root resolution for explicit roots, named stores, local native roots, and committed project pointers.
- Add typed project declarations for one external store pointer and direct read-only store references.
- Add read-only `context` and expanded `doctor` output for selected roots and direct references.
- Add personal worksets with policy-known opener profiles and explicit launch execution.
- Keep VCS transport, dependency satisfaction, lifecycle authority, and release claims outside store and workset semantics.
- Add positive and negative fixtures for resolution, local state, references, worksets, transport boundaries, and receipt safety.

## Impact

- **Policy**: Add typed Nickel contracts, named bounds, opener profiles, receipt fields, and required non-claims.
- **Core**: Add pure store admission, registry transition, root resolution, reference indexing, context, and workset planning.
- **CLI**: Add store, context, configuration export, and workset command families.
- **Local state**: Add canonical JSON locator and workset state under the Cairn data directory.
- **Documentation**: Add a Cairn user guide based on the reviewed OpenSpec interaction model.
- **Testing**: Add deterministic positive and negative core, CLI, fixture, and receipt coverage.

## Out of Scope

- Cloning, fetching, pulling, pushing, branching, merging, or opening pull requests.
- Reusing the global registry as an ambient root locator.
- Adding a machine-wide default lifecycle store.
- Treating references as dependencies, evidence, requirements, policy, trust, or release authority.
- Recursively loading transitive references.
- Running arbitrary operator-configured opener commands.
- Claiming that a launched editor opened, retained, or saved every workset member.

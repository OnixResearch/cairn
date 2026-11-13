# Changelog

All notable changes to Cairn are recorded here. The format follows
[Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and releases use
[Semantic Versioning](https://semver.org/spec/v2.0.0.html).

Cairn receipts are evidence about declared inputs and policy, not proof of code
correctness or release eligibility. See `docs/` for the boundary of each
surface.

## [Unreleased]

### Added

- Opt-in `cairn prose check [--root DIR] FILE...` authoring diagnostics from the
  pinned englishc library, with JSON and machine-envelope output, UTF-8 source
  spans, and review-only suggestions. Findings never rewrite documents or change
  lifecycle gate outcomes; invalid inputs still fail.
- Human text, non-blocking GitHub warning annotations, and SARIF 2.1.0 prose
  presentation; deterministic active-change selection; Nickel-exported rule
  profiles and reasoned next-line suppressions. Default JSON v1 and machine
  semantics remain intact.
- Opt-in content-bound prose v2 reports and independent freshness rechecking,
  including complete current diagnostic recomputation. Technical-document
  projection preserves original source positions; requirement context and
  meaning-change hints remain advisory. Aggregate bytes, diagnostics, traversal,
  and rendered reports have explicit hard ceilings.

### Removed

- Post-release wording and source-text assertion tests. Historical documentation
  relationships remain declared links, not claims of executable verification.

## [0.2.1] - 2026-11-12

### Changed

- Publish the reviewed `cap-root-core` and `durable-file-publication` sources as
  public GitHub repositories and resolve Cargo and Nix from those snapshot
  commits. The vendored copies are removed.
- `cap-root-core` pins `OnixResearch/cap-root` at
  `b1911e42295887c399e17cad29ce298c2847f59d`.
- `durable-file-publication` pins the `reviewed-951c27f` snapshot of
  `OnixResearch/durable-file-publication` at
  `6d5510ddb33860bf47a49b63085105c7ff88f4d5`.
- The bootstrap source pin admits the exact GitHub repositories, revisions,
  content hashes, consumer scope, and licenses.

### Non-claims

- The GitHub repositories are source snapshots, not mirrors of private
  development history. Canonical Radicle repository identities remain recorded
  in provenance.

## [0.2.0] - 2026-11-12

First public release. `v0.1.0` was a local, pre-public tag; this release adds
the public install path, the native lifecycle layout, and the agent, evidence,
and release workflows documented in `README.md`.

### Added

- Native `.cairn/specs`, `.cairn/changes`, and `.cairn/archive` lifecycle layout.
  Legacy `cairn/`, `ironspec/`, and `openspec/` trees are migration inputs only.
- Policy-owned workflow profiles (`spec-driven`, `assurance`, `outcome-machine`,
  `no-spec-delta`) plus `workflow status` and `workflow instructions`.
- One versioned machine outcome for direct CLI agents through
  `cairn --machine <command>`.
- Artifact workflow inspection, change readiness reports, and the
  implementation-ready projection.
- Agent guidance context, generated project-local agent workflows, agent run
  receipts, and task-scoped MCP discovery.
- MCP shell with read tools and receipt-producing commands.
- Developer-loop profiles and check-evidence planning, execution, and
  verification.
- Gate interchange contract for binding one native gate receipt to an exact
  candidate, policy, evidence set, and tool cohort.
- Release evidence indexing, current-head and audited-payload checks,
  materialization, retention planning, and durable release-run ledgers.
- Authenticated evidence profiles and the detached source-review producer and
  verifier boundary.
- State-machine receipts, nominal evidence identities, and bounded Preserves
  receipt operations.
- Cross-repository adoption reports, change dependency graphs, workspace
  contract matrices, and a global registry.
- RFD, ADR, verification-obligation, release-decision, and policy-exception
  collaboration over Radicle COBs.
- Worksets, kanban/mindmap/graph views, lifecycle stores, and compact archive
  and unpack operations.
- `cairn setup plan` consumer bootstrap with exact policy and guidance plans.
- `cairn template apply`, `cairn scaffold change`, and `cairn migrate` previews.
- Release workflow that builds the Nix package on tagged commits and runs a
  foreign-working-directory smoke test.

### Changed

- Generated policy is the runtime boundary. `cairn policy export --check`
  rejects stale checked-in JSON.
- Nickel 1.17 cohort pin with the `nickel-export` cutover.
- Radicle COB Git storage moved behind the standalone
  `onix-cairn-radicle-cob-git-store` package.
- Bounded tree and producer source pins moved to explicit immutable revisions.
- Policy source adopts the documented dual-license grant while the CLI remains
  `AGPL-3.0-or-later` and the core, gate contract, and COB store remain
  `MIT OR Apache-2.0`.
- Lifecycle and policy inputs are bounded, and generated policy is installed
  with the packaged binary.

### Fixed

- Manifest, bundle, config, probe, and authentication inputs reject unsafe
  artifact paths, symlinked leaves and parents, oversized input, and entries
  outside the declared output parent.
- Package build, validation, and evidence commands fail closed on malformed,
  stale, or contradictory inputs instead of reporting partial success.

### Packaging and installation

- Dependency sources use anonymous public HTTPS. Nix flake inputs and Cargo git
  dependencies no longer require SSH credentials. `cap-root-core` and
  `durable-file-publication` are vendored at their reviewed revisions, and Kiln
  resolves from GitHub, so builds do not depend on Radicle seed availability.
- A native arm64 GitHub workflow builds the package and runs the lifecycle smoke
  for a requested ref.
- `nix profile install github:OnixResearch/cairn` and
  `nix run github:OnixResearch/cairn#cairn` work from any working directory.
  The packaged wrapper points `CAIRN_POLICY` at the bundled generated policy;
  an explicit `--policy` argument still wins.
- `cargo install --git https://github.com/OnixResearch/cairn cairn-cli --locked`
  builds the CLI from source. Policy export and lifecycle policy generation also
  need the pinned Nickel cohort and `nickel-export` on `PATH`.

### Compatibility notes

- v0.1.0 lifecycle material is not read as an implicit fallback. Use
  `cairn migrate` explicitly when importing old trees.
- The documented stable receipt fields remain compatibility surfaces. Other
  commands, schemas, and policy may change in a minor release with release notes.
- x86_64 Linux is covered by the release gate. aarch64 Linux builds the package
  and passes the lifecycle smoke on a native runner; full `nix flake check
  --all-systems` coverage remains unclaimed.

### Non-claims

- A passing gate or receipt proves conformance to selected policy and declared
  inputs only. It does not prove code correctness, evidence truth, build
  reproducibility, deployment safety, or release eligibility.
- MCP and agent surfaces do not establish ambient trust in an agent or model.
- Release evidence is local and trusted unless it is independently signed and
  verified under an authenticated profile.

## [0.1.0] - 2026-05-26

Pre-public local tag. Native lifecycle commands, generated policy checks, and a
local release-evidence smoke were verified on x86_64 Linux. No public support,
registry publication, or aarch64 claim was made.

[Unreleased]: https://github.com/OnixResearch/cairn/compare/v0.2.1...HEAD
[0.2.1]: https://github.com/OnixResearch/cairn/compare/v0.2.0...v0.2.1
[0.2.0]: https://github.com/OnixResearch/cairn/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/OnixResearch/cairn/releases/tag/v0.1.0

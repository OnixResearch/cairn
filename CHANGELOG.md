# Changelog

All notable changes to Cairn are recorded here. The format follows
[Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and releases use
[Semantic Versioning](https://semver.org/spec/v2.0.0.html).

Cairn receipts are evidence about declared inputs and policy, not proof of code
correctness or release eligibility. See `docs/` for the boundary of each
surface.

## [Unreleased]

### Added

- Convergence reads Mantle realization receipt v2 and reports the selected
  root's disposition. A version 2 root must record `built` or `reused`, sign
  with the `builder` role and a fingerprint the case trusts in the new
  `trusted_builder_keys` list; a `reused` root must also supply the case's
  `original_realization_receipt`, whose recorded digest, `built` disposition,
  NAR hash and signer fingerprint Cairn re-checks, else the row reports the new
  `convergence.reuse_link_invalid`. `bound.root_disposition` and
  `bound.original_receipt_blake3` report the result; a version 1 receipt on
  `cache-only-preserve-v1` reports `preserved`. Cairn compares recorded
  fingerprints and the recorded link and does not verify signature bytes.
  Advisory only; no gate changes.

- Lifecycle repair for duplicated accepted requirement IDs: a MODIFIED sync
  delta may retarget one conflicting block by its unique accepted heading when
  that block's ID is duplicated, provided the merged document is duplicate-free.
  Repairs run before the other operations, the merged document is rechecked, and
  ambiguous headings (`delta_merge.ambiguous_accepted_heading_target`) and
  introduced duplicates (`delta_merge.introduced_duplicate_id`) are refused with
  the accepted bytes preserved. Core and CLI fixtures model Terrane's fetch-add
  and constant-folding duplicates; a replayed repair is an `already_applied`
  no-op.
- A scoped `MIT OR Apache-2.0` grant for the consumer-facing typed Nickel
  contracts under `contracts/**` and `cairn-policy/assurance-targets.ncl` at
  every revision, including Octet's vendored copies of
  `contracts/assurance-targets.ncl` at `3dc654d3…` and `f20bcb4a…`, and
  byte-for-byte copies elsewhere. The grant excludes generated JSON exports,
  other policy files, the CLI and third-party material; `license-policy.tsv`
  enforces its row, paths and revisions. Prior grants remain valid.
- Registry-derived assurance target rows: `requirement_from_registry` and
  `specification_from_registry` in `contracts/assurance-targets.ncl` select the
  single `cairn requirement-registry export` row for a requirement id and fail
  closed on a missing id, a duplicate id or a foreign registry schema. A fixture
  derives the octet example's rows and exports byte-identical JSON; three
  negatives cover the failure modes. Rust admission still re-verifies all six
  fields. Advisory only; no gate changes.
- Assurance targets and read-only `cairn convergence inspect`: a Nickel targets
  contract binding exact registry rows to `reviewed`, `bounded_check` or
  `checked_proof`, a projection to Octet requirement contracts that Rust
  re-verifies, and a `cairn.convergence.v1` receipt that binds Mantle
  realization records and Octet decisions per row, with `complete` requiring
  both current targets and full coverage. Advisory only; no gate changes.
  `fixtures/convergence/pilot` retains a real pilot: Octet's Aeneas
  external-consumer theorem, realized through Mantle's preserve route under one
  pilot key, observed by the verification bridge and judged by Octet, is
  `current`. Twenty negatives cover one binding check each. Mantle records are
  read within their producers' own bounds.
- Read-only MCP tool `cairn_convergence_inspect`: with `case`, optional `root`
  and `policy`, it returns the same `cairn.convergence.v1` receipt as
  `cairn convergence inspect` through the shared inspect path. It is a member of
  the `full`, `inspect`, `author` and `verify` discovery profiles, reads every
  input bounded and link-free, rejects unknown arguments, and maps an
  unadmitted schema to the command's rejection receipt.
- Offline agent lifecycle cost measurement: typed scenario and cohort contracts,
  checked accounting, saved-observation import and comparison, with token data
  explicitly unavailable rather than estimated and representation-only verdicts.
- Explicit MCP `structured-summary` response profile: one complete structured
  receipt plus bounded status text, retaining the legacy default, native hashes,
  error behaviour and capability-first mutation denial.
- Policy-owned MCP catalog profiles (`full`, `inspect`, `author`, `verify`), exact
  single-descriptor lookup and opt-in identity-bound `tools/list` pagination, with
  the unpaged catalog unchanged and profiles never granting capability.
- Versioned `cairn.selected-work.v1` CLI/MCP and portable Rust DTO for explicit
  change/task cohorts, typed requirement and acceptance references, native phase
  readiness, prerequisite closure, identity-checked local detail and consumer
  admission. An opt-in `--obligation-definition` carries the bound workflow
  obligation predicate, structured guidance, typed per-obligation facts with
  recomputable obligation, guidance and remaining-condition identities, and the
  evaluation, under a separate `obligations` cohort identity; without it the
  section is explicitly `not_selected`.
- Opt-in finite requirement semantics with revision-bound contracts over finite
  nominal domains, deterministic normalization, separate contract, interpretation
  and review identities, and read-only `cairn requirement-contract inspect`.
- Scoped claim derivations: a closed rule vocabulary over native producer
  conclusions with unresolved/supported/refuted/disputed states, counterevidence
  retention, residual assumptions and a read-only explanation command.
- Readiness-backed drain view with an explicit per-change evidence manifest,
  declared dependency order and preview-only actions; generated guidance now
  requests readiness and exact artifact routes without adding authority.
- Incremental observation pages, unchanged results and exact entry differences
  bound to a complete cohort identity, with stateless cursors and guarded paths.
- Compact project-local entry guidance derived from the checked workflow catalog,
  with freshness checks, preview-first installation and report-only inspection of
  copied guides.
- Alternative support maintenance across evidence invalidation, assumption
  withdrawal and revision changes, retaining historical withdrawal reasons.
- Opt-in bounded readiness agent view and identity-checked blocker/action/stage
  detail retrieval, retaining full receipt identity, stage decisions, complete
  selected entries, omission counts and exact standalone JSON byte bounds.
- Exact source-bound lifecycle context reads for requirement IDs, authored task
  IDs and heading paths through CLI/MCP, with UTF-8 byte spans, stale-source
  refusal, separate document validity and explicit prerequisite/authority routes.
- Opt-in finite requirement semantics: revision-bound contracts over finite
  nominal domains with total observable mappings, deterministic normalization,
  separate contract/interpretation/review identities, and read-only
  `cairn requirement-contract inspect` that keeps well-formedness, reviewed
  correspondence, finite example evaluation and proof evidence distinct.
- Opt-in revision classification: `cairn revision classify` compares two
  requirement revisions or two policy-fragment revisions over declared common
  domains. Equivalence is classified as editorial or clarifying, containment as
  strengthening or weakening, and incomparable, unsupported or bound-exhausted
  pairs as incompatible or undetermined, each with a replayable counterexample
  witness. Assumption, activation and guarantee relations are reported
  separately; old receipts are never current after a revision, and evidence
  transfer stays disabled and explicit.
- Opt-in bounded requirement consistency analysis: `cairn requirement-analysis
  inspect` answers shared-assumption satisfiability, per-requirement activation
  reachability, joint guarantee compatibility and guarantee-withholding
  redundancy over admitted typed contracts, with total observable tables,
  pairwise-minimal conflicts and a pure witness replay. Vacuity is reported only
  relative to a satisfiable shared premise; contradictory sets are inconsistent,
  and every bound, unsupported form and exhausted search stays explicit.
- Opt-in workflow obligations: one Nickel definition now drives both agent
  guidance and an evaluable completion predicate for the `verify` workflow and a
  bounded `verify.bounded-exception` example. `cairn obligation
  list|explain|export` publishes the bound predicate, obligation summaries,
  alternatives and evidence requirements read-only; action preconditions,
  expected observations, progress facts and completion stay separate, a bounded
  temporal trace extension keeps missing or untrusted evidence unknown, and an
  accepted exception requires current applicability plus the compensating
  control carried by its snapshot. Activity counts, checked boxes and agent
  success statements are recorded but never treated as evidence.
- Opt-in concurrent-change footprints: `cairn footprints inspect` derives
  compatibility obligations between two declared change footprints, naming the
  combined candidate, participants, composition order and required evidence, and
  reporting coexistence, invariant preservation and commutativity as separate
  propositions with `conflict`, `needs_joint_evidence`,
  `supported_within_declared_scope` or `unknown`. Individual receipts never
  discharge a combined obligation, a finite witness never establishes
  preservation over all compositions, and prior applicability is invalidated when
  participants, dependencies, configuration or composition change.
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

- `docs/replayable-completion.md`, an acceptance report that explains one
  executed completion decision as an argument over requirement revisions,
  admitted evidence, assumptions and inference rules, records the replay
  commands and the drift rule, and states what the decision does not establish.
  It is documentation for roadmap issue #4; codifying the argument as a checked
  requirement needs a companion review command and is not claimed here.
- Owner-scoped producer-job admission for convergence: the assurance policy
  admits producer-job schemas per owner (`octet` →
  `octet-mantle-producer-job/v1`, `trellis` →
  `trellis-mantle-producer-job/v1`), and a handoff `job.json` is admitted by its
  own `owner` with that owner's schema. A missing owner, an unknown owner and a
  schema/owner mismatch fail closed with stable issue codes in both the bounded
  CLI read and the pure evaluator. The retained Octet pilot stays `current`; a
  synthetic Trellis job binds.

### Changed

- The generated agent-workflow command catalog places
  `cairn readiness <change>` in the `readiness` class instead of `gate`, and
  adds `drain-plan-readiness` (`cairn drain-plan --view readiness --root PATH`)
  in the `readiness` class. Before, the `apply` workflow told the agent to run
  `cairn readiness` although it allows `readiness` and not `gate`, and `drain`
  named a drain-plan command the catalog did not list. A test over the shipped
  default policy now checks that every `cairn` command a workflow's goal,
  evidence, progress rule, immediate action, output shape, constraints or
  non-claims tells the agent to run resolves, by longest command-word prefix,
  to a catalog entry in a class that workflow allows; the test fails on the
  previous default. It reads the text as Markdown, so emphasis inside or around
  a command, code spans and link labels do not hide a command, and fence info
  strings, link destinations, unpaired markers such as `_cairn` and
  capitalized "Cairn" are not commands.

  Policy admission does not read workflow text. An unpublished draft of this
  change (`8718b567`) made admission refuse any policy whose descriptor text
  named an uncatalogued command or one outside the descriptor's classes, which
  refused every unmodified copy of the previous default; that admission
  refusal is removed, and consumer policies load as before. Generated-workflow
  admission and lookup diagnostics copy at most 64 bytes of a catalog ID,
  command class, workflow, adapter or profile name, marking a truncated value
  with its length. The generated policy hash is now
  `2d388789ecfd5dd0a45b4fa02ad9c18a9740cee75c0dd89f96968b6ae52980cb`, and the
  command catalog and workflow catalog identities change, so installed entry
  guides report `stale-source`.

- `release-readiness` refuses a malformed consumer-selected gate list. When the
  policy declares `release_readiness_policy.required_external_commands`, it must
  be a non-empty array of unique, non-blank strings, and `release_readiness_policy`
  must be an object. Otherwise `repository_gate_summary.policy` is
  `invalid-consumer-selected-gates` with a null command list and an `issue`
  diagnostic that names the field and entry indices but never an entry's text,
  the `repository_gate_policy_listed` check fails, and the verdict is
  `fail`. Cairn no longer substitutes the maintained-local gates for a malformed
  list. A policy without the list still reports the maintained-local gates.

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

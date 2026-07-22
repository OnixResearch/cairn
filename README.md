# cairn

<img width="400" height="400" alt="Cairn project logo" src="https://github.com/user-attachments/assets/5f3ecfb2-0abb-42c8-b03c-8cd35afa7b77" />

`cairn` is a Rust lifecycle engine for native repository lifecycle work.
Repositories use a `cairn/` lifecycle layout; legacy `openspec/` trees remain
supported only as an explicit import/migration source. Cairn adds a
Nickel-authored policy/config/schema layer and a Slotcar-enforced
functional-core / imperative-shell boundary.

Boundaries:

- Nickel owns structured policy: project config, artifact schemas, lifecycle
  stages, gates, reports, findings, task markers, sync/archive policy, and
  bounded exemptions.
- `cairn-core` owns pure deterministic transforms only. The strict core lives
  under `crates/cairn-core/src/verified/` so Slotcar's `verified_purity`
  checks continuously guard it.
- `cairn-cli` owns imperative shell behavior: argument parsing, filesystem
  discovery, policy file reads, stdout/stderr, process exits, and JSON
  presentation.
- Runtime commands consume checked-in generated policy JSON by default; Nickel
  evaluation is a development/export step.

## Project status

Cairn is under active development. Receipt fields explicitly documented as
stable are compatibility surfaces; other commands, schemas, and policies may
evolve. A passing lifecycle gate establishes conformance to the selected Cairn
policy and inputs, not source-code correctness, proof transfer, artifact truth,
or release eligibility.

## Layout

```text
cairn-policy/        Nickel contracts, defaults, fixtures, generated JSON
crates/cairn-core/   Pure Rust models, policy parsing, repo/status/read/gate/doctor/EARS-style validation
crates/cairn-cli/    CLI/MCP shell for init/change/spec/status/doctor/validate/policy/migrate/gate plus safe sync/archive planning/execution
fixtures/               Fixture legacy and native lifecycle repositories
docs/tutorials/         Step-by-step operator tutorials
docs/hermes/            Copyable Hermes skill source for cairn workflow
examples/               Copyable command examples
tools/                  Export/check helpers
```

Native project lifecycle content lives under:

```text
cairn/specs/         Accepted specs
cairn/changes/       Active changes
cairn/archive/       Archived completed changes
```

Commands use the native `cairn/` layout for normal runtime operations. Legacy
`openspec/` content is read only by the explicit `migrate openspec` import bridge;
there is no implicit runtime fallback. `cairn init` and `cairn change
create` create native layout artifacts.

## Make a Cairn

Once the repository is public, the flake can be run without a local install:

```bash
export REPO=/path/to/your/repo
nix run github:OnixResearch/cairn#cairn -- init --root "$REPO"
nix run github:OnixResearch/cairn#cairn -- validate --root "$REPO"
```

Pin a tag or commit instead of `main` for repeatable automation. For local
Cairn development, run the same workflow from a source checkout:

```bash
export CAIRN_SOURCE=$PWD
export REPO=/path/to/your/repo

nix run path:$CAIRN_SOURCE#cairn -- init --root "$REPO"
nix run path:$CAIRN_SOURCE#cairn -- template list --root "$REPO"
nix run path:$CAIRN_SOURCE#cairn -- template apply generic-cli --root "$REPO"
nix run path:$CAIRN_SOURCE#cairn -- template apply generic-cli --root "$REPO" --execute
nix run path:$CAIRN_SOURCE#cairn -- scaffold change add-friendly-start \
  --root "$REPO" \
  --request "Add a friendly start path for first-time users." \
  --execute
nix run path:$CAIRN_SOURCE#cairn -- validate --root "$REPO"
nix run path:$CAIRN_SOURCE#cairn -- gate proposal add-friendly-start --root "$REPO"
nix run path:$CAIRN_SOURCE#cairn -- gate design add-friendly-start --root "$REPO"
nix run path:$CAIRN_SOURCE#cairn -- gate tasks add-friendly-start --root "$REPO"
```

These commands create the native `cairn/` lifecycle, apply a quickstart
template, scaffold a deterministic change package, and print validation/gate
receipts. Template and scaffold commands preview changes unless `--execute` is
passed; sync/archive are dry-run by default too.

Tutorial: [`docs/tutorials/make-a-cairn.md`](docs/tutorials/make-a-cairn.md).
Copyable example: [`examples/make-a-cairn/README.md`](examples/make-a-cairn/README.md).

## Local checks

```bash
nix run path:$PWD#cairn -- policy export --check
cargo fmt --check
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
cargo tigerstyle check --output-format json --artifact-dir artifacts/slotcar-run
nix run path:$PWD#cairn -- validate --root .
nix flake check path:$PWD -L
nix run path:$PWD#cairn -- doctor --root fixtures/basic
nix run path:$PWD#cairn -- status --root fixtures/basic
```

`nix flake check` includes policy freshness, Rust formatting, workspace tests,
Cairn validation, traceability coverage, Clippy, the Slotcar consumer check,
and a self-dogfood check that runs the built `cairn` CLI against this
repository plus the fixture repos. Maintained gates use Rust CLI surfaces and
Nickel policy evaluation; Cairn does not keep checked-in Python or shell helper
scripts for these checks. `cairn validate` checks change/task policy
plus EARS-style requirement strength and GIVEN/WHEN/THEN scenario structure for
accepted specs and active change delta specs. Slotcar artifacts should be written
under `artifacts/`, which is ignored by git.

## Traceability

Cairn owns the maintained requirement traceability gate:

```bash
cairn traceability coverage --root .
cairn traceability coverage --root . --json
```

The command loads Nickel-authored traceability profiles from generated policy,
scans accepted requirements plus configured evidence roots, and emits a
BLAKE3-bound receipt with profile identity, counts, missing IDs, dangling IDs,
input hash, policy hash, and receipt hash. `cairn tracey coverage --root .`
remains a compatibility alias for existing scripts, but maintained CI gates and
release-readiness evidence use `cairn traceability coverage` and do not require
the external Tracey binary or daemon. External Tracey commands such as
`tracey query status`, `tracey query uncovered`, and `tracey query untested` are
optional developer diagnostics only.

The reusable traceability marker core parses in-memory marker strings such as
`r[...]`, `r[impl ...]`, `r[verify ...]`, `r[depends ...]`, and `r[related ...]`
into typed link DTOs, checks a declared verb vocabulary, optionally resolves
links against in-memory requirement registry rows, and reports deterministic
syntax/linkage diagnostics. Repository file walking, source span capture, policy
profile selection, stdout/stderr, and release-readiness interpretation remain in
the consuming shell. Passing marker validation proves only marker syntax,
vocabulary conformance, and declared linkage health; it does not prove
requirement satisfaction, implementation correctness, verification coverage,
proof soundness, release eligibility, lifecycle readiness, or downstream
behavioral correctness.

## Runtime evidence and external witnesses

`cairn runtime-evidence verify --chain <chain.json> [--checkpoint <checkpoint.json>]`
checks subject-scoped runtime action chains and optional witness checkpoints.
runtime evidence records use Cairn-owned BLAKE3 identities, policy-declared
capture origins, hash-only or redacted input/outcome identities, and explicit
non-claims. A local chain proves presented-sequence integrity only; a matching
checkpoint proves a bounded witnessed prefix. Cairn does not decide whether a
witness authority is independent, socially trusted, or legally trusted, and it
does not prove agent correctness or pre-action policy enforcement. r[verify cairn.runtime_evidence.docs.claims]

Imported Halo/AARM-style metadata may preserve external hash algorithms and
canonical forms as adapter metadata, but Cairn computes BLAKE3 identities for
Cairn-owned records, checkpoints, and receipts before using them in lifecycle or
release evidence. r[verify cairn.runtime_evidence.docs.compatibility]

## How Cairn uses EARS

EARS means "Easy Approach to Requirements Syntax". Alistair Mavin, Philip
Wilkinson, Adrian Harwood, and Mark Novak published it at IEEE Requirements
Engineering 2009 after work at Rolls-Royce on constrained natural-language
requirements.

Useful references:

- [Easy Approach to Requirements Syntax (EARS), DOI 10.1109/RE.2009.9](https://doi.org/10.1109/RE.2009.9)
- [IEEE Xplore record](https://ieeexplore.ieee.org/document/5328509)
- [Alistair Mavin's EARS overview](https://alistairmavin.com/ears/)

Cairn uses an EARS-style shape so requirements are easy for agents and
reviewers to check. Each requirement gets a stable `r[...]` id and a clear
normative word such as `MUST`, `SHALL`, `SHOULD`, or `MAY`. Scenario blocks use
`GIVEN`, `WHEN`, `THEN`, and optional `AND` lines as acceptance examples around
the requirement. Those scenario blocks are Cairn's gate-friendly convention;
the original EARS patterns use keywords such as `While`, `When`, `Where`, and
`If ... then` inside requirement sentences.

```markdown
### Requirement: Semantic review CLI
r[cairn.semantic_review.semantic_review_receipts.cli] The system MUST expose semantic review receipt validation through a deterministic CLI command.

#### Scenario: Valid receipt passes
r[cairn.semantic_review.semantic_review_receipts.cli.cli.valid]
- GIVEN a valid semantic review receipt
- WHEN the operator runs `cairn review validate <receipt>`
- THEN Cairn emits a valid deterministic validation receipt
```

## How Cairn uses policy

Policy is Cairn's reviewed rulebook. It says which lifecycle stages exist, which
artifacts each stage expects, which task markers are valid, which gate stages run,
which receipt fields are public, which hash algorithm and canonical JSON form are
used, and which labels, replay cases, redaction classes, and optional orchestration
profiles are allowed.

Policy source lives in Nickel under `cairn-policy/`. Runtime commands do not run
Nickel. During development, `cairn policy export` checks the Nickel contracts and
writes the checked-in runtime bundle at `cairn-policy/generated/cairn-policy.json`.
Rust then consumes that generated JSON and evaluates repository snapshots against
it.

Policy keeps rules out of hidden code literals. For example:

- gate receipts include `policy` and `policy_hash`;
- `sync`, `archive`, and migration plans bind `policy_hash` into their receipts;
- agent and replay receipts compare evidence against current policy hashes;
- registry labels and receipt schemas are policy-owned vocabularies;
- malformed or stale policy fails closed with deterministic diagnostics.

Run this before trusting generated policy changes:

```bash
nix run path:$PWD#cairn -- policy export --check
```

## Deterministic enforcement model

Cairn enforcement is deterministic when every decision follows this sequence:

1. Export policy from reviewed Nickel and confirm generated JSON is fresh.
2. Read a bounded repository snapshot: lifecycle files, policy JSON, and command inputs.
3. Hash the snapshot into `input_hash` and hash policy into `policy_hash`.
4. Evaluate the snapshot in `cairn-core` without filesystem, clock, process, or network access.
5. Emit a receipt with `valid` or `verdict`, issues, `input_hash`, `policy_hash`, and `receipt_hash`.
6. Allow mutation only through commands that re-check blockers and write a mutation manifest.
7. Let CI, release-readiness, or operator policy reject failed or stale receipts.

The boundary is explicit: Cairn can block its own `sync`, `archive`, migration,
and MCP mutation tools, but it cannot stop a person or script from editing files
directly. To make enforcement hard rather than advisory, require passing Cairn
receipts in CI before merge or release.

## Hardening verification artifacts

Traceability links are necessary but not sufficient. A `r[verify ...]` marker
says a requirement has a verification artifact; the artifact itself must carry
the behavioral claim.

A stronger verification artifact has:

- a stable requirement id, such as `r[cairn.example.behavior]`;
- an executable check or reviewed proof next to `r[verify cairn.example.behavior]`;
- a clear oracle: exact assertion, expected issue code, proof obligation, or receipt field;
- positive and negative coverage for valid, invalid, missing, malformed, and boundary inputs;
- deterministic inputs, fixtures, and command lines that can run in CI;
- a receipt or test result bound to current content and policy hashes.

Use docs-only verification refs for documentation or non-claim requirements. For
behavioral requirements, prefer executable tests, property tests, model checks,
or formal proofs. The stronger the artifact, the stronger the claim; Cairn then
records which artifact was present and which checks ran against exact content.

## Deterministic rails

Cairn evidence proves bounded facts, not full correctness. A receipt says:

```text
for content hash H
with policy hash P
tool T produced result R
and receipt hash B binds them
```

That is useful because another operator or CI job can re-check the same inputs,
policy, tool result, and receipt binding. It is still lossy: traceability
coverage does not prove the code satisfies a requirement, tests do not prove all
cases, and a receipt hash does not prove the tool has no bugs. Stronger claims need stronger
rails, such as property tests, model checking, or formal verification.

The reusable lifecycle receipt core in `cairn-core` validates only shared envelope
shape: verdict vocabulary, diagnostic rows, command identity, evidence summary,
named hash-profile rules, receipt-hash consistency, and explicit non-claims. It
adapts existing Cairn gate/validate/sync/archive receipts and downstream
receipt-shaped evidence without taking ownership of policy decisions. Passing the
core proves envelope syntax, hash consistency, diagnostic shape, and non-claim
presence only; it does not prove release eligibility, lifecycle readiness,
policy satisfaction, evidence truth, source-code correctness, verifier soundness,
or downstream behavioral correctness. r[verify cairn.release_evidence.lifecycle_receipt_core.docs]

The Preserves evidence ingest core validates imported `.preserves` carriers by
schema label, producer repo, canonical BLAKE3 identity, evidence role, claim
scope, payload opacity, optional Cairn adapter ID, and non-claim text. Opaque
payloads stay opaque: Cairn records carrier identity and declared role/scope only
unless a Cairn-owned adapter is registered for the declared schema and role. A
passing ingest report does not prove evidence truth, runtime correctness,
transport correctness, producer authorization, source-code correctness, verifier
soundness, or release eligibility. r[verify cairn.preserves_evidence_ingest.docs]
r[verify cairn.preserves_evidence_ingest.docs.non_claims]

The generic evidence-chain readiness core validates typed canonical-envelope,
Valence, Mantle, source, binary, policy, projection, claim-scope, and non-claim
links through an explicit profile registry. Optional absent evidence is skipped;
optional present and required evidence fail closed when invalid. Passing emits a
BLAKE3 chain identity bound to Cairn gate policy, but does not establish measured
bytes, producer authorization, payload truth, or release eligibility. See
[`docs/evidence-chain-readiness.md`](docs/evidence-chain-readiness.md).
r[verify cairn.stack_provenance_gates.evidence_chain_readiness.validation]

The ast-grep lifecycle gate validates parsed scan/test receipts rather than
running ast-grep from pure code. Repositories or CLI shells own command
execution, file discovery, stdout/stderr capture, and raw report loading; the
Cairn core checks in-memory receipt shape, tool identity, BLAKE3 rule-bundle
identity, repository scope, output format, findings summary, policy mode, source
span compatibility, and non-claim text. Inventory and advisory modes record
findings without treating them as release blockers; blocking and codemod-review
modes fail closed according to policy. Passing the gate does not prove source
correctness, requirement satisfaction, proof discharge, or release eligibility.
r[verify cairn.ast_grep_gate.shell_boundary]

## Structured lifecycle and probes

Structured lifecycle source is now a typed data path beside the existing Markdown
layout. Nickel contracts in `cairn-policy/structured-lifecycle-contracts.ncl`
validate authored lifecycle bundles, `cairn lifecycle export` writes or checks
canonical JSON runtime bundles, and Rust revalidates generated data before
trusting it. Markdown remains a compatibility view/import path through
`cairn lifecycle render-markdown` and `cairn lifecycle import-markdown`; rendered
Markdown is not the authoritative source when structured lifecycle data exists.
r[impl cairn.structured_lifecycle.typed_authoring] r[impl cairn.structured_lifecycle.runtime_bundle]
r[impl cairn.structured_lifecycle.markdown_projection] r[impl cairn.structured_lifecycle.migration]
r[impl cairn.structured_lifecycle.format_strategy]

Probe execution is explicit. Default `cairn gate proposal|design|tasks` commands
read lifecycle artifacts, policy, declarations, and saved receipts; they do not
run probe commands. CI should run probes first with `cairn probe run --out ...`,
store the probe receipts, then run gates that verify the saved receipts. Kani
receipts follow the same saved-evidence boundary; see
[`docs/kani-receipt-gates.md`](docs/kani-receipt-gates.md). Inline
`--run-probes` is denied by default unless a later policy explicitly enables a
bounded execution profile. Probe receipts store command identity, status class,
stdout/stderr BLAKE3 hashes, fixture hashes, assertion results, lifecycle bundle
hash, policy hash, and receipt hash. r[impl cairn.probe_execution_boundary.default_gate_no_execution]
r[impl cairn.probe_execution_boundary.explicit_probe_run]
r[impl cairn.probe_execution_boundary.gates_verify_receipts]
r[impl cairn.probe_execution_boundary.inline_probe_mode]
r[impl cairn.probe_execution_boundary.fail_closed]

Format boundary: Nickel contracts own authoring validation, generated JSON is the
initial runtime interchange, Preserves remains a future canonical encoding
candidate, and Steel is only a future optional bounded probe DSL rather than the
lifecycle source of truth. r[verify cairn.structured_lifecycle.format_strategy.nickel_authoring]
r[verify cairn.structured_lifecycle.format_strategy.generated_json]
r[verify cairn.structured_lifecycle.format_strategy.preserves_candidate]
r[verify cairn.structured_lifecycle.format_strategy.steel_not_source]
r[verify cairn.structured_lifecycle.format_strategy.steel_probe_dsl]

## Backend-agnostic collaborative changes

Cairn's collaboration semantics do not require Radicle. The neutral
`cairn.change-collaboration.v1` state carries a workspace ID, typed base
revision, policy hash, creator, exact active-change artifacts, revision DAG,
conflicts, lifecycle assertions, and evidence. BLAKE3 content and state
identities derive from that semantic state rather than a transport object ID.
Accepted specs, implementation source, and executed archive commits remain
canonical Git content. r[impl cairn.collaboration_backend.semantic_core]

The built-in `portable-json.v1` backend works with no Radicle profile, node, or
repository ID:

```bash
cairn collaboration import my-change \
  --backend portable-json.v1 \
  --store /path/to/my-change.cairn.json \
  --workspace example/workspace \
  --base-revision opaque:review-one \
  --actor operator-label \
  --root /path/to/repo [--execute]

cairn collaboration status my-change \
  --backend portable-json.v1 \
  --store /path/to/my-change.cairn.json \
  --root /path/to/repo

cairn collaboration checkout my-change \
  --backend portable-json.v1 \
  --store /path/to/my-change.cairn.json \
  --root /path/to/repo [--execute]

cairn collaboration update my-change \
  --backend portable-json.v1 \
  --store /path/to/my-change.cairn.json \
  --actor operator-label \
  --root /path/to/repo [--execute]

cairn collaboration conflicts my-change \
  --backend portable-json.v1 \
  --store /path/to/my-change.cairn.json \
  --root /path/to/repo [--show-head proposal=REVISION]

cairn collaboration resolve my-change \
  --backend portable-json.v1 \
  --store /path/to/my-change.cairn.json \
  --actor operator-label \
  --choose-head proposal=REVISION \
  --root /path/to/repo [--lifecycle open] [--execute]
```

Import, update, and checkout are preview-first. Portable import reads bounded
exact artifacts, computes a deterministic root operation, validates the complete
neutral state, and emits a plan without writing unless `--execute` is present.
Update compares bounded exact local artifacts with conflict-free provider state,
emits sorted `SetArtifact` actions with complete current revision bases, and
classifies byte-equivalent input as a no-op. Executed update serializes the store
and local change, rechecks policy, state, actor, artifacts, and plan, validates a
prepared envelope, atomically replaces only the expected store, and reloads every
BLAKE3 identity. Import similarly refuses an existing or unsafe store and
atomically exposes a validated prepared file. Status is bounded and read-only.
Checkout rechecks the envelope and local destination while locked, then uses the
same no-overlay exact-byte materialization contract as the COB-only adapter. Exact
destinations are no-ops; divergent, partial, symlinked, non-UTF-8, oversized,
conflicted, or unsupported destinations fail closed.

`conflicts` reports every current competing head through sorted revision, actor,
deletion, byte-length, and BLAKE3 metadata. `--show-head ARTIFACT=REVISION`
returns one exact bounded candidate. `resolve` requires exactly one
`--choose-head ARTIFACT=REVISION` for every conflicted artifact, copies each
selected value into a new revision whose basis supersedes every current head,
and refuses missing, extra, duplicate, stale, or non-head choices. A lifecycle
conflict also requires the explicit conservative `--lifecycle open`; Cairn never
carries an approval-like head or stale evidence forward. Portable conflicted
envelopes record `content_root: null` while retaining validated state and
envelope identities. Resolution is preview-first and uses the same store and
repository locks, full rechecks, prepared replacement, and reload verification
as update. r[impl cairn.collaboration_resolution.inspection]
r[impl cairn.collaboration_resolution.plan]
r[impl cairn.collaboration_resolution.portable_execution]
r[verify cairn.collaboration_resolution.portable_execution.success]
r[verify cairn.collaboration_resolution.portable_execution.blocked]

r[impl cairn.collaboration_update.portable_execution]
r[verify cairn.collaboration_update.portable_execution.success]
r[verify cairn.collaboration_update.portable_execution.blocked]
r[impl cairn.collaboration_backend.portable_state]
r[impl cairn.collaboration_backend.portable_shell]
r[impl cairn.collaboration_backend.cli]

The portable `--actor`, `--workspace`, and base revision are caller declarations.
They are not signatures or remote authority. Receipts label the backend and this
authority class, bind workspace/base/state/content/policy/file identities, and
state that local success does not prove publication, synchronization, remote
trust, gate validity, evidence truth, dependency readiness, archive execution,
source correctness, or release eligibility.
r[impl cairn.collaboration_backend.receipts]
r[verify cairn.collaboration_backend.non_claims.provider]

Cairn also provides a local, COB-only adapter. It uses `radicle-cob` as a
format/CRDT library over an explicit Git repository; it does not load a Radicle
profile or identity document and does not run or control a Radicle node. The
selected Git repository must define repository-local `user.name` and `user.email`
for format-required commit metadata; these display fields grant no Cairn
authority and global Git config is not accepted as a hidden fallback:

```bash
cairn collaboration import my-change \
  --backend radicle-cob.v1 \
  --git-repository /path/to/git/repository \
  --keystore /path/to/ssh-keystore \
  --workspace workspace-id \
  --delegate z6MkDelegatePublicKey \
  --root /path/to/repo [--execute]

cairn collaboration status my-change \
  --backend radicle-cob.v1 \
  --git-repository /path/to/git/repository \
  --root /path/to/repo

cairn collaboration update my-change \
  --backend radicle-cob.v1 \
  --git-repository /path/to/git/repository \
  --keystore /path/to/ssh-keystore \
  --root /path/to/repo [--execute]

cairn collaboration conflicts my-change \
  --backend radicle-cob.v1 \
  --git-repository /path/to/git/repository \
  --root /path/to/repo [--show-head proposal=REVISION]

cairn collaboration resolve my-change \
  --backend radicle-cob.v1 \
  --git-repository /path/to/git/repository \
  --keystore /path/to/ssh-keystore \
  --choose-head proposal=REVISION \
  --root /path/to/repo [--lifecycle open] [--execute]
```

`cairn cob import|status|checkout|update|conflicts|resolve` is the direct entry
point. Import, update, checkout, and resolution remain preview-first. Read-only
status and conflict inspection require no key. Signed mutation loads only the
explicit keystore, proves that its public key matches the planned actor, acquires
a per-change lock in the selected Git directory, rechecks every preview input,
writes a standard signed COB change and namespaced `refs/cobs` reference, reloads
the typed object, and verifies the planned content root. The signed operation ID
and resulting state hash remain provider-assigned and are never precomputed.

The root operation records the creator key and a bounded canonical delegate set.
For neutral-schema objects this embedded set is the complete immutable mutation
authority: caller claims cannot add authority later. Existing
`cairn.radicle-change-cob.v1` objects with no embedded delegates remain readable
and retain their exact legacy canonical content and state hashes; the creator can
still mutate them, but removed identity-document delegate authority is not
silently reintroduced.

Cairn intentionally has no collaboration sync, discovery, seeding, follow,
announce, connect/fetch, peer address, node ID, control socket, Radicle home, or
profile behavior. Moving COB refs between repositories is an external transport
concern. Local success proves only the selected Git repository, signed object
history, embedded authority, typed reduction, and bounded filesystem observations;
it does not prove publication, synchronization, convergence, peer availability,
remote trust, lifecycle correctness, evidence truth, or release eligibility.

The dedicated compatibility gate verifies standard namespaced Git refs, signed
COB commit manifests, typed reload, malformed-history rejection, and absence of
the umbrella `radicle` and `radicle-node` crates:

```bash
nix build path:$PWD#checks.x86_64-linux.radicle-cob-format-compatibility --no-link -L
```

r[impl cairn.radicle_cob_only.store]
r[verify cairn.radicle_cob_only.store.roundtrip]
r[verify cairn.radicle_cob_only.store.negative]
r[impl cairn.radicle_cob_only.authority]
r[verify cairn.radicle_cob_only.authority.allowed]
r[verify cairn.radicle_cob_only.authority.denied]
r[verify cairn.radicle_cob_only.authority.bounded]
r[impl cairn.radicle_cob_only.cli]
r[verify cairn.radicle_cob_only.cli.explicit]
r[verify cairn.radicle_cob_only.cli.reject_runtime]
r[impl cairn.radicle_cob_only.no_runtime]
r[verify cairn.radicle_cob_only.no_runtime.sync]
r[verify cairn.radicle_cob_only.no_runtime.artifacts]
r[impl cairn.radicle_cob_only.compatibility]
r[verify cairn.radicle_cob_only.compatibility.legacy]
r[verify cairn.radicle_cob_only.compatibility.git_refs]
r[verify cairn.radicle_cob_only.compatibility.signatures]
r[verify cairn.radicle_cob_only.compatibility.malformed]
r[impl cairn.radicle_cob_only.dependencies]
r[verify cairn.radicle_cob_only.dependencies.excludes_runtime]
r[impl cairn.radicle_cob_only.docs]

The pure core remains transport-free: it receives explicit actor/delegate facts,
reduces revisions and conflicts, validates portable envelopes, derives sorted
artifact-update actions with complete revision bases, and derives a sorted
materialization plan to `proposal.md`, `design.md`, `tasks.md`, `metadata.json`,
and `specs/<stable-id>/spec.md`. Repository access, files, locks, signing,
identity lookup, clocks, process execution, and JSON output stay in provider
shells. Cross-change dependency existence, cycles, readiness, and archive
ordering remain aggregate Cairn repository decisions after all selected provider
states are materialized. Ordinary update still refuses content or lifecycle
conflicts and never treats local bytes as an implicit winner. Explicit resolution
retains every competing revision, requires complete current-head choices, and
creates one superseding operation; it does not rewrite history or perform a
semantic/textual merge. Passing inspection, update, or resolution receipts prove
only the recorded local DAG plan, authority observation, and provider mutation
result. They do not prove merge correctness, losing-head invalidity, lifecycle
approval, evidence truth, network publication, peer convergence, remote trust,
source correctness, archive execution, or release eligibility.
r[impl cairn.collaboration_resolution.receipts]
r[verify cairn.collaboration_resolution.receipts.modes]
r[verify cairn.collaboration_resolution.non_claims.local]
r[impl cairn.collaboration_update.plan]
r[verify cairn.collaboration_update.plan.exact]
r[verify cairn.collaboration_update.plan.noop]
r[verify cairn.collaboration_update.plan.blocked]
r[impl cairn.collaboration_update.receipts]
r[verify cairn.collaboration_update.receipts.modes]
r[verify cairn.collaboration_update.non_claims.local]

## Lifecycle views

Cairn can project the same reviewed lifecycle facts as a kanban board, containment
mindmap, or typed knowledge graph. JSON is the deterministic default; Markdown,
Mermaid, and DOT are explicit static formats, and `--interactive` opens a
read-only terminal viewer. Graph canvases use the pinned Subwayrat
`rat-nodegraph` component without making `../rats/` a runtime checkout
dependency. Visual coordinates, selection, pan, and zoom remain display-only and
never enter Cairn receipt hashes. See [`docs/lifecycle-views.md`](docs/lifecycle-views.md).
r[impl cairn.lifecycle_views.docs] r[verify cairn.lifecycle_views.docs.boundary]

## Running with Nix

For a full first-use flow, start with the [Make a Cairn tutorial](docs/tutorials/make-a-cairn.md)
or the [copyable example](examples/make-a-cairn/README.md).

```bash
nix run path:$PWD#cairn -- init --root /path/to/repo
nix run path:$PWD#cairn -- change create native-lifecycle --root /path/to/repo
nix run path:$PWD#cairn -- template list --root /path/to/repo
nix run path:$PWD#cairn -- template apply agent-workflow --root /path/to/repo --execute
nix run path:$PWD#cairn -- scaffold change first-change --root /path/to/repo --request "Add the first reviewed capability" --execute
nix run path:$PWD#cairn -- discover-specs --root /path/to/repo --include src --include docs --out target/reverse-spec-draft.md
nix run path:$PWD#cairn -- change list --root fixtures/basic
nix run path:$PWD#cairn -- change depend list demo --root fixtures/basic
nix run path:$PWD#cairn -- change depend add demo prerequisite --root fixtures/basic
nix run path:$PWD#cairn -- change graph --root fixtures/basic --format json
nix run path:$PWD#cairn -- change graph --root fixtures/basic --format mermaid
nix run path:$PWD#cairn -- view kanban --root . --format markdown
nix run path:$PWD#cairn -- view mindmap --root . --format mermaid
nix run path:$PWD#cairn -- view graph --root . --interactive
nix run path:$PWD#cairn -- change next --root fixtures/basic
nix run path:$PWD#cairn -- change show demo --root fixtures/basic
nix run path:$PWD#cairn -- spec list --root fixtures/basic
nix run path:$PWD#cairn -- spec show demo --root fixtures/basic
nix run path:$PWD#cairn -- status --root fixtures/basic
nix run path:$PWD#cairn -- doctor --root fixtures/basic
nix run path:$PWD#cairn -- validate --root .
nix run path:$PWD#cairn -- review profiles
nix run path:$PWD#cairn -- review validate review.json --root .
nix run path:$PWD#cairn -- review ledger --include reviews --root .
nix run path:$PWD#cairn -- release-readiness --root . --review-ledger target/review-ledger.json
nix run path:$PWD#cairn -- review revision validate revision.json --root .
nix run path:$PWD#cairn -- gate tasks demo --root fixtures/basic
nix run path:$PWD#cairn -- sync demo --root fixtures/basic
nix run path:$PWD#cairn -- migrate openspec --root fixtures/basic
nix run path:$PWD#cairn -- agent-receipt preserves-hash fixtures/agent-receipts/positive.json
nix run path:$PWD#cairn -- preserves-corpus update --path fixtures/preserves-canonicalization/corpus.json
```

Local change dependencies use the optional `depends_on` array in a change's
`metadata.json` to order active changes inside one repository. `cairn change
depend add <change> <dependency>` and `cairn change depend remove <change>
<dependency>` preview metadata edits by default and write only with `--execute`.
The authoring commands reject invalid ids, unknown changes, duplicate edges,
self-dependencies, and cycle-forming edits before mutation. `cairn change
graph --format json|mermaid|dot` renders the same local DAG for automation or
review, and `cairn change next` reports the first dependency-ready active change
from the drain order. Archived dependencies are treated as satisfied; active
dependencies block sync/archive execution until the dependency archives. These
same-repo ordering dependencies are separate from cross-repo receipt/evidence
dependencies, which require explicit evidence inputs and do not add remote nodes
to the local graph. r[impl cairn.change_dependency_graph.docs.discoverability]
r[verify cairn.change_dependency_graph.docs.discoverability.local]
r[verify cairn.change_dependency_graph.docs.discoverability.boundary]

For the Preserves receipt/hash operator workflow, including the JSON receipt boundary, read-only inspection, corpus update rail, and CI drift guards, see [`docs/preserves-receipts.md`](docs/preserves-receipts.md). For Preserves dependency upgrades, use [`docs/preserves-dependency-bumps.md`](docs/preserves-dependency-bumps.md).

r[impl cairn.determinism.preserves_operator_docs.discoverability]
r[impl cairn.determinism.preserves_dependency_bump_runbook.discoverability]

`review validate <receipt.json>` validates externally produced semantic review
receipts without running reviewers or interpreting model output. The receipt schema
records `schema_version`, `review_kind`, `decision`, `reviewer.source/id`, hashed
`reviewed_inputs`, and categorized findings; when an input path exists under
`--root`, Cairn checks the stated BLAKE3 hash against the local artifact. Non-
approved decisions require at least one finding categorized by the built-in
omission/systematic/incoherent/infra taxonomy. `review ledger --include <path>`
loads review receipts from files or directories and emits deterministic counts,
input receipt hashes, invalid-input reports, and small-sample warnings. `review
revision validate <receipt.json>` validates revision handoff receipts linking
prior review hashes to revision artifacts and finding dispositions. `review
profiles` prints the policy-owned review kinds, reviewer sources, decisions,
severities, and profile criteria. `release-readiness --review-ledger <path>`
consumes a deterministic `review ledger` receipt and fails the readiness bundle
when that supplied ledger is invalid or reports a failing verdict; when no ledger
is supplied and no conventional `docs/semantic-review-ledger.json` exists, the
readiness receipt records an explicit skipped semantic-review check. These
commands validate structure, hashes, and policy vocabularies only; they do not call LLMs, execute reviewer code, publish
remote state, or claim to prove semantic correctness. r[impl cairn.semantic_review.semantic_review_receipts.docs] r[impl cairn.semantic_review.semantic_review_receipts.docs.docs.non_claims] r[impl cairn.semantic_review.semantic_review_boundary.boundary.boundary.claims]

`template list` and `template apply <id>` provide a reviewed first-use catalog for
`agent-workflow`, `generic-cli`, `rust-nix`, `docs-lifecycle`, and
`release-evidence` onboarding profiles. Template application is preview-first
unless `--execute` is passed; executed receipts list created files and BLAKE3-bound
mutation manifests. Templates create quickstart artifacts only and do not write
accepted specs, contact network services, or claim remote publishing/signing.
r[impl cairn.developer_experience.onboarding_templates.quickstart]
r[impl cairn.developer_experience.onboarding_template_catalog_expansion.docs]

`scaffold change <name> --request <text>` turns explicit local request material
into a gate-ready draft change package with proposal, design, tasks, and a delta
spec. The command records an input hash, supports preview or `--execute`, and
uses deterministic templates rather than built-in LLM/network inference. For
AI-assisted lifecycle drafts, record only safe provenance metadata and require a
separate accepted evidence path; see
[`docs/ai-assisted-draft-provenance.md`](docs/ai-assisted-draft-provenance.md).
r[impl cairn.developer_experience.prompt_to_change_scaffold.docs]

`discover-specs --include <path>` is a read-only reverse-spec discovery rail for
existing repositories. Includes may name explicit files or safe repository-relative
directories; directory includes expand deterministically over text-like source,
docs, tests, and config files while skipping hidden/build/vendor artifacts. The
command inventories explicit evidence paths, emits review-needed candidate
requirements with evidence paths and input hashes, and only writes draft artifacts
when `--out` is provided. Candidates are not accepted specs or implementation proof
until reviewed through the normal Cairn lifecycle.
r[impl cairn.developer_experience.reverse_spec_discovery.docs]
r[impl cairn.developer_experience.reverse_spec_directory_inventory.docs]

`doctor [--root PATH] [--policy PATH]` is read-only and emits a compact JSON
health receipt for the selected layout, policy load status, discovered spec/change
counts, validation issue count, and diagnostic checks. Unlike `validate`, it
reports policy read/parse failures as JSON instead of aborting before a receipt.

`gate proposal|design|tasks <change>` evaluates AgentKit/OpenGate-compatible
stage semantics from Cairn-owned Nickel policy data and returns a JSON
receipt. Stable public gate fields are `change`, `stage`, `verdict`, `valid`,
`issues`, `policy`, `layout`, `policy_hash`, `input_hash`, and `receipt_hash`.
Those receipt/action/hash field declarations are owned by
`cairn-policy/default.ncl`, exported to `cairn-policy/generated/cairn-policy.json`,
and regression-tested against CLI JSON output. Runtime commands consume the
generated JSON policy and do not import, execute, or shell out to Nickel. The
gate also reads active change artifacts into a pure core snapshot: unknown
`r[...]` references in proposal/design/tasks fail, tasks-stage checkbox tasks
must reference a known delta requirement or carry an explicit `[depends:...]`
marker from the generated marker vocabulary, and `gate-exempt[...]` markers must
match the generated exemption marker contract (`reason=`, `owner=`, and ISO-like
`until=YYYY-MM-DD`).

`sync` and `archive` are dry-run by default. Use `--execute` only when you want
local filesystem mutation after reviewing the plan. Stable public lifecycle
receipt fields are `actions`, `blocked`, `change`, `dry_run`, `input_hash`,
`layout`, `mutated`, `mutation_manifest`, `plan_hash`, `policy`, `policy_hash`,
`reasons`, and `receipt_hash`; sync receipts also include `delta_specs`.
Lifecycle action entries expose stable `kind`, `path`, and `description` fields.
`input_hash`, `plan_hash`, and `receipt_hash` are computed from root-portable
JSON: checkout/temp-directory prefixes are normalized before hashing while
operator-facing `path` and `root` fields remain display paths. Executed
`sync`, `archive`, and `migrate` receipts include a `mutation_manifest` with
before/after BLAKE3 content hashes and a manifest hash; dry-run receipts set it
to `null`. Sync/archive
execution first enforces the configured deterministic gate when the base plan is
otherwise unblocked. Executed sync merges delta
requirement sections into accepted specs; it preserves accepted spec title,
purpose, and `## Requirements` wrapper content instead of copying partial delta
documents over accepted specs:

```bash
nix run path:$PWD#cairn -- sync ready --root /path/to/repo --execute
nix run path:$PWD#cairn -- archive ready --root /path/to/repo --execute
```

## Lossless archive compaction

Archive compaction is separately dry-run-first. Generate and review a deterministic
plan, execute that exact saved plan, verify the resulting local representation,
and restore one archive when expanded bytes are needed again:

```bash
nix run path:$PWD#cairn -- compact archive --root . --profile lossless-pack --out target/compact-plan.json
nix run path:$PWD#cairn -- compact archive --root . --plan target/compact-plan.json --execute
nix run path:$PWD#cairn -- compact verify --root .
nix run path:$PWD#cairn -- compact unpack --root . --archive 2026-05-18-example-change
nix run path:$PWD#cairn -- compact unpack --root . --archive 2026-05-18-example-change --execute
```

The lossless pack stores raw file objects under BLAKE3 identities, a canonical
sorted manifest for each archive, a pack manifest, and one retained pointer per
compacted archive. Merkle-style `archive_root`, `packed_archives_root`, and
`pack_root` values prove which content the local representation includes and
allow a pack or object to be checked independently. The compaction ledger is a
different structure: every receipt binds `prev_chain_hash`, `receipt_hash`, and
`chain_hash`, so chain verification detects removed, modified, inserted, or
reordered operations within the observed ledger. Genesis is explicitly derived
from the policy hash that created the ledger; each later receipt binds the policy
used for that operation, so policy updates do not discard prior history. Each
operation receipt also binds `pre_state_root`, `post_state_root`,
`packed_archives_root`, `retained_manifest_root`, `lossiness`, `policy_hash`,
`plan_hash`, and `mutation_manifest_hash` in addition to the predecessor,
receipt, and chain hashes.

Execution rechecks `policy_hash`, `input_hash`, and `plan_hash`; writes and
verifies objects, manifests, prepared pointers, the mutation manifest, receipt,
and ledger before pruning expanded archives; then finalizes retained pointers.
`compact verify` checks ledger continuity, receipt hashes, pack roots, object
bytes, pointer safety, mutation-manifest linkage, and orphaned or prepared
artifacts left by interrupted execution. `compact unpack` verifies the whole
state before restoring files to a prepared directory, checks every restored
BLAKE3 hash and byte length, and only then renames the exact-byte tree into
`cairn/archive/`. Retained pointers and packs remain after unpack so the prior
compaction receipt stays auditable. Re-running unpack against identical restored
bytes is a no-op.

Only `lossless-pack` is executable. `lossy-summary` is rejected unless a future
reviewed policy and receipt contract explicitly enables irreversible tombstones
and visible non-claims. A passing verification proves local representation
integrity only: it does **not** prove archived change correctness, release
readiness, remote trust, semantic review completeness, or that an unanchored
final ledger suffix was never removed. Preserve verification receipts or ledger
tips in separately controlled evidence when that stronger audit property is
required.

r[impl cairn.sync_archive.compaction.docs]
r[impl cairn.sync_archive.compaction.docs.chain_merkle]
r[impl cairn.sync_archive.compaction.docs.non_claims]

## Agent/CI determinism replay

`cairn determinism replay --root <fixture-or-repo>` is a read-only workspace
preflight for agents and CI. Its default case list is exported from
`cairn-policy/generated/cairn-policy.json` under `determinism_replay_policy`, so
case ids, command kinds, fixture roots, comparison modes, display-only fields,
hash fields, and normalization rules are reviewed as policy instead of hidden CLI
literals. Cases currently cover status, validate, proposal/design/tasks gates,
sync/archive/migration dry-runs, agent-receipt validation, release-readiness, MCP
describe, and selected MCP read-only calls. Replay copies fixture roots when
cross-root comparison is requested, runs cases repeatedly, and emits a single
deterministic receipt.

For CI triage, select one case with `--case <id>` or one policy group with
`--group <id>`, render compact logs with `--format summary` or `--format jsonl`,
and write the full artifact with `--out <path>`:

```bash
cairn determinism replay --root . --group smoke --format summary --out target/replay.json
```

Replay groups are policy-owned named suites (`smoke`, `agent`, `release`, and
`full`) whose membership lives in generated policy. Replay exits nonzero when
evidence is invalid, including unknown case IDs, unknown group IDs, selector
conflicts, hidden entropy, mutation-denied preflights, or mismatches. Stable public determinism replay fields are `command`, `root`, `valid`, `verdict`,
`available_cases`, `available_groups`, `selected_cases`, `selected_groups`,
`entropy_inventory`, `entropy_assumptions`, `entropy_policy_hash`,
`environment_attestation`, `summary`, `cases`, `skipped`, `mismatches`,
`nondeterminism_failures`, `mutability_observations`, `normalization_rules`,
and `receipt_hash`; the group
selector fields are `available_groups`, `selected_groups`. Case entries expose stable `id`,
`command_kind`, `args`, `fixture_root`, `expected_mutability`,
`comparison_modes`, `normalization_rules`, `display_only_paths`, `hash_fields`,
`runs`, and `portable_evidence_hash` fields. `entropy_inventory` is policy-owned
and declares each source `id`, class, source name, handling mode, receipt field,
and description; `entropy_assumptions` is the compact non-rejected subset and
`entropy_policy_hash` is a BLAKE3 hash of the reviewed inventory. Current
inventory coverage includes clock/date defaults, locale/timezone formatting,
cwd/root path normalization, rejected archive-date env vars, filesystem
order/metadata, external command versions, and Nix/build context. Rejected
entropy failures disclose source names and handling but never environment values.
`environment_attestation` is policy-owned and binds the replay to stable Cairn
package/version identity, generated policy hash, entropy policy hash, selected
cases/groups, replay scope, reviewed environment facts, and
`attestation_hash`. It intentionally omits raw environment values, usernames,
absolute checkout roots, and machine-specific temporary or store paths.
Saved replay receipts can be rechecked with
`cairn determinism verify-receipt <receipt.json> --root .`; verification is
read-only and recomputes the replay receipt hash, compares current generated
policy and attestation hashes, validates selected cases/groups, and emits
nonzero deterministic failure receipts for malformed, stale, or tampered replay
evidence. Stable public replay verification fields are `command`, `root`,
`path`, `valid`, `verdict`, `issues`, `current_replay_policy_hash`,
`replay_policy_hash`, `replay_receipt_hash`, `stated_replay_receipt_hash`,
`selected_cases`, `selected_groups`, `environment_attestation_hash`, and
`receipt_hash`. r[verify cairn.supersede.replay_receipt_verification.docs]
Mismatch entries expose stable `case`, `comparison`, `field_path`, `class`,
`expected`, `actual`, and `reproduce` fields. The default oracle rejects mutation flags and records
mutation observations as deterministic failures.

## Agent-run receipts

`cairn-core` includes pure helpers for deterministic agent-run receipts:
`agent_run_receipt_hash`, `agent_run_receipt_issues`, and
`evaluate_agent_run_completion`. Agent wrappers can use these to produce a
canonical receipt with `schema_version`, `task_id`, `agent_identity`,
`input_hash`, `plan_hash`, `tool_manifest`, `mutation_manifest`,
`verification_manifest`, `nondeterminism_manifest`, `replay_evidence`,
`verdict`, and `receipt_hash`. The `replay_evidence` object binds the agent
run to deterministic replay preflight evidence with stable
`replay_receipt_hash`, `replay_policy_hash`, `replay_selected_cases`,
`replay_selected_groups`, `replay_verdict`, and optional
`replay_environment_attestation_hash` fields, plus optional
display-only `replay_artifact_path`. Agent wrappers should run
`cairn determinism replay --group agent` and record the resulting `agent` group
and expanded case list. `agent-receipt validate <path>` checks that replay
evidence cites a passing replay verdict, the current generated policy hash, the
policy-owned `agent` replay group, and every case expanded from that group.
Missing, stale, failed, or insufficient replay evidence emits deterministic issue
codes such as `replay_evidence.missing`, `replay_evidence.policy_hash_stale`,
`replay_evidence.missing_group_coverage`, and
`replay_evidence.insufficient_coverage`. The CLI can replay a stored receipt
with `agent-receipt validate <path>` and emits deterministic validation JSON for
positive and negative fixtures. Stable public agent receipt validation fields
are `command`, `path`, `input_kind`, `valid`, `verdict`, `issues`,
`agent_receipt_hash`, `input_hash`, `stated_agent_receipt_hash`,
`current_replay_policy_hash`, `required_replay_group`, `required_replay_cases`, `replay_evidence`,
`max_content_bytes`, and `receipt_hash`. Wrappers that publish multiple artifacts
can verify the whole chain with
`cairn agent-receipt verify-bundle <bundle.json> --root .`; the bundle references
an agent receipt, saved replay receipt, saved replay verification receipt,
optional release-readiness receipt, and optional coverage-audit receipt. Stable public agent evidence bundle verification fields
are `command`, `root`, `valid`, `verdict`, `issues`,
`current_replay_policy_hash`, `required_replay_group`, `agent_receipt_hash`,
`replay_receipt_hash`, `replay_verification_receipt_hash`,
`environment_attestation_hash`, `release_readiness_receipt_hash`,
`coverage_audit_receipt_hash`, and
`receipt_hash`.

Cairn opens operator-declared repository, evidence-bundle, registry, release,
replay, and scan roots as typed capability directories before shell code reads,
writes, copies, traverses, or removes relative paths. Traversal, absolute-path,
and symlink escape attempts fail before access outside the selected root. This
bounds local filesystem authority only: it does not prove artifact truth, replay
completeness, lifecycle correctness, verifier soundness, or release readiness.
r[verify cairn.release_evidence.cap_std_boundary.docs]

The first-class publication path is
`cairn evidence export --root . --out target/cairn-evidence/`, which materializes
coverage audit, replay, replay verification, release-readiness, agent bundle,
bundle verification, evidence manifest, and manifest verification artifacts in a
portable directory. Stable public evidence export fields are `command`, `root`,
`output`, `valid`, `verdict`, `issues`, `artifacts`, `manifest_hash`,
`manifest_verification_receipt_hash`, and `receipt_hash`. Consumers can verify a
received export with `cairn evidence verify --root . --bundle target/cairn-evidence/`,
which rehashes exported artifacts, composes the nested verifiers, and emits
stable fields `command`, `root`, `bundle`, `valid`, `verdict`, `issues`,
`artifacts`, `export_receipt_hash`, `manifest_hash`,
`manifest_verification_receipt_hash`, and `receipt_hash`. Consumers can then
materialize a bounded local copy with
`cairn evidence import --root . --bundle target/cairn-evidence/ --into target/cairn-imported-evidence/`,
which verifies first, copies only declared export artifacts, rejects undeclared
bundle files, and emits stable fields `command`, `root`, `bundle`, `into`,
`valid`, `verdict`, `issues`, `artifacts`, `verification_receipt_hash`, and
`receipt_hash`. Imported evidence can be republished with
`cairn evidence republish --root . --imported target/cairn-imported-evidence/ --out target/cairn-republished-evidence/`,
which verifies the import receipt, rehashes declared imported artifacts, rejects
extra or unsafe imported files, and emits stable fields `command`, `root`,
`imported`, `out`, `valid`, `verdict`, `issues`, `artifacts`,
`import_receipt_hash`, and `receipt_hash`. Published bundles can be cataloged
with `cairn evidence index --root . --bundle target/cairn-republished-evidence/ --out target/cairn-evidence-index.json`
and checked with `cairn evidence index verify --root . --index target/cairn-evidence-index.json`.
The index rehashes declared bundle artifacts, records the bundle kind, canonical
bundle receipt, receipt chain, artifact roles/paths/hashes, and emits stable
fields `command`, `root`, `bundle`, `out`, `valid`, `verdict`, `issues`,
`bundle_kind`, `bundle_receipt_hash`, `receipt_chain`, `policy_hash`,
`artifacts`, and `receipt_hash` without raw host paths. Multiple published
indexes can be aggregated with
`cairn evidence registry build --root . --index target/cairn-evidence-index.json --out target/cairn-evidence-registry.json --channel stable --release 2026-05-19 --platform x86_64-linux --compatibility-class current`
and checked with
`cairn evidence registry verify --root . --registry target/cairn-evidence-registry.json`.
Mirrored or offline copies can be checked before consumption with
`cairn evidence registry mirror-verify --root . --registry target/cairn-evidence-registry.json --mirror target`.
The mirror verifier reuses the registry's safe `index_relative` entries, rehashes
mirrored indexes, preserves labels (`channel`, `release`, `platform`,
`compatibility_class`), index hashes, bundle kinds, bundle receipt hashes,
receipt chains, artifact hashes, and policy hashes, and emits stable fields `command`, `root`, `out` or `registry`, `valid`,
`verdict`, `issues`, `entry_count`, `entries`, `registry_hash`, and
`receipt_hash` without raw host paths. Downstream agents can consume a registry
with
`cairn evidence registry select --root . --registry target/cairn-evidence-registry.json --channel stable --release 2026-05-19 --platform x86_64-linux --bundle-kind republished --out target/selected-cairn-evidence-index.json`
or produce a metadata-only plan with
`cairn evidence registry fetch-plan --root . --registry target/cairn-evidence-registry.json --out target/cairn-evidence-fetch-plan.json`.
Selection verifies the registry first, requires exactly one matching entry, and
binds selectors, registry receipt hash, registry verification receipt hash, and
selected entry metadata into a portable receipt. Fetch plans verify the registry
first, sort plan entries deterministically, and list safe index refs, label
metadata, receipt chains, artifact roles/paths/hashes, and policy hashes without
fetching network
content, credentials, raw host paths, or environment values. A verified plan can
then be locally materialized from an offline mirror/source directory with
`cairn evidence registry fetch-execute --root . --plan target/cairn-evidence-fetch-plan.json --source target/cairn-republished-evidence/ --out target/cairn-fetch-cache/`.
Fetch execution rejects unverified plans and unsafe relative paths, verifies each
artifact's BLAKE3 hash before and after copying, emits deterministic failure
receipts for missing or tampered artifacts, and records stable fields such as
`fetch_plan_receipt_hash`, `registry_receipt_hash`,
`registry_verification_receipt_hash`, `materialized_count`, `materialized`, and
`receipt_hash`; it still performs no network transport, credential handling, or
remote trust validation. r[cairn.supersede.fetch_execute_policy_docs.docs.flow]
r[cairn.supersede.fetch_execute_policy_docs.docs.non_claims] The outer publication/index layer can also be generated with
`cairn evidence-manifest generate --bundle bundle.json --agent-receipt agent.json --replay replay.json --replay-verification replay-verify.json --registry registry.json --registry-verification registry-verify.json --registry-selection registry-selection.json --registry-fetch-plan registry-fetch-plan.json --out manifest.json --root .`
and verified with `cairn evidence-manifest verify <manifest.json> --root .`; manifest artifacts
list stable roles such as `agent_evidence_bundle`, `agent_receipt`,
`replay_receipt`, `replay_verification`, optional `release_readiness`, and
optional `coverage_audit`, optional `evidence_registry`, optional
`evidence_registry_verification`, optional `evidence_registry_selection`, and
optional `evidence_registry_fetch_plan` with
expected hashes, schema labels, and optional public signer/anchor metadata.
Generation produces canonical role ordering and artifact hashes so wrappers do
not need custom JSON or hashing glue. Verification is read-only, composes
existing validators, and does not claim cryptographic signature or remote anchor
validity. Stable public evidence manifest verification fields are `command`,
`root`, `valid`, `verdict`, `issues`, `manifest_hash`, `artifacts`,
`coverage_audit_receipt_hash`, `signer_metadata_hash`, `anchor_metadata_hash`, and `receipt_hash`. See
`docs/agent-run-receipts.md` for wrapper guidance,
validation commands, and privacy rules. The checked-in release evidence index for verified payload commit `b253c9c` lives as an immutable audited snapshot at `docs/release-evidence/payload-b253c9c.md`; checked-in files must not claim to be the mutable current HEAD. Operators can generate the current clean HEAD evidence index into an artifact path with `cairn release-evidence index --root . --out target/release-evidence-current-head/index.md --artifact-dir target/release-evidence-current-head`; the command composes the existing release-readiness, evidence export, verify, import, republish, index, and index-verify rails, keeps bulky generated receipts in the artifact directory, fails closed when the checkout is dirty, and records only the evaluated payload HEAD rather than any later documentation commit. Operators can check generated or audited indexes with `cairn release-evidence check-index --root . --index target/release-evidence-current-head/index.md --mode current-head` or `--mode audited-payload`; release readiness also checks the conventional checked-in `docs/release-evidence-current-head.md` path if it exists so stale current-head claims fail closed. Producers can materialize a local publication tree from verified generated evidence with `cairn release-evidence publish --root . --index target/release-evidence-current-head/index.md --artifact-dir target/release-evidence-current-head --publish-dir target/release-evidence-publish --channel stable --release YYYY-MM-DD --retention local`; the command verifies the source evidence index, copies the Markdown index plus required artifacts under safe relative paths, emits `manifest.json` and a receipt with source index, source receipt, verification receipt, materialized artifact, label, and manifest hashes, and rejects unsafe paths, hash drift, malformed labels, and invalid source evidence. Operators can plan local retention over materialized publication trees with `cairn release-evidence retention-plan --root . --publish-root target/release-evidence-publish-root --out target/release-evidence-retention-plan.json --channel stable --keep-latest 1` and dry-run or execute local rotation with `cairn release-evidence rotate --root . --plan target/release-evidence-retention-plan.json [--execute]`; retention planning verifies each candidate `manifest.json`/`receipt.json`, emits deterministic keep/supersede/prune-candidate decisions, and rotation verifies the saved plan before writing a mutation manifest. The generated index, materialized publication tree, and retention/rotation plan are local/trusted evidence only: they do not create or move tags, validate credentials or network transport, prove remote mirror trust, validate signatures, upload to public release pages, delete remote evidence, or claim public unattended production readiness. r[impl release_evidence.automated_current_head_index.operator_docs] r[verify release_evidence.automated_current_head_index.operator_docs] r[verify release_evidence.automated_current_head_index.operator_docs.non_claims] r[impl release_evidence.publish_materialization.operator_docs] r[verify release_evidence.publish_materialization.operator_docs] r[verify release_evidence.publish_materialization.operator_docs.non_claims] r[impl release_evidence.retention_rotation.operator_docs] r[verify release_evidence.retention_rotation.operator_docs] r[verify release_evidence.retention_rotation.operator_docs.non_claims] r[verify release_evidence.current_head_staleness_guard.operator_docs] r[verify release_evidence.current_head_staleness_guard.archive_ready] r[verify release_evidence.current_head_evidence_index.behavior_preserved.focused_checks] r[verify cairn.supersede.agent_run_receipts.schema]
r[verify cairn.supersede.agent_receipt_replay_evidence.docs]
r[verify cairn.supersede.agent_receipt_replay_group_coverage.docs]
r[verify cairn.supersede.agent_evidence_bundle_verification.docs]
r[verify cairn.supersede.signed_evidence_manifest.docs]
r[verify cairn.supersede.evidence_manifest_generate.docs]
r[verify cairn.supersede.determinism_coverage_release_evidence.release]
r[verify cairn.supersede.determinism_coverage_release_evidence.bundle_manifest]
r[verify cairn.supersede.determinism_coverage_release_evidence.docs]
r[verify cairn.supersede.determinism_coverage_release_evidence.archive]
r[verify cairn.supersede.agent_run_receipts.completion_gate]

r[impl release_evidence.current_head_staleness_guard.operator_docs]
r[verify release_evidence.current_head_staleness_guard.operator_docs]
r[verify release_evidence.current_head_staleness_guard.archive_ready]

## MCP server

`cairn mcp` exposes Cairn lifecycle receipts through a bounded MCP-style
JSON-RPC tool surface. The default server lists read-only inspection tools,
including `cairn_change_graph`, and dry-run planning tools only; mutation tools
are omitted unless the server starts with `--allow-mutations`. Tool responses include short text plus
`structuredContent` receipts with Cairn hashes and an `mcp` read/destructive
annotation object. See `docs/cairn-mcp.md`. r[verify cairn.supersede.mcp_server.verification.docs]

## Import legacy lifecycle content

`migrate openspec` is also dry-run by default. It copies accepted specs, active
changes, and archived changes from legacy `openspec/` into native `cairn/`
layout only when `--execute` is passed. Existing native destination files are
reported as blockers and are never overwritten. Stable public migration receipt
fields are `command`, `root`, `source_layout`, `destination_layout`, `actions`,
`conflicts`, `blocked`, `dry_run`, `mutated`, `mutation_manifest`, `policy`,
`policy_hash`, `source_manifest_hash`, `destination_manifest_hash`,
`manifest_hash`, and `receipt_hash`. Migration action entries expose stable
`source`, `destination`, `source_hash`, and `destination_hash` fields.

## Hermes skill source

`docs/hermes/cairn-workflow-skill.md` is the repo-maintained Hermes skill
source for future `cairn` work. It captures the FCIS boundary,
Cairn-native lifecycle workflow, explicit legacy migration bridge,
Slotcar/Nix verification rails, command contracts, and known pitfalls without
transient session state or secrets. Copy or patch it into a local Hermes skill
named `cairn-workflow` when updating agent workflow knowledge.

## License

Cairn uses a package boundary: `cairn-core` and reusable lifecycle templates are `MIT OR Apache-2.0`; the `cairn-cli` executable shell is `AGPL-3.0-or-later`. Complete texts are in [LICENSES](LICENSES), and the authoritative map is [docs/licensing.md](docs/licensing.md).

The split does not relicense dependencies, revoke prior grants, or turn lifecycle conformance into source-correctness or release-eligibility evidence.

## References

- [`../rats/subwayrat/`](https://github.com/brittonr/subwayrat) — maintained
  Ratatui canvas and node-graph components used by the opt-in read-only lifecycle
  viewer; Cairn pins a public revision and excludes visual layout from evidence
  hashes.
- `../trellis/`: local verified-primitive reference for graph,
  cycle-detection, topological-order, and dependency-readiness model shapes used
  when designing Cairn dependency graph work.
- [oxidecomputer/steno](https://github.com/oxidecomputer/steno) — saga execution, recovery-log, and compensation prior art for multi-step lifecycle operations.
- [oxidecomputer/rfd-api](https://github.com/oxidecomputer/rfd-api) and [oxidecomputer/rfd-site](https://github.com/oxidecomputer/rfd-site) — lifecycle document processing, rendered asset, state transition, search, webhook, and scanner prior art.
- [oxidecomputer/expectorate](https://github.com/oxidecomputer/expectorate) — golden snapshot update prior art for deterministic CLI output, receipts, and rendered report fixtures.
- [oxidecomputer/bundle-cat](https://github.com/oxidecomputer/bundle-cat) and [oxidecomputer/support-bundle-viewer](https://github.com/oxidecomputer/support-bundle-viewer) — read-only bundle exploration prior art for Cairn evidence bundle inspection.
- [oxidecomputer/omicron](https://github.com/oxidecomputer/omicron), [oxidecomputer/hubris](https://github.com/oxidecomputer/hubris), [oxidecomputer/humility](https://github.com/oxidecomputer/humility), [oxidecomputer/crucible](https://github.com/oxidecomputer/crucible), [oxidecomputer/propolis](https://github.com/oxidecomputer/propolis), [oxidecomputer/helios](https://github.com/oxidecomputer/helios), [oxidecomputer/maghemite](https://github.com/oxidecomputer/maghemite), [oxidecomputer/opte](https://github.com/oxidecomputer/opte), and [oxidecomputer/dendrite](https://github.com/oxidecomputer/dendrite) — study-only system references for observability, archive shape, debugging workflows, and operator diagnostics; Cairn does not adopt their domain-specific correctness claims.
- [joweeba/dpub](https://github.com/joweeba/dpub) — durable publish-run, status/explain/resume, and release-evidence ledger reference; Cairn adapts ledger and evidence-shape ideas without adopting remote publishing authority.
- [joweeba/tla2](https://github.com/joweeba/tla2) — TLA+/state-machine tooling reference for optional lifecycle transition receipts; Cairn validates bounded receipt identity and non-claims rather than embedding a model checker.
- [`radicle-cob`](https://crates.io/crates/radicle-cob) — signed Git-backed Collaborative Object format, operation-DAG evaluation, object/type identity, and generic storage traits used by `onix-cairn-radicle-cob`; Cairn owns explicit storage selection, embedded authority, lifecycle policy, evidence interpretation, and mutation locking without the Radicle runtime.

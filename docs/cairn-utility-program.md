# Cairn utility work

## Outcome and scope

The program reduces manual setup, repeated diagnosis, receipt construction, verification selection, and cross-repository coordination.
Cairn maintainers own the program. Repository owners retain implementation, execution, integration, and release decisions.

This document indexes five native change packages. It is not completion evidence or a second task system.
All packages use `spec-driven`. Their proposals name the current consumer, durable contribution, maintenance owner, and repeatability evidence.
The existing `outcome-machine` profile remains available. This program does not change its completion requirements.

| Change | Immediate consumer outcome | Reuse |
|---|---|---|
| `make-consumer-bootstrap-repeatable` | A new consumer reaches a valid native lifecycle without manual policy copying | Nickel Export, Cap Root, Durable File Publication, generated agent workflows |
| `unify-change-readiness-and-next-actions` | An operator obtains one exact blocker and next-action report | Existing Cairn workflow, gate, drain, completion, and machine contracts |
| `produce-check-evidence-with-stack-adapters` | Cairn and a Kiln integration fixture produce current check evidence without bespoke receipt scripts | Bounded Exec, Kiln, Lattice, Mantle, Octet, Valence |
| `derive-change-verification-guidance` | Cairn developers see which declared checks a source change affects | Cairn trace index and developer-loop profiles, Octet source evidence, Valence spans |
| `expose-cross-repository-adoption-gaps` | Workspace operators distinguish an upstream delivery from consumer adoption | `stack-deps`, `stack-doctor`, Cairn registry and dependency receipts, Valence |

## Order and integration

Bootstrap and unified readiness can start independently.
Evidence production uses the published completion contract and can also start independently.
Verification guidance follows the readiness report and evidence producer, so it can reuse their action and applicability contracts.
The adoption report follows unified readiness and preserves the producer-owned dependency vocabulary.
Only these hard local prerequisites enter change metadata. Priority alone does not create an ordering edge.

Each change retains a narrow owner and independent positive and negative tests.
No package authorizes network access, service changes, bulk updates, implementation, or publication during planning.
Future implementation uses the normal isolated-worktree lifecycle.

## Stack reuse record

The following immutable revisions define the review starting points. They are not automatic admission decisions or newly installed dependencies.
Implementation must acquire each selected revision from its published source, inspect the exact contract, and retain consumer conformance evidence.
Cargo and Nix must bind the same revision. Nix alone generates any lockfile change.
Sibling paths are research locators only. Product execution cannot depend on those paths.

| Component | Source and immutable revision | Selected boundary |
|---|---|---|
| Nickel Export | `https://github.com/OnixResearch/nickel-export`, `257fafc1c746f1faf156207043a4c826bfb16d49` | Existing Cairn Nix pin. Declared input, evaluator, output, and freshness identities |
| Cap Root | `rad:z3cWBCK7VPbEmbL5MAvKJWKjCE745`, `8991e2572578c16e590c2f1c0915db6c46c5d9cb` | Lexical relative-path and leaf admission. Cairn retains directory capabilities and no-follow I/O |
| Durable File Publication | `rad:z3tAR4For7qw8ZirkJzoDw1VNDDLM`, `951c27f59003cea9bfdb40ed4d89653d50fada1f` | One-file publication. Cairn retains installation transactions, recovery, and ownership |
| Bounded Exec | `rad:z2CpqLFpdP36fZXYUK5ZNWxMibpCo`, `29dac88ecded94457572db3fdfaaaab95fa91525` | Child-process limits and teardown, through its published Git source |
| Kiln | `rad:z2wsvXm5S2sJGvuV1k5JHiwi1PbKE`, `5d99f4e9bb954743443e340baece31256b4c53f1` | CI candidate, attempt, provider, and result meaning. Cairn does not create another CI scheduler |
| Lattice | `https://github.com/OnixResearch/lattice`, `70496e67c7fd4a8b05914161a8e09de2759bebc8` | `lattice-workflow-contract` and workflow observations through Kiln-owned execution |
| Mantle | `https://github.com/OnixResearch/mantle`, `0bbab77e23ab95caa4e4184b9945cfaa5c8040e2` | `mantle-build-contract` and bounded build observations. Builds do not substitute for tests |
| Octet | `https://github.com/OnixResearch/octet`, `890f424cd7b1b2015f39c0eb5747f1bd1e576028` | Source-analysis and checked-path evidence under the selected exact scope |
| Valence | `https://github.com/OnixResearch/valence`, `e40c76b4d2070a29636e00c85c0dff93f03dba2f` | Evidence identity, roles, linkage, spans, and non-claims |
| Workspace dependency tools | Workspace root commit `22515fb7bc01d378312053d389d4bbdd55637a50` | `stack-deps` audit/reconciliation and `stack-doctor` saved observations. No source import into Cairn |

The workspace tools remain external producers. Their immutable source and tool identities belong in supplied receipts.
The source row does not assert a new published library API or authorize an ambient root checkout dependency.

OnixResearch owner-authorized reuse applies to exclusively owned source. Each adoption records source, revision, imported scope, and the owner-grant basis.
This grant does not change public licenses or cover third-party material. Preserve required notices and third-party license obligations.

## Reuse decisions and exclusions

- Reuse existing Cairn models before extracting another graph or lifecycle package.
- Keep Kiln CI state and Lattice execution outside Cairn. No scheduler, worker placement, retry loop, or daemon belongs in these changes.
- Reuse the existing Valence interchange path where it fits. Do not relabel recorded linkage as verified behavior.
- Keep the first report formats in the current CLI and MCP shells. Site and Subwayrat remain optional later views over the same receipt.
- Defer `schema-identity-core` at `dace0c70eef55baaaa1af2c9162a7f64d629299a`. Bootstrap needs explicit policy compatibility, not a new schema-lineage graph.
- Defer new Choregraph or Campaign adoption. Cairn already owns the bounded dependency model needed for these reports.
- Do not introduce runtime Nickel evaluation into ordinary lifecycle commands. Explicit export and installation paths retain the evaluation boundary.

## Related unmerged work

`gate-compiler-bound-code-evidence` exists on the separate `plan/rust-content-identity` branch.
It defines optional lifecycle acceptance of exact compiler-bound evidence, not source-diff check selection.

`require-candidate-bound-rust-assurance` exists on the separate `plan/rust-assurance-6b781b9` branch.
It strengthens consumer assurance requirements, not evidence production or daily diagnosis.

These plans are not implementation dependencies in this program. Do not copy their source or claim their contracts exist in current main.
Any later integration must reconcile the published contracts and retain each existing owner.

## Acceptance and utility

Each package requires deterministic success and failure fixtures, current-source reproduction, and maintained consumer examples.
Planning gates check document conformance only. They do not establish implementation success, useful speedups, or release readiness.

The first runtime pilot uses Cairn itself. Evidence production also uses a disposable Kiln consumer fixture with pinned producer contracts.
The adoption pilot uses saved Cairn-to-Octet and Octet-to-Lattice relationship observations, not live mutation of those repositories.

Record the named cohort, source identities, commands, and observations for each utility comparison.
Measure first-use success, time to a correct next action, manual receipt edits, repeated command errors, and stale-evidence rejection.
Do not count archives, tasks, or generated files as utility evidence.

For the adoption-report operator contract, supported versions, authority map, diagnostic actions, and bounded-stage interpretation, see [Cross-repository adoption report](adoption-report.md).

# Oxide lifecycle processing references

Cairn uses the following repositories as reference-only design inputs. No source is copied by this change; any future code reuse requires a separate upstream-license review. r[impl cairn.lifecycle_orchestration.oxide_patterns.reference_inventory]

| Repository | Intended adaptation | License posture | Non-adoption boundary |
|---|---|---|---|
| `oxidecomputer/steno` | deterministic saga steps, recovery logs, compensation decisions | reference-only; review upstream license before copying code | no saga implementation or domain correctness claim is imported |
| `oxidecomputer/rfd-api` | source revisions, state transitions, webhook and scan inputs | reference-only; review upstream license before copying code | no hosted service, API authority, or publication claim is imported |
| `oxidecomputer/rfd-site` | rendered assets and local search-index processing | reference-only; review upstream license before copying code | no site deployment or rendering correctness claim is imported |
| `oxidecomputer/expectorate` | explicit golden comparison and reviewed refresh | reference-only; review upstream license before copying code | no automatic unreviewed fixture mutation is imported |
| `oxidecomputer/bundle-cat` | read-only bundle member and identity inspection | reference-only; review upstream license before copying code | no evidence-truth or release-eligibility claim is imported |
| `oxidecomputer/support-bundle-viewer` | bounded read-only bundle presentation | reference-only; review upstream license before copying code | no support-bundle authority or remote mutation is imported |

The following systems are study-only sources for operator diagnostics, observability, archive shape, and debugging workflows: `oxidecomputer/omicron`, `oxidecomputer/hubris`, `oxidecomputer/humility`, `oxidecomputer/crucible`, `oxidecomputer/propolis`, `oxidecomputer/helios`, `oxidecomputer/maghemite`, `oxidecomputer/opte`, and `oxidecomputer/dendrite`. Cairn imports none of their storage, VMM, firmware, network, operating-system, or hardware correctness claims. r[verify cairn.lifecycle_orchestration.oxide_patterns.reference_inventory.study_only]

`cairn lifecycle-patterns validate --input <bundle.json>` validates in-memory saga rows, document plans, golden approvals, bundle member hashes, and reference boundaries through the pure core. The CLI only reads bounded JSON and renders a receipt; it does not refresh snapshots, process webhooks, run scans, compensate operations, or mutate evidence bundles. r[impl cairn.lifecycle_orchestration.oxide_patterns.saga_logs] r[impl cairn.lifecycle_orchestration.oxide_patterns.rfd_processing] r[impl cairn.lifecycle_orchestration.oxide_patterns.golden_snapshots] r[impl cairn.lifecycle_orchestration.oxide_patterns.bundle_inspector]

Passing validation proves only deterministic shape, declared hash linkage, reviewed refresh intent, and bounded reference/non-claim conformance. It does not prove lifecycle correctness, evidence truth, domain correctness, or release eligibility.

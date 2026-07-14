## Design

The adaptation uses Cairn's existing functional-core / imperative-shell split. Pure core modules model lifecycle steps, saga logs, document processing plans, snapshot comparisons, bundle indexes, and reference-inventory rows. CLI shells own filesystem traversal, Git/JJ inspection, webhook receipt, process execution, rendering, and mutation.

### Decisions

1. **Saga execution is deterministic state, not hidden orchestration.** Steno-style sagas are useful for multi-step operations such as sync, archive, evidence export, and release-readiness. Cairn saga logs must record step identity, input hash, output hash, compensation status, and recovery decision without hiding side effects in the core.

2. **Lifecycle document processing remains repo-local.** RFD-style processors inform source revision tracking, rendered assets, state transitions, and search indexing. Cairn must not require a hosted service before local lifecycle validation remains usable.

3. **Golden snapshots are explicit update artifacts.** Expectorate-style updates are useful only when fixture refresh is deliberate. Cairn snapshot refresh must be opt-in, reviewable in VCS, and paired with negative fixtures for malformed or stale outputs.

4. **Evidence bundle inspection is read-only by default.** support-bundle-viewer and bundle-cat patterns map to a Cairn bundle explorer that lists contents, hashes, receipt roles, non-claim boundaries, and validation status without mutating bundle files.

5. **Large Oxide systems are study-only references.** Omicron, Hubris, Humility, Crucible, Propolis, Helios, Maghemite, OPTE, and Dendrite may inform observability, operator diagnostics, archive formats, and debugging workflows. Cairn must not adopt their domain-specific correctness claims.

6. **Positive and negative tests are required.** Positive fixtures cover recoverable lifecycle logs, valid document-processing plans, accepted golden updates, and readable evidence bundles. Negative fixtures cover missing compensation, stale source revisions, unreviewed golden refreshes, corrupt bundles, and overclaiming study references.

### Validation shape

A first implementation should add pure lifecycle-log and bundle-index fixtures, then CLI shell smoke tests for rendering and dry-run behavior. Cairn validation and proposal/design/tasks gates must pass before the change is implemented.

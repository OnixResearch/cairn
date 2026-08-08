## Context

Cairn's existing lifecycle receipts make repository changes more deterministic, but agent execution still has a gap: the model can read mutable context, call tools, mutate files, run checks, and summarize results without producing a compact canonical artifact that another process can validate. A Cairn agent-run receipt should not try to store a full transcript. It should instead commit to the minimal evidence needed to replay, compare, and gate a run.

## Decisions

### 1. Agent runs use a canonical receipt envelope

**Choice:** Define a canonical JSON receipt envelope with stable fields for `schema_version`, `task_id`, `agent_identity`, `input_hash`, `plan_hash`, `tool_manifest`, `mutation_manifest`, `verification_manifest`, `nondeterminism_manifest`, `verdict`, and `receipt_hash`. r[cairn.supersede.agent_run_receipts.schema]

**Rationale:** A small stable schema is easier to enforce than raw transcripts and can be hashed with the same deterministic BLAKE3 approach Cairn already uses for lifecycle receipts.

### 2. Mutation must cite an approved plan

**Choice:** Any receipt that reports filesystem, lifecycle, network-write, secret, deploy, or other side-effecting actions must include a `plan_hash` derived from the canonical pre-mutation plan. r[cairn.supersede.agent_run_receipts.plan_link]

**Rationale:** Plan-before-mutation creates a durable boundary between intent and side effects. It also lets policy compare actual mutations against approved scope.

### 3. Nondeterminism is explicit data

**Choice:** The receipt records nondeterministic sources as classified entries: wall-clock reads, network reads, model/provider identity, environment variables, remote APIs, random values, and externally supplied artifacts. r[cairn.supersede.agent_run_receipts.nondeterminism]

**Rationale:** Agents cannot be fully deterministic while relying on models and live tools, but they can be explicit about entropy. Hidden nondeterminism should fail validation rather than silently weakening replay.

### 4. Completion is gateable, not prose-based

**Choice:** A completion gate validates the receipt before an agent or wrapper may claim the task is done. The gate checks verdict consistency, required verification evidence, dirty-state disclosure, artifact hashes, and plan/mutation consistency. r[cairn.supersede.agent_run_receipts.completion_gate]

**Rationale:** The most damaging agent failure mode is premature completion. A deterministic gate gives operators a machine-checkable "done" contract.

### 5. Replay fixtures define the compatibility contract

**Choice:** Maintained fixtures provide prompt/input manifests, selected tool manifests, planned mutations, verification results, and expected canonical receipts. Repeated generation over the same fixture must produce byte-identical canonical receipt content and hashes. r[cairn.supersede.agent_run_receipts.replay]

**Rationale:** Agent infrastructure changes frequently. Fixtures let Cairn detect schema drift, accidental entropy, and missing evidence without depending on a live model.

## Risks / Trade-offs

- Capturing too much transcript detail would make receipts noisy and privacy-sensitive; this change should prefer bounded manifests and hashes over raw logs.
- Requiring plan linkage may slow tiny edits; wrappers can generate minimal plans for low-risk local changes instead of skipping the contract.
- Model output itself is nondeterministic; the receipt should validate captured inputs/evidence and wrapper behavior, not assert that a model will always choose the same text.

# Cairn Supersession Delta: Portable Determinism Usage Rails

## ADDED Requirements

### Requirement: Portable receipt hash inputs
r[cairn.supersede.portable_receipts] The system MUST distinguish operator-facing display fields from canonical hash inputs so semantically identical repository evidence can remain portable across checkout roots.

#### Scenario: Cross-root receipts preserve semantic hashes
r[cairn.supersede.portable_receipts.cross_root]
- GIVEN two filesystem roots containing byte-identical Cairn lifecycle content at different absolute paths
- WHEN the operator runs a deterministic read, gate, sync dry-run, archive dry-run, migration dry-run, release-readiness, or agent-receipt validation command against each root
- THEN canonical policy, input, plan, manifest, and receipt hashes that describe repository semantics MUST match across the two roots
- AND any operator-facing absolute path fields MUST be excluded from portable hash material unless the command declares them as explicit inputs

#### Scenario: Display paths remain actionable
r[cairn.supersede.portable_receipts.display_paths]
- GIVEN a receipt includes a path for an operator to inspect or mutate
- WHEN the CLI renders JSON for a local command
- THEN the receipt MAY include display path fields
- AND the receipt MUST identify which canonical root-relative path or hash input was used for deterministic evidence

#### Scenario: Absolute external inputs are explicit
r[cairn.supersede.portable_receipts.explicit_absolute_inputs]
- GIVEN a command depends on an absolute path outside the selected repository root
- WHEN the command includes that path in deterministic evidence
- THEN the receipt MUST declare the absolute path as an explicit external input or represent it by bounded metadata and content hash
- AND hidden ambient absolute paths MUST NOT affect evidence hashes

### Requirement: Determinism replay oracle
r[cairn.supersede.determinism_replay] The system MUST provide a maintained replay oracle for commands whose receipts claim deterministic evidence.

#### Scenario: Repeated command receipts are stable
r[cairn.supersede.determinism_replay.repeated]
- GIVEN an unchanged repository fixture and unchanged explicit command inputs
- WHEN the replay oracle runs the same Cairn command multiple times
- THEN normalized JSON output and declared evidence hash fields MUST match for every run

#### Scenario: Cross-root command receipts are stable
r[cairn.supersede.determinism_replay.cross_root]
- GIVEN the replay oracle copies the same fixture into two distinct root paths
- WHEN it runs the same deterministic Cairn command against both roots
- THEN normalized JSON output and declared portable hash fields MUST match
- AND differences limited to documented display-only path fields MUST NOT fail the replay check

#### Scenario: Hidden nondeterminism is rejected
r[cairn.supersede.determinism_replay.hidden_entropy]
- GIVEN a command fixture whose output or evidence hash would depend on an uncaptured clock, environment variable, random value, filesystem ordering, external command probe, or ambient absolute path
- WHEN the replay oracle or receipt gate evaluates the command
- THEN it MUST fail with a deterministic issue code naming the nondeterminism class

### Requirement: MCP reproducibility surface
r[cairn.supersede.mcp_reproducibility] The system MUST treat MCP descriptions and tool-call receipts as deterministic agent-facing Cairn use surfaces.

#### Scenario: MCP describe output is stable
r[cairn.supersede.mcp_reproducibility.describe]
- GIVEN the Cairn MCP server is invoked with `--describe`
- WHEN the maintained regression suite runs the description command repeatedly
- THEN the emitted tool names, schemas, mutability flags, and descriptions MUST be byte-stable or normalized-stable

#### Scenario: MCP tool calls match CLI semantics
r[cairn.supersede.mcp_reproducibility.call]
- GIVEN an MCP tool call wraps a Cairn read, validate, gate, sync dry-run, archive dry-run, migration dry-run, release-readiness, or agent-receipt validation operation
- WHEN the maintained regression suite compares the MCP receipt with the corresponding CLI receipt
- THEN both surfaces MUST agree on verdict, validity, issue codes, policy hash, input hash, plan/hash fields, and mutation state

## MODIFIED Requirements

### Requirement: BLAKE3 evidence identifiers
r[cairn.supersede.blake3_receipts] The system MUST use BLAKE3 hashes to identify canonical policy, input snapshots, plans, manifests, and receipts for Cairn decisions that claim validation, gate, sync, archive, migration, release-readiness, MCP, traceability, or agent-run evidence.

#### Scenario: Hashing remains canonical and bounded
r[cairn.supersede.blake3_receipts.canonical]
- GIVEN Cairn computes evidence hashes
- WHEN hashes are generated
- THEN the hashed material MUST exclude nondeterministic process noise, display-only absolute paths, filesystem traversal order, and wall-clock timestamps unless supplied as explicit bounded inputs
- AND portable repository artifact paths MUST be normalized relative to the selected lifecycle root before entering canonical hash material

### Requirement: Agent-run nondeterminism manifest
r[cairn.supersede.agent_run_receipts.nondeterminism] The system MUST record and classify nondeterministic inputs that can affect agent-run or Cairn-command replay.

#### Scenario: Environment and path entropy are captured
r[cairn.supersede.agent_run_receipts.nondeterminism.environment]
- GIVEN a receipt-producing run reads environment variables, archive date inputs, external command availability, absolute paths outside the repository root, or machine-specific metadata
- WHEN the run emits or validates a Cairn receipt
- THEN each replay-relevant source MUST be represented in the nondeterminism manifest with a class, bounded metadata, and content hash where content is available
- AND policy-forbidden uncaptured sources MUST fail validation deterministically

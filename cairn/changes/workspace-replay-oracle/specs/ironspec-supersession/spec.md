## MODIFIED Requirements

### Requirement: Determinism replay oracle
r[cairn.supersede.determinism_replay] The system MUST provide a maintained replay oracle for commands whose receipts claim deterministic evidence, and that oracle MUST be invocable by agents/CI as an operator-facing workspace preflight.

#### Scenario: Workspace oracle covers public deterministic surfaces
r[cairn.supersede.determinism_replay.workspace_oracle]
- GIVEN an operator invokes the workspace replay oracle against a Cairn repository or fixture root
- WHEN the oracle builds its default plan
- THEN the plan MUST include maintained cases for status, validate, gate, sync dry-run, archive dry-run, migration dry-run, release-readiness, MCP describe, and selected MCP read-only calls where fixtures support those surfaces
- AND every case MUST declare command kind, arguments, expected mutability, comparison mode, display-only path allowances, and hash fields under comparison

#### Scenario: Repeated command receipts are stable
r[cairn.supersede.determinism_replay.repeated]
- GIVEN an unchanged repository fixture and unchanged explicit command inputs
- WHEN the replay oracle runs the same Cairn command multiple times in the same root
- THEN normalized JSON output and declared evidence hash fields MUST match for every run
- AND mismatch reporting MUST identify the command case and JSON field path that changed

#### Scenario: Cross-root command receipts are stable
r[cairn.supersede.determinism_replay.cross_root]
- GIVEN the replay oracle copies the same fixture into two distinct root paths
- WHEN it runs the same deterministic Cairn command against both roots
- THEN normalized JSON output and declared portable hash fields MUST match
- AND differences limited to documented display-only path fields MUST NOT fail the replay check
- AND undocumented path-dependent differences MUST fail the replay check

#### Scenario: Normalization is explicit and bounded
r[cairn.supersede.determinism_replay.normalization]
- GIVEN the replay oracle compares command outputs
- WHEN a field differs between roots or repeated runs
- THEN the field MAY be ignored only if the case plan declares it as display-only or nondeterminism evidence excluded from portable hashes
- AND the oracle receipt MUST include the normalization rule identifier used for the field

#### Scenario: Oracle receipt summarizes deterministic evidence
r[cairn.supersede.determinism_replay.receipt]
- GIVEN the replay oracle finishes its plan
- WHEN it emits JSON
- THEN the receipt MUST include sorted cases, per-run output hashes, normalized-output hashes, portable evidence hashes, skipped cases, mismatch diagnostics, nondeterminism failures, mutability observations, verdict, and receipt hash
- AND the oracle receipt hash MUST exclude display-only temp-root values from portable hash material

#### Scenario: Oracle receipt schema is stable
r[cairn.supersede.determinism_replay.receipt.schema]
- GIVEN agents parse the oracle receipt
- WHEN a compatible Cairn release emits the replay receipt
- THEN the top-level keys and documented nested diagnostic keys MUST be stable, sorted, and covered by golden or schema tests

#### Scenario: MCP cases are replayed through the same oracle
r[cairn.supersede.determinism_replay.mcp_cases]
- GIVEN the replay plan includes MCP describe or read-only tool calls
- WHEN the oracle runs those cases repeatedly and across roots
- THEN MCP outputs MUST be compared with the same normalized JSON and portable hash rules as CLI cases
- AND read-only MCP calls that correspond to CLI commands SHOULD compare shared semantic fields with the CLI receipt

#### Scenario: Replay is non-mutating by default
r[cairn.supersede.determinism_replay.no_mutation]
- GIVEN a replay case attempts to run an execute command or a command reports mutation
- WHEN the oracle is running in default mode
- THEN the oracle MUST fail that case with a deterministic mutation-denied issue code
- AND the fixture root MUST remain unchanged relative to its pre-case manifest

#### Scenario: Hidden nondeterminism is rejected
r[cairn.supersede.determinism_replay.hidden_entropy]
- GIVEN a command fixture whose output or evidence hash would depend on an uncaptured clock, environment variable, random value, filesystem ordering, external command probe, or ambient absolute path
- WHEN the replay oracle or receipt gate evaluates the command
- THEN it MUST fail with a deterministic issue code naming the nondeterminism class
- AND the issue MUST include bounded metadata sufficient for an operator to reproduce or suppress the class by policy

#### Scenario: Mismatches are actionable
r[cairn.supersede.determinism_replay.mismatch_diagnostics]
- GIVEN the oracle detects any repeated-run or cross-root mismatch
- WHEN it records the failure
- THEN each mismatch MUST name the command case, comparison mode, JSON field path, mismatch class, expected hash or value summary, and actual hash or value summary
- AND mismatch entries MUST be sorted deterministically

### Requirement: Release-readiness evidence bundle
r[cairn.supersede.release_readiness] The system MUST provide a release-readiness evidence bundle that an operator can run before claiming Cairn is ready for agent/CI use.

#### Scenario: Release-readiness references replay evidence
r[cairn.supersede.release_readiness.determinism_replay]
- GIVEN the workspace replay oracle exists
- WHEN `release-readiness` emits its evidence bundle
- THEN it MUST either include the replay oracle receipt hash or explicitly list the replay oracle as a required external operator preflight
- AND the release-readiness receipt MUST make the distinction machine-readable

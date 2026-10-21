## Context

The failing downstream report and absence of consumer fixtures are retained facts; do not repeat failed campaigns or classify this configuration error as a guest bug. Existing uncommitted consumer-selection work and producer commit `35e6bd4c4d6e9d5a3639a0e70ca922faf3b94a48` already implement much of the required mechanism. Preserve their provenance and unrelated work.

## Decisions

### Decision: Consumer-owned explicit input selection

Reuse the typed `release_readiness_policy` record: `id`, `mcp_completion_contract`, `mcp_completion_results`, `preserves_corpus`, and `required_external_commands`. Paths are component-safe and repository-relative. Reject invalid selection, missing files and policy mismatch; absence alone retains self-host defaults. Do not introduce skip flags, check applicability exemptions or invented result rows.

### Decision: One selected policy throughout the MCP seam

Pass the CLI-selected policy into MCP completion validation and record its identity in the smoke receipt. Standalone smoke and enclosing release readiness must assess the same consumer inputs and policy. The smoke remains read-only and must still reject invalid tool arguments and deny mutations.

### Decision: Reuse real producer evidence

Integrate the completed bounded dependency-observation and native-Nix payload mechanisms. Require genuine full consumer command/dependency obligations and original completion validation. An exit-zero failed result, changed dependency, stale policy or invalid corpus must not pass.

## Verification

Retain positive consumer CLI/MCP runs and negative policy, result, path and corpus cases. Run required repository gates against the exact integrated cohort. Only a passing immutable cohort may enter ChaosControl's repository-owned four-import migration. Preserve the original production-readiness criteria and historical receipts.

## Risks

Source and policy identity changes stale completion rows; regenerate through real execution rather than refreshing hashes by hand. Selected path admission does not establish hostile-filesystem isolation. Passing MCP seam evidence is bounded conformance, not release eligibility or completion of the separate historical replay requirements.

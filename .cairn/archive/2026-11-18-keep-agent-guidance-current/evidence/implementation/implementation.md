# Implementation notes

Status: source implementation prepared on `change/keep-agent-guidance-current-20261117`; repository checks and projection generation are owned by the integration pass and are not claimed here.

## Source changes

- `cairn-policy/contracts.ncl`: added `GeneratedWorkflowCommand`, `GeneratedWorkflowEntryGuide`, the native `.cairn` layout contract, and policy fields `command_catalog` and `entry`.
- `cairn-policy/default.ncl`: added the checked command catalog projection, a compact entry guide descriptor with discovery/readiness/stage/profile routes, Nickel-owned entry and stage-route text bounds, and the mandatory boundary and evidence sets. Profile names are one shared projection (`AgentGuidanceProfiles`) used by both agent guidance and the entry guide.
- `crates/cairn-core/src/verified/generated_agent_workflow/`: entry guide admission, rendering into the existing per-adapter file set, command-catalog identity, legacy-layout rejection, entry-budget failure, copied-guide report-only inspection, and interrupted-installation recovery blockers.
- `crates/cairn-cli/src/generated_agent_workflow_shell/`: entry selection through the existing `show` command, report-only `inspect-copy`, pending transaction observation, and additive receipt fields.
- `fixtures/generated-agent-guidance/examples.json`: maintained command/layout/profile/adapter examples with named expected outcomes checked against the admitted catalog without executing product effects.

## Generated identities

- Entry file IDs use the existing adapters: `cairn-entry/SKILL.md`, `cairn-entry.md` (Claude and Codex).
- Entry descriptor identity binds the entry descriptor, generator version, command-catalog identity, and selected adapter identity.
- Manifest and plan receipts continue to bind policy hash, catalog identity, adapter version, file hashes, and prior state.
- `AdmittedGeneratedWorkflowCatalog.command_catalog_identity` exposes the catalog projection identity.

## Verification status

Focused tests were added for adapter route equivalence, legacy-layout rejection, insufficient entry budgets, unknown routes/profiles/adapters, command-catalog drift, copied-guide classification, recovery blockers, and byte-preserving denial. No build, test, formatter, or lifecycle check was executed in this worktree; the integration pass generates `cairn-policy/generated/cairn-policy.json` and runs the repository rails.

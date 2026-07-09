## Why

Basalt and Aspen currently fail normal Cairn validation with stale generated policy JSON when checked against the current Cairn binary. That is a modularity failure: downstream lifecycle content should declare which Cairn policy schema it targets, and Cairn should provide compatibility diagnostics instead of an opaque parse failure.

This change adds policy schema compatibility contracts so repos can validate native Cairn layouts against declared policy versions and know when policy export refresh is required.

## What Changes

- Add a policy schema compatibility manifest for generated Cairn policy JSON.
- Record policy schema version, required fields, optional compatibility shims, producer Cairn version, and refresh command.
- Add diagnostics that distinguish stale generated policy from malformed lifecycle content.
- Add positive and negative fixtures for current, stale, missing-field, and unsupported-version policies.

## Impact

- Downstream repos can fail with actionable policy-refresh diagnostics.
- Cairn can evolve policy schema without hidden consumer breakage.
- Release readiness can require current policy schema for selected repos.

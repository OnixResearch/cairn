# Design: Preserves release-readiness summary

The release-readiness implementation should compose existing deterministic rails:

- confirm the Preserves corpus fixture exists;
- run or report the dry-run corpus freshness check without mutation;
- mention the read-only `agent-receipt preserves-hash` surface and dependency-bump runbook;
- emit deterministic issue codes if corpus freshness cannot be proven.

Tests should prove stable receipt fields, fail-closed corpus issues, and no changes to public agent receipt validation.

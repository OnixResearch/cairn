## Context

Cairn runtime commands consume checked-in generated policy JSON. When the generated policy shape lags the current binary, validation can fail before lifecycle artifacts are evaluated. That makes consumer repos look broken even when their changes are structurally valid.

## Design

Add a pure compatibility checker over parsed policy metadata. The CLI shell reads the generated policy JSON and passes schema fields to the core. The checker emits one of: current, compatible-with-warning, refresh-required, or unsupported.

The policy metadata includes schema version, producer identity, field set digest, and refresh command. Missing newly required fields should produce a deterministic `policy-refresh-required` diagnostic when the older policy schema is known.

## Alternatives

### Require all repos to refresh immediately

Rejected. Repos should refresh, but the tool should distinguish stale policy from content errors.

### Keep parse failures as-is

Rejected. Opaque parse failures increase coupling to the current binary internals.

## Risks

- Compatibility shims can hide real drift. Mitigate by making shims explicit, bounded, and visible in receipts.
- Policy metadata can itself drift. Mitigate with policy export freshness checks.

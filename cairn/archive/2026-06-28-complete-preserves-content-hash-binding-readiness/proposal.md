# Proposal: Complete Preserves content-hash binding and readiness

## Why

The previous Preserves promotion slice introduced `preserves_content_hash`, but two operator-readiness gaps remain:

- content-hash binding should appear on the durable receipt surfaces that claim evidence/replay/release content identity, not only on the direct agent-receipt preserves-hash command;
- release-readiness currently fails because Tracey has incomplete coverage markers and the positive agent receipt fixture carries a stale replay policy hash after generated policy changed.

## What Changes

- Add `preserves_content_hash` to replay/evidence/release-evidence receipts where content identity is reported alongside `receipt_hash`.
- Refresh generated policy schema declarations for the new hash field on those receipt surfaces.
- Refresh the positive agent receipt fixture's replay policy hash and receipt hash so MCP smoke validation is structured and valid.
- Repair Tracey coverage markers for the new Preserves requirements.

## Impact

- **Files**: CLI receipt builders, generated/Nickel policy, agent receipt fixture, Tracey markers, lifecycle specs.
- **Testing**: `cairn validate`, focused cargo tests/checks, `tracey coverage`, `agent-receipt smoke-mcp`, and `release-readiness`.
- **Compatibility**: additive. Existing `receipt_hash` stays present and remains the compatibility surface.

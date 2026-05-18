# Align Supersession Native Layout

## Why

The native lifecycle migration changed `cairn init` and `cairn change create` to create `cairn/` layout artifacts, but older accepted supersession requirements still describe legacy `openspec/` creation paths. That drift weakens the accepted spec as the product authority.

## What Changes

- **Supersession spec alignment**: Update initialization and change creation requirements to name native `cairn/` paths.
- **Legacy clarity**: Preserve legacy `openspec/` fallback as read/migration compatibility, not the default creation target.
- **Evidence**: Verify both upstream OpenSpec validation and native Cairn validation after sync.

## Capabilities

### Modified Capabilities
- `cairn-supersession`: Align accepted lifecycle path requirements with native layout behavior.

## Impact

- **Files**: `openspec/specs/cairn-supersession/spec.md` via delta sync.
- **APIs**: No CLI behavior change.
- **Dependencies**: None.
- **Testing**: `openspec validate --all --strict`, `nix run path:$PWD#cairn -- validate --root .`, and focused CLI tests if docs/spec-only checks reveal drift.

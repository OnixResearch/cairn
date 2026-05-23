## Why

Semantic review criteria should not be hard-coded into Rust or scattered across prompts. Cairn needs policy-owned review profiles for plan, design, code, and diff review gates so teams can evolve criteria while keeping receipts structurally stable.

## What Changes

- Add Nickel policy contracts for semantic review profiles.
- Define required criteria groups for plan, design, code, and diff review kinds.
- Include allowed reviewer sources and required receipt fields per profile.

## Impact

- **Files**: Cairn policy, pure core receipt/profile validators, CLI validation/analytics commands, fixtures, tests, and docs as applicable.
- **Testing**: Positive/negative receipt fixtures, policy export checks, focused CLI tests, validation/gate receipts, and Tracey coverage refs.

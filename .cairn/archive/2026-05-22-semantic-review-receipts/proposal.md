## Why

Cairn has deterministic lifecycle gates, but it lacks a first-class artifact for semantic plan/design/code review outcomes. Rothrock-style review gates produce trust evidence about omissions, systematic mistakes, and incoherence; Cairn should be able to validate and preserve those review outcomes without performing the review itself.

## What Changes

- Define a `review_receipt.v1` schema for plan, design, code, and diff review outcomes.
- Bind each receipt to reviewed artifacts with stable hashes and root-relative paths.
- Add `cairn review validate <receipt>` for structural validation and deterministic issue reporting.
- Add fixtures, tests, and docs that distinguish semantic review evidence from deterministic Cairn gates.

## Impact

- **Files**: Cairn policy, pure core receipt/profile validators, CLI validation/analytics commands, fixtures, tests, and docs as applicable.
- **Testing**: Positive/negative receipt fixtures, policy export checks, focused CLI tests, validation/gate receipts, and Tracey coverage refs.

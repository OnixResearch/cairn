## Context

Cairn now exposes an optional Steel lifecycle orchestration seam through two CLI subcommands:

- `cairn steel-orchestration receipt`
- `cairn steel-orchestration default-gate`

The default checked-in policy keeps Steel disabled, and the CLI emits deterministic receipts instead of executing Steel host effects. The missing piece is a stable operator-facing document that shows the accepted surface without implying that Steel is the default lifecycle engine.

## Decisions

### 1. Document the accepted CLI surface, not an activation runbook

**Choice:** Add a focused `docs/steel-orchestration.md` reference with command examples, expected verdicts, and field semantics.

**Rationale:** The current capability is an optional, policy-gated orchestration shell. Documentation should make safe defaults and failure modes obvious without encouraging operators to bypass policy evidence gates.

### 2. Keep examples deterministic and secret-free

**Choice:** Use the default disabled policy examples plus a placeholder lowercase 64-character hash for selected-default gate shape examples.

**Rationale:** The examples should be copyable for local smoke checks and should not depend on private evidence, provider payloads, credentials, or environment-specific paths.

### 3. Make no-effects behavior explicit

**Choice:** Document `effects_executed: false` as a required receipt invariant for the current Steel orchestration shell.

**Rationale:** Operators need to distinguish reviewed planning/comparison evidence from Rust/Nickel-owned effect execution.

## Risks / Trade-offs

- The docs intentionally do not provide a production Steel-default enablement recipe; default promotion remains policy- and evidence-gated.
- Receipt hashes in examples are illustrative for the current default policy shape and should not be treated as a compatibility API.

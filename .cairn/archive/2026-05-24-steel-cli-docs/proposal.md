## Why

Steel lifecycle orchestration is implemented, archived, and smoke-tested, but operators currently need to infer the public CLI behavior from specs, tests, or raw receipt output. The accepted `steel-orchestration receipt` and `steel-orchestration default-gate` surfaces need a concise copyable reference that explains safe default behavior, evidence-gated default promotion, and the no-effects guarantee.

## What Changes

- Add user-facing documentation for `cairn steel-orchestration receipt` and `cairn steel-orchestration default-gate`.
- Document expected PASS/FAIL examples for the default disabled policy and a selected-default policy with complete evidence hashes.
- Document stable receipt fields, redaction expectations, and `effects_executed: false` semantics.

## Impact

- **Files**: `docs/steel-orchestration.md`, `cairn/changes/steel-cli-docs/**`, accepted lifecycle-orchestration spec on sync/archive.
- **Testing**: Cairn validation, proposal/design/tasks gates, Tracey coverage, Markdown whitespace/diff checks, and a post-doc CLI smoke for the documented default receipt/default-gate surfaces.

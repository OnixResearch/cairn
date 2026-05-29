# Typed Lifecycle Probes

## Why

Cairn lifecycle artifacts are still authored as Markdown, so the strongest part of the system is text parsing rather than typed data. That makes requirement identity, scenario shape, task linkage, probe evidence, and migration semantics harder to validate before runtime. It also blurs the difference between documentation, source of truth, and executable evidence.

Cairn should move toward typed lifecycle source while preserving Markdown as a rendered view for humans. The authoring source should be Nickel plus contracts because Cairn already trusts Nickel for policy export, and the runtime should keep consuming generated canonical data without invoking Nickel. Probes should become first-class lifecycle artifacts that bind requirements to deterministic executable evidence.

## What Changes

- Add a structured lifecycle authoring model backed by Nickel contracts for specs, changes, requirements, scenarios, tasks, probes, evidence declarations, and format metadata. r[cairn.structured_lifecycle.typed_authoring]
- Add a generated canonical lifecycle bundle consumed by Rust at runtime, with deterministic hashes and fail-closed stale-bundle checks. r[cairn.structured_lifecycle.runtime_bundle]
- Treat Markdown as a projection/import format rather than the authoritative lifecycle source once structured source exists. r[cairn.structured_lifecycle.markdown_projection]
- Add first-class structured probes that bind requirement IDs to executable commands, receipt assertions, fixture inputs, positive and negative coverage, and deterministic evidence hashes. r[cairn.structured_lifecycle.probes]
- Define the format strategy: Nickel contracts own authoring validation, generated JSON remains the initial runtime interchange, Preserves may later become a canonical receipt/IR encoding, and Steel may only be an optional bounded probe DSL after data predicates are insufficient. r[cairn.structured_lifecycle.format_strategy]
- Add migration and compatibility rails so existing Markdown lifecycle repositories can be converted, rendered, and checked without silently changing accepted semantics. r[cairn.structured_lifecycle.migration]

## Impact

This change creates a path away from Markdown-owned lifecycle semantics without removing the current Markdown workflow in one step. The first implementation can support structured lifecycle artifacts beside existing Markdown, export canonical runtime data, and prove equivalence through validation receipts. Later changes can tighten gates until structured source becomes the default and Markdown becomes only a view.

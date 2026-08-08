# Design: Split evidence registry CLI source

## Context

The registry CLI source is cohesive at the command-family level but too large for efficient follow-up changes. The current seams are already visible in function groups: argument/options parsing, label policy validation, registry build/verify, mirror verification, consume selectors, select/fetch-plan, and fetch-execute.

## Decisions

### 1. Preserve `evidence_registry` as the public module

`crates/cairn-cli/src/evidence_registry.rs` will become or remain the command-family facade. Focused submodules will hold implementation details while the facade keeps `run_evidence_registry` available to `evidence.rs`.

### 2. Split by behavioral subfamily, not by generic utility type

The split will prefer modules such as labels, build/verify, mirror, consume/select/fetch-plan, and fetch-execute over premature generic utility modules. Shared helpers stay private to the registry module tree where possible.

### 3. Behavior-preserving verification

The change is a refactor. Existing CLI commands, receipt field names, deterministic hashes, labels, issue codes, and tests must remain stable. Focused tests must include the dedicated `evidence_registry` integration target created in the previous slice.

## Risks

- **Visibility churn**: Moving functions may require `pub(super)` boundaries. Mitigation: export only narrow helpers within the module tree.
- **Receipt drift**: Any incidental JSON ordering or field change would break deterministic evidence. Mitigation: run focused registry tests plus full workspace tests.
- **Over-splitting**: Too many tiny modules can obscure review. Mitigation: split only the main cohesive subfamilies needed to reduce the hotspot.

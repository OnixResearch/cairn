## Phase 1: Policy and DTO boundary

- [x] [serial] Define policy-owned Steel orchestration profiles, allowed host functions, deterministic budgets, redaction classes, fallback modes, and receipt schema versions. [r[cairn.lifecycle_orchestration.steel_lifecycle_orchestration.policy_declared_surface]]
- [x] [serial] Add Rust DTOs for Steel lifecycle action envelopes and fail-closed validation for malformed, unsupported, or unauthorized requests. [r[cairn.lifecycle_orchestration.steel_lifecycle_orchestration.typed_action_envelopes], r[cairn.lifecycle_orchestration.steel_lifecycle_orchestration.rust_owned_authority]]

## Phase 2: Execution shell and receipts

- [x] [serial] Add an optional Steel orchestration shell that evaluates reviewed scripts, maps output into Rust-owned action envelopes, and preserves normal Cairn behavior when Steel is disabled or unavailable. [r[cairn.lifecycle_orchestration.steel_lifecycle_orchestration.optional_fallback], r[cairn.lifecycle_orchestration.steel_lifecycle_orchestration.no_core_replacement]]
- [x] [serial] Add the initial no-eval CLI receipt shell for disabled, denied-profile, and runtime-unavailable fallback Steel orchestration attempts. [r[cairn.lifecycle_orchestration.steel_lifecycle_orchestration.optional_fallback], r[cairn.lifecycle_orchestration.steel_lifecycle_orchestration.deterministic_receipts]]
- [x] [serial] Emit deterministic, redacted receipts for successful, denied, disabled, and fallback Steel orchestration attempts. [r[cairn.lifecycle_orchestration.steel_lifecycle_orchestration.deterministic_receipts]]

## Phase 3: Fixtures and verification

- [x] [serial] Add positive and negative fixtures covering allowed typed plans, denied host effects, malformed policy, missing Steel runtime, deterministic receipt stability, and secret redaction. [r[cairn.lifecycle_orchestration.steel_lifecycle_orchestration.typed_action_envelopes], r[cairn.lifecycle_orchestration.steel_lifecycle_orchestration.policy_declared_surface], r[cairn.lifecycle_orchestration.steel_lifecycle_orchestration.deterministic_receipts]]
- [x] [serial] Add comparison-mode coverage that proves Steel can plan beside an existing Rust/Nickel workflow without executing effects. [r[cairn.lifecycle_orchestration.steel_lifecycle_orchestration.comparison_mode_no_effects]]
- [ ] [serial] Add default-promotion gates that require policy-selected workflow profiles, equivalence receipts, fallback receipts, CI reproducibility evidence, and operator-visible diagnostics before any workflow can default to Steel. [r[cairn.lifecycle_orchestration.steel_lifecycle_orchestration.evidence_gated_default]]
- [ ] [serial] Verify the change with policy export, formatting, focused tests, lifecycle validation, Tracey coverage, and the relevant Nix checks before sync/archive. [r[cairn.lifecycle_orchestration.steel_lifecycle_orchestration.no_core_replacement]]

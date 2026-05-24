## Context

Cairn should remain a deterministic Rust/Nickel lifecycle engine, not a Lisp runtime with incidental receipts. The opportunity is to make Steel a first-class optional orchestration seam: a human-reviewable programmable layer that replaces ad-hoc workflow glue while preserving Cairn's fail-closed receipt model.

The target shape mirrors the emerging Clankers/Tile pattern: Steel is trusted only to produce typed plans or call explicitly exposed host functions; Rust owns DTO validation, capability checks, effect execution, rollback/preflight decisions, and receipt hashing.

## Decisions

### 1. Steel produces plans; Rust executes effects

**Choice:** Steel lifecycle scripts MUST output typed Cairn action envelopes or call capability-gated host functions. Rust MUST validate the envelope and execute any filesystem, process, network, lifecycle mutation, or evidence-writing effect.

**Rationale:** This keeps Cairn's trust boundary reviewable. Steel can make workflows programmable without gaining ambient authority.

### 2. Nickel policy declares the Steel surface

**Choice:** The allowed script profiles, host functions, budgets, redaction classes, fallback modes, and receipt schema versions MUST be declared in policy-owned configuration.

**Rationale:** Operators need a static, diffable review surface before Steel orchestration can run in CI or agent sessions.

### 3. Receipts are mandatory and deterministic

**Choice:** Every Steel orchestration attempt MUST produce a deterministic receipt, including disabled/fallback/denied outcomes.

**Rationale:** Cairn's value is evidence. Steel must improve workflow customization without creating invisible control flow.

### 4. Steel is optional and non-recursive

**Choice:** Cairn MUST continue to work without Steel. Steel orchestration MUST compose existing Cairn primitives and MUST NOT replace core validators, policy export, receipt hashing, lifecycle mutation manifests, Tracey, Nix verification, or release-readiness rails.

**Rationale:** Steel should replace ad-hoc orchestration glue, not the enforcement substrate.

## Risks / Trade-offs

- **Sandbox overclaim:** Steel embedding alone is not a security boundary. The spec must describe Rust-owned authority and deterministic host functions instead of claiming generic sandboxing.
- **Workflow sprawl:** A programmable layer can hide complexity. Policy-owned profiles, script hashes, and receipt summaries are required to keep review practical.
- **CI determinism:** Steel runtime availability and evaluation limits must be explicit so CI can fail closed or fall back with receipts.

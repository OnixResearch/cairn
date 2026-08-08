## Context

Competitors win early adoption with named profiles. Cairn should preserve deterministic receipts while giving users concrete entry points.

## Decisions

### 1. Additive reviewed templates

**Choice:** Add `docs-lifecycle` and `release-evidence` templates to the policy-equivalent Rust catalog used by the current CLI.

**Rationale:** Additive templates improve first-use coverage without changing existing IDs or mutation semantics.

### 2. Quickstart-only content

**Choice:** Generated files explain commands and boundaries, but do not write accepted specs or claim hosted publication/signing.

**Rationale:** Cairn onboarding should lower friction without overclaiming trust or remote features.

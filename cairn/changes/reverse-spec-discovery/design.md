## Context

Cairn can migrate legacy OpenSpec layouts, but a code-first repository without lifecycle history still needs humans or agents to manually infer accepted specs. A reverse discovery rail should create reviewable candidates and evidence receipts while avoiding the overclaim that inferred specs are automatically correct.

## Decisions

### 1. Evidence-first candidate output

**Choice:** Emit discovered capabilities as candidate requirements with source evidence references, confidence classes, and explicit review-needed markers.

**Rationale:** Reverse discovery is useful triage, not a substitute for operator review or Cairn gates.

### 2. Bounded explicit inputs

**Choice:** Require operator-provided include paths or policy-approved discovery scopes, enforce max file counts and byte limits, and sort all inputs canonically.

**Rationale:** This prevents hidden traversal surprises, unstable receipts, and accidental ingestion of private artifacts.

### 3. Candidate-to-change seam

**Choice:** Discovery produces a receipt and optional draft delta artifacts, but accepted specs change only through the normal Cairn change lifecycle.

**Rationale:** This preserves the reviewable package model and avoids silently promoting inferred behavior into source of truth.

## Risks / Trade-offs

- Static discovery can miss behavior or infer weak requirements; receipts must label candidates as review-needed.
- Source snippets can leak sensitive data; output should prefer hashes, paths, symbols, and bounded excerpts with redaction rules.

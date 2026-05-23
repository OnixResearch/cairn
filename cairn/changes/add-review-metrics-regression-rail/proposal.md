# Proposal: Review metrics regression rail

## Summary

Add a reusable Cairn semantic-review rail that turns repeated sanitized review metrics into a synchronized registry of diagnostics, fixture coverage, operator guidance, and task/spec obligations. Downstream projects can keep project-specific categories locally while Cairn owns the generic anti-drift contract.

## Motivation

Clankers surfaced repeated review omissions such as missing auto-fix guidance and missing prompt wording. A project-local checker can catch those today, but the pattern is generic: once a category is declared, its docs, positive fixtures, negative fixtures, task references, spec references, and sanitized evidence must not drift apart.

Cairn already owns semantic review receipts, policy profiles, completeness checks, and ledger analytics. This change defines the next layer: a deterministic registry/coverage rail for metrics-derived regression categories.

## Scope

- Define a policy/registry-backed review metrics regression rail under `semantic-review`.
- Require every declared category to bind diagnostic code, metrics class/stage, docs guidance, spec requirement id, positive fixture, negative fixture, and sanitized evidence provenance.
- Require deterministic fail-closed receipts for missing or out-of-sync bindings.
- Keep raw prompts, secrets, provider payloads, private transcripts, and credentials out of metrics evidence.
- Preserve downstream ownership of project-specific category bodies and fixtures.

## Non-Goals

- Importing Clankers-specific diagnostic categories into Cairn policy in this slice.
- Performing LLM/remote semantic review.
- Replacing downstream project checkers before a reusable Cairn CLI/API exists.

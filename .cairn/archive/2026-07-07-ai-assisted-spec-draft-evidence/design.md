# Design: AI-assisted spec draft evidence

## Approach

AI assistance is represented as provenance on a draft artifact, not as verification. The trusted artifact is the later machine-check receipt, human-review decision, or both.

## Draft provenance

Draft provenance records:

- authored artifact kind;
- originating agent or assistant class when available;
- prompt/session redaction class, not raw prompts;
- target requirement or property id;
- resulting validation command or review decision;
- status as draft, accepted, revised, rejected, or diagnostic-only.

## Gate behavior

A task may mention AI-assisted drafting, but it is not complete unless the task also has machine-check evidence, human-review acceptance, or an explicit docs-only rationale. Gates reject AI-only verification claims.

## Fixture strategy

Positive fixtures cover an AI-drafted Kani contract with a passing Kani receipt and an AI-drafted Cairn spec with human-review acceptance. Negative fixtures cover AI-only proof claims, missing validation status, missing target requirement, and rejected draft reuse.

## What is explicitly not provided

- No raw prompt storage requirement.
- No claim that AI-generated contracts are correct before verification.
- No replacement for human review where review is required by policy.

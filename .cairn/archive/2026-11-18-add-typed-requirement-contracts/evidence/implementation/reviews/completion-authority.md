# Correspondence review: completion_authority

- Requirement: `cairn.supersede.agent_run_receipts.completion_authority`
- Requirement row: `5427af464b1ebc00db599a9ec90cdee48d0ce7b4a0b2652c6dc1d134bd89451b`
- Contract: `0da83d7244efc08cc3c03d52ad8510933992b53dcc61f06cef9c4d3381a6eb0b`
- Interpretation: `8b04a2b9d425c8b417693f33c621ef5d922475d5df342565d9e91ceca0bbe648`
- Mapping: `3403dc05eace9131de6e0b0c498a8e39d6156f3c75bc29f221fb4fdddcd9383e`

## Reviewed statement

A task or agent run must not be represented as authoritatively complete unless a
current passing Cairn completion-validation receipt covers the final subject and
every repository-required check.

## Assessment

The activation condition is the explicit `completion_claimed` caller assertion,
so model text is never interpreted as authority. The guarantee then requires
receipt presence, a passing receipt that is current, schema-current and
classified authoritative, subject equality with the final subject, and coverage
of every selected check. Each conjunct is a separate observable, so a caller
cannot satisfy the implication by supplying prose, a process exit status or a
task checkbox.

## Scope of this approval

Approval covers the declared supported portion only. Receipt origin and
authenticity, schema and receipt-hash validation implementation, wrapper
honesty, policy correctness, arbitrary check sets and broader release authority
are correctly excluded.

## Reviewer declaration

Independent review performed by the parent integration owner on 2026-11-18 by
reading the accepted requirement and the normalized contract. This is a recorded
declaration, not a signature, not proof, and not a human domain-expert review.
Decision: approved.

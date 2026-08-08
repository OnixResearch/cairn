# Formal proof readiness fixture

Cairn can count verification roles and validate stack-provenance evidence, but release-readiness reviewers need one frozen fixture that demonstrates how a Valence graph report derived from an Octet formal-proof export is consumed without upgrading recorded-only or manual-review records.

## Scope

- Add a Cairn readiness fixture for a Valence stack graph/readiness input that contains one eligible `property` formal-proof record and explicit recorded-only/manual-review context.
- Require readiness receipts to report property, recorded-only, boundary, unknown, verified, accepted, and blocked counts without treating identity-only evidence as proof.
- Add negative fixtures for stale Valence graph input, missing policy/input hashes, manual-review promotion, missing non-claims, failed upstream proof status, and receipt overclaims.
- Keep readiness policy Cairn-owned; Valence provides stack semantics and Octet provides formal-proof receipts.

## Impact

- **Cairn** gets an executable release-readiness seam fixture for formal-proof stack evidence.
- **Valence** remains the stack identity/linkage verifier.
- **Octet** remains the producer of formal-proof receipts and provenance.
- **Reviewers** can see exactly what Cairn counts and what remains recorded-only.

## Non-claims

This change proves Cairn readiness receipt handling for a frozen input fixture only. It does not prove upstream source correctness, verifier soundness, Valence correctness, Octet correctness, whole-program correctness, semantic equivalence, or release eligibility outside the configured readiness policy.

## Design

Cairn registers a Trellis proof evidence profile under the generic evidence-chain readiness gate. The pure evaluator checks typed chain links and policy requirements for proof role and assumption compatibility. It never parses Verus source, proof IR, verifier logs, or Preserves internals; those are owned upstream by Trellis, Kamacite, and Valence.

### Decisions

- **Profile-specific readiness modes.** Policy can make Trellis proof evidence optional, require recorded evidence, or require accepted formal-proof evidence for selected requirement/property IDs.
- **Role must match policy.** Recorded-only evidence cannot satisfy required-formal-proof mode.
- **Assumption policy is explicit.** Missing, stale, or disallowed assumptions fail required modes and invalidate present optional evidence.
- **Chain links are typed.** Cairn validates Kamacite, Valence, Mantle, source/binary, and policy hashes as distinct domains.
- **Payloads remain opaque.** Cairn gates readiness over metadata and upstream validation receipts only.
- **No release eligibility claim.** Passing Trellis proof readiness means policy-conformant proof evidence is present, not that the release is correct or eligible by itself.

### Validation shape

Positive fixtures cover optional absent proof evidence, optional recorded proof evidence, required recorded proof evidence, and required accepted formal-proof evidence. Negative fixtures cover missing canonical envelope hash, stale Valence/Mantle hash, wrong proof role, missing requirement/property scope, disallowed assumptions, malformed digest, unsupported claim scope, projection drift, source/binary mismatch, and overclaiming text.

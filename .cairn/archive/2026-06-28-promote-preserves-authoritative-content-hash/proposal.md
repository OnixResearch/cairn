# Proposal: Promote Preserves to authoritative content hash

## Why

The accepted `format_strategy.preserves_candidate` gate requires four things
before Preserves can be made authoritative for receipt encoding: byte-stable
hash material, diagnostic mapping to source IDs, receipt-verification
compatibility, and equivalent positive/negative fixtures. Three were already
proven by the Preserves canonicalization corpus, golden consumer, and CI
guards. The remaining clause — diagnostic mapping to source IDs — is delivered
by the `add-preserves-receipt-field-drift-diagnostics` change. Once that lands,
the gate is fully satisfied and promotion is permissible.

ROI-02 deliberately listed "Do not promote Preserves hashes into stored
agent-run receipt schemas" as a non-goal. This change relaxes that non-goal in
a compatibility-preserving way, because the blocker (field-level drift
diagnostics) no longer exists.

The motivation for promoting now is concrete: today `input_hash`,
`plan_hash`, and `receipt_hash` content identity depend on `serde_json`
serialization choices (key ordering, float formatting). Those are stable only
as long as every producer and consumer agrees on serde's exact output. A real
canonical, language-agnostic encoding removes that residual fragility.

## What Changes

- Introduce `preserves_content_hash` as an authoritative *additive* canonical
  content-hash field on receipt material that claims content identity, computed
  from the existing `canonical_preserves_hash` seam.
- Keep the JSON `receipt_hash` field as the public, stored, compatibility and
  verification surface — unchanged.
- Make `preserves_content_hash` optional initially: receipts without it remain
  valid until a reviewed policy flips a requirement flag.
- Add `cairn receipt verify-content-hash <path>` to recompute canonical
  Preserves bytes and compare against the stored field, failing closed on
  mismatch or tamper.
- Bind `preserves_content_hash` into agent-run receipts, evidence bundles,
  replay receipts, and release-evidence paths where content identity is
  claimed, alongside existing JSON hashes.

## Impact

- **Files**: `crates/cairn-core/src/verified/evidence.rs` (content-hash binding
  helper), receipt emitters across `cairn-cli`, a new `verify-content-hash`
  command, generated policy (requirement flag + field declaration), fixtures,
  `docs/preserves-receipts.md`, and evidence/replay/agent-receipt tests.
- **Testing**: positive (field present and correct), negative (tampered field
  fails closed), compatibility (receipts without the field still validate),
  and boundary (key-reordered/float-equivalent inputs produce equal content
  hashes), plus Cairn gates, release-readiness, and `preserves-corpus-fresh`.
- **Compatibility**: additive. No stored receipt, evidence bundle, manifest, or
  existing verifier loses behavior; `receipt_hash` contracts are unchanged.

## Non-Goals

- Do not remove, replace, or alter the JSON `receipt_hash` field or its
  verification surface.
- Do not break stored receipts produced before this change.
- Do not introduce signing, transport, or remote-registry changes.
- Do not make `preserves_content_hash` required in this change; that is a later
  reviewed policy flip after the field is proven stable.

## Dependencies

- Depends on `add-preserves-receipt-field-drift-diagnostics`, which satisfies
  the diagnostic-mapping clause of the promotion gate.

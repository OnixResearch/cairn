# Design: Formal proof readiness fixture

## Shape

Add a frozen readiness fixture that models Cairn consuming a Valence stack graph/readiness input:

```text
Valence graph/readiness input summary
  -> Cairn stack-provenance/readiness validation
  -> Cairn release-readiness receipt
```

The fixture contains one eligible property record backed by an Octet-owned formal-proof receipt, plus recorded-only/manual-review records that must not count as verified. The pure core validates parsed fixture values and policy data. The CLI shell may load fixture files and emit JSON receipts.

## Invariants

1. **Verified count equals eligible property count.** Readiness receipts must count only validated property records as verified.
2. **Manual-review blocks promotion.** Manual-review and imported-reference records stay recorded-only unless an upstream Octet-owned proof receipt is present in the Valence input.
3. **Readiness input is content-bound.** Cairn records the Valence input hash, policy hash, and readiness receipt hash.
4. **Staleness fails closed.** A stale Valence graph digest, stale sidecar digest, missing policy hash, or missing input hash blocks readiness.
5. **No policy delegation.** Valence verifies stack semantics; Cairn decides readiness under Cairn policy.

## Positive fixture

The positive fixture includes a Valence readiness input with one eligible property record, explicit recorded-only/manual-review context records, matching BLAKE3 identities, required non-claims, and policy-compatible role counts. Cairn readiness validation passes and reports the property record as verified while leaving recorded-only records unverified.

## Negative fixtures

Negative fixtures cover:

- Valence input digest mismatch;
- sidecar or graph digest mismatch;
- manual-review or recorded-only record counted as verified;
- failed upstream proof status marked as property;
- missing required non-claims;
- missing policy/input/receipt hashes;
- readiness report text claiming upstream correctness or release eligibility from receipt envelope validation alone.

## Validation

Focused validation should run:

```bash
nix run path:$PWD#cairn -- policy export --check
nix develop -c cargo test -p cairn-core formal_proof_readiness_fixture
nix run path:$PWD#cairn -- validate --root .
nix run path:$PWD#cairn -- gate proposal formal-proof-readiness-fixture --root .
nix run path:$PWD#cairn -- gate design formal-proof-readiness-fixture --root .
nix run path:$PWD#cairn -- gate tasks formal-proof-readiness-fixture --root .
nix run path:$PWD#cairn -- release-readiness --root .
```

## Risks

- **Vacuous pass**: require explicit property and recorded-only counts so `0 verified` is never reported as success by implication.
- **Upstream overclaim**: require non-claims from Valence and Octet to remain visible in Cairn's receipt.
- **Fixture divergence**: bind all fixture inputs with BLAKE3 and include stale-digest negative fixtures.

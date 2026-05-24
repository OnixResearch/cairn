# Steel orchestration CLI

Cairn exposes Steel lifecycle orchestration as an optional planning/control-plane seam. Steel does not replace the Rust/Nickel lifecycle authority: Rust still validates action envelopes, owns policy enforcement, executes effects, and emits deterministic receipts.

The default generated policy keeps Steel disabled. That is the safe baseline operators should expect from a fresh checkout.

r[impl cairn.lifecycle_orchestration.steel_lifecycle_orchestration.operator_docs]

## Commands

Run from the Cairn repository root:

```sh
nix run path:$PWD#cairn -- steel-orchestration receipt --root .
nix run path:$PWD#cairn -- steel-orchestration default-gate --root .
```

When testing an in-tree checkout through Cargo, use the explicit binary name:

```sh
nix develop -c cargo run -p cairn-cli --bin cairn -- steel-orchestration receipt --root .
```

## Disabled default receipt

Under the checked-in default policy, `receipt` should pass while reporting that Steel is disabled:

```json
{
  "outcome": "disabled",
  "mode": "disabled",
  "profile_id": "disabled.lifecycle.default",
  "steel_enabled": false,
  "steel_runtime_available": false,
  "effects_executed": false,
  "fallback_reason": "policy_disabled",
  "valid": true,
  "verdict": "PASS"
}
```

`effects_executed: false` means the Steel orchestration shell did not execute filesystem, process, network, lifecycle mutation, or evidence-writing effects. Existing Rust/Nickel lifecycle commands remain the effecting implementation.

r[verify cairn.lifecycle_orchestration.steel_lifecycle_orchestration.operator_docs]

## Default-gate under the disabled policy

`default-gate` is intentionally fail-closed under the default policy:

```sh
nix run path:$PWD#cairn -- steel-orchestration default-gate --root .
```

Expected shape:

```json
{
  "default_candidate": false,
  "steel_enabled": false,
  "issues": [
    { "code": "steel_policy_disabled" },
    { "code": "profile_not_default_candidate" }
  ],
  "valid": false,
  "verdict": "FAIL"
}
```

Providing evidence hashes does not bypass a disabled or non-default policy profile. Policy must first select a default-capable profile.

## Selected-default gate shape

A policy-selected default profile must use mode `selected_default` or `broad_default`, include fallback modes, and provide all required evidence hashes:

```sh
HASH=aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa
nix run path:$PWD#cairn -- steel-orchestration default-gate --root . \
  --policy /path/to/selected-default-policy.json \
  --profile selected.lifecycle.closeout \
  --equivalence-receipt-hash "$HASH" \
  --fallback-receipt-hash "$HASH" \
  --ci-reproducibility-receipt-hash "$HASH" \
  --diagnostics-receipt-hash "$HASH"
```

With a valid selected-default policy and complete lowercase 64-character hex evidence hashes, the gate emits `valid: true` and `verdict: PASS`. Missing evidence emits deterministic `missing_promotion_evidence` issues instead.

## Receipt and redaction rules

Steel orchestration receipts are bounded evidence objects. Operators can rely on these stable review signals:

- `schema_version`: receipt family and version.
- `profile_id` and `mode`: selected Steel orchestration policy profile.
- `steel_enabled`: whether policy enabled the Steel orchestration surface.
- `steel_runtime_available`: whether a reviewed script/envelope was available for evaluation.
- `effects_executed`: currently always `false` for the Steel orchestration shell.
- `issues`: deterministic denial, fallback, malformed policy, or missing evidence diagnostics.
- `receipt_hash`: BLAKE3 hash of the canonical receipt with `receipt_hash` excluded.

Receipts must not include credentials, raw host secrets, private provider payloads, or unredacted environment values. Script and action details are reduced to reviewed identities, hashes, safe summaries, or redacted placeholders.

## What this does not prove

- It does not prove Steel is enabled by default.
- It does not grant Steel ambient authority to mutate the repository or execute host effects.
- It does not replace `validate`, `gate`, `sync`, `archive`, Tracey, policy export, or release-readiness implementations.
- It does not make placeholder hashes production evidence; real promotion requires reviewed receipts for equivalence, fallback behavior, CI reproducibility, and operator diagnostics.

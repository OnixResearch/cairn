# Standalone Nickel export baseline and parity evidence

Date: 2026-07-14

## Pinned cohort

- Source: `github.com/OnixResearch/nickel-export`
- Revision: `257fafc1c746f1faf156207043a4c826bfb16d49`
- Evaluator identity: `nixpkgs:nickel`
- Evaluator cohort: `nickel-lang-cli-1.16.0`

Cargo and Nix release inputs both bind the immutable revision. Local path or branch overrides are not release identities.

## Before-change baseline

The legacy command completed policy freshness, structured-lifecycle check/export, stale-output checking, and invalid-policy rejection before generation logic changed.

| Observation | BLAKE3 |
|---|---|
| `cairn-policy/generated/cairn-policy.json` exact bytes | `d87c7f7cfe21836cf06598dd51225992c879fe1560d99d6e7c1a13c9614aa47c` |
| structured-lifecycle generated JSON exact bytes | `b0bd4b75e0ba1739a8708b61f4a654aaa67c11313c8a8c63ce72b1394d2b75b7` |
| invalid-marker Nickel diagnostic bytes | `ee458a0b5aad0013e5bc3b64478b6c263da443f4b4b5db587857e3c2149f1509` |

## Authority and rollback

The Cairn wrapper runs the legacy and canonical observation paths over the same source, dependency closure, evaluator, format, and destination identity. Status, stdout bytes, and stderr bytes must match exactly. The legacy bytes remain the value passed into Cairn-owned parsing and lifecycle semantic checks. Any parity or canonical-admission drift fails before a generated destination is written.

Cairn continues to own policy validation, structured-lifecycle validation, gates, replay, generated destinations, mutation decisions, and release readiness. The standalone receipt proves exact declared identities only; it does not prove evaluator equivalence, semantic correctness, gate soundness, deployability, or release eligibility.

Rollback keeps the legacy `nickel export` result authoritative and requires a paired positive and negative regression before a drift classification can be accepted.

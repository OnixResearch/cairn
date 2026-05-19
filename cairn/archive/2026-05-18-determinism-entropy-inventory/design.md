# Design: determinism entropy inventory

## Context

The replay oracle is policy-owned and portable across roots, but entropy handling is still partly implicit. The only active rejection is a Rust literal list of archive-date environment variables. This makes it harder for agents/CI to audit determinism assumptions and harder for reviewers to extend entropy handling without code spelunking.

## Goals

- Make entropy handling a reviewed generated-policy contract.
- Keep receipts stable, root-portable, and secret-safe.
- Preserve existing negative hidden-entropy behavior.
- Keep replay read-only and deterministic.

## Non-Goals

- Capturing raw environment variable values.
- Recording host-specific absolute paths, usernames, machine IDs, or full tool paths.
- Adding live probes for every tool version.
- Changing agent receipt replay coverage semantics.

## Decisions

### 1. Policy-owned inventory entries

**Choice:** Add `determinism_replay_policy.entropy_inventory` as an array of entries with:

- `id`
- `class`
- `source`
- `handling`
- `receipt_field`
- `description`

**Rationale:** The inventory lives beside replay cases/groups, making entropy handling reviewable in the same generated policy artifact.

### 2. Stable replay receipt fields

**Choice:** Add top-level replay receipt fields:

- `entropy_inventory`
- `entropy_assumptions`
- `entropy_policy_hash`

**Rationale:** Agents can inspect the full inventory and a compact hash. Assumptions are stable display-safe summaries, not raw host data.

### 3. Rejected entropy from policy

**Choice:** `handling = "reject"` entries drive nondeterminism failures. For this slice, `CAIRN_ARCHIVE_DATE` and `IRONSPEC_ARCHIVE_DATE` remain rejected when present.

**Rationale:** Preserves current safety while removing hidden Rust ownership of the rejected-source list.

### 4. Secret-safe disclosures

**Choice:** Receipts disclose source names/classes/handling and do not disclose env values or host-specific absolute paths.

**Rationale:** Replay receipts may be attached to CI/agent evidence; they must stay portable and safe to share.

## Risks / Trade-offs

- **Schema churn:** New stable fields update generated policy hashes and goldens. Mitigation: regenerate fixtures/goldens only after policy changes pass focused tests.
- **Inventory incompleteness:** Initial inventory is a reviewed baseline, not a perfect ontology. Mitigation: include explicit entries for clock/date, locale/timezone, cwd/root paths, env vars, filesystem order/metadata, external commands, and Nix/build context.
- **Accepted spec sync drift:** Sync may rewrite prior replay scenarios. Mitigation: inspect accepted spec after sync and rerun Tracey.

## Validation Plan

- `nix run path:$PWD#cairn -- validate --root .`
- `nix run path:$PWD#cairn -- gate proposal|design|tasks determinism-entropy-inventory --root .`
- `nix develop -c python3 tools/export-policy.py --check`
- Focused replay/schema/entropy tests in `cargo test -p cairn-cli determinism_replay -- --nocapture` and `cargo test -p cairn-cli public_receipt -- --nocapture`
- Full `cargo test -p cairn-cli` and `cargo test -p cairn-core`
- `python3 tools/check-tracey-coverage.py --root .`
- `nix run path:$PWD#cairn -- release-readiness --root .`
- `nix run path:$PWD#cairn -- determinism replay --root . --group smoke --format summary --out /tmp/cairn-entropy-smoke.json`
- `nix run path:$PWD#cairn -- determinism replay --root . --format summary --out /tmp/cairn-entropy-full.json`

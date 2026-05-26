# Preserves receipt workflow

Preserves support in Cairn is an internal deterministic evidence seam. It gives operators a typed/canonical hash lens for receipt-shaped values, but it does **not** replace the public JSON receipt surface.

## Boundary

Use JSON `receipt_hash` for compatibility and stored/public receipt validation:

- agent-run receipts still store and validate the JSON `receipt_hash` field;
- `cairn agent-receipt validate <path>` keeps its existing JSON contract;
- evidence bundles, manifests, replay receipts, and release evidence continue to bind JSON receipt hashes.

Use Preserves hashes for internal inspection and drift detection:

- Preserves hashes are BLAKE3 hashes of packed canonical Preserves bytes;
- the current public operator surface is a read-only inspection receipt;
- checked-in corpus hashes make Preserves/serde canonicalization drift reviewable.

## Inspect an agent receipt Preserves hash

Run:

```sh
nix run .#cairn -- agent-receipt preserves-hash fixtures/agent-receipts/positive.json
```

The command is read-only. It emits a deterministic inspection receipt containing:

- `command = "agent-receipt preserves-hash"`
- `path`
- `input_hash`
- `bounded_read_limit`
- `valid` / `verdict` / `issues`
- `agent_receipt_preserves_hash`
- the inspection receipt's own JSON `receipt_hash`

Do not copy `agent_receipt_preserves_hash` into stored agent-run receipts unless a future reviewed Cairn change explicitly promotes such a schema. The positive fixture's current internal Preserves hash is:

```text
12dbd475fffb1529d1f81f095371052a9d00e598beeb168d793325389ecce029
```

## Check the canonicalization corpus

Run the corpus update rail in dry-run mode:

```sh
nix run .#cairn -- preserves-corpus update --path fixtures/preserves-canonicalization/corpus.json
```

Dry-run is the default. A clean corpus exits 0 with `valid: true`, `stale_count: 0`, and `mutated: false`. If Preserves, serde conversion, or fixture content changes make any `expected_hash` stale, the dry-run exits nonzero and reports the stale cases without writing the file.

For intentional drift, review the dependency/content change first, then run the explicit mutation rail:

```sh
nix run .#cairn -- preserves-corpus update --path fixtures/preserves-canonicalization/corpus.json --execute
```

`--execute` rewrites only stale `expected_hash` strings in case order, preserving case ids, descriptions, values, ordering, and formatting where practical. Commit the corpus diff together with the reviewed change that caused the drift.

## CI guard expectations

Nix checks make accidental Preserves drift fail closed:

```sh
nix build path:$PWD#checks.x86_64-linux.preserves-corpus-fresh --no-link -L
nix build path:$PWD#checks.x86_64-linux.preserves-corpus-stale-detects-drift --no-link -L
```

- `preserves-corpus-fresh` runs the dry-run update command against the checked-in corpus and expects no stale hashes.
- `preserves-corpus-stale-detects-drift` deliberately corrupts one expected hash and proves the dry-run command fails without mutation.

During a Preserves dependency bump, CI should either remain green with a clean no-op corpus check, or the reviewed change should include an intentional `--execute` corpus hash update plus the associated focused tests.

## Non-claims

- Preserves is not a replacement for JSON public/stored receipt schemas.
- CI never runs `--execute` automatically.
- Preserves inspection receipts do not grant additional trust beyond the checked input receipt and the current Cairn policy/evidence chain.
- Secret-bearing values should never be placed in fixtures or inspection receipts; use bounded metadata and hashes instead.

r[impl cairn.determinism.preserves_operator_docs.workflow]
r[impl cairn.determinism.preserves_operator_docs.discoverability]
r[impl cairn.determinism.preserves_operator_docs.validation]

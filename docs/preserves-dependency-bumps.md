# Preserves dependency-bump runbook

Use this runbook when changing the `preserves` crate, Preserves transitive dependencies, or Cairn serde/receipt material that can affect canonical Preserves bytes.

Preserves remains an internal deterministic evidence seam. A dependency bump must not change public/stored JSON receipt schemas or the compatibility meaning of JSON `receipt_hash`.

## Preconditions

1. Start from a clean synced worktree.
2. Make only the reviewed dependency/conversion change first.
3. Do not edit `fixtures/preserves-canonicalization/corpus.json` by hand.
4. Never place secrets or credential-bearing values in Preserves fixtures, inspection receipts, or runbook evidence.

## 1. Dry-run drift detection first

Run the corpus update rail without `--execute`:

```sh
RUSTC_WRAPPER= CARGO_TARGET_DIR=/home/brittonr/.cargo-target/cairn-preserves \
  nix develop -c cargo run -p cairn-cli --bin cairn -- \
  preserves-corpus update --path fixtures/preserves-canonicalization/corpus.json
```

Expected no-drift result:

- command exits 0;
- `valid: true`;
- `stale_count: 0`;
- `mutated: false`;
- `fixtures/preserves-canonicalization/corpus.json` remains unchanged.

If this fails with stale cases, stop and review the dependency/conversion change before refreshing hashes.

## 2. Run CI drift guards

Run both Nix guards:

```sh
nix build path:$PWD#checks.x86_64-linux.preserves-corpus-fresh --no-link -L
nix build path:$PWD#checks.x86_64-linux.preserves-corpus-stale-detects-drift --no-link -L
```

The fresh guard proves the checked-in corpus is clean. The stale-detection guard corrupts one expected hash in a copy and proves dry-run failure is fail-closed and non-mutating.

CI must never run `--execute` automatically.

## 3. No-drift path

If the dry-run and both Nix guards pass without corpus changes:

```sh
RUSTC_WRAPPER= CARGO_TARGET_DIR=/home/brittonr/.cargo-target/cairn-preserves \
  nix develop -c cargo test -p cairn-core preserves -- --nocapture
RUSTC_WRAPPER= CARGO_TARGET_DIR=/home/brittonr/.cargo-target/cairn-preserves \
  nix develop -c cargo test -p cairn-cli --test preserves_corpus -- --nocapture
RUSTC_WRAPPER= CARGO_TARGET_DIR=/home/brittonr/.cargo-target/cairn-preserves \
  nix develop -c cargo test --workspace
```

Then land the dependency bump without changing corpus `expected_hash` fields.

## 4. Intentional-drift path

Use this path only after reviewing why Preserves canonical bytes changed and confirming the change is acceptable.

Refresh the corpus through the explicit mutation rail:

```sh
RUSTC_WRAPPER= CARGO_TARGET_DIR=/home/brittonr/.cargo-target/cairn-preserves \
  nix develop -c cargo run -p cairn-cli --bin cairn -- \
  preserves-corpus update --path fixtures/preserves-canonicalization/corpus.json --execute
```

Review the diff:

```sh
git diff -- fixtures/preserves-canonicalization/corpus.json
```

Acceptable corpus diffs should rewrite only stale `expected_hash` strings. Case ids, descriptions, values, ordering, and formatting should remain stable unless the same reviewed change explicitly modifies fixture coverage.

After refreshing, rerun:

```sh
RUSTC_WRAPPER= CARGO_TARGET_DIR=/home/brittonr/.cargo-target/cairn-preserves \
  nix develop -c cargo test -p cairn-core preserves -- --nocapture
RUSTC_WRAPPER= CARGO_TARGET_DIR=/home/brittonr/.cargo-target/cairn-preserves \
  nix develop -c cargo test -p cairn-cli --test preserves_corpus -- --nocapture
nix build path:$PWD#checks.x86_64-linux.preserves-corpus-fresh --no-link -L
nix build path:$PWD#checks.x86_64-linux.preserves-corpus-stale-detects-drift --no-link -L
RUSTC_WRAPPER= CARGO_TARGET_DIR=/home/brittonr/.cargo-target/cairn-preserves \
  nix develop -c cargo test --workspace
```

## 5. Lifecycle closeout

Before committing a dependency bump, run the Cairn rails for the active change:

```sh
RUSTC_WRAPPER= CARGO_TARGET_DIR=/home/brittonr/.cargo-target/cairn-preserves \
  nix develop -c cargo run -p cairn-cli --bin cairn -- validate --root .
RUSTC_WRAPPER= CARGO_TARGET_DIR=/home/brittonr/.cargo-target/cairn-preserves \
  nix develop -c cargo run -p cairn-cli --bin cairn -- tracey coverage --root .
git diff --check
```

For changes with a Cairn package, also run proposal/design/tasks gates before archive.

## Non-claims

- This runbook does not authorize changing public/stored JSON receipt schemas.
- This runbook does not promote Preserves hashes into stored agent-run receipt fields.
- This runbook does not permit automatic corpus mutation in CI.
- This runbook does not replace operator guidance in `docs/preserves-receipts.md`; it narrows the dependency-bump path.

r[impl cairn.determinism.preserves_dependency_bump_runbook.workflow]
r[impl cairn.determinism.preserves_dependency_bump_runbook.intentional_drift]
r[impl cairn.determinism.preserves_dependency_bump_runbook.discoverability]
r[impl cairn.determinism.preserves_dependency_bump_runbook.validation]

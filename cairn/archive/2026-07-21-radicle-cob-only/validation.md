# Validation: Radicle COB-Only Collaboration

## Result

Cairn now owns a local signed Git COB provider through direct `radicle-cob`, `radicle-crypto`, `radicle-oid`, `git2`, and ref-format primitives. The public boundary is `radicle-cob.v1` / `cairn cob` over an explicit Git repository and a mutation-only keystore. The umbrella `radicle` crate, `radicle-node`, profile and identity-document loading, node control, peer discovery, connect/fetch synchronization, and convergence claims are absent.

## Format and State Evidence

- The direct store writes standard signed COB commits, manifests, operation parents, object/type IDs, and `refs/namespaces/<actor>/refs/cobs/<type>/<object>` references.
- Typed create, load, update, concurrent-head evaluation, complete-basis resolution, and exact reload pass through `radicle_cob::Evaluate` without a Radicle runtime.
- Signed author and namespace identity, manifest type, target loading, and operation ancestry are validated fail closed. Malformed, unsigned, wrong-type, mismatched-author, and stale-reference histories are rejected.
- Failed create and update postconditions roll visible refs back with compare-and-swap checks. Repository-scoped COB mutation locks reject contention and clean up on release or unwinding.
- New root state records bounded immutable creator/delegate keys. Legacy roots keep their historical canonical hashes and are creator-only in this provider; embedded legacy delegate mutations are rejected.
- Concurrent losing revisions remain in Git history. Resolution must supersede every current head and never rewrites historical operations.

## Shell and CLI Evidence

- `cairn cob import|status|checkout|update|conflicts|resolve` and matching `cairn collaboration ... --backend radicle-cob.v1` commands use explicit `--git-repository` paths.
- Only signed mutations accept `--keystore`; import alone accepts `--workspace` and explicit `--delegate` keys. Status and conflict inspection require neither identity documents nor keystores.
- Signed writes require repository-local non-empty `user.name` and `user.email` as non-authoritative Git display metadata. Global Git configuration is not accepted as a hidden fallback.
- Mutation execution rechecks policy, signer, repository state, local artifacts, and the previewed plan while holding repository checkout and COB locks in one order.
- `cairn cob sync` fails before locks, writes, sockets, or network activity and directs operators to external Git transport. `collaboration sync` is removed.
- Receipts bind the direct format-library versions and explicitly disclaim publication, availability, synchronization, convergence, durability, remote authority, source correctness, and release eligibility.

## Deterministic Rails

- `cargo fmt --all -- --check` passed.
- `cargo clippy --workspace --all-targets -- -D warnings` passed.
- `cargo test --workspace` passed, including 391 core tests, both 73-test CLI binary suites, three adapter unit tests, and three direct format-compatibility integration tests.
- Proposal, design, and tasks gates passed. `cairn validate --root . --strict` passed before and after accepted-spec sync.
- Accepted-spec sync applied seven bounded deltas, retiring runtime synchronization and two-peer convergence requirements while adding the COB-only boundary.
- Traceability passed with `1679/1679` accepted requirement and scenario IDs referenced, with no missing or dangling markers.
- `cairn evidence export` passed with a complete portable bundle, and `cairn release-readiness --root .` passed.
- `nix run path:$PWD#cairn -- policy export --check` passed.
- `nix build path:$PWD#checks.x86_64-linux.self-dogfood --no-link -L` passed.
- `nix flake check path:$PWD -L` completed with `all checks passed` on `x86_64-linux`; incompatible `aarch64-linux` checks were omitted. The flake run included the dedicated `radicle-cob-format-compatibility` dependency audit and direct format tests.

## Adversarial Findings

The first dedicated Nix run exposed an implicit dependency on host Git `user.name`. Cairn now checks non-empty repository-local commit metadata before signed mutation, the fixtures set it explicitly, and a negative test proves missing local metadata fails closed.

A workspace-wide all-target Clippy run exposed a receipt-version constant imported only for tests. The import is now test-scoped and the full warnings-as-errors rail passes.

An initial flake retry used a log file under the evaluated source tree and Nix correctly rejected the changing source hash. The final flake run wrote its log outside the repository and completed against a stable source snapshot.

Pre-sync evidence export correctly failed because active delta markers and retired accepted synchronization requirements could not simultaneously satisfy accepted-spec traceability. After bounded sync, strict validation, traceability, evidence export, release-readiness, self-dogfood, and the full flake rail all passed.

## Bounded Claims

Passing local format tests proves compatibility only for the pinned direct library cohort and the exercised signed Git histories. It does not prove compatibility with every Radicle version or arbitrary external COB type.

Git refs and operation IDs prove only the checked local repository observations. Cairn does not publish, discover, seed, announce, fetch, synchronize, or converge repositories. Repository-local display metadata grants no authority; only the signed creator/delegate policy does.

Cooperative locks and compare-and-swap rollback do not sandbox unrelated processes with equivalent host authority and do not claim whole-repository crash durability. Passing lifecycle, evidence, and Nix gates does not prove source-code correctness, remote trust, external transport correctness, or universal release eligibility.

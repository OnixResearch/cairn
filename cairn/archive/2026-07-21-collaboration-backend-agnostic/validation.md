# Validation: Backend-Agnostic Change Collaboration

## Result

Cairn active-change import, status, and checkout now work through a transport-neutral collaboration command and an executable `portable-json.v1` backend with no Radicle profile, repository ID, node, or external helper. Radicle remains a built-in adapter and compatibility command rather than the semantic owner.

## Architecture Evidence

- Neutral `cairn.change-collaboration.v1` APIs use workspace and typed base-revision identities and accept non-Git revision kinds.
- The reducer preserves exact artifacts, deterministic revision heads, same-artifact conflicts, lifecycle/evidence behavior, and BLAKE3 content/state identities.
- Legacy `cairn.radicle-change-cob.v1` fixtures hash through the original canonical `repository_id` and `base_git_oid` shape; generic wrappers return the same historical identities.
- New Radicle import planning uses the neutral schema with a `git_oid` adapter revision, and the real Git-backed adapter suite creates/reloads that state while preserving legacy adapter tests.
- Portable envelopes require neutral state, full structural/protocol validation, recomputed content/state/envelope hashes, and reject even malformed state whose attacker-facing hashes were recomputed.

## Shell and CLI Evidence

- `cairn collaboration import|status|checkout --backend portable-json.v1` parses and executes without `--rad-home` or `--rid`.
- Portable import is preview-first, double-scans execution inputs under an exclusive store lock, refuses existing/unsafe stores, validates an exclusive prepared file, atomically exposes it, and reloads it.
- Status is bounded and read-only. Checkout rechecks the envelope and local plan under both a provider store lock and a repository-scoped per-change lock shared with the Radicle shell.
- Exact bytes round-trip from an active change through portable import, status, deletion, checkout preview, and checkout execution.
- Negative tests cover unsupported/backend-mismatched arguments, recomputed malformed envelopes, tampered hashes, existing stores, symlink stores, stale store locks, cross-backend repository locks, conflicts, and divergent local destinations.
- A real CLI smoke executed portable import and status against the active change and emitted passing receipts under `target/` without Radicle context.

## Deterministic Rails

- Focused neutral core tests passed with 10 tests.
- Real Git-backed Radicle adapter tests passed with 14 tests.
- Portable CLI tests passed with positive and negative workflows; existing Radicle CLI tests also passed.
- Full `cargo test --workspace` passed.
- Full `cargo clippy --workspace --all-targets -- -D warnings` and `cargo fmt --check` passed.
- Verified-core purity search passed for every new `collaboration_*.rs` core module.
- Policy freshness, Cairn validation, proposal/design/tasks gates, and sync passed.
- Traceability coverage passed with `1643/1643` accepted requirement and scenario IDs referenced.
- `nix flake check path:$PWD -L` completed with `all checks passed` on `x86_64-linux`; `aarch64-linux` was omitted as incompatible.
- Post-archive `nix build path:$PWD#checks.x86_64-linux.self-dogfood --no-link -L` passed.

## Adversarial Findings

Three mechanism families were considered. A neutral semantic v2 plus provider adapters and a second backend survived. A neutral projection over Radicle v1 and a provider trait over Radicle-shaped types were rejected because both still required fabricated Radicle/Git identities.

The first post-implementation audit found that recomputed portable hashes could accompany structurally malformed reducer state. Full state validation now checks context, protocol limits, operation inventory, revision bases, artifacts, evidence keys, lifecycle references, and required histories before trusting the envelope.

The second audit found that provider-specific locks did not serialize Radicle and portable checkouts targeting the same active-change directory. Every backend now acquires the same repository-scoped per-change checkout lock and rechecks destination state while holding it.

The configured advisory model was reachable during the first architecture review and supported the selected neutral-v2 route, but its post-implementation fetch failed. No advisory output was used as validation evidence.

## Bounded Claims

Portable actor, workspace, and revision values are caller declarations, not signatures or remote authority. A valid portable envelope proves deterministic local structure and hash consistency, not authenticity. Cooperative locks do not sandbox unrelated processes with equivalent host authority, and atomic rename does not claim fsync-level crash durability for the whole repository.

No live Radicle node, peer synchronization, network publication, or third provider was exercised. Passing collaboration receipts do not prove remote trust, gate validity, evidence truth, dependency readiness, source correctness, archive execution, or release eligibility.

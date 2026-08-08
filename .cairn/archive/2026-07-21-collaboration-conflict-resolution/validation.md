# Validation: Explicit Collaboration Conflict Resolution

## Result

Cairn now exposes backend-neutral `collaboration conflicts` and preview-first `collaboration resolve` for portable and Radicle providers. Inspection reports every current competing artifact and lifecycle head through deterministic metadata and can disclose one explicitly requested exact artifact value. Resolution requires one current-head choice for every conflicted artifact, creates one new revision per artifact whose basis supersedes every current head, and resolves lifecycle conflicts only through an explicit evidence-free `open` reset.

## Architecture Evidence

- The pure resolver validates change, policy, actor, complete selectors, current heads, and lifecycle intent; computes sorted actions and deterministic BLAKE3 operation/plan identities; simulates the complete operation; and rejects any result that remains conflicted.
- Selected values may include deletion. Losing revisions, actors, bases, and values remain in history; resolution never rewrites or prunes provider state.
- Portable execution uses caller-declared creator identity, portable-store and shared repository-change locks, policy/state/choice rechecks, validated prepared replacement, and exact envelope/state/content reload verification.
- Radicle execution independently reloads current canonical identity authority, verifies signer possession, serializes through the per-change update lock, signs one typed operation, reloads the same object, requires no remaining conflicts, verifies the planned content root, and records the provider-assigned result state hash.
- Exact head content is opt-in and limited to one named current head. Default inspection receipts contain revision, actor, deletion, byte-length, hash, and typed lifecycle metadata rather than every artifact value.

## Portfolio and Adversarial Audit

Three mechanisms were evaluated:

1. **Complete-basis successor revisions** survived: one explicit selected value is copied into a new revision whose basis names every current competing head.
2. **State rewrite/pruning** was rejected because it erases losing revisions and breaks historical evidence.
3. **Implicit local-file winner** was rejected because ordinary filesystem bytes are not an explicit conflict choice and would create hidden local-wins semantics.

The implementation audit found that the original portable envelope required a conflict-free content root, making portable conflict resolution unreachable. The envelope now represents conflicted content as `content_root: null` while retaining complete state and envelope validation. Conflict-free `Some(content_root)` serializes in the original JSON shape and a compatibility test proves its envelope hash is unchanged. A negative test proves `content_root: null` cannot be used for conflict-free state.

An advisory review questioned whether an absent portable content root could be accepted as conflict-free. Deterministic positive/negative envelope tests show that `null` is accepted only when the validated reducer reports artifact conflict; conflict-free state requires the exact computed root. Advisory output was not treated as validation evidence.

The audit also bounded Radicle preview semantics: signed COB operation IDs are provider-assigned, so Radicle preview binds exact actions and expected content root rather than advertising an impossible precomputed state hash. Execution records the observed post-signature state hash.

## Deterministic Evidence

- Focused core tests passed for complete inspection, one-head disclosure, exact complete bases, multi-artifact resolution, deletion, lifecycle reset, deterministic hashes, unauthorized/stale/incomplete/duplicate choices, no-conflict input, and plan drift.
- Portable provider tests passed for conflicted envelope round trip, preview and execution, exact reload, wrong actor, incomplete choices, stale lock, stale prepared state, symlink store, strict arguments, and non-claims.
- Radicle adapter tests passed for signed resolution, conflict-free reload, stale-plan rejection, current-identity rejection, and explicit profile/signer execution.
- Standalone CLI dispatch smoke passed for portable `collaboration conflicts` and a correctly blocked no-conflict `collaboration resolve`.
- Full `cargo test --workspace`, `cargo fmt --check`, and `cargo clippy --workspace --all-targets -- -D warnings` passed.
- Policy freshness, Cairn validation, proposal/design/tasks gates, and verified-core purity passed.
- Traceability coverage passed with `1674/1674` accepted requirement and scenario IDs referenced.
- `nix flake check path:$PWD -L` completed with `all checks passed` on `x86_64-linux`; `aarch64-linux` was omitted as incompatible.

## Bounded Claims

Selecting one existing head does not perform semantic or textual merge and does not prove that losing heads are invalid. Operators may publish a later reviewed merged edit through ordinary update after resolving ambiguity. Resetting lifecycle to `open` does not prove review, approval, current evidence, or dependency readiness. Portable actor identity remains a caller declaration. Local signed Radicle resolution does not prove node publication, peer replication, availability, convergence, or remote trust. Passing receipts do not prove source correctness, archive execution, release eligibility, fsync-level crash durability, future authority, or resistance to unrelated processes with equivalent host authority.

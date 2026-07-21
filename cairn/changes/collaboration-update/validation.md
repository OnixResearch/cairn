# Validation: Preview-First Collaboration Update

## Result

Cairn collaborative changes are now editable after import through one backend-neutral update plan and preview-first portable or Radicle execution. Exact local additions, edits, and removals become sorted `SetArtifact` actions with complete current revision bases; equal bytes are a no-op; content and lifecycle conflicts fail closed.

## Architecture Evidence

- `cairn-core` owns deterministic snapshot comparison, identity/policy/authority checks, action selection, complete revision bases, simulated content identity, portable application, and plan hashing without filesystem, environment, process, clock, async, or output effects.
- Portable execution acquires the provider store lock and shared repository-change lock, reloads policy/state/local bytes, recomputes the complete plan and candidate envelope, validates a prepared file, atomically replaces only the expected store, and reloads every BLAKE3 identity.
- Radicle execution rechecks explicit profile, current canonical identity, creator-or-delegate authority, object/state identity, policy, local bytes, and plan; signed typed mutation occurs under an exclusive update lock and must reload with the planned content root.
- Provider receipts distinguish caller-declared portable identity from Radicle profile/current-identity/signer observations and bind input state/content, policy, changed artifacts, revision bases, expected content, execution mode, blockers, and mutation result.

## Deterministic Evidence

- Focused pure-core positive and negative update tests passed.
- Radicle adapter tests passed for signed update/reload, stale-plan rejection, false delegate claims, update lock behavior, and real explicit profile/signer execution.
- Portable and Radicle CLI tests passed for explicit arguments, previews, no-op behavior, exact update, BLAKE3 receipts, unauthorized actors, stale locks, prepared-state collisions, unsafe stores, and drift blockers.
- Full `cargo test --workspace` passed.
- Full `cargo clippy --workspace --all-targets -- -D warnings` and `cargo fmt --check` passed.
- Policy freshness, Cairn validation, proposal/design/tasks gates, and verified-core purity passed.
- Traceability coverage passed with `1657/1657` accepted requirement and scenario IDs referenced.
- `nix flake check path:$PWD -L` completed with `all checks passed` on `x86_64-linux`; `aarch64-linux` was omitted as incompatible.

## Defect Found During Validation

The first full workspace run failed evidence export because implementation markers for the active delta were intentionally dangling until the new requirements entered accepted specs. Executed Cairn sync added `cairn/specs/collaboration-update/spec.md`; traceability then passed at `1657/1657`, and the previously failing evidence-export integration passed.

An adversarial advisory review proposed stale actor, stale policy, and interleaving risks. Repository inspection and deterministic tests confirmed that portable execution rechecks the fixed creator declaration and current policy under provider plus repository locks, while Radicle execution independently rechecks current identity authority and signer state before mutation. Advisory output was not treated as validation evidence.

## Bounded Claims

Ordinary update refuses conflicts and does not inspect or resolve competing revision contents. Portable actor identity remains a caller declaration. Local signed Radicle mutation does not prove node publication, peer replication, availability, or remote trust. Passing update receipts do not prove gate validity, evidence truth, dependency readiness, source correctness, archive execution, release eligibility, fsync-level crash durability, future authority, or resistance to unrelated processes with equivalent host authority.

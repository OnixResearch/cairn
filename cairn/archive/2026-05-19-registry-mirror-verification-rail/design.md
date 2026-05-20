# Design: registry mirror verification rail

## Context

The evidence chain now reaches exported bundles, bundle verification, import/republish, published indexes, registries, registry selection/fetch plans, channel labels, and manifest-bound registry artifacts. A consumer can prove a registry artifact once they have it locally, but a mirrored/offline directory still needs a single deterministic receipt proving the mirror contains exactly the registry/index graph being consumed.

## Goals

- Provide `cairn evidence registry mirror-verify --root PATH --registry FILE --mirror DIR`.
- Keep the command read-only and filesystem-local.
- Reuse registry verification logic instead of inventing a parallel registry semantics.
- Reopen each declared index through safe `index_relative` paths rooted at the mirror directory.
- Emit stable issue codes for missing/unreadable/tampered registry or index artifacts.
- Keep receipt fields portable: no absolute mirror paths, raw URLs, credentials, or environment values.

## Non-Goals

- No network fetch command in this slice.
- No signing or key management.
- No retention policy or mirror pruning.

## Decisions

### 1. Mirror directory is the trust boundary for file reopening

**Choice:** The verifier accepts the expected registry file path and a mirror directory. It hashes/verifies the registry and then resolves each `index_relative` from the registry against the mirror directory.

**Rationale:** Existing registries already record safe relative index references. Reusing those references lets mirrors be relocatable while preventing absolute/traversal paths.

### 2. Compose existing registry verification

**Choice:** Mirror verification first runs the existing registry verifier and then adds mirror-specific index reachability/hash checks.

**Rationale:** Registry invariants, label validation, duplicate checks, and bundle/index hash checks stay in one implementation path.

### 3. Receipt-only proof

**Choice:** The command prints and optionally writes no artifacts other than its JSON receipt.

**Rationale:** This rail proves mirror integrity without modifying mirrors or executing fetches. Future fetch execution can consume this receipt as a prerequisite.

## Risks

- **Duplicate semantics drift:** mitigated by calling existing registry verification.
- **Path leakage:** mitigated by normalizing `root` and `mirror` fields and using relative issue field paths.
- **Tracey timing:** implementation refs are restored after sync so accepted specs contain the requirement IDs.
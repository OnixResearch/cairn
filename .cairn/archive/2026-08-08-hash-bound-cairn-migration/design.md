## Context

Migration planning sorts destination paths and records source and destination BLAKE3 hashes. Execution currently builds a new plan from live files and copies it without a reviewed-plan argument. The copy shell checks destination presence during planning, but a later `fs::copy` can replace a destination created after that check. Normal lifecycle commands select `.cairn/` without reporting that legacy lifecycle content remains unmigrated.

## Decisions

### Decision: Bind execution to a stable plan hash

**Choice:** Compute `plan_hash` from the source layout, destination layout, sorted actions, conflicts, and policy hash. Exclude preview or execution mode from this projection. Require an exact `--plan-hash` match for `--execute`.

**Rationale:** The same reviewed facts produce the same plan identity. Any source, destination, policy, or action change rejects mutation.

### Decision: Recheck and publish without replacement

**Choice:** Read each source into owned bytes, compare its BLAKE3 hash with the planned hash, and check all destinations before mutation. Publish each file with create-new semantics instead of `fs::copy`.

**Rationale:** Owned bytes close the source read-to-copy gap. Create-new semantics prevent destination replacement after preflight.

### Decision: Fail closed on legacy-only lifecycle roots

**Choice:** Normal lifecycle commands reject a root that has old lifecycle subdirectories and lacks native lifecycle subdirectories. The diagnostic directs the operator to `cairn migrate cairn`.

**Rationale:** A failure is safer than an empty status that can hide active changes. This check does not read legacy content as runtime lifecycle input.

## Risks / Trade-offs

- A host failure during multi-file publication can leave a partial `.cairn/` tree. Create-new semantics prevent overwrite, and the source remains unchanged for recovery.
- Requiring `--plan-hash` is an intentional mutation-contract change.
- The legacy diagnostic must not reject a pointer-only `.cairn/` project configuration when native lifecycle content lives in a registered store.

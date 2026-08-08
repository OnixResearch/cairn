# Radicle Status and Checkout CLI Delta

## ADDED Requirements

### Requirement: Typed read-only status
r[cairn.radicle_status_checkout.status] The system MUST expose read-only status for one uniquely identified typed Cairn change COB using explicit Radicle home and repository inputs without acquiring a signer.

#### Scenario: Status reports reduced state and current observations
r[cairn.radicle_status_checkout.status.typed]
- GIVEN exactly one typed Cairn change COB matches the requested change identifier
- WHEN the operator requests Radicle status
- THEN Cairn MUST report object, state, and semantic content identities when available
- AND Cairn MUST report lifecycle, evidence, current profile authority, policy, and local materialization observations without mutation

#### Scenario: Conflicts remain visible
r[cairn.radicle_status_checkout.status.conflict]
- GIVEN the typed Cairn change COB has concurrent heads for the same artifact
- WHEN the operator requests status
- THEN Cairn MUST report the conflicting artifact identifiers
- AND Cairn MUST NOT emit a partial materialization as checkout-ready

### Requirement: Deterministic checkout plan
r[cairn.radicle_status_checkout.checkout_plan] The system MUST deterministically compare the complete conflict-free materialized COB view with a capability-bounded local active-change snapshot before checkout execution.

#### Scenario: Absent and exact destinations are safe
r[cairn.radicle_status_checkout.checkout_plan.safe]
- GIVEN a conflict-free typed state and a local destination that is absent or byte-equivalent
- WHEN Cairn plans checkout
- THEN an absent destination MUST produce a sorted exact-byte creation plan
- AND a byte-equivalent destination MUST produce an idempotent no-op plan

#### Scenario: Divergent destinations are blocked
r[cairn.radicle_status_checkout.checkout_plan.divergent]
- GIVEN the local destination is partial, divergent, symlinked, non-UTF-8, oversized, or contains unsupported entries
- WHEN Cairn plans checkout
- THEN Cairn MUST report deterministic blockers
- AND Cairn MUST NOT plan an in-place overwrite or partial overlay

### Requirement: Current authority checkout guard
r[cairn.radicle_status_checkout.checkout_authority] The system MUST authorize filesystem checkout mutation from the repository's current canonical identity document, MUST prove possession of the matching profile signer, and MUST recheck the typed object while the local checkout lock is held.

#### Scenario: Current delegate may execute
r[cairn.radicle_status_checkout.checkout_authority.allowed]
- GIVEN the explicit profile actor is a delegate in the current canonical repository identity document
- WHEN Cairn acquires checkout authority
- THEN Cairn MUST prove possession of the matching profile signer before filesystem writes
- AND the guard MUST bind the actor, canonical identity OID, repository ID, object ID, and typed state

#### Scenario: Retired delegate and drift are rejected
r[cairn.radicle_status_checkout.checkout_authority.denied]
- GIVEN the profile actor is not a current delegate or the identity, object, or state changed after preview
- WHEN Cairn attempts checkout execution
- THEN Cairn MUST reject before writing prepared or active-change files

### Requirement: Preview-first exact checkout execution
r[cairn.radicle_status_checkout.checkout_execution] The system MUST expose preview-first `cairn radicle checkout <change>` and MUST require explicit execution before materializing exact native active-change bytes.

#### Scenario: Preview does not mutate
r[cairn.radicle_status_checkout.checkout_execution.preview]
- GIVEN a valid checkout plan
- WHEN the operator omits `--execute`
- THEN Cairn MUST emit the plan without writing locks, prepared files, or active-change files

#### Scenario: Execute prepares and atomically exposes exact bytes
r[cairn.radicle_status_checkout.checkout_execution.atomic]
- GIVEN an authorized drift-free checkout plan with an absent destination
- WHEN the operator executes checkout
- THEN Cairn MUST write and verify every exact byte in a capability-bounded prepared directory
- AND Cairn MUST atomically rename the complete directory into the native active-change destination
- AND the post-checkout scan MUST classify the local destination as byte-equivalent

### Requirement: Status and checkout receipts
r[cairn.radicle_status_checkout.receipts] The system MUST emit deterministic BLAKE3-bound receipts for status and checkout that identify repository, object, state, content, actor, canonical identity, policy observation, lifecycle/evidence condition, local classification, file hashes, blockers, execution mode, and mutation result.

#### Scenario: Blocked checkout is explicit and non-mutating
r[cairn.radicle_status_checkout.receipts.blocked]
- GIVEN checkout is conflicted, unauthorized, divergent, or drifted
- WHEN Cairn emits its receipt
- THEN the receipt MUST report deterministic blockers and no successful mutation
- AND the portable receipt hash MUST exclude the Radicle home path

### Requirement: Bounded status and checkout claims
r[cairn.radicle_status_checkout.non_claims] The system MUST state that local typed status and checkout do not prove network publication, peer availability, remote trust, gate validity, evidence truth, dependency readiness, future authority, source correctness, archive execution, or release eligibility.

#### Scenario: Local materialization is not lifecycle approval
r[cairn.radicle_status_checkout.non_claims.local_only]
- GIVEN a passing local status or checkout receipt
- WHEN Cairn documents the result
- THEN Cairn MUST limit the claim to the stated local typed reduction and materialization observations
- AND Cairn MUST NOT claim peer synchronization, lifecycle approval, or resistance to unrelated processes with equivalent host filesystem authority

## Context

Previous deterministic slices established portable replay hashes, policy-owned replay cases/groups, agent replay evidence, and policy-owned entropy inventory. The remaining gap is not another replay case: it is a compact reviewed statement of the environment facts under which replay ran.

## Goals

- Emit a stable `environment_attestation` object in every replay receipt.
- Drive required/disclosed/rejected attestation facts from generated policy.
- Keep attestation portable across checkout roots and temporary directories.
- Avoid raw environment values, host paths, usernames, or secrets.
- Let agent receipts optionally bind to the attestation via a hash.

## Non-Goals

- Full SLSA/provenance implementation.
- Capturing complete Nix store closure contents.
- Trusting arbitrary host environment variables.
- Making agent receipt attestation evidence mandatory in this slice.

## Decisions

### 1. Policy-owned fact inventory

**Choice:** Add `environment_attestation` entries to `determinism_replay_policy` with stable `id`, `class`, `source`, `handling`, `receipt_field`, and `description` fields.

**Rationale:** The reviewed policy should say which environment facts matter. Rust code should render/verify the policy, not hide the contract.

### 2. Stable attestation receipt object

**Choice:** The replay receipt gets `environment_attestation` containing sorted policy facts plus normalized observed fields: `cairn_version`, `policy_hash`, `entropy_policy_hash`, `selected_cases`, `selected_groups`, `replay_scope`, `attestation_policy_hash`, and `attestation_hash`.

**Rationale:** Agents need one object/hash to bind, while preserving existing top-level replay fields.

### 3. Secret-safe rendering

**Choice:** Disclosed facts describe identities and hashes, not raw env values or absolute store/checkout paths. Build/toolchain context remains policy-disclosed unless already represented by stable package version/hash fields.

**Rationale:** Deterministic receipts are durable artifacts and must not become secret or host fingerprint dumps.

### 4. Optional agent binding

**Choice:** Agent receipts may include `replay_environment_attestation_hash`; validation checks its shape and reports it when present, but does not require it yet.

**Rationale:** This preserves compatibility with existing wrappers while providing the next stronger binding for agents/CI.

## Risks / Trade-offs

- **Policy hash churn:** New policy fields will require regenerating fixtures/goldens. Mitigate with focused fixture update scripts and full replay tests.
- **Overclaiming environment proof:** The attestation is scoped to reviewed deterministic facts, not complete supply-chain provenance. Docs must state this clearly.
- **Tracey/spec sync drift:** Sync can rewrite accepted spec sections; verify Tracey after sync and repair refs if needed.

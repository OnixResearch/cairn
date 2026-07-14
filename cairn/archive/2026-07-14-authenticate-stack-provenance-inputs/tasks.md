## Phase 1: Measured evidence inputs

- [x] [serial] Define typed measured-byte inputs for artifacts, sidecars, receipts, graph summaries, and policy exports. r[cairn.stack_provenance_gates.authenticated_inputs.measurement]
- [x] [serial] Implement shell-owned BLAKE3 measurement and canonical receipt identity recomputation. r[cairn.stack_provenance_gates.authenticated_inputs.receipt_identity]
- [x] [serial] Refactor readiness cores to compare measured typed identities rather than trusting caller-supplied digest fields. r[cairn.stack_provenance_gates.authenticated_inputs.core]

## Phase 2: Producer and policy authentication

- [x] [serial] Add explicit trusted-producer signature policy and bounded shell verification observations. r[cairn.stack_provenance_gates.authenticated_inputs.signature_policy]
- [x] [serial] Generate accepted policy identities from canonical Nickel exports and reject placeholders. r[cairn.stack_provenance_gates.authenticated_inputs.policy_identity]
- [x] [serial] Make authenticated stack-provenance and function-address gates required for the Onix release profile. r[cairn.stack_provenance_gates.authenticated_inputs.required_release_profile]

## Phase 3: Fixtures and documentation

- [x] [parallel] Add positive fixtures with matching measured bytes, canonical receipt identities, current policy identities, and authorized signatures. r[cairn.stack_provenance_gates.authenticated_inputs.fixtures.positive]
- [x] [parallel] Add negative fixtures for fabricated hashes, tampered bytes, wrong domains, stale policy, placeholders, missing signatures, wrong purpose, unauthorized signer, and revoked keys. r[cairn.stack_provenance_gates.authenticated_inputs.fixtures.negative]
- [x] [parallel] Document trust-root inputs, measured-byte boundaries, release-profile requirements, migrations, and non-claims. r[cairn.stack_provenance_gates.authenticated_inputs.docs]

## Phase 4: Verification

- [x] [serial] Require the archived Valence `harden-preserves-integrity-boundaries` receipt before accepting imported corrected-profile evidence. r[cairn.stack_provenance_gates.authenticated_inputs.cross_repo_dependency]
- [x] [serial] Run focused positive and negative core, CLI, policy, and signature tests. r[cairn.stack_provenance_gates.authenticated_inputs.final_validation]
- [x] [serial] Add a checked-in CI workflow that runs only `nix flake check`. r[cairn.stack_provenance_gates.authenticated_inputs.flake_check_ci]
- [x] [serial] Run `nix flake check`, Cairn validation/gates, and release-readiness negative probes before sync and archive. r[cairn.stack_provenance_gates.authenticated_inputs.final_validation]

## ADDED Requirements

### Requirement: Octet second MIR selected-fixture receipt contract

r[content_addressed_rust_evidence.mir_second_fixture] Cairn MUST define downstream receipt expectations for a second Octet MIR selected-fixture shape while preserving default and non-fixture MIR as unsupported planned evidence.

#### Scenario: Selector is closed and exact

r[content_addressed_rust_evidence.mir_second_fixture.closed_selector]

- GIVEN Octet exposes a second selected MIR fixture shape
- WHEN a downstream lifecycle package or validator references that MIR fixture evidence
- THEN the evidence MUST identify exactly one closed fixture selector value
- AND unsupported shapes and aliases MUST be rejected deterministically rather than coerced or treated as broader MIR support.

#### Scenario: Receipt identity binds fixture, object, and toolchain

r[content_addressed_rust_evidence.mir_second_fixture.receipt_identity]

- GIVEN a receipt for the second selected MIR fixture shape
- WHEN Cairn or a downstream consumer validates the receipt contract
- THEN the receipt MUST bind `family=mir`, selected fixture shape, fixture path, fixture function, linked Function Object v1 hash, rustc/toolchain identity, extractor identity, and support status.

#### Scenario: Replay evidence includes positive and negative controls

r[content_addressed_rust_evidence.mir_second_fixture.replay_evidence]

- GIVEN the second selected MIR fixture is implemented in Octet
- WHEN the evidence is reviewed for downstream use
- THEN the package MUST identify replayable positive evidence for inventory, emit/check roundtrip, fixture metadata, object-hash linkage, and aggregate Valence coverage.

#### Scenario: Fail-closed boundaries are preserved

r[content_addressed_rust_evidence.mir_second_fixture.fail_closed]

- GIVEN unsupported shapes, selector aliases, tampered fixture/object linkage, or default/non-fixture MIR evidence
- WHEN the evidence is emitted, checked, or replayed
- THEN unsupported or tampered material MUST fail closed or remain `unsupported-planned` with deterministic diagnostics.

#### Scenario: MIR fixture evidence remains non-proof material

r[content_addressed_rust_evidence.mir_second_fixture.non_claims]

- GIVEN a selected MIR fixture receipt validates
- WHEN Cairn renders, validates, republishes, or references that receipt
- THEN the receipt MUST NOT claim semantic equivalence, program correctness, compiler-version-independent identity, panic freedom, unsafe freedom, drop behavior proof, async behavior proof, or exhaustive MIR coverage.

## Phase 1: Cairn planning package

- [x] [serial] I1 Create the native Cairn proposal, design, tasks, and delta spec for the second Octet MIR selected-fixture receipt contract. r[content_addressed_rust_evidence.mir_second_fixture.closed_selector] r[content_addressed_rust_evidence.mir_second_fixture.receipt_identity]
- [x] [serial] V1 Validate and gate the planning package. command=`nix run path:$PWD#cairn -- validate --root .`; input=`cairn/changes/add-octet-second-mir-fixture-receipt/`; asserts=`active package validates and proposal/design/tasks gates return valid=true/verdict=PASS`; evidence=`terminal transcript` r[content_addressed_rust_evidence.mir_second_fixture.replay_evidence]

## Phase 2: Octet implementation drain

- [ ] [serial] I2 Add the second exact MIR fixture selector in Octet parser, DTO, inventory, emit/check, docs, and object-corpus fixture surfaces. r[content_addressed_rust_evidence.mir_second_fixture.closed_selector] r[content_addressed_rust_evidence.mir_second_fixture.receipt_identity]
- [ ] [parallel] V2 Add positive Octet evidence for the new selector, including fixture inventory, emit/check roundtrip, object-hash linkage, fixture metadata, and aggregate Valence gate coverage. r[content_addressed_rust_evidence.mir_second_fixture.replay_evidence]
- [ ] [parallel] V3 Add negative Octet evidence for unsupported shapes, alias rejection, tampered fixture/object linkage, and default/non-fixture MIR remaining `unsupported-planned`. r[content_addressed_rust_evidence.mir_second_fixture.fail_closed]
- [ ] [serial] V4 Sync/archive the Cairn package only after Octet implementation evidence is linked or summarized for downstream replay. r[content_addressed_rust_evidence.mir_second_fixture.non_claims]

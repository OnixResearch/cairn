## Phase 1: Producer and consumer contract

- [x] [serial] Implement the bounded source/claim/policy DTOs and deterministic logical identities, preserving exact source and claim-root linkage. r[cairn.source_review.contract]
- [x] [serial] Implement external signing-request bytes, detached Ed25519 assembly, explicit currentness/author policy, distinct-key approval counting and authenticated rejection attribution. r[cairn.source_review.authentication]
- [x] [serial] Expose the read-only bounded capability-confined CLI without private keys, signing, or implicit trust. r[cairn.source_review.shell]

## Phase 2: Behavioral verification and documentation

- [x] [serial] Exercise positive and negative core cases and the actual operator signing-request/assemble/verify CLI; record exact source inputs, commands and results. r[cairn.source_review.contract] r[cairn.source_review.authentication] r[cairn.source_review.shell]
- [x] [serial] Document the producer/consumer workflow, compatibility boundary and required non-claims; retain test-key disclaimers and no release/witness authority. r[cairn.source_review.boundary]
- [x] [serial] Run package formatting, full workspace lib/tests, strict Clippy, policy freshness, native validation and all change gates; then sync/archive and record post-archive validation before committing and publishing the verified branch. r[cairn.source_review.contract] r[cairn.source_review.authentication] r[cairn.source_review.shell] r[cairn.source_review.boundary]

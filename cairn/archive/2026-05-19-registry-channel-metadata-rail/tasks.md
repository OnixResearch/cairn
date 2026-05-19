## Phase 1: Spec and policy shape

- [x] [serial] Define registry channel metadata requirements and selectors. r[registry_channel_metadata.labels]
- [x] [serial] Add policy-owned label constraints and public receipt schema coverage. r[registry_channel_metadata.policy]

## Phase 2: Implementation

- [x] [serial] Implement registry build label flags, validation, entry binding, and duplicate conflict checks. r[registry_channel_metadata.build]
- [x] [serial] Implement select/fetch-plan label selectors and portable receipts. r[registry_channel_metadata.consume]
- [x] [serial] Update docs, tests, fixtures, and generated policy. r[registry_channel_metadata.verify]

## Phase 3: Verification and archive

- [x] [serial] Run targeted registry-channel tests and Rust/Nickel gates. r[registry_channel_metadata.verify]
- [x] [serial] Sync/archive the change, rerun final gates, commit, and push. r[registry_channel_metadata.verify]

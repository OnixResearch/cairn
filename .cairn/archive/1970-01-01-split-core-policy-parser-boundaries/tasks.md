## Phase 1: Implementation

- [x] I1 [serial] Map the existing core policy parser responsibilities and public API surface. r[developer_experience.core_policy_parser_boundary_split.map]
- [x] I2 [serial] Split the monolithic policy parser into focused modules behind the same public facade. r[developer_experience.core_policy_parser_boundary_split.modules]
- [x] I3 [serial] Preserve policy parsing behavior, Steel profile validation, and error variants/messages. r[developer_experience.core_policy_parser_boundary_split.behavior]

## Phase 2: Verification

- [x] V1 [serial] Run focused policy parser tests. r[developer_experience.core_policy_parser_boundary_split.tests]
- [x] V2 [serial] Run Cairn validation, Tracey coverage, diff checks, and the final workspace gate before archive. r[developer_experience.core_policy_parser_boundary_split.validation]

## Context

The evidence chain now supports export, verify, import, republish, index, registry build/verify, and registry select/fetch-plan. Registry entries are selected through stable structural data such as bundle kind, policy hash, index hash, artifact role/hash, and bundle receipt hash. That is deterministic but awkward for agents that need a reviewed operational target like `stable` or a release date.

## Goals

- Add stable registry labels that are authored through CLI inputs and constrained by policy.
- Preserve deterministic ordering, duplicate detection, and portable receipt hashes.
- Allow select/fetch-plan to filter by channel, release, platform, and compatibility class.
- Reject invalid labels and conflicting channel entries deterministically.

## Non-Goals

- No network fetching or mirror URL handling.
- No credential or secret labels.
- No cryptographic signing changes.
- No Python or shell helper scripts.

## Decisions

### 1. Labels are explicit registry metadata

**Choice:** Registry build accepts optional `--channel`, `--release`, `--platform`, and `--compatibility-class` flags and stores those labels on each registry entry.

**Rationale:** Labels belong to the registry catalog layer rather than the raw bundle. A published index may appear in different registries with different operational labels while preserving the underlying bundle/index hashes.

**Alternative:** Put labels into evidence indexes. Rejected for this slice because index semantics are currently bundle-publication facts; channel assignment is a registry/operator decision.

### 2. Policy constrains label values

**Choice:** Nickel policy defines allowed label sets and simple release syntax constraints. The CLI loads generated policy and validates label values before writing receipts.

**Rationale:** Agents and CI need labels that are reviewable data, not hidden Rust literals or ad hoc strings.

### 3. Duplicate channel conflicts are deterministic

**Choice:** Registry build rejects multiple entries that declare the same non-empty channel/release/platform/compatibility tuple with different index hashes.

**Rationale:** A single selector should not silently choose between conflicting entries. Empty labels remain allowed for structural-only registries.

### 4. Consumers use the same label selectors

**Choice:** Registry select/fetch-plan add `--channel`, `--release`, `--platform`, and `--compatibility-class` selectors. `select` still requires exactly one match; `fetch-plan` may emit multiple matches.

**Rationale:** This composes with existing selection/fetch-plan semantics and preserves deterministic ambiguity handling.

## Risks / Trade-offs

- **Policy/schema churn:** public receipt schemas and fixture hashes will change. Mitigation: regenerate generated policy, update fixture hashes intentionally, run public receipt tests.
- **Overconstraining labels:** only stable high-value labels are added now. Future labels can be additive policy keys.
- **Channel ambiguity:** reject duplicate conflicting channel tuples at build time and preserve select-time ambiguity detection.
# ironspec-supersession delta: Repo-owned traceability rail

## ADDED Requirements

### Requirement: Authoritative repo-owned traceability checker
r[cairn.supersede.traceability_ownership.authoritative_checker]
Cairn MUST evaluate accepted requirement coverage with a repo-owned deterministic checker that does not invoke the external Tracey binary, Tracey daemon, network, clock, or ambient process state.

#### Scenario: Complete coverage passes
r[cairn.supersede.traceability_ownership.authoritative_checker.complete]
- GIVEN an in-memory traceability profile, accepted requirement markers, and implementation or verification markers for every accepted requirement
- WHEN Cairn classifies coverage
- THEN the result MUST report zero missing requirement IDs, zero dangling reference IDs, and `valid=true`.

#### Scenario: Missing implementation or verification fails
r[cairn.supersede.traceability_ownership.authoritative_checker.missing]
- GIVEN an accepted `r[...]` requirement without a matching `r[impl ...]` or `r[verify ...]` marker in the configured evidence roots
- WHEN Cairn classifies coverage
- THEN the result MUST report the requirement ID in the missing collection and `valid=false`.

#### Scenario: Dangling reference fails
r[cairn.supersede.traceability_ownership.authoritative_checker.dangling]
- GIVEN a configured evidence root containing an `r[impl ...]`, `r[verify ...]`, `r[depends ...]`, or `r[related ...]` marker for an unknown requirement ID
- WHEN Cairn classifies coverage
- THEN the result MUST report the reference ID in the dangling collection and `valid=false`.

### Requirement: Traceability coverage command
r[cairn.supersede.traceability_ownership.command]
Cairn MUST expose `cairn traceability coverage --root <path>` as the maintained operator command for requirement traceability coverage and MUST preserve `cairn tracey coverage --root <path>` as a compatibility alias until an explicit deprecation change removes it.

#### Scenario: Maintained command emits deterministic evidence
r[cairn.supersede.traceability_ownership.command.maintained]
- GIVEN a repository root with a valid traceability profile and source-controlled requirement/evidence files
- WHEN an operator runs `cairn traceability coverage --root <path> --json`
- THEN Cairn MUST emit deterministic JSON containing the selected profile, requirement count, referenced count, missing IDs, dangling IDs, verdict, input hash, policy hash, and receipt hash.

#### Scenario: Compatibility alias preserves behavior
r[cairn.supersede.traceability_ownership.command.alias]
- GIVEN a repository root accepted by the maintained traceability command
- WHEN an operator runs `cairn tracey coverage --root <path> --json`
- THEN Cairn MUST evaluate the same repo-owned checker and emit an equivalent coverage verdict without invoking external Tracey.

### Requirement: Nickel-owned traceability profiles
r[cairn.supersede.traceability_ownership.configured_profiles]
Cairn MUST define traceability scan profiles in reviewed Nickel policy data or repo-local Nickel configuration and MUST export checked deterministic runtime profile data before the CLI trusts it.

#### Scenario: Default Cairn profile preserves current roots
r[cairn.supersede.traceability_ownership.configured_profiles.cairn_default]
- GIVEN the default Cairn repository profile
- WHEN Cairn loads traceability configuration
- THEN the profile MUST scan accepted specs under `cairn/specs` and implementation or verification markers in Rust and Nickel source roots used by the current maintained gate.

#### Scenario: Trellis-style profile is representable
r[cairn.supersede.traceability_ownership.configured_profiles.trellis]
- GIVEN a downstream repository whose requirements live in `docs/requirements.md` and evidence markers live in `src/**/*.rs` plus `tests/**/*.rs`
- WHEN maintainers author a Nickel traceability profile for that repository shape
- THEN Cairn MUST represent that profile without requiring `.config/tracey/config.styx` or external Tracey evaluation.

#### Scenario: Malformed profile fails closed
r[cairn.supersede.traceability_ownership.configured_profiles.malformed]
- GIVEN a traceability profile with an empty identifier, unsupported marker verb, unsafe path, or unsupported source kind
- WHEN policy export or CLI profile loading validates the profile
- THEN Cairn MUST reject the profile with deterministic diagnostics before scanning source files.

### Requirement: External Tracey is optional developer tooling
r[cairn.supersede.traceability_ownership.optional_tracey]
Cairn MUST NOT require external Tracey for maintained CI gates, release-readiness evidence, Nix checks, or authoritative coverage receipts, and MAY document external Tracey only as optional developer diagnostics.

#### Scenario: Maintained gates do not require Tracey
r[cairn.supersede.traceability_ownership.optional_tracey.gates]
- GIVEN a checkout where the external `tracey` binary is absent
- WHEN maintained Cairn validation, release-readiness, and Nix traceability checks run
- THEN those checks MUST use the repo-owned Cairn traceability command and MUST NOT fail solely because external Tracey is unavailable.

#### Scenario: Docs avoid overclaiming Tracey coverage
r[cairn.supersede.traceability_ownership.optional_tracey.docs]
- GIVEN operator documentation mentions optional external Tracey commands
- WHEN the documentation describes maintained release evidence
- THEN it MUST state that the authoritative gate is Cairn-owned traceability coverage and that marker coverage does not prove behavioral correctness.

### Requirement: Traceability rail validation
r[cairn.supersede.traceability_ownership.validation]
Cairn MUST validate the repo-owned traceability rail with positive and negative tests, policy checks, lifecycle gates, and repository verification before archive.

#### Scenario: Positive and negative fixtures cover the rail
r[cairn.supersede.traceability_ownership.validation.fixtures]
- GIVEN complete, missing-reference, dangling-reference, malformed-profile, alias-compatibility, and Trellis-style fixture repositories or snapshots
- WHEN focused traceability tests run
- THEN passing fixtures MUST emit valid receipts and failing fixtures MUST emit deterministic diagnostics without panics or mutation.

#### Scenario: Repository gates prove integration
r[cairn.supersede.traceability_ownership.validation.gates]
- GIVEN the implementation and documentation updates are complete
- WHEN maintainers run policy export, formatting, workspace tests, Clippy, Cairn validation/gates, traceability coverage, Slotcar, release-readiness, and flake check
- THEN those checks MUST pass before the change is synced and archived.

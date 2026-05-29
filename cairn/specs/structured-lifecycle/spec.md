# Structured Lifecycle Specification

## Purpose

Defines structured lifecycle authoring, canonical runtime bundles, Markdown projection, first-class probes, and bounded format strategy for Cairn lifecycle artifacts.

## Requirements

### Requirement: Typed lifecycle authoring
r[cairn.structured_lifecycle.typed_authoring] The system MUST support typed lifecycle source for specs, changes, requirements, scenarios, tasks, probes, evidence declarations, and format metadata.

#### Scenario: Nickel contracts validate structured source
r[cairn.structured_lifecycle.typed_authoring.nickel_contracts]
- GIVEN a repository contains structured lifecycle source
- WHEN the operator runs lifecycle source export or check
- THEN Cairn MUST validate the source against Nickel contracts for required fields, stable IDs, duplicate-free IDs, normative requirement strength, scenario clauses, task links, and probe shape.

#### Scenario: Rust revalidates generated lifecycle data
r[cairn.structured_lifecycle.typed_authoring.rust_revalidation]
- GIVEN generated lifecycle data is loaded by Cairn runtime commands
- WHEN the data is missing required fields, contains duplicate IDs, contains invalid requirement links, or contains invalid probe declarations
- THEN Cairn MUST reject the generated data with deterministic issue codes before trusting it.

### Requirement: Canonical lifecycle runtime bundle
r[cairn.structured_lifecycle.runtime_bundle] The system MUST consume a generated canonical lifecycle bundle for runtime lifecycle validation without invoking Nickel during normal runtime commands.

#### Scenario: Runtime commands remain Nickel-free
r[cairn.structured_lifecycle.runtime_bundle.nickel_free_runtime]
- GIVEN a repository has already generated its canonical lifecycle bundle
- WHEN Cairn validates, gates, renders receipts, syncs, archives, or plans migration
- THEN runtime evaluation MUST use loaded canonical lifecycle data and MUST NOT execute Nickel as part of the trusted runtime path.

#### Scenario: Stale or tampered generated bundle fails closed
r[cairn.structured_lifecycle.runtime_bundle.stale_generated_bundle]
- GIVEN structured lifecycle source and generated lifecycle data disagree
- WHEN the operator runs lifecycle export with freshness checking or a gate that requires fresh structured data
- THEN Cairn MUST report the stale or tampered bundle with deterministic diagnostics and MUST NOT treat the generated data as authoritative.

#### Scenario: Bundle hash participates in receipts
r[cairn.structured_lifecycle.runtime_bundle.receipt_hash_binding]
- GIVEN Cairn emits a receipt for structured lifecycle validation, gate evaluation, probe execution, migration, import, or render behavior
- WHEN the receipt is materialized
- THEN the receipt MUST include or bind the canonical lifecycle bundle hash and the policy hash when policy participates.

### Requirement: Markdown projection and import
r[cairn.structured_lifecycle.markdown_projection] The system MUST treat Markdown lifecycle artifacts as projection or compatibility input once structured lifecycle source exists.

#### Scenario: Rendered Markdown is a view
r[cairn.structured_lifecycle.markdown_projection.render_view]
- GIVEN structured lifecycle source exists for a spec or change
- WHEN the operator renders Markdown output
- THEN Cairn MUST produce deterministic Markdown as a human-readable view and MUST NOT make the rendered Markdown the authoritative lifecycle source.

#### Scenario: Markdown import reports lossy conversions
r[cairn.structured_lifecycle.markdown_projection.lossy_import]
- GIVEN an existing Markdown lifecycle artifact contains ambiguous prose, unknown markers, malformed scenarios, or unsupported verification references
- WHEN the operator imports it into structured lifecycle source
- THEN Cairn MUST preserve source metadata and report every lossy or ambiguous conversion with deterministic issue codes.

#### Scenario: Existing Markdown workflows remain compatible during migration
r[cairn.structured_lifecycle.markdown_projection.compatibility]
- GIVEN a repository has not migrated to structured lifecycle source
- WHEN the operator runs existing Cairn validation or gate commands
- THEN Cairn SHOULD preserve existing Markdown behavior while emitting migration guidance where policy enables it.

### Requirement: First-class probe evidence
r[cairn.structured_lifecycle.probes] The system MUST support first-class probes that bind requirements to deterministic executable evidence and explicit oracles.

#### Scenario: Probe declares command, fixtures, and assertions
r[cairn.structured_lifecycle.probes.declared_oracle]
- GIVEN a behavioral requirement references a probe
- WHEN Cairn validates the probe declaration
- THEN the probe MUST declare requirement IDs, fixture inputs, command shape, expected status class, assertion operators, evidence role, and coverage class.

#### Scenario: Probe execution emits deterministic receipt
r[cairn.structured_lifecycle.probes.execution_receipt]
- GIVEN a valid probe is selected for execution
- WHEN Cairn runs or replays the probe through an authorized execution surface
- THEN Cairn MUST emit a deterministic receipt with probe ID, requirement IDs, coverage class, command identity, output or artifact hashes, assertion results, issue codes, bundle hash, policy hash when applicable, and receipt hash.

#### Scenario: Positive and negative probes are distinguishable
r[cairn.structured_lifecycle.probes.positive_negative]
- GIVEN a requirement has executable probe evidence
- WHEN Cairn summarizes evidence strength for gates or release readiness
- THEN Cairn MUST distinguish positive, negative, malformed-input, boundary, and regression coverage rather than counting all probe links as equal.

#### Scenario: Probe assertions use data predicates first
r[cairn.structured_lifecycle.probes.data_predicates_first]
- GIVEN a probe assertion can be expressed as a receipt field comparison, issue-code check, exit-status class, artifact existence check, or BLAKE3 file-hash check
- WHEN the assertion is authored
- THEN Cairn SHOULD represent the assertion as structured data rather than embedded executable code.

### Requirement: Format strategy and executable DSL boundaries
r[cairn.structured_lifecycle.format_strategy] The system MUST keep lifecycle authoring, canonical encoding, and optional executable probe logic behind separate trust boundaries.

#### Scenario: Nickel contracts own authoring validation
r[cairn.structured_lifecycle.format_strategy.nickel_authoring]
- GIVEN a new structured lifecycle artifact is authored
- WHEN the artifact is checked before runtime consumption
- THEN Nickel contracts MUST validate the authoring shape and defaults before canonical data is exported.

#### Scenario: Generated JSON is the initial runtime interchange
r[cairn.structured_lifecycle.format_strategy.generated_json]
- GIVEN Cairn introduces structured lifecycle runtime data
- WHEN the first implementation exports canonical lifecycle data
- THEN generated JSON SHOULD remain the initial runtime interchange format unless a stronger canonical format has proven equivalent positive and negative behavior.

#### Scenario: Preserves is a bounded canonical encoding candidate
r[cairn.structured_lifecycle.format_strategy.preserves_candidate]
- GIVEN Cairn evaluates Preserves for lifecycle or receipt encoding
- WHEN Preserves is added to the structured lifecycle path
- THEN Cairn MUST prove byte-stable hash material, diagnostic mapping to source IDs, compatibility with receipt verification, and equivalent positive and negative fixtures before making it authoritative.

#### Scenario: Steel is not lifecycle source of truth
r[cairn.structured_lifecycle.format_strategy.steel_not_source]
- GIVEN Steel or another executable DSL is available
- WHEN lifecycle source is authored, validated, gated, synced, archived, or hashed
- THEN Cairn MUST NOT treat executable DSL code as the base lifecycle source of truth.

#### Scenario: Steel probe DSL is optional and bounded
r[cairn.structured_lifecycle.format_strategy.steel_probe_dsl]
- GIVEN data-predicate probe assertions are insufficient for a reviewed probe
- WHEN Steel-based probe logic is enabled
- THEN Cairn MUST require explicit policy enablement, deterministic budgets, capability declarations, redaction rules, Rust-owned authorization, and deterministic receipts before executing the DSL.

### Requirement: Structured lifecycle migration rails
r[cairn.structured_lifecycle.migration] The system MUST provide deterministic migration rails from Markdown-owned lifecycle content to structured lifecycle source.

#### Scenario: Migration plan is preview-first
r[cairn.structured_lifecycle.migration.preview_first]
- GIVEN a repository contains Markdown lifecycle artifacts
- WHEN the operator requests structured lifecycle migration
- THEN Cairn MUST emit a dry-run plan by default with source paths, destination paths, source hashes, generated artifact hashes, lossy-conversion issues, and blockers.

#### Scenario: Executed migration writes a mutation manifest
r[cairn.structured_lifecycle.migration.mutation_manifest]
- GIVEN a structured lifecycle migration plan is unblocked and the operator requests execution
- WHEN Cairn writes structured lifecycle files or generated bundles
- THEN Cairn MUST record a mutation manifest with before and after BLAKE3 content hashes for every written path.

#### Scenario: Migration does not silently delete Markdown
r[cairn.structured_lifecycle.migration.no_silent_delete]
- GIVEN Cairn migrates Markdown lifecycle artifacts to structured source
- WHEN the migration executes
- THEN Cairn MUST NOT delete original Markdown artifacts unless a later explicit policy and command authorize removal with its own receipt.

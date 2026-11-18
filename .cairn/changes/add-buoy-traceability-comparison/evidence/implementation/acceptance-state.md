# Buoy comparison acceptance state

Status: **core evaluator verified; one CLI identity test outstanding.**

## Verified

- `cargo test -p cairn-core --lib traceability_comparison` → **3/3 pass**.
- `cargo test -p cairn-cli --test buoy_comparison` → **3/4 pass**
  (`comparison_reports_executed_native_observations_and_declared_upstream`,
  `comparison_rejects_restated_observations_and_unsupported_pass`,
  `comparison_withholds_when_declared_only_cases_remain`).
- Two compile defects were fixed during acceptance: `domain_hash_json` was passed
  `&Vec<Value>` instead of a `Value` in the report-material assembly.
- A real logic defect was fixed: `restated_observation_rejected` and
  `restated_native_expectation_rejected` were both rendered as `issues.is_empty()`,
  i.e. inverted and shared. They are now derived independently from the presence
  of `comparison.restated_observation_accepted` and
  `comparison.restated_native_expectation_accepted`, so a rejected restatement
  reports `true`.

## Outstanding

`comparison_identity_binds_source_mapping_fixture_and_policy` mutates the
manifest's `upstream.revision` and expects the comparison to succeed with a
different `comparison_identity`. The command instead rejects the mutated manifest
(`valid: false`, `verdict: fail`), so `assert_success` fails.

No policy-level revision pin was found: the pinned revision appears only in the
test fixture and a core test helper, and `cairn-policy` carries no expected
revision. The rejection therefore comes from another admission rule — most likely
a manifest self-identity or fixture-set check — and must be identified before
deciding whether the implementation or the test expectation is wrong.

Scope reminder recorded in the receipt and docs: upstream was **not executed**;
`upstream.executed=false`, `artifact_available` reflects the declared artifact,
the disposition is `blocked`, and `complete=false`. All upstream observations are
declared fixtures. No archive, sync or completion claim is made here.

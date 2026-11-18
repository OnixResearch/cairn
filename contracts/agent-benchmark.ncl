# Typed authoring shapes for saved observations. Rust admission owns cross-field rules,
# trace completeness, uniqueness, cohort compatibility and checked arithmetic.
let Digest = std.contract.from_predicate (fun value =>
  std.is_string value && std.string.is_match "^[0-9a-f]{64}$" value
) in
let Scope = std.contract.from_predicate (fun value =>
  std.is_string value && std.string.is_match "^[a-z][a-z0-9._-]{0,95}$" value
) in
let Natural = std.contract.from_predicate (fun value =>
  std.is_number value && value == std.number.floor value && value >= 0
) in
let Bounded = fun maximum => std.contract.from_predicate (fun value =>
  std.is_number value && value == std.number.floor value && value > 0 && value <= maximum
) in
let OneOf = fun alternatives => std.contract.from_predicate (fun value =>
  std.is_string value && std.array.any (fun alternative => alternative == value) alternatives
) in
let Nullable = fun Contract => std.contract.custom (fun label value =>
  if value == null then 'Ok value else std.contract.check Contract label value
) in
{
  Bounds = {
    max_repetitions | Bounded 32,
    max_calls_per_run | Bounded 256,
    max_input_bytes | Bounded 1048576,
    max_scope_bytes | Bounded 96,
    max_event_count | Bounded 1000000,
    max_token_count | Bounded 1000000000000000000,
    max_duration_ms | Bounded 3600000,
  },
  Policy = {
    schema | OneOf ["cairn.agent-benchmark.policy.v1"],
    bounds | Bounds,
    non_claims | Array String,
  },
  Accounting = {
    method | OneOf ["provider_usage", "exact_tokenizer"],
    # For provider usage this names the immutable usage source; for tokenizer
    # usage these identify its immutable implementation revision and encoding.
    usage_source | optional | Nullable Digest,
    implementation_revision | optional | Nullable Digest,
    encoding | optional | Nullable Digest,
  },
  Cohort = {
    source_projection | Digest,
    product_revision | Digest,
    policy | Digest,
    fixture | Digest,
    oracle | Digest,
    model | Digest,
    agent_configuration | Digest,
    catalog | Digest,
    presentation | Digest,
    accounting | Accounting,
  },
  Usage = {
    uncached_input | Natural,
    cached_input | Natural,
    output | Natural,
    reasoning | optional | Nullable Natural,
    reasoning_inclusion | OneOf ["included_in_output", "additional_to_output"],
  },
  Call = {
    call_id | Digest,
    sequence | Natural,
    usage | optional | Nullable Usage,
    # Exact exposed content identity is mandatory for external tokenization.
    content_identity | optional | Nullable Digest,
  },
  Trace = {
    declared_calls | Natural,
    complete | Bool,
    calls | Array Call,
    retry_count | Natural,
    tool_call_count | Natural,
    repeated_read_count | Natural,
    duration_ms | Natural,
  },
  OracleObservation = {
    verdict | OneOf ["completed", "correctly_blocked", "rejected", "inconclusive"],
    agent_claimed_completion | Bool,
  },
  Run = {
    run_id | Digest,
    cohort | Cohort,
    trace | Trace,
    oracle | OracleObservation,
  },
  Scenario = {
    scope | Scope,
    mode | OneOf ["representation_replay", "live_host"],
    repetitions | Bounded 32,
    max_calls_per_run | Bounded 256,
  },
  Input = {
    schema | OneOf ["cairn.agent-benchmark.input.v1"],
    scenario | Scenario,
    runs | Array Run,
  },
  Comparison = {
    schema | OneOf ["cairn.agent-benchmark.comparison.v1"],
    allowed_candidate_differences | Array (OneOf ["product_revision", "source_projection", "agent_configuration", "catalog", "presentation"]),
    baseline | Input,
    candidate | Input,
  },
  # Output schemas describe redacted results only; raw transcript content has no field.
  Totals = {
    uncached_input | Natural,
    cached_input | Natural,
    output | Natural,
    reasoning | Natural,
    reasoning_complete | Bool,
    billed_output | Natural,
    billed_total | Natural,
    retry_count | Natural,
    tool_call_count | Natural,
    repeated_read_count | Natural,
    duration_ms | Natural,
    complete_cost | Bool,
  },
  Report = {
    schema | OneOf ["cairn.agent-benchmark.report.v1"],
    input_hash | Digest,
    policy_hash | Digest,
    scope | Scope,
    mode | OneOf ["representation_replay", "live_host"],
    repetitions | Bounded 32,
    cohort_hash | Digest,
    product_revision | Digest,
    attempted_runs | Natural,
    outcomes | { completed | Natural, correctly_blocked | Natural, false_completion | Natural, failed | Natural, incomplete | Natural },
    totals | Totals,
    runs | Array {
      run_id | Digest,
      outcome | OneOf ["completed", "correctly_blocked", "false_completion", "failed", "incomplete"],
      cost_available | Bool,
      totals | Totals,
    },
    tokens_per_correct_completion | optional | Nullable { attempted_tokens | Natural, correct_completions | Natural },
    tokens_per_correct_block | optional | Nullable { attempted_tokens | Natural, correct_blocks | Natural },
    non_claims | Array String,
  },
  ComparisonReport = {
    schema | OneOf ["cairn.agent-benchmark.comparison-report.v1"],
    baseline | Report,
    candidate | Report,
    comparable | Bool,
    drift_fields | Array String,
    verdict | OneOf ["improved", "not_improved", "withheld"],
    withheld_reasons | Array String,
    scoped_token_change | optional | Nullable {
      baseline_attempted_tokens | Natural,
      baseline_correct_completions | Natural,
      candidate_attempted_tokens | Natural,
      candidate_correct_completions | Natural,
    },
    non_claims | Array String,
  },
}

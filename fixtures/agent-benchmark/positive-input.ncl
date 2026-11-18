# Synthetic accounting/schema control, not recorded provider or live-host usage.
let Contract = import "../../contracts/agent-benchmark.ncl" in
{
  schema = "cairn.agent-benchmark.input.v1",
  scenario = { scope = "representation-fixture", mode = "representation_replay", repetitions = 1, max_calls_per_run = 4 },
  runs = [{
    run_id = "4444444444444444444444444444444444444444444444444444444444444444",
    cohort = {
      source_projection = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa", product_revision = "9999999999999999999999999999999999999999999999999999999999999999",
      policy = "bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb", fixture = "cccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccc", oracle = "dddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddd",
      model = "eeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee", agent_configuration = "ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff",
      catalog = "1111111111111111111111111111111111111111111111111111111111111111", presentation = "2222222222222222222222222222222222222222222222222222222222222222",
      accounting = { method = "provider_usage", usage_source = "3333333333333333333333333333333333333333333333333333333333333333" },
    },
    trace = {
      declared_calls = 1, complete = true,
      calls = [{ call_id = "5555555555555555555555555555555555555555555555555555555555555555", sequence = 0,
        usage = { uncached_input = 4, cached_input = 1, output = 3, reasoning = null, reasoning_inclusion = "included_in_output" },
        content_identity = null }],
      retry_count = 0, tool_call_count = 1, repeated_read_count = 0, duration_ms = 1,
    },
    oracle = { verdict = "completed", agent_claimed_completion = true },
  }],
} | Contract.Input

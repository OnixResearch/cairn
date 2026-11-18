let Contract = import "../../contracts/agent-benchmark.ncl" in
{ schema = "cairn.agent-benchmark.policy.v1", bounds = {
 max_repetitions = 0, max_calls_per_run = 128, max_input_bytes = 1048576,
 max_scope_bytes = 96, max_event_count = 1000000, max_token_count = 1000000, max_duration_ms = 3600000,
}, non_claims = [] } | Contract.Policy

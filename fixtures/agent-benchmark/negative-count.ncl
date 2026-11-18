let Contract = import "../../contracts/agent-benchmark.ncl" in
{ uncached_input = -1, cached_input = 0, output = 1, reasoning = null, reasoning_inclusion = "included_in_output" } | Contract.Usage

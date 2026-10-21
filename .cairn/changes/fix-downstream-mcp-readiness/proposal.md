## Why

ChaosControl's required `mcp_agent_smoke` fails because Cairn unconditionally loads Cairn self-host completion fixtures beneath the consumer root. Supplying copied fixtures or suppressing the check would not prove downstream completion. The shell also fails to propagate an explicitly selected policy into MCP completion validation.

## What Changes

- Reuse the existing typed consumer release selection implementation, allowing consumer-owned completion contract/results and Preserves paths plus maintained external commands.
- Propagate the exact selected policy through standalone MCP smoke and release readiness. Preserve self-host defaults only when no selection exists; malformed or missing selected inputs fail closed.
- Reuse the existing bounded dependency/native Nix producer repair rather than manufacturing passing completion rows.
- Exercise real CLI/MCP acceptance and failure rejection, then the required Cairn cohort gates and supported ChaosControl policy migration.

## Impact

Maintainers: Cairn CLI/core producer and policy maintainers. Current consumer: ChaosControl release readiness. This is the user's explicit `fix cairn mcp` request, scoped to the MCP blocker; historical replay artifacts, host permissions and unrelated acceptance criteria are unchanged. Consumer setup is repeatable through its declared policy and real producer inputs, not upstream fixture copying.

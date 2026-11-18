# Measurement verification rails

Source revision: this branch (`change/measure-agent-lifecycle-token-cost-20261117`),
based on `1cca6fe1b573218cd8ea4b5ecfb482f57a8c81b5`.

## Rails executed

| Rail | Command | Result |
|---|---|---|
| Workspace tests | `nix develop -c cargo test --workspace` | pass |
| Clippy | `nix develop -c cargo clippy --workspace --all-targets -- -D warnings` | pass |
| Formatting | `nix develop -c cargo fmt --check` | pass |
| Policy freshness | `cairn policy export --check` | `policy fresh: cairn-policy/generated/cairn-policy.json` |
| Lifecycle validation | `cairn validate --root .` | `valid: true`, 0 issues |
| Traceability | `cairn traceability coverage --root .` | `2372/2372 referenced` |
| Rename classification | `nix build .#checks.x86_64-linux.rename-classification` | pass |

## Behavioural controls

`cargo test --workspace` includes the fresh-root oracle control, which invokes
native completion, gates and readiness and keeps missing or stale evidence from
becoming success. Core and CLI controls cover exact accounting, distinct cached
input, complete cohorts, duplicate calls, overflow, mixed cohorts, missing usage,
false success, unsafe paths and truncated observations.

The retained baseline under `evidence/implementation/baseline/` records the
pre-change command observations with bytes and BLAKE3 identities. Tokens are
`unavailable` there because no provider usage or exact tokenizer observation was
supplied; byte counts are never converted into token estimates.

## Scoped limitations

- `nix flake check` was not run as a single invocation; the named rails above
  were run individually. Its Octet attribute is currently red repository-wide
  for reasons tracked by `use-canonical-octet-integration`; that failure is
  independent of this change and is not claimed as passing here.
- No live agent-host campaign was run. Every efficiency claim in this change is
  representation-scoped; end-to-end token improvement is explicitly withheld.

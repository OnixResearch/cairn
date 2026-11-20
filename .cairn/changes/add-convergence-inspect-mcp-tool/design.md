# Design

## Context

`cairn convergence inspect` reads one case with `MAX_POLICY_BYTES` and the
policy's own bounds, refuses links, and prints either one `cairn.convergence.v1`
receipt or a closed `status: "rejected"` receipt before exiting 1. The pure
decision stays in `cairn_core::convergence::evaluate_convergence`.

MCP tools are declared in `mcp/schema.rs`, dispatched in `mcp/mod.rs` only
after `catalog.describe` and `validate_mcp_tool_input`, and admitted by the
`cairn-policy/mcp-discovery.ncl` profiles, whose export is checked fresh.

## Decisions

### Decision: One inspect function, two transports

**Choice:** Extract the CLI's policy read and rejection mapping into
`convergence::inspect_with_policy(case, root, Option<&Path>) -> Result<Value,
Box<dyn Error>>`. The CLI prints its value and exits 1 when its status is
`rejected`; `mcp_convergence_inspect` validates its string arguments, resolves
`root` with the shared MCP default and returns the same value as
`structuredContent`.

**Rationale:** The requirement is parity, not a second implementation. Keeping
the policy read, the bounded link-free case read and the rejection shape in one
function means the two transports cannot disagree on bytes or issue codes.
`Box<dyn Error>` still reports an unreadable policy as a transport error, as the
command does.

### Decision: Rejections are receipts; argument errors are errors

**Choice:** A missing or unknown argument fails in `validate_mcp_tool_input`
before dispatch, so the transport reports an error and reads nothing. An input
the case or policy rejects returns the same `status: "rejected"` receipt the
command prints, with `isError: false`, inside `structuredContent`.

**Rationale:** This is how other read tools map invalid arguments and how the
command maps rejected inputs. A rejection is a well-formed advisory receipt with
stable issue codes, not a transport failure; callers can inspect it exactly as
they inspect the CLI output.

### Decision: Register with the read-only inspect tools

**Choice:** Add `cairn_convergence_inspect` to the `Inspect` list (which the
`inspect`, `author` and `full` profiles share through `AllRead`) and to the
`verify` profile list in `cairn-policy/mcp-discovery.ncl`, and declare it in
`mcp_read_tools` with `readOnlyHint=true` and `destructiveHint=false`.

**Rationale:** The tool inspects a saved corpus and mutates nothing, so it
belongs with the other read-only inspection tools rather than the plan or
mutation groups. No profile gains a mutation tool or a required route.

### Decision: The structured receipt is the command's receipt

**Choice:** Return the receipt value unchanged. Do not add an `mcp` annotation
object, summary-only fields, a wrapper hash or a transformed schema; the
existing response profile still chooses the transport summary text.

**Rationale:** The acceptance condition compares `receipt_blake3`, and the
`cairn.convergence.v1` schema is closed. Adding transport fields would make the
two receipts differ and would change the hash the command seals.

### Decision: Tests use the synthetic corpus

**Choice:** Extend `crates/cairn-cli/tests/convergence.rs`, whose fixture builds
the synthetic case at test time, with MCP calls through the real binary. Assert
structured equality with the CLI receipt, identical repeated output, and the
three negative mappings; assert catalog membership and annotations in every
profile.

**Rationale:** The synthetic corpus already exercises the full inspect path
without large retained records, and driving the real binary keeps the MCP
transport and discovery policy in the test. The retained pilot corpus keeps its
existing CLI test unchanged.

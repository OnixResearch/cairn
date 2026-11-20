# Convergence delta

## ADDED Requirements

### Requirement: MCP inspection parity

r[cairn.convergence.mcp_inspect] The read-only MCP tool `cairn_convergence_inspect` MUST accept `case`, `root` and `policy` with the same meanings as the `cairn convergence inspect` flags and MUST return the same `cairn.convergence.v1` receipt for the same inputs. It MUST share the command's inspect path rather than duplicate its logic, MUST NOT write a file, spawn a process, use the network or start Nickel, Mantle or Octet, MUST read every input bounded and without following links, and MUST reject unknown arguments before reading any input.

#### Scenario: The MCP receipt equals the CLI receipt

- GIVEN a convergence case that the command admits
- WHEN the caller invokes `cairn_convergence_inspect` with the same `case`, `root` and `policy`
- THEN `structuredContent` MUST equal the receipt the command prints
- AND its `receipt_blake3` MUST equal the command's `receipt_blake3`
- AND a repeated call MUST return an identical result.

#### Scenario: Rejections are explicit and read nothing further

- GIVEN a request missing `case`, a request with an unknown argument, an input with a schema the policy does not admit, or a symbolic link among the case inputs
- WHEN the tool is invoked
- THEN the missing or unknown argument MUST be rejected before any input is read
- AND the unadmitted schema or linked input MUST yield the command's `status: "rejected"` receipt with its issue code
- AND no row MUST be reported `current`.

# CLI Machine Contract Specification Delta

## ADDED Requirements

### Requirement: Machine contract policy is typed and deterministic

r[cairn.cli_machine_contract.policy] Cairn MUST define the machine schema version, exit classes, diagnostic fields, ordering, bounds, process-status mapping, and non-claims through typed Nickel policy and a checked runtime projection.

#### Scenario: Valid machine policy exports deterministically

r[cairn.cli_machine_contract.policy.valid]
- GIVEN machine contract policy has one known schema version, unique exit classes, complete diagnostic fields, and bounded status mappings
- WHEN policy export and runtime admission run
- THEN equivalent policy values MUST produce the same canonical projection and identity
- AND machine-mode commands MUST use the checked projection without evaluating Nickel.

#### Scenario: Invalid machine policy fails admission

r[cairn.cli_machine_contract.policy.invalid]
- GIVEN machine contract policy has an unknown class, duplicate code, missing field, unsafe target rule, or invalid process-status mapping
- WHEN policy export or runtime admission runs
- THEN Cairn MUST reject the machine contract with stable diagnostics
- AND it MUST NOT emit an envelope under the invalid contract.

### Requirement: Machine mode is explicit and versioned

r[cairn.cli_machine_contract.mode] Cairn MUST expose an opt-in top-level `--machine` mode with a policy-owned schema version. Default CLI output and MCP JSON-RPC framing MUST remain outside this mode.

#### Scenario: Explicit machine invocation selects the contract

r[cairn.cli_machine_contract.mode.select]
- GIVEN a supported Cairn command and valid arguments
- WHEN the operator invokes `cairn --machine <command> ...`
- THEN Cairn MUST use the versioned machine envelope
- AND it MUST NOT change the command result selected by the same default invocation.

#### Scenario: Incompatible presentation options fail clearly

r[cairn.cli_machine_contract.mode.incompatible]
- GIVEN a machine invocation also selects an incompatible human or alternate presentation mode
- WHEN top-level argument admission runs
- THEN Cairn MUST reject the combination with a stable diagnostic
- AND it MUST NOT fall back to mixed or ambiguous output.

### Requirement: Controlled outcomes emit one JSON envelope

r[cairn.cli_machine_contract.envelope] Cairn MUST emit exactly one JSON object on stdout for every dispatcher-controlled machine-mode success or failure. The object MUST include `schema_version`, `ok`, `command`, `exit_class`, `result`, and sorted `diagnostics`.

#### Scenario: Machine success emits one envelope

r[cairn.cli_machine_contract.envelope.success]
- GIVEN a machine-mode command returns a valid command result
- WHEN the controlled output boundary serializes the outcome
- THEN stdout MUST contain exactly one valid envelope with `ok` set to true
- AND `result` MUST contain the unchanged command result while controlled stderr remains empty.

#### Scenario: Controlled failure emits one envelope

r[cairn.cli_machine_contract.envelope.failure]
- GIVEN argument parsing, dispatch, policy loading, root selection, file admission, command planning, or serialization preparation returns a typed error
- WHEN the controlled output boundary serializes the outcome
- THEN stdout MUST contain exactly one valid envelope with `ok` set to false and `result` set to null
- AND sorted diagnostics MUST describe the failure while controlled stderr remains empty.

### Requirement: Diagnostics and exit classes are stable

r[cairn.cli_machine_contract.diagnostics] Cairn MUST classify controlled outcomes with policy-owned exit classes and stable diagnostic codes. Each diagnostic MUST include bounded `code`, `severity`, `message`, and `target` fields, with an optional bounded `fix`.

#### Scenario: Stable diagnostics replace message matching

r[cairn.cli_machine_contract.diagnostics.stable]
- GIVEN two equivalent typed failures have different display context
- WHEN Cairn builds machine diagnostics
- THEN both outcomes MUST retain the same diagnostic code and exit class
- AND ordering MUST depend on policy order, code, and target rather than display message text.

#### Scenario: Unknown error mapping fails closed

r[cairn.cli_machine_contract.diagnostics.unknown]
- GIVEN a controlled error has no admitted policy mapping
- WHEN machine classification runs
- THEN Cairn MUST use the bounded `internal_error` class and a stable fallback code
- AND it MUST NOT expose debug values, backtraces, or unbounded source text.

### Requirement: Transport success is separate from lifecycle validity

r[cairn.cli_machine_contract.exit_status] Machine envelope success MUST describe transport completion only. Existing nested `valid`, `blocked`, `verdict`, or equivalent command fields and nonzero process statuses MUST retain their documented meanings.

#### Scenario: Lifecycle failure remains distinct from transport failure

r[cairn.cli_machine_contract.exit_status.nested]
- GIVEN a command returns a valid structured receipt whose lifecycle verdict is blocked or invalid
- WHEN machine mode wraps that command result
- THEN the envelope MAY set `ok` to true because a result exists
- AND the nested lifecycle verdict MUST remain unchanged and authoritative.

#### Scenario: Machine mode preserves nonzero failure status

r[cairn.cli_machine_contract.exit_status.failure]
- GIVEN a controlled failure produces a machine failure envelope
- WHEN the process exits
- THEN it MUST retain a nonzero status mapped from the policy-owned exit class
- AND the presence of JSON MUST NOT convert the failure into process success.

### Requirement: Existing result receipts remain compatible

r[cairn.cli_machine_contract.compatibility] Machine mode MUST nest existing command result values without changing their fields, meanings, canonical bytes, or receipt hashes. Default CLI output and MCP JSON-RPC output MUST remain unchanged.

#### Scenario: Nested receipt identity is preserved

r[cairn.cli_machine_contract.compatibility.nested]
- GIVEN a command result has a stable receipt hash in default mode
- WHEN machine mode returns that result inside its envelope
- THEN the nested result MUST retain the same receipt identity
- AND envelope metadata MUST remain outside the nested receipt hash.

#### Scenario: Default CLI and MCP surfaces remain unchanged

r[cairn.cli_machine_contract.compatibility.default]
- GIVEN unchanged inputs for a compatibility CLI fixture or MCP request
- WHEN the command runs without `--machine`
- THEN documented output fields, stderr behavior, and protocol framing MUST remain unchanged
- AND no machine envelope may appear implicitly.

### Requirement: Machine diagnostics preserve privacy

r[cairn.cli_machine_contract.privacy] Machine envelopes MUST NOT add environment values, usernames, credentials, secret file content, raw backtraces, or unbounded process output. Path targets MUST use bounded normalized forms when available.

#### Scenario: Sensitive values are not added to diagnostics

r[cairn.cli_machine_contract.privacy.secret]
- GIVEN a failing fixture contains secret-like environment, path, or file values outside an admitted display field
- WHEN Cairn builds a machine failure envelope
- THEN no diagnostic or envelope field may contain those values
- AND the stable code and bounded target MUST still identify the failure class.

#### Scenario: Existing receipt privacy remains authoritative

r[cairn.cli_machine_contract.privacy.nested]
- GIVEN a successful nested receipt has an existing privacy and redaction contract
- WHEN machine mode wraps the receipt
- THEN the envelope MUST NOT expand or reinterpret redacted values
- AND it MUST NOT add raw observations that the nested receipt omits.

### Requirement: Outcome classification preserves the functional-core boundary

r[cairn.cli_machine_contract.functional_boundary] Cairn MUST keep typed outcome classification, diagnostic normalization, ordering, and envelope construction pure. The CLI MUST own arguments, dispatch, error capture, policy loading, serialization, output streams, and process status.

#### Scenario: Outcome classification remains pure

r[cairn.cli_machine_contract.functional_boundary.pure]
- GIVEN typed command or error values and admitted machine policy
- WHEN Cairn classifies and constructs an envelope
- THEN the result MUST be deterministic without filesystems, environments, processes, clocks, networks, mutation, or output effects.

#### Scenario: Shell writes one completed value

r[cairn.cli_machine_contract.functional_boundary.shell]
- GIVEN the pure core returns one complete machine-envelope value
- WHEN the CLI serializes a controlled outcome
- THEN the shell MUST perform one bounded stdout write path
- AND it MUST NOT interleave display diagnostics on controlled stderr.

### Requirement: The machine-output claim is bounded

r[cairn.cli_machine_contract.boundary] Cairn MUST limit the one-envelope guarantee to errors that reach its controlled output boundary. It MUST NOT claim guaranteed output after operating-system termination, process kill, memory exhaustion, pre-boundary panic, broken stdout, or failure before Cairn starts.

#### Scenario: Host-level failures remain outside the claim

r[cairn.cli_machine_contract.boundary.host]
- GIVEN Cairn cannot reach or complete the controlled output boundary because of a host-level failure
- WHEN documentation describes machine mode
- THEN it MUST state that no envelope is guaranteed
- AND external supervisors MUST remain responsible for timeout, signal, and missing-output handling.

#### Scenario: Machine envelope does not promote evidence claims

r[cairn.cli_machine_contract.boundary.nonclaim]
- GIVEN a valid machine envelope contains a nested receipt
- WHEN an agent consumes the envelope
- THEN the envelope MUST NOT claim nested evidence truth, command correctness, mutation safety, host integrity, or release eligibility
- AND all nested claim boundaries MUST remain in force.

### Requirement: Machine contract verification and documentation are complete

r[cairn.cli_machine_contract.verification] The change MUST include positive and negative policy, core, parser, dispatcher, command, compatibility, exit-status, privacy, and boundary tests.

#### Scenario: Positive and negative suites close the contract

r[cairn.cli_machine_contract.verification.suites]
- GIVEN valid machine invocations and malformed, missing, unsafe, private, incompatible, and host-boundary fixtures
- WHEN focused tests and repository lifecycle checks run
- THEN valid cases MUST emit deterministic envelopes
- AND invalid controlled cases MUST emit stable failures without compatibility drift.

### Requirement: Agent CLI documentation defines the contract

r[cairn.cli_machine_contract.docs] Cairn MUST document machine invocation, envelope fields, exit classes, diagnostic codes, nested verdicts, compatibility, privacy, and uncontrolled failure boundaries.

#### Scenario: Agent can handle every documented outcome

r[cairn.cli_machine_contract.docs.complete]
- GIVEN an agent follows the machine-contract guide
- WHEN it receives success JSON, failure JSON, nonzero status, or missing output
- THEN the guide MUST define the required handling for each case
- AND it MUST not instruct the agent to parse display message text for decisions.

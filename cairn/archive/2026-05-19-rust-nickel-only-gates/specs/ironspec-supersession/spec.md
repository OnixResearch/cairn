# ironspec-supersession delta: Rust/Nickel-only gates

## ADDED Requirements

### Requirement: Rust/Nickel policy export gate
r[cairn.supersede.rust_nickel_only_gates.policy_export]
Cairn SHALL expose policy export and freshness validation through a Rust CLI command that uses Nickel only for Nickel policy evaluation.

#### Scenario: Policy export freshness is checked without Python helpers
GIVEN a Cairn repository with checked-in generated policy
WHEN an operator runs `cairn policy export --check`
THEN the command SHALL render Nickel policy, validate semantic invariants in Rust, compare the generated JSON, and exit nonzero when the generated policy is stale.

### Requirement: Rust/Nickel traceability coverage gate
r[cairn.supersede.rust_nickel_only_gates.tracey_coverage]
Cairn SHALL expose accepted requirement coverage checking through a Rust CLI command.

#### Scenario: Tracey bootstrap coverage is enforced without Python helpers
GIVEN accepted specs with `r[...]` requirement markers
WHEN an operator runs `cairn tracey coverage --root .`
THEN the command SHALL require each accepted requirement to have an implementation or verification reference in Rust or Nickel source and SHALL reject missing or dangling references deterministically.

### Requirement: Rust rename classification gate
r[cairn.supersede.rust_nickel_only_gates.rename_classification]
Cairn SHALL expose legacy rename classification through a Rust CLI command.

#### Scenario: Rename classification rejects unresolved legacy usage
GIVEN source-controlled Cairn text files
WHEN an operator runs `cairn rename-classification`
THEN the command SHALL classify legacy Ironspec occurrences and exit nonzero if any occurrence remains in the `needs-fix` bucket.

### Requirement: No checked-in Python or shell helper scripts
r[cairn.supersede.rust_nickel_only_gates.no_python_scripts]
Cairn SHALL NOT rely on checked-in Python or shell helper scripts for maintained gates.

#### Scenario: Repository gate helpers are Rust or Nickel
GIVEN the source-controlled Cairn repository
WHEN an operator lists maintained helper scripts and Nix check dependencies
THEN checked-in Python and shell helper scripts SHALL be absent and maintained checks SHALL use Rust CLI surfaces and Nickel policy instead.

### Requirement: Strict validation exit mode
r[cairn.supersede.rust_nickel_only_gates.gates]
Cairn SHOULD provide a strict validation mode for CI orchestration.

#### Scenario: Validation can fail by exit code
GIVEN a Cairn repository with validation issues
WHEN an operator runs `cairn validate --strict --root PATH`
THEN the command SHALL emit the validation receipt and exit nonzero when `valid` is false.

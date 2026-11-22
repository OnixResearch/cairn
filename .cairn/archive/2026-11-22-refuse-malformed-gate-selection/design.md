# Design

## Context

`repository_gate_summary` in `crates/cairn-cli/src/tracey.rs` reads the admitted
policy's `release_readiness_policy.required_external_commands`. Before this
change it used the list only when every step of an `Option` chain succeeded,
so any malformed value fell through to the maintained-local list. The
release-readiness receipt reports component failures as checks with
`passed: false` and an `issue`, and the command still prints the receipt with
`valid: false` and `verdict: fail`.

## Decisions

### Decision: Absence and malformation are different outcomes

**Choice:** A missing `release_readiness_policy` or a missing
`required_external_commands` key selects the maintained-local gates. A present
key whose value is not a non-empty array of unique, non-blank strings, or a
`release_readiness_policy` that is not an object, is malformed.

**Rationale:** Absence is the documented way to accept Cairn's maintained
gates. A present value states the consumer's intent, so replacing it with a
different list misreports what the consumer selected. Duplicate and blank
entries carry no meaning as gates and indicate an authoring error. Lattice's
own Nickel contract for this field already refuses the same forms.

### Decision: Refuse inside the receipt, not by aborting the command

**Choice:** A malformed list yields the summary policy
`invalid-consumer-selected-gates` with `required_commands: null` and an `issue`
string; the `repository_gate_policy_listed` check copies the issue and fails,
which makes the receipt verdict `fail`.

**Rationale:** Every other release-readiness component fails closed this way,
and consumers already gate on `valid` and `verdict`. A null list cannot be
mistaken for "no gates required", which an empty array could suggest. Policies
without the list and well-formed lists keep byte-identical receipts.

### Decision: Diagnostics name the field and entry indices, never entry text

**Choice:** The issue names `release_readiness_policy.required_external_commands`
and, for entry errors, zero-based indices only (`[1] must be a string`,
`[1] must not be blank`, `[3] duplicates entry [1]`). No diagnostic copies an
entry's text.

**Rationale:** An operator can locate the entry from the receipt alone, and
tests can assert the exact text. The policy file may be up to 16 MiB, and the
issue appears in both the summary and the failed check, so echoing a command
would let the receipt grow with the input.

### Decision: Check uniqueness with a set

**Choice:** Uniqueness is checked with a map from entry to its first index,
while a vector keeps the declared order for the receipt.

**Rationale:** A linear scan per entry is quadratic; a well-formed list of
100,000 short commands fits in the policy limit and would need about 5e9
comparisons. The map keeps validation linear in the total entry size.

## Risks / Trade-offs

- A consumer that relied on the silent fallback with a malformed list now gets
  a failing receipt. No known consumer does: Cairn's own policy omits the list,
  Kiln's omits it, and the Lattice and ChaosControl lists are well-formed and
  typed by their own Nickel contracts.
- Leading or trailing whitespace inside a non-blank command is kept as written;
  only fully blank entries are refused.

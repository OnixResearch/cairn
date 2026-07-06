## Design: EARS property vocabulary gate

### The c-earchin model

c-earchin translates an EARS requirement's response (the text after "shall")
against a fixed vocabulary of pure-boolean predicates. If the response
resolves, the requirement becomes a real property witness. If it does not
resolve, the requirement is marked `recorded_only` — it carries structure
but proves nothing, and `chelis prove` does not report a vacuous pass.

### Vocabulary

16 predicates: the c-earchin nine (`is_non_negative`, `is_positive`,
`within_tolerance`, `is_monotone_increasing`, `is_bounded_above`,
`is_bounded_below`, `is_equal`, `converges_to`, `is_error_rejection`) plus
seven trellis-shaped predicates for the systems domains trellis covers
(`preserves_invariant`, `bounded_resource`, `lock_order_acyclic`,
`monotone_advance`, `no_deadlock`, `crash_safe`, `linearizable`).

### Classification

`classify_ears_response` uses keyword matching: the response text is searched
for substrings that indicate the predicate. For example, "produce non-negative
call prices" resolves to `is_non_negative`; "reject the order" resolves to
`is_error_rejection`; "preserve the log matching property" resolves to
`preserves_invariant`.

### Strict vs non-strict

- **Strict mode**: the report is invalid if any response is unresolved. This
  is the c-earchin `--strict` guarantee: a clean strict run means every
  requirement became a real property.
- **Non-strict mode**: unresolved responses are accepted as `recorded_only`
  and counted separately. This lets teams keep a complete EARS corpus even
  where a response has no matching predicate.

### Connection to Phase 1 and Phase 2

- Phase 1 `verification_role`: unresolved responses carry
  `VerificationRole::RecordedOnly`, so cairn's `evidence verify` counts them
  separately from `Property` artifacts.
- Phase 2 `provenance`: `resolve_requirement` on a `ProvenanceChain` can check
  that every resolved requirement has at least one function object discharging
  it.
- Phase 3 trellis `property-spec-index`: the index provides the dispatch
  targets — when cairn classifies a response as `is_monotone_increasing`, it
  can point to the trellis property group that contains the corresponding
  `proof fn` or `proptest`.

### What is explicitly NOT provided

- A full EARS parser (the classifier operates on the response text, not the
  full EARS clause structure with WHEN/WHILE/IF/THEN/WHERE).
- Automatic dispatch to trellis properties (the index is the target, but
  automatic linking is a future change).
- A claim that resolved predicates are actually verified (classification says
  the response *maps to* a predicate; verification requires running the
  property).

## Why

Cairn uses EARS for requirement structure and gates, but it does not check whether a requirement's response (the text after "shall") actually resolves to a checkable property. c-earchin shows the target: an EARS response maps to a fixed vocabulary of pure-boolean predicates, and strict mode fails if any response doesn't resolve. This change adds the vocabulary classifier and coverage checker, turning cairn from "requirement has a reference" (Tracey) into "requirement response is discharged by a checkable predicate" — the c-earchin guarantee.

## What Changes

- Add `cairn-core/src/verified/ears_property_vocabulary.rs`: the 16-predicate vocabulary (c-earchin nine plus trellis-shaped: `preserves_invariant`, `bounded_resource`, `lock_order_acyclic`, `monotone_advance`, `no_deadlock`, `crash_safe`, `linearizable`). `classify_ears_response` maps response text to a predicate via keyword matching. `check_ears_vocabulary_coverage` produces a report: strict mode fails on unresolved responses, non-strict accepts them as `recorded_only`. `EarsVocabularyReport` with `to_json()` and boundary non-claim.
- Register the module in `verified/mod.rs`.
- 13 positive and negative tests covering classification (non-negative, monotone, reject, preserves, deadlock, unresolvable, empty) and coverage (strict fail, strict pass, non-strict accept, empty, boundary, vocabulary stability).

## Impact

- **Files**: `crates/cairn-core/src/verified/ears_property_vocabulary.rs` (new), `crates/cairn-core/src/verified/mod.rs`.
- **Testing**: `cargo test -p cairn-core` (148 passed: 135 original + 13 new), `cargo fmt --check` (clean).
- **Non-claims**: EARS vocabulary coverage records which responses resolve to checkable predicates; unresolved responses are recorded_only and prove nothing. This is an epistemic-honesty rail, not a correctness proof.

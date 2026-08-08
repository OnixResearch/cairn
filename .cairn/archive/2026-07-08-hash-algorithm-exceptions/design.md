## Design

Define a Nickel-authored hash policy with default algorithm, accepted field patterns, exception rows, and migration state. Exception rows name algorithm, owner repo, schema/field scope, reason, allowed consumers, and non-claims.

A pure checker receives parsed schema/evidence field metadata and the policy. The shell gathers candidate fields from configured roots. The checker rejects undeclared non-BLAKE3 identity fields, malformed digest syntax, and exceptions without owner/reason.

This does not require converting protocol-bound legacy hashes immediately; it makes them explicit and reviewable.

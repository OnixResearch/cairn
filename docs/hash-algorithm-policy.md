# Hash algorithm policy

Cairn-owned content identity defaults to BLAKE3. Other algorithms are accepted only when a bounded exception records the owner, exact field scope, reason, allowed consumers, migration status, and non-claims.

`cairn hash-policy verify --input <manifest.json>` validates a manifest containing a `policy` object and observed hash `fields`. The shell reads JSON; the pure core validates field metadata and returns deterministic diagnostics.

Passing validation means the named field uses either the stack default or a declared exception. It does not prove artifact truth, producer correctness, digest collision resistance beyond the named algorithm, or semantic correctness.

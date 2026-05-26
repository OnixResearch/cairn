# Design: Preserves corpus explain CLI

Add a read-only subcommand next to `preserves-corpus update`. It should parse the corpus using the existing bounded parser/validation logic, then emit a receipt with case count, case ids, descriptions, expected hash presence/shape, duplicate id issues, and a receipt hash.

The command must not call the rewrite path and should fail closed for malformed corpus material.

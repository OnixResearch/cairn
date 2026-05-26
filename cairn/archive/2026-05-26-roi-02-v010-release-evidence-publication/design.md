# Design: v0.1.0 release evidence publication

Run `cairn release-evidence index` into `target/release-evidence-v0.1.0`, verify the index, then materialize a local publication tree with `release-evidence publish`. The committed docs snapshot records payload commit, tag, command chain, receipt hashes, artifact hashes, and non-claims.

The checked-in snapshot is immutable audited evidence for the v0.1.0 payload commit, not a conventional `docs/release-evidence-current-head.md` mutable claim.

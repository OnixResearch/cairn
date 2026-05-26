# Design: CLI maintainability hotspot inventory

Use a deterministic line-count scan over Rust source/test files and commit a docs inventory with the top hotspots, proposed future split boundaries, and focused verification commands. The inventory is advisory evidence only; future refactors still need their own Cairn package when behavior or module boundaries change.

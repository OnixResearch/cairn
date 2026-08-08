# Design: Post-tag remote install smoke

Use `nix build github:OnixResearch/cairn/v0.1.0#cairn` from an isolated temp directory, then invoke the built binary against a temporary Cairn root. The evidence report records the tag, resolved binary path class, commands, verdicts, and non-claims without preserving machine-specific temp paths.

The smoke is intentionally smaller than `nix flake check`: it proves the released install surface starts and can operate on a fresh lifecycle root.

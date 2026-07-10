## Tasks

- [x] [serial] r[cairn.ast_grep_gate.profile] Define the ast-grep lifecycle gate profile and policy modes.
- [x] [depends:ast-grep-gate-profile] r[cairn.ast_grep_gate.receipt] Add pure receipt validation for tool identity, rule bundle BLAKE3 hash, scope, output format, findings summary, and non-claims.
- [x] [depends:ast-grep-gate-receipt] r[cairn.ast_grep_gate.shell_boundary] Keep ast-grep execution, file discovery, and log capture in CLI or repo shell surfaces.
- [x] [depends:ast-grep-gate-receipt] r[cairn.ast_grep_gate.negative_fixtures] Add negative fixtures for missing identity, missing non-claims, stale scope, unsupported output format, and proof/release overclaims.
- [x] [depends:ast-grep-gate-negative-fixtures] r[cairn.ast_grep_gate.valence_compat] Preserve optional source-span and verification-role fields for Valence-compatible exports.
- [x] [depends:ast-grep-gate-valence-compat] r[cairn.ast_grep_gate.validation] Run fixture tests, malformed receipt tests, policy export checks, Cairn gates, and focused lifecycle validation.

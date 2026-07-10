## Design

1. Define an ast-grep lifecycle gate profile that can consume already-produced ast-grep test and scan results.
2. Keep command execution, filesystem walking, stdout/stderr capture, and tool discovery in CLI shell code or repository-owned scripts.
3. Validate parsed receipts in pure core code: tool identity, rule bundle hash, scan scope, finding counts, severity posture, output format, and non-claim labels.
4. Use BLAKE3 for Cairn-owned rule bundle and receipt identity fields unless the wrapped tool emits an interoperability hash that must be preserved as metadata.
5. Support policy-declared modes: inventory-only, advisory warning, blocking error, and codemod-review evidence.
6. Require malformed-receipt negative fixtures for missing rule bundle identity, missing non-claims, stale scope, unsupported output format, and overclaiming proof/release labels.
7. Preserve Valence export compatibility by retaining source spans and verification roles when a repository forwards ast-grep findings downstream.

## Non-Goals

- No built-in ast-grep rule catalog in Cairn core.
- No execution of ast-grep from pure validation code.
- No claim that a clean ast-grep scan satisfies requirements, proves implementation correctness, or grants release readiness.

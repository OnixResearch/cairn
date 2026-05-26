# Design: current-HEAD release evidence snapshot

Use existing release-evidence commands rather than duplicating receipt or index logic.

Implementation path:

1. Run `cairn release-evidence index --root . --out target/release-evidence-current-head/index.md --artifact-dir target/release-evidence-current-head`.
2. Run `cairn release-evidence check-index --root . --index target/release-evidence-current-head/index.md --mode current-head`.
3. Run `cairn release-readiness --root .` and preserve the receipt path/hash in the operator report.
4. Keep generated evidence under `target/`; if documentation is committed, record verified code commit and evidence-recording commit separately.

The slice is evidence-only unless implementation gaps appear in existing commands.

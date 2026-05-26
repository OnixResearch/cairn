# ROI 02: v0.1.0 release evidence publication

## Why

Cairn has a v0.1.0 tag and local release gate, but the checked-in audited evidence snapshot predates the release. A v0.1.0 evidence report makes the release boundary easier to audit without replacing generated artifacts or claiming mutable current-HEAD status.

## Scope

- Generate release evidence index and publication artifacts for the clean tagged payload.
- Verify the generated index and local publication receipt.
- Commit only a compact audited summary; leave bulky generated artifacts under `target/`.

## Non-goals

- No tag movement, upload, remote deletion, signature assertion, or public unattended production-readiness claim.

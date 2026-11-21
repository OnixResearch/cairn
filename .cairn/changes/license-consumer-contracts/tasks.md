# Tasks

- [x] [serial] [task:grant] Add the scoped `MIT OR Apache-2.0` row and grant prose for `contracts/**` and `cairn-policy/assurance-targets.ncl`, covering every revision and the two vendored revisions, to `docs/licensing.md` and mirror it in `cairn-policy/LICENSING.md`. r[cairn.license_boundary.consumer_contracts_grant]
- [x] [serial] [task:inventory] [after:grant] Add the grant row, paths and vendored revisions to `license-policy.tsv` and attach the requirement's verify marker to `tools/check-license-boundary.rs`. r[cairn.license_boundary.consumer_contracts_grant]
- [x] [serial] [task:docs] [after:inventory] Record the grant in `CHANGELOG.md`. r[cairn.license_boundary.consumer_contracts_grant]

## Validation and completion

- [x] [serial] [task:positive] [after:docs] Run the license-boundary script on the repository and record the passing transcript. r[cairn.license_boundary.consumer_contracts_grant]
- [x] [serial] [task:negative] [after:positive] Delete the new grant row in a scratch copy, run the same script and record the failing transcript. r[cairn.license_boundary.consumer_contracts_grant]
- [x] [serial] [task:lifecycle] [after:negative] Run `policy export --check`, `validate` and the proposal, design and tasks gates; sync and archive with `CAIRN_ARCHIVE_DATE`, then post-archive validate and coverage. r[cairn.license_boundary.consumer_contracts_grant]

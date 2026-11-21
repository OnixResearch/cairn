# Tasks

- [x] [serial] [task:policy] Replace `schemas.octet_producer_job` with the typed owner-scoped `schemas.producer_jobs` map admitting `octet` and `trellis`, and regenerate `cairn-policy/generated/assurance-targets.json` the repository way. r[cairn.convergence.inputs]
- [x] [serial] [task:admission] [after:policy] Add the pure owner admission: unknown owner, schema/owner mismatch and missing owner each fail closed with a stable code, and the policy rejects an empty, blank or case-duplicate owner map. r[cairn.convergence.inputs]
- [x] [serial] [task:cli] [after:admission] Apply the same owner admission to the CLI's bounded handoff `job.json` read, keeping the existing `job_id` binding unchanged. r[cairn.convergence.inputs]
- [x] [serial] [task:tests] [after:cli] Keep the retained Octet pilot `current` unchanged; add a synthetic Trellis positive and one negative per rejection code at both the core and CLI boundaries. r[cairn.convergence.inputs]
- [x] [serial] [task:docs] [after:tests] Document the owner-scoped admission in `docs/convergence.md` and record the change in `CHANGELOG.md`. r[cairn.convergence.inputs]

## Validation and completion

- [x] [serial] [task:focused] [after:docs] Run the cairn-core and cairn-cli convergence tests and the convergence pilot test, plus fmt and clippy. r[cairn.convergence.inputs]
- [x] [serial] [task:policy-check] [after:focused] Run `cairn policy export --check` and the repository `validate`. r[cairn.convergence.inputs]
- [x] [serial] [task:gates] [after:policy-check] Pass the proposal, design and tasks gates, then sync and archive with `CAIRN_ARCHIVE_DATE`, followed by post-archive validate and coverage. r[cairn.convergence.inputs]
- [x] [serial] [task:flake] [after:gates] Run the full `nix flake check path:$PWD -L --keep-going`. r[cairn.convergence.inputs]

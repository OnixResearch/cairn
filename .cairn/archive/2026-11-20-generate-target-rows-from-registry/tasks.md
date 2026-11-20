# Tasks

- [x] [serial] [task:contract] Add `requirement_from_registry` and `specification_from_registry` to `contracts/assurance-targets.ncl`, selecting exactly one row by id and failing closed on a foreign schema, a missing id or a duplicate id. r[cairn.assurance_targets.row_binding]
- [x] [serial] [task:fixtures] [after:contract] Add the registry-derived positive fixture and the missing, duplicate and wrong-schema negatives, and wire them into the `assurance-targets-contracts` flake check. r[cairn.assurance_targets.row_binding]
- [x] [serial] [task:guidance] [after:fixtures] Document generating the row and specification from the registry export in `docs/convergence.md` and `CHANGELOG.md`. r[cairn.assurance_targets.row_binding]
- [x] [serial] [task:verification] [after:guidance] Run the flake check, the assurance targets tests, `policy export --check`, `validate`, the three gates, sync, archive, post-archive `validate` and `traceability coverage`, and the full flake check; record transcripts in `evidence/`. r[cairn.assurance_targets.row_binding]

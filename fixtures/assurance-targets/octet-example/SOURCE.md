# Octet example golden

Projection-only fixture. It reproduces clause 0 of Octet's requirement-contract
example and executes no proof.

## Copied bytes

- `octet-clause-0.json`: clause 0 of `standards/requirements/example-contract.json`
  at Octet tag `v0.1.1-rc.1`, commit `4aba4019f77f95cb9c3e25ff27964898d708cef0`,
  re-serialized with `jq -S`. Source file BLAKE3:
  `d7f59e10290e8f7e4aeba57bc47276e92f2f50a4ea63aeabcc831959a51415a7`.
- `octet-contract-meta.json`: `contract_id`, `reviewed_specification`,
  `assumptions`, `exclusions` and `non_claims` from the same file and revision.
- `generated.json`: the generated source record for clause 0. `sources` and the
  four `required_bindings` lists are copied from that clause; they stand in for
  the record a producer would generate and are not authored in the targets.

## Fixture-local values

- `targets.json` binds two synthetic rows. Octet's
  `standards/requirements/example-spec.md` (BLAKE3
  `78afa0f6c7761fbbf7f727507508410660866dd75ff35618ea27209478cc0814` at the same
  commit) carries no `r[...]` markers, so no Cairn registry row exists for it.
  Each `source_span` is the byte span of the matching bullet,
  `requirement_body_blake3` is the BLAKE3 of those bytes, and `row_blake3` is the
  BLAKE3 of the other five fields as compact sorted JSON. These rows are not
  produced by Cairn's registry algorithm and are not admitted against a registry.
- The `checked_proof` target's `evidence_job` names the recorded fixture rail
  `octet.aeneas_lean_proof_rails.fixture` that produced the example evidence (the
  clause's `required_bindings.policy_ids`). It is not a Mantle producer job.
- The `reviewed` target (`octet.requirements.example.reviewed_scope`) must
  produce no clause.

No sibling checkout is read when the fixture is checked.

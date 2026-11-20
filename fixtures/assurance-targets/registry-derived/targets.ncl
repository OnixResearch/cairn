# The octet example targets, with each target's registry row and the contract's
# specification derived from a checked-in registry export instead of copied.
# Its export must equal `../octet-example/targets.json` byte for byte.
let Contract = import "../../../contracts/assurance-targets.ncl" in
let registry = import "./requirement-registry.json" in
let row = fun requirement_id =>
  Contract.requirement_from_registry registry requirement_id
in
{
  schema_version = "cairn.assurance-targets.v1",
  contracts = [
    {
      contract_id = "octet.requirements.example",
      specification = Contract.specification_from_registry
        registry
        "octet.aeneas_lean_proof_rail.theorem_adequacy",
      assumptions = [
        {
          affected_clause_ids = [
            "octet.requirements.example.assumption.lean_toolchain"
          ],
          assumption_id = "assumption.lean_toolchain",
          identity = "lean-toolchain:4ff9ff39f3ec4e1015d318f317230c2a568a3aded1b49c07a9ab814af2cdbefa",
          required_empirical_evidence = [
            "check.lean_toolchain"
          ],
          statement = "The retained Lean toolchain bytes are the reviewed toolchain identity.",
          trusted_boundary_identity = "aeneas-lean-provider-pin:bc2eb6f941369fa3c695e30598b5758b11e4f0de"
        }
      ],
      exclusions = [
        {
          clause_id = "octet.requirements.example.excluded.gpu_paths",
          exclusion_id = "exclusion.gpu_paths",
          reason = "The fixture covers only the checked Aeneas/Lean CPU proof receipt.",
          reviewed_specification_blake3 = "78afa0f6c7761fbbf7f727507508410660866dd75ff35618ea27209478cc0814"
        }
      ],
      non_claims = [
        "does not authenticate the reviewer or producer",
        "does not prove Aeneas, Charon, or Lean soundness",
        "does not prove whole-program correctness",
        "does not establish release eligibility"
      ],
      targets = [
        {
          requirement = row "octet.aeneas_lean_proof_rail.theorem_adequacy",
          assurance = "checked_proof",
          checked_scope = [
            "formal-proof:fixture-only:octet.aeneas_lean_proof_rails.fixture#rail-0"
          ],
          claim = "The checked Aeneas/Lean fixture theorem aeneas.fixture.safe_add is present and accepted over the generated model.",
          evidence_classes = [
            "proof_card"
          ],
          evidence_ids = [
            "aeneas.fixture.safe_add"
          ],
          evidence_job = {
            definition = "standards/proof-rails/aeneas-lean/fixtures/valid",
            job_id = "octet.aeneas_lean_proof_rails.fixture"
          },
          informal_remainder = "Translation soundness, verifier soundness, whole-program correctness and release eligibility remain outside the fixture theorem.",
          negative_controls = [],
          provider = "aeneas-lean"
        },
        {
          requirement = row "octet.requirements.example.reviewed_scope",
          assurance = "reviewed",
          claim = "GPU execution paths are outside the reviewed scope of this fixture.",
          informal_remainder = "The exclusion is reviewed prose only; no evidence job discharges it."
        }
      ]
    }
  ]
} | Contract.TargetsFile

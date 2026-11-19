# Advisory assurance-target and convergence policy.
# Admitted vocabulary, foreign schema identifiers and bounds only. Independent of
# the legacy default policy, so it changes no existing policy or receipt hash
# material. Runtime commands read the generated export and never start Nickel.
let Assurance = std.contract.from_predicate (fun value =>
  std.array.elem value ["reviewed", "bounded_check", "checked_proof"]
) in
let Positive = std.contract.from_predicate (fun value =>
  std.is_number value && value == std.number.floor value && value > 0
) in
let NonEmpty = std.contract.from_predicate (fun value =>
  std.is_string value && std.string.length value > 0
) in
{
  schema = "cairn.assurance-targets-policy.v1",
  targets_schema = "cairn.assurance-targets.v1",
  sources_schema = "cairn.assurance-target-sources.v1",
  case_schema = "cairn.convergence-case.v1",
  receipt_schema = "cairn.convergence.v1",
  assurances = ["reviewed", "bounded_check", "checked_proof"],
  tiers = {
    bounded_check = "empirical_check",
    checked_proof = "formal_proof",
  },
  schemas = {
    registry = "cairn.content-bound-requirement-registry.v1",
    octet_contract = "octet-requirement-contract/v1",
    octet_evidence = "octet-requirement-evidence/v1",
    octet_decision = "octet-requirement-decision/v1",
    octet_proof_cards = "octet-proof-cards/v1",
    octet_formal_proof_envelope = "octet-formal-proof-envelope/v1",
    octet_producer_job = "octet-mantle-producer-job/v1",
    mantle_observation = "mantle-external-evidence-observation-v1",
    mantle_realization_receipt = "mantle-foreign-realization-receipt-v1",
    mantle_store_info = "mantle-store-info-v3",
    mantle_build_report = "crunch-build-report-v1",
  },
  octet = {
    passing_aggregate = "passed",
    # Any clause status outside this map is reported `unsupported`.
    clause_states = {
      passed = "current",
      failed = "failed",
      stale_binding = "stale",
      unsupported = "unsupported",
      missing = "incomplete",
      incomplete = "incomplete",
      unresolved = "incomplete",
    },
  },
  mantle = {
    realization_complete_status = "complete",
    realized_states = ["realized", "provenance-audited"],
    proof_acceptance_relay = "not-evaluated-by-mantle",
  },
  bounds = {
    max_contracts = 64,
    max_targets_per_contract = 256,
    max_registry_rows = 4096,
    max_text_bytes = 4096,
    max_list_items = 256,
    max_input_bytes = 4194304,
    max_document_bytes = 1048576,
    max_trusted_signers = 8,
  },
  non_claims = [
    "a current target shows only that, for one revision, the supplied Mantle records and Octet decision bind to the declared target",
    "does not show that any code is correct or that the evidence is true",
    "does not show that a reused output was re-verified; a cache hit preserves an earlier acceptance and is not fresh proof work",
    "does not show that the supplied records were honestly produced; Cairn re-runs and authenticates none of them and does not verify signatures",
    "does not show that a verifier, builder, sandbox or the Nickel exporter is sound",
    "does not show that Mantle built the tool graph from source unless the recorded route says so",
    "does not show that a claim captures its requirement's full meaning; see informal_remainder",
    "does not establish release eligibility or change any gate, readiness state or verification role",
  ],
} | {
  schema | NonEmpty,
  targets_schema | NonEmpty,
  sources_schema | NonEmpty,
  case_schema | NonEmpty,
  receipt_schema | NonEmpty,
  assurances | Array Assurance,
  tiers | { bounded_check | NonEmpty, checked_proof | NonEmpty },
  schemas | {
    registry | NonEmpty,
    octet_contract | NonEmpty,
    octet_evidence | NonEmpty,
    octet_decision | NonEmpty,
    octet_proof_cards | NonEmpty,
    octet_formal_proof_envelope | NonEmpty,
    octet_producer_job | NonEmpty,
    mantle_observation | NonEmpty,
    mantle_realization_receipt | NonEmpty,
    mantle_store_info | NonEmpty,
    mantle_build_report | NonEmpty,
  },
  octet | {
    passing_aggregate | NonEmpty,
    clause_states | { _ | std.contract.from_predicate (fun value =>
      std.array.elem value ["current", "failed", "stale", "unsupported", "incomplete"]
    ) },
  },
  mantle | {
    realization_complete_status | NonEmpty,
    realized_states | Array NonEmpty,
    proof_acceptance_relay | NonEmpty,
  },
  bounds | {
    max_contracts | Positive,
    max_targets_per_contract | Positive,
    max_registry_rows | Positive,
    max_text_bytes | Positive,
    max_list_items | Positive,
    max_input_bytes | Positive,
    max_document_bytes | Positive,
    max_trusted_signers | Positive,
  },
  non_claims | Array NonEmpty,
}

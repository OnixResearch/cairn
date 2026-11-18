# Advisory scoped-claim derivation policy.
# Bounds and admitted vocabulary only. Independent of the legacy default policy,
# so it changes no existing policy or receipt hash material.
{
  schema = "cairn.scoped-claim-policy.v1",
  claims = {
    schema = "cairn.scoped-claim.v1",
    kinds = [
      "observation",
      "structural_relationship",
      "bounded_property",
      "checker_accepted_property",
      "reviewed_requirement_correspondence",
    ],
    propositions = [
      "check_passed",
      "receipt_structurally_valid",
      "native_completion_eligible",
      "reviewed_correspondence",
      "reviewed_completion_decision",
    ],
  },
  rules = [
    {
      id = "reviewed_completion_v1",
      version = 1,
      assurance_scope = "native completion accepted with reviewed correspondence; no behavioral satisfaction",
    },
  ],
  bounds = {
    max_claims = 256,
    max_derivations = 512,
    max_producers = 256,
    max_premises = 8,
    max_depth = 16,
  },
} | {
  schema | String,
  claims | { schema | String, kinds | Array String, propositions | Array String },
  rules | Array { id | String, version | Number, assurance_scope | String },
  bounds | {
    max_claims | Number,
    max_derivations | Number,
    max_producers | Number,
    max_premises | Number,
    max_depth | Number,
  },
}

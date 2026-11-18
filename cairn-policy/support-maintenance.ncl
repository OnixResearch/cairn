# Advisory support-maintenance policy.
# Bounds and admitted change vocabulary only. Independent of the legacy default
# policy, so it changes no existing policy or receipt hash material.
{
  schema = "cairn.support-maintenance-policy.v1",
  change_kinds = [
    "evidence_stale",
    "evidence_withdrawn",
    "assumption_withdrawn",
    "evidence_admitted",
    "claim_revised",
    "policy_changed",
    "context_changed",
  ],
  bounds = {
    max_claims = 256,
    max_branches = 512,
    max_evidence = 512,
    max_premises = 8,
    max_assumptions = 128,
    max_events = 256,
    max_obligations = 256,
    max_depth = 16,
  },
} | {
  schema | String,
  change_kinds | Array String,
  bounds | {
    max_claims | Number,
    max_branches | Number,
    max_evidence | Number,
    max_premises | Number,
    max_assumptions | Number,
    max_events | Number,
    max_obligations | Number,
    max_depth | Number,
  },
}

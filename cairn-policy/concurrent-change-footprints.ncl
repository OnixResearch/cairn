# Advisory concurrent-change footprint policy.
# Bounds only. The core owns footprint semantics, interaction derivation,
# obligation outcomes and the finite exploration ceilings enforced in Rust.
# This file is independent of the legacy default policy, so it changes no
# existing policy or receipt hash material.
{
  schema = "cairn.concurrent-change-footprints-policy.v1",
  bounds = {
    max_changes = 32,
    max_revisions = 128,
    max_assumptions = 256,
    max_guarantees = 256,
    max_invariants = 256,
    max_invalidations = 256,
    max_interfaces = 256,
    max_unknown = 32,
    max_compositions = 32,
    max_evidence = 512,
    max_conditions = 128,
    max_domain_values = 64,
    max_worlds = 262144,
    max_steps = 2000000,
    max_depth = 32,
  },
} | {
  schema | String,
  bounds | {
    max_changes | Number,
    max_revisions | Number,
    max_assumptions | Number,
    max_guarantees | Number,
    max_invariants | Number,
    max_invalidations | Number,
    max_interfaces | Number,
    max_unknown | Number,
    max_compositions | Number,
    max_evidence | Number,
    max_conditions | Number,
    max_domain_values | Number,
    max_worlds | Number,
    max_steps | Number,
    max_depth | Number,
  },
}

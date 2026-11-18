# Advisory revision-classification bounds.
# Nickel owns every admitted bound; Rust keeps only implementation ceilings.
# Automated evidence transfer is deliberately disabled until separate qualification.
{
  schema = "cairn.revision-classification-policy.v1",
  comparison = {
    max_domains = 64,
    max_observables = 64,
    max_domain_values = 256,
    max_worlds = 1048576,
    max_steps = 10000000,
    max_evidence = 1024,
  },
  policy_fragment = {
    max_checks = 128,
    max_roles = 64,
    max_assumptions = 64,
    max_assumption_sets = 4096,
    max_accepted = 10000000,
  },
  transfer = {
    enabled = false,
    admission_rules = [],
  },
} | {
  schema | String,
  comparison | {
    max_domains | Number,
    max_observables | Number,
    max_domain_values | Number,
    max_worlds | Number,
    max_steps | Number,
    max_evidence | Number,
  },
  policy_fragment | {
    max_checks | Number,
    max_roles | Number,
    max_assumptions | Number,
    max_assumption_sets | Number,
    max_accepted | Number,
  },
  transfer | { enabled | Bool, admission_rules | Array String },
}

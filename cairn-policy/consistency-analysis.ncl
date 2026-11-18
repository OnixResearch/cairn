# Advisory requirement-consistency analysis policy.
# Bounds and the enabled query vocabulary only. Independent of the legacy
# default policy, so it changes no existing policy or receipt hash material.
{
  schema = "cairn.requirement-consistency-policy.v1",
  queries = [
    "assumptions_satisfiable",
    "activation_reachable",
    "guarantees_compatible",
    "guarantee_constraining",
  ],
  bounds = {
    max_contracts = 8,
    max_guarantees = 16,
    max_cells = 4096,
    max_states = 250000,
    max_evaluations = 4000000,
    max_depth = 32,
  },
} | {
  schema | String,
  queries | Array String,
  bounds | {
    max_contracts | Number,
    max_guarantees | Number,
    max_cells | Number,
    max_states | Number,
    max_evaluations | Number,
    max_depth | Number,
  },
}

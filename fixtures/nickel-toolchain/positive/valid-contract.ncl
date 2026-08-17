let Cohort = {
  cohort | String,
  runtime_evaluation | Bool,
} in
({ cohort = "1.17.0", runtime_evaluation = false } | Cohort)

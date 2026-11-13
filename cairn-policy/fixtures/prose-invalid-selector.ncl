# Deliberately unknown selector; export must fail Rust admission.
{
  schema_version = "cairn.prose-profiles.v1",
  profiles = [{ name = "invalid", include_rules = ["invented_rule"], exclude_rules = [] }],
}

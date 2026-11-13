# Positive custom profile exercising exclusion precedence.
{
  schema_version = "cairn.prose-profiles.v1",
  profiles = [{ name = "reviewed", include_rules = ["E0001"], exclude_rules = ["subject_verb_agreement"] }],
}

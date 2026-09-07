let C = import "../../../contracts/verification-impact.ncl" in
let h = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa" in
{
  evidence_id = "after-receipt",
  check_ids = ["demo-tests", "workspace-tests"],
  applicability = {
    subject_hash = h,
    policy_hash = h,
    dependency_hashes = [],
    source_projection_hash = h,
  },
} | C.prior_evidence

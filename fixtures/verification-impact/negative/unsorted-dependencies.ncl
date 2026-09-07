let C = import "../../../contracts/verification-impact.ncl" in
let h = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa" in
{
  evidence_id = "unsorted-dependencies",
  check_ids = ["demo-tests"],
  applicability = {
    subject_hash = h,
    policy_hash = h,
    # Reverse-sorted dependency identities must fail the canonical contract.
    dependency_hashes = ["ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff", h],
    source_projection_hash = h,
  },
} | C.prior_evidence

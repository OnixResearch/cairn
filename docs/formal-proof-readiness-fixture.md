# Formal-proof readiness fixture

Cairn's formal-proof readiness fixture is a frozen release-evidence seam for
checking how Cairn handles a Valence stack readiness input derived from an
Octet-owned formal-proof export. The fixture records Valence input identity,
Valence graph digest, sidecar digest, upstream Octet proof identity, policy hash,
input hash, readiness receipt hash, verification-role counts, accepted/blocked
counts, upstream proof status, and required non-claims.

Passing validation proves Cairn receipt handling under the configured readiness
policy only. It does not prove upstream source correctness, verifier soundness,
Valence correctness, Octet correctness, whole-program correctness, semantic
equivalence, or release eligibility outside the configured readiness policy.
Valence owns stack identity/linkage semantics, Octet owns formal-proof receipt
production, and Cairn owns release-readiness policy decisions.

r[verify cairn.release_evidence.formal_proof_readiness_fixture.docs]
r[verify cairn.release_evidence.formal_proof_readiness_fixture.docs.non_claims]

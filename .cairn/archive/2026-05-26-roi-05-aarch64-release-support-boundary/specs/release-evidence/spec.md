# Release Evidence Specification Delta

## ADDED Requirements

### Requirement: aarch64 release support boundary is explicit
r[release_evidence.aarch64_support_boundary.docs] The system SHOULD document the architecture support boundary for release evidence when a full multi-system Nix check was not performed.

#### Scenario: aarch64 non-claim is visible
r[release_evidence.aarch64_support_boundary.non_claims]
- GIVEN release-readiness evidence was produced on x86_64 and Nix omitted incompatible aarch64 checks
- WHEN an operator reads release evidence notes
- THEN the notes SHALL state that aarch64 release support still requires builder-backed validation before being claimed.

#### Scenario: Future validation path is concrete
r[release_evidence.aarch64_support_boundary.validation]
r[release_evidence.aarch64_support_boundary.priority]
- GIVEN a suitable aarch64 builder or evaluator is available
- WHEN an operator wants to expand release support
- THEN the notes SHALL name the future command path and SHALL keep unavailable builder noise separate from current product-local release proof.

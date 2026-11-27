# Design

## Anti-Ceremony Design Principle

A gate meant to curb specification inflation MUST NOT itself introduce paperwork for normal work. Accordingly:

1. **Zero overhead for single-repo non-plumbing changes:** If a change touches only the local repository and does not declare cross-repo dependencies or new receipt/policy schemas, the check passes immediately with zero extra fields required.
2. **Scoped trigger (`policy.capability_payload_budget`, default-off):** Repositories opt in via policy by specifying:
   - `enabled: bool` (default `false`)
   - `max_cross_repo_targets: usize` (e.g., `2` or `3`)
   - `enforcement: warn | error`
3. **Structured check in `cairn validate` / `cairn gate proposal`:**
   - Inspects cross-repo declarations (`cross_repo_dependencies.rs`) and proposal/design metadata.
   - When `cross_repo_targets > 1` (or the change adds a new cross-repo receipt/gate schema), verifies that:
     - A concrete `Capability Payload` section or metadata binding identifies the non-toy workload or measured defect unblocked (rejecting placeholder or self-referential plumbing-only justifications).
     - If `cross_repo_targets > max_cross_repo_targets`, an explicit `Scope Justification` explains why the work cannot be staged inside one repo first.
4. **Explicit Non-Claims:** Static budget gating checks declared cross-repo blast radius and payload naming; it does not semantically prove that the named payload is valuable — human review remains the final arbiter.

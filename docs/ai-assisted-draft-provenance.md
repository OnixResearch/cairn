# AI-assisted draft provenance

r[verify cairn.ai_spec_drafts.operator_docs]
r[verify cairn.ai_spec_drafts.operator_docs.visible]

Cairn may record AI-assisted draft provenance for lifecycle specs, Kani
contracts, harnesses, and evidence notes. This metadata is never verification
evidence by itself.

## Safe fields

Use schema `cairn.ai_spec_draft_provenance.v1`. Record only bounded metadata:

- `artifact_kind`
- `assistant_class`
- `redaction_class`
- `target_requirement`
- `validation_command`
- `review_decision`
- `draft_status`
- `acceptance_path`
- `validation_status`
- `evidence_role`

Do not store raw prompts, provider payloads, system prompts, account identifiers,
or conversation transcripts.

## Accepted paths

AI-drafted artifacts complete only through a separate accepted path:

- `machine_check`: a passing validation status plus a passing machine receipt.
- `human_review`: an approved review decision.
- `docs_only`: explicit docs rationale and no `property` evidence role.

`ai_only` is rejected as verification evidence. Rejected or superseded drafts
cannot be cited as accepted evidence.

This provenance is useful for reviewability and traceability, but it does not
prove requirement satisfaction, proof correctness, model soundness, or release
readiness.

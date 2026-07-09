# Requirement ID Registry

Cairn supports a bounded requirement ID registry for cross-repo `r[...]` references. The registry maps a requirement ID to its owner repo, bounded namespace, accepted spec path, lifecycle status, explicit aliases, and allowed consumers.

The validation command is:

```bash
cairn requirement-registry verify --input fixtures/requirement-id-registry/positive/owned-id.json
```

## Contract

Registry inputs use schema `cairn.requirement-id-registry.v1`. Each row includes:

- `requirement_id`: canonical `r[...]` ID without the `r[]` wrapper;
- `namespace`: bounded namespace containing an owner and domain segment;
- `owner_repo`: repo that owns the accepted requirement semantics;
- `accepted_spec`: accepted spec path for review;
- `status`: `active`, `deprecated`, or `retired`;
- `aliases`: explicit compatibility IDs with status and reason;
- `allowed_consumers`: repos or roles allowed to cite the ID.

## Non-claims

Registry resolution proves ownership and linkage only. It is not requirement satisfaction, not implementation correctness, not test coverage, and not release eligibility. Consumers still need their own verification, provenance, release-readiness, and local policy receipts.

## Failure modes

Validation fails closed for unknown IDs, wrong owner expectations, stale aliases, inactive canonical rows, disallowed consumers, malformed rows, missing non-claims, and namespace overreach.

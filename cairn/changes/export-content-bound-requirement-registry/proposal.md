# Proposal: Export a content-bound requirement registry

## Summary

Add a deterministic Cairn registry export that binds each accepted requirement to its repository, declared revision, accepted specification bytes, and current policy.

## Motivation

Current traceability receipts bind aggregate requirement and evidence manifests. Downstream artifacts still carry bare requirement ID strings without repository or specification identity.

A duplicate ID, moved requirement, stale specification, or wrong repository can therefore look valid until a human compares separate receipts.

Cairn owns accepted requirement discovery and policy evaluation. It is the correct producer for a portable, content-bound requirement registry.

## Scope

- Add read-only `cairn requirement-registry export` and `verify` commands.
- Define a pure registry model with BLAKE3 identities for rows and the complete registry.
- Bind repository identity, declared revision, accepted specification content, policy, and safe source locations.
- Add positive and negative fixtures for current, stale, duplicate, malformed, and cross-repository inputs.
- Make the registry available as an optional Cairn evidence artifact role.

## Affected specs

- `requirement-id-registry`: content-bound export, current-root verification, and downstream claim boundaries.

## Non-goals

- Do not prove that a declared revision names the supplied filesystem bytes.
- Do not prove requirement satisfaction, implementation correctness, or evidence truth.
- Do not contact a VCS, network service, registry, or transparency log from the pure core.
- Do not replace existing traceability coverage receipts.
- Do not make publication, signing, or release eligibility implicit.

## Compatibility

Existing marker syntax and traceability reports remain valid. Strict downstream profiles can require the new registry without changing legacy report hashes.

## Completion evidence

The change is complete when export and verification are deterministic, stale rows fail closed, evidence publication can carry the registry, and all focused gates pass.

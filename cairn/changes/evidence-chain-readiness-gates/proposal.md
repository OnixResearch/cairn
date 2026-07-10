## Why

Cairn should not grow one readiness gate per evidence kind. Function-address readiness should become the first profile of a generalized evidence-chain gate that validates canonical receipt linkage, release binding, policy hashes, claim scopes, and non-claim boundaries for any supported stack evidence kind.

## What Changes

- Define a generic evidence-chain readiness input and gate model.
- Validate canonical envelope hash, Valence validation hash, Mantle binding hash, evidence kind, profile version, source/binary linkage, policy hashes, claim scope, optional/required disposition, projections, and non-claims.
- Register function-address as the first supported evidence-chain profile.
- Add positive and negative fixtures for profile-neutral readiness behavior.

## Impact

Cairn can gate stack evidence readiness consistently across function-address, proof, lint, dependency, build, attestation, and lifecycle profiles without interpreting evidence payload internals.

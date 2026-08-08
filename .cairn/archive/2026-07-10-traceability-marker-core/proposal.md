## Why

Requirement and evidence markers such as `r[...]`, `r[impl ...]`, `r[verify ...]`, dependency markers, and related-evidence links are used by Cairn, Octet, Valence, and stack lifecycle artifacts. The grammar and link DTOs should be reusable, while each repo keeps its own source scanning and policy interpretation.

This change defines a Cairn-owned traceability marker core for parsing and validating marker syntax, marker verbs, requirement/evidence links, and link diagnostics.

## What Changes

- Define a reusable marker grammar and DTO model for requirement IDs, implementation markers, verification markers, dependency markers, and related-evidence markers.
- Add marker verb vocabulary and owner/consumer boundary checks.
- Add positive fixtures for accepted Cairn markers and downstream Octet/Valence marker examples.
- Add negative fixtures for malformed IDs, unsupported verbs, dangling links, namespace overreach, duplicate links where disallowed, and proof overclaims.
- Keep repository file scanning, marker discovery, policy profiles, and release-readiness interpretation in each consuming shell.

## Impact

- **Cairn** owns reusable marker parsing and link DTOs.
- **Octet/Valence/other consumers** can share marker grammar without adopting Cairn lifecycle policy wholesale.
- **Validation** remains pure and fixture-backed before downstream integration.

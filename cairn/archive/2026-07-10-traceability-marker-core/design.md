## Design

The traceability marker core is a pure parser and validator in `cairn-core`. It consumes in-memory strings and optional registry rows, then returns typed marker DTOs or diagnostics. Shells own file walking, span capture, policy loading, and report rendering.

### Decisions

1. **Marker grammar is shared.** `r[...]` requirement IDs and role-prefixed markers such as implementation, verification, dependency, and related markers share one parser and stable DTO model.

2. **Verbs are vocabulary rows.** Supported marker verbs are validated against a declared vocabulary rather than free-form strings.

3. **Registry linkage is optional input.** The core can validate links against an in-memory requirement registry, but it does not discover accepted specs itself.

4. **Consumers keep policy.** Octet may interpret proof markers differently than Cairn lifecycle markers; the shared core only parses, normalizes, and checks bounded link health.

5. **No satisfaction claim.** A parsed or registry-resolved marker proves syntax and declared linkage only. It does not prove implementation, verification coverage, behavioral correctness, release readiness, or proof soundness.

### Validation shape

Positive tests cover bare requirement IDs, implementation markers, verification markers, dependency markers, related markers, and registry-resolved links. Negative tests cover malformed IDs, unsupported verbs, wrong namespace, dangling IDs, duplicate links where policy rejects them, missing owner rows, and overclaims that link resolution proves requirement satisfaction.

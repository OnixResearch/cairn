# Expand the advisory prose authoring workflow

## Why

Specification and public-document authors need usable local feedback, reproducible configuration, precise technical-document positions, and safe CI/editor exports without changing lifecycle authority.

## What Changes

- Add explicit human text, GitHub warning annotation, and SARIF formats while retaining default JSON v1 and machine transport.
- Select one active change's authoring documents with deterministic, confined, bounded traversal.
- Export bounded rule profiles from Nickel and support reasoned next-line suppressions.
- Add opt-in content-bound v2 reports and independent current-input rechecking before reuse.
- Preserve original positions through technical-document masking, attach requirement context through existing traceability parsing, and flag meaning-change review risks.
- Bound aggregate input, diagnostics, traversal, and rendered output; exhaustion is not a successful partial report.

## Impact

Cairn CLI maintainers own the command, profiles, adapters, and compatibility contracts; english-rs owns grammar rules. The immediate consumer is Cairn's change-authoring loop. The durable capability is repeatable offline authoring feedback with explicit stale-result rejection and non-blocking export. Consumer behavior fixtures, a real CLI smoke, policy export freshness, and required repository gates provide repeatability evidence. No automatic fixes, mandatory grammar gates, quality scores, external publication, or semantic requirement verdicts are added.

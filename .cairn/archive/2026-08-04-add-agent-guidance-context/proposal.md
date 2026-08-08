## Why

Agents need project facts and artifact-specific guidance while they author lifecycle work. Today those facts live in informal repository files, and Cairn does not expose a bounded, identity-linked instruction context.

The lifecycle-store change introduces typed project declarations and direct read-only references. A separate follow-up can extend that contract without expanding the active store package or confusing advisory guidance with enforceable policy.

## What Changes

- Extend typed project declarations with bounded local context references, artifact guidance, and operation guidance.
- Keep policy constraints, project facts, and advisory guidance as distinct output classes.
- Add deterministic context selection by workflow profile, artifact, and policy-known operation.
- Extend workflow instructions and read-only context output with selected text, source paths, BLAKE3 identities, bounds, and authority labels.
- Add local and portable receipt identities that keep raw context out of portable evidence.
- Reject globs, URLs, commands, environment expansion, unsafe paths, stale projections, unknown selectors, and unsupported authority fields.
- Add positive and negative privacy, bounds, ordering, stale-source, non-authority, and compatibility fixtures.

## Impact

- **Policy**: Add typed context and guidance selectors, bounds, authority labels, receipt fields, and non-claims.
- **Core**: Add pure declaration admission, context selection, ordering, envelope construction, and portable identity material.
- **CLI and MCP**: Extend read-only context and workflow instruction results.
- **Project configuration**: Extend `.cairn/project.ncl` and its checked JSON projection after lifecycle-store support lands.
- **Documentation**: Add an agent context and guidance guide.
- **Testing**: Add positive and negative policy, core, CLI, MCP, projection, privacy, and evidence coverage.

## Out of Scope

- Expanding the active lifecycle-store change package.
- Treating guidance as policy, evidence, a gate, or dependency satisfaction.
- Fetching remote context, executing commands, reading environment values, or expanding globs.
- Detecting every secret or semantic contradiction in user-authored text.
- Adding tool-specific generated skills.

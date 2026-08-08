## Why

Cairn has a repository-maintained Hermes workflow document, but tool-specific agent instructions can drift from CLI, MCP, receipt, and lifecycle contracts. OpenSpec reduces onboarding cost through generated cross-tool workflows.

Cairn can adapt that delivery model after its artifact, machine, and guidance contracts stabilize. One typed source can generate small project-local adapters without granting prompts lifecycle authority.

## What Changes

- Add policy-owned workflow descriptors for explore, propose, apply, verify, drain, and archive actions.
- Keep descriptor prose compact, goal-first, and explicit about success, evidence, effects, and non-claims.
- Add versioned policy-known adapters for Agent Skills, Claude, and Codex formats.
- Add read-only list, show, plan, and freshness-check commands plus preview-first project-local installation.
- Add BLAKE3 manifests for source, adapter, generated files, and prior-state identities.
- Make explore a read-only workflow with no artifact creation or mutation command authorization.
- Reject implicit tool detection, user-home writes, modified-file overwrite, unknown adapters, stale plans, network access, model calls, and prompt authority promotion.
- Add positive and negative generation, installation, freshness, compatibility, permission, and claim-boundary fixtures.

## Impact

- **Policy**: Add typed workflow descriptors, adapters, command permission classes, prompt bounds, manifests, and non-claims.
- **Core**: Add pure descriptor admission, rendering, file planning, freshness, and manifest identity logic.
- **CLI**: Add an `agent workflow` command family with preview-first installation.
- **Documentation**: Add generated workflow setup, update, removal, and authority guidance.
- **Testing**: Add positive and negative policy, render, adapter, install, freshness, permission, and compatibility coverage.

## Out of Scope

- Detecting installed agent tools or writing to user home directories.
- Calling models, running agents, sending network requests, or installing remote packages.
- Guaranteeing that an agent follows generated text.
- Replacing the CLI, MCP, lifecycle gates, semantic review, or evidence receipts.
- Supporting every OpenSpec tool adapter in the first slice.

# Generated agent workflows

<!-- r[impl cairn.generated_agent_workflows.docs] -->
<!-- r[impl cairn.generated_agent_workflows.docs.install] -->

Cairn can generate project-local workflow guidance from its checked policy projection.

The initial catalog contains these workflows:

- `explore`
- `propose`
- `apply`
- `verify`
- `drain`
- `archive`

The `explore` workflow is read-only. Its policy descriptor forbids lifecycle writes and mutation-plan execution.

## Inspect the catalog

List all workflow descriptors and adapters:

```bash
cairn agent workflow list --policy cairn-policy/generated/cairn-policy.json
```

Inspect one descriptor:

```bash
cairn agent workflow show explore --policy cairn-policy/generated/cairn-policy.json
```

These commands read the checked JSON policy. They do not evaluate Nickel at runtime.

## Select an adapter

Cairn supports these versioned adapters:

| Adapter | Example destination | Generated path form |
|---|---|---|
| `agent-skills` | `.agents/skills` | `cairn-<workflow>/SKILL.md` |
| `claude` | `.claude/commands` | `cairn-<workflow>.md` |
| `codex` | `.codex/prompts` | `cairn-<workflow>.md` |

An adapter changes only the representation. It cannot add commands, authority, lifecycle semantics, or runtime hooks.

You must supply an explicit project root and a safe project-relative destination. Cairn rejects absolute paths, traversal, glob paths, and home-directory paths.

Cairn does not detect installed agent tools. It does not write user-home configuration.

## Preview and install

Preview an Agent Skills installation:

```bash
cairn agent workflow install \
  --root . \
  --adapter agent-skills \
  --destination .agents/skills
```

The preview reports exact paths, BLAKE3 identities, freshness states, blockers, a manifest identity, and a content-bound `plan_hash`.

Execute only the unchanged plan:

```bash
cairn agent workflow install \
  --root . \
  --adapter agent-skills \
  --destination .agents/skills \
  --execute \
  --plan-hash <PLAN_HASH>
```

Cairn takes a project-local lock and observes the policy and destination again. A changed policy, manifest, destination, or file invalidates the plan.

Cairn writes prepared files through capability-relative paths. It uses atomic file replacement and writes the manifest after workflow files.

## Check freshness

Run a read-only freshness check:

```bash
cairn agent workflow check \
  --root . \
  --adapter agent-skills \
  --destination .agents/skills
```

The receipt uses these freshness states:

- `current`
- `missing`
- `stale-source`
- `modified`
- `unmanaged-conflict`
- `unsupported-version`
- `symlink-conflict`

Cairn can replace a stale generated file only when its bytes match the prior manifest identity. Cairn blocks modified or unmanaged files.

Cairn also blocks a missing managed file during update. This rule prevents an update from hiding an unexpected deletion.

## Obsolete generated files

The first implementation slice does not delete generated files.

Plan and check receipts report obsolete manifest-owned paths as `removal_candidates`. Review and remove these paths outside Cairn when your project policy permits deletion.

## Manifest and privacy boundary

Each destination has one adapter-specific JSON manifest. The manifest records:

- generator and adapter versions
- policy and catalog identities
- exact project-relative file paths
- generated and expected-prior BLAKE3 identities
- removal candidates
- explicit non-claims

Receipts omit host-specific root paths, environment values, prompts, model output, credentials, and user-global configuration.

Generated text is deterministic guidance. It does not prove agent compliance, gate success, implementation correctness, or release eligibility.

# Make a Cairn

Create a native Cairn lifecycle in a repository, apply an onboarding template, scaffold a first change package, and run the gates.

End state:

- `cairn/` lifecycle directories exist.
- One quickstart document from an onboarding template exists.
- One active change package exists under `cairn/changes/`.
- `validate` plus proposal/design/tasks gates pass.

## Prerequisites

Use either an installed `cairn` binary or this repository through Nix. From a Cairn source checkout:

```bash
export CAIRN_SOURCE=/path/to/cairn
export REPO=/path/to/your/repo
```

Commands below use this source checkout form:

```bash
nix run path:$CAIRN_SOURCE#cairn -- <command>
```

If `cairn` is already on `PATH`, replace each `nix run path:$CAIRN_SOURCE#cairn --` prefix with `cairn`.

## Initialize lifecycle directories

```bash
nix run path:$CAIRN_SOURCE#cairn -- init --root "$REPO"
```

Creates this native layout:

```text
cairn/specs/
cairn/changes/
cairn/archive/
```

Legacy `openspec/` content is not read as an implicit fallback. Use `migrate openspec` only when you explicitly want to import legacy lifecycle material.

## Apply a template

List onboarding templates:

```bash
nix run path:$CAIRN_SOURCE#cairn -- template list --root "$REPO"
```

Current templates:

- `agent-workflow` for AI-assisted lifecycle work.
- `generic-cli` for a small command-line project.
- `rust-nix` for a Rust/Nix repository.
- `docs-lifecycle` for docs-only repositories.
- `release-evidence` for local/offline release evidence workflows.

Preview first:

```bash
nix run path:$CAIRN_SOURCE#cairn -- template apply generic-cli --root "$REPO"
```

Apply after reviewing the receipt:

```bash
nix run path:$CAIRN_SOURCE#cairn -- template apply generic-cli --root "$REPO" --execute
```

Templates create quickstart artifacts only. They do not create accepted specs, contact network services, sign releases, or publish remote state.

## Scaffold a first change

Scaffold turns request text into a deterministic draft package. It does not call an LLM or infer requirements from remote state.

```bash
nix run path:$CAIRN_SOURCE#cairn -- scaffold change add-friendly-start \
  --root "$REPO" \
  --request "Add a friendly start path for first-time users."
```

If the preview looks right, write the files:

```bash
nix run path:$CAIRN_SOURCE#cairn -- scaffold change add-friendly-start \
  --root "$REPO" \
  --request "Add a friendly start path for first-time users." \
  --execute
```

The generated package contains:

```text
cairn/changes/add-friendly-start/proposal.md
cairn/changes/add-friendly-start/design.md
cairn/changes/add-friendly-start/tasks.md
cairn/changes/add-friendly-start/specs/developer-experience/spec.md
```

## Validate and gate the change

Run lifecycle validation:

```bash
nix run path:$CAIRN_SOURCE#cairn -- validate --root "$REPO"
```

Run all stage gates:

```bash
nix run path:$CAIRN_SOURCE#cairn -- gate proposal add-friendly-start --root "$REPO"
nix run path:$CAIRN_SOURCE#cairn -- gate design add-friendly-start --root "$REPO"
nix run path:$CAIRN_SOURCE#cairn -- gate tasks add-friendly-start --root "$REPO"
```

Each gate prints a deterministic JSON receipt with validity, issues, policy hash, input hash, and receipt hash fields.

## Work, sync, and archive

After implementation and project-specific verification, preview lifecycle promotion:

```bash
nix run path:$CAIRN_SOURCE#cairn -- sync add-friendly-start --root "$REPO"
nix run path:$CAIRN_SOURCE#cairn -- archive add-friendly-start --root "$REPO"
```

Write lifecycle changes only after reviewing the plans:

```bash
nix run path:$CAIRN_SOURCE#cairn -- sync add-friendly-start --root "$REPO" --execute
CAIRN_ARCHIVE_DATE=$(date +%F) nix run path:$CAIRN_SOURCE#cairn -- archive add-friendly-start --root "$REPO" --execute
```

`sync` and `archive` are dry-run by default. Blocked plans must not mutate even when `--execute` is passed.

## Next steps

- Read the compact copy/paste example in [`../../examples/make-a-cairn/README.md`](../../examples/make-a-cairn/README.md).
- Use `discover-specs --include <path>` to draft review-needed candidate requirements from existing local files.
- Use `release-readiness --root "$REPO"` when the repository has lifecycle content and verification rails ready for a release bundle.

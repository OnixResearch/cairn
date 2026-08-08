# Make a Cairn example

Run this from a Cairn source checkout to create a disposable native lifecycle repository.

```bash
export CAIRN_SOURCE=$(pwd)
export REPO=$(mktemp -d)

nix run path:$CAIRN_SOURCE#cairn -- init --root "$REPO"
nix run path:$CAIRN_SOURCE#cairn -- template apply generic-cli --root "$REPO" --execute
nix run path:$CAIRN_SOURCE#cairn -- scaffold change add-friendly-start \
  --root "$REPO" \
  --request "$(cat examples/make-a-cairn/request.md)" \
  --execute

nix run path:$CAIRN_SOURCE#cairn -- validate --root "$REPO"
nix run path:$CAIRN_SOURCE#cairn -- gate proposal add-friendly-start --root "$REPO"
nix run path:$CAIRN_SOURCE#cairn -- gate design add-friendly-start --root "$REPO"
nix run path:$CAIRN_SOURCE#cairn -- gate tasks add-friendly-start --root "$REPO"

(cd "$REPO" && find . -type f | sort | sed 's#^./##')
```

Expected files:

```text
.cairn/changes/add-friendly-start/design.md
.cairn/changes/add-friendly-start/proposal.md
.cairn/changes/add-friendly-start/specs/developer-experience/spec.md
.cairn/changes/add-friendly-start/tasks.md
docs/cairn-quickstart.md
```

Use `docs-lifecycle` instead of `generic-cli` for a docs-only repository, or `rust-nix` for a Rust/Nix project.

Remove the disposable repository when done:

```bash
rm -rf "$REPO"
```

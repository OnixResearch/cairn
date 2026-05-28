# Oracle checkpoint: package policy export runtime

Date: 2026-05-27  
Owner: coding agent  
Decision: keep V1/V2 marked complete. The validation claims are supported by inspected local command output and a final archived-state rerun.

## Question checked

Did the archived `package-policy-export-runtime` change actually pass its validation tasks after sync/archive?

## Inspected evidence

### Cairn gates

Source: pueue task `63`, run from `/home/brittonr/git/cairn`.

Commands:

```sh
nix run path:$PWD#cairn -- gate proposal package-policy-export-runtime --root .
nix run path:$PWD#cairn -- gate design package-policy-export-runtime --root .
nix run path:$PWD#cairn -- gate tasks package-policy-export-runtime --root .
```

Observed output excerpts:

- Proposal gate: `"stage": "proposal"`, `"valid": true`, `"verdict": "PASS"`, receipt `2819b7264f43dde47325826dcd332c3fd754361fd521a3e3a80cd960ad418df0`.
- Design gate: `"stage": "design"`, `"valid": true`, `"verdict": "PASS"`, receipt `a7ef43c7b23f1199f9ec1abb4145f06d09388b7333a584be193d233df77ad9e4`.
- Tasks gate before completion update: `"stage": "tasks"`, `"valid": true`, `"verdict": "PASS"`, receipt `4b3312ef76615dc86cdf298a8b21d2ea51aebcad9762f80820d5ede7044fa5c8`.
- Tasks gate after marking complete: `"stage": "tasks"`, `"valid": true`, `"verdict": "PASS"`, receipt `b7b34a8029de1af868109bf5f77b4365a43b4dfb14c33b1b744b0c6b0f6b0ee2`.

### Focused runtime verification

Source: pueue task `66`, run from `/home/brittonr/git/cairn`.

Commands:

```sh
nix run path:$PWD#cairn -- policy export --check
nix build path:$PWD#checks.x86_64-linux.packaged-policy-export-runtime --no-link -L
nix run path:$PWD#cairn -- validate --root .
nix run path:$PWD#cairn -- tracey coverage --root .
nix flake check path:$PWD -L
```

Observed output excerpts:

- Packaged policy export printed `policy fresh: cairn-policy/generated/cairn-policy.json`.
- `checks.x86_64-linux.packaged-policy-export-runtime` built after wrapping binaries; build log showed wrapped `.cairn-wrapped` and `.ironspec-wrapped` under the package output.
- Cairn validation printed `"changes": 1`, `"specs_validated": 13`, `"valid": true` before archive.
- Tracey coverage printed `tracey bootstrap coverage ok: 854/854 referenced` before the post-archive Tracey ref fix.
- Flake check printed `all checks passed!`; Nix warned `aarch64-linux` was omitted.

### Final archived-state verification

Source: pueue task `71`, run from `/home/brittonr/git/cairn` after archive and Tracey verification references were added.

Commands:

```sh
nix develop -c cargo fmt --check
nix develop -c cargo test -p cairn-cli --test post_release_docs -- --nocapture
nix run path:$PWD#cairn -- validate --root .
nix run path:$PWD#cairn -- tracey coverage --root .
nix flake check path:$PWD -L
```

Observed output excerpts:

- Focused `post_release_docs` test result: `6 passed; 0 failed`.
- Cairn validation printed `"changes": 0`, `"specs_validated": 12`, `"valid": true` after archive.
- Tracey coverage printed `tracey bootstrap coverage ok: 859/859 referenced` after the accepted spec gained five new requirement/scenario ids.
- `nix flake check path:$PWD -L` evaluated `checks.x86_64-linux.packaged-policy-export-runtime` and printed `running 14 flake checks...` followed by `all checks passed!`.
- Nix again warned `aarch64-linux` was omitted; this matches existing release-support non-claim policy and is not evidence of a failed x86_64 gate.

## Decision

The validation-dependent claims in `tasks.md` are now locally auditable. V1 and V2 remain checked because proposal/design/tasks gates, focused packaged-runtime validation, archive-state validation, Tracey coverage, and flake checks passed on x86_64 Linux.

## Next action

No code action remains for this finding. If a future reviewer cannot inspect pueue logs, use this checkpoint plus the committed implementation/test/docs diffs as the evidence trail. For release tagging, still generate fresh release evidence for the exact release payload commit.

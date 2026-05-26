# ROI 03: Git remote relocation cleanup

## Why

Pushes currently succeed but print GitHub relocation warnings. Updating the configured remote removes recurring operator noise before future release or drain work.

## Scope

- Update the local `origin` URL to the canonical `git@github.com:OnixResearch/cairn.git`.
- Record the expected canonical URL in operator docs.
- Verify the worktree remains clean and synced.

## Non-goals

- No credential rotation, branch rewrite, force push, or remote repository mutation beyond future normal pushes.

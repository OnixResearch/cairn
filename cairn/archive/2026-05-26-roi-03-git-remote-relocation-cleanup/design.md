# Design: Git remote relocation cleanup

Use `git remote set-url origin git@github.com:OnixResearch/cairn.git`, then inspect `git remote -v` and branch status. Because `.git/config` is not tracked, the durable repo change is a small operator note documenting the canonical remote and the reason relocation warnings should be treated as configuration drift.

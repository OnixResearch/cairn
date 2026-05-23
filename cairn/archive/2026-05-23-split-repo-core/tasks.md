# Tasks: Split Repo Core

- [x] [serial] Define behavior-preserving split requirements for repo core read models. r[cli_maintainability.repo_core_modules.focused_modules]
- [x] [serial] Split `repo.rs` into a facade plus focused pure-core submodules. r[cli_maintainability.repo_core_modules.focused_modules.visible_boundaries]
- [x] [serial] Preserve the public `verified::repo` API through narrow facade re-exports. r[cli_maintainability.repo_core_modules.focused_modules.narrow_bridges]
- [x] [serial] Run focused repo-core tests and repository validation gates. r[cli_maintainability.repo_core_modules.behavior_preserved.focused_checks]
- [x] [serial] Run formatting, linting, Tracey coverage, and final repository checks. r[cli_maintainability.repo_core_modules.behavior_preserved.repository_checks]

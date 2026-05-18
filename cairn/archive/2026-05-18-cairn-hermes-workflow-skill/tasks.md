# Tasks

## Phase 1: Spec

- [x] [serial] Define the canonical Cairn Hermes workflow skill requirement. r[cairn.supersede.hermes_skill.canonical_cairn]
- [x] [serial] Define compatibility behavior for the older Ironspec/OpenSpec-named workflow skill. r[cairn.supersede.hermes_skill.redirect]

## Phase 2: Skill artifacts

- [x] [serial] Rename the repository-maintained skill source to `docs/hermes/cairn-workflow-skill.md`. r[cairn.supersede.hermes_skill.canonical_cairn]
- [x] [serial] Update the repo skill source with current Cairn-native lifecycle, MCP, receipt, release-readiness, Tracey, Slotcar, and Nix guidance. r[cairn.supersede.hermes_skill.canonical_cairn]
- [x] [serial] Create/update live Hermes `cairn-workflow` and redirect the old local `ironspec-workflow` skill. r[cairn.supersede.hermes_skill.redirect]

## Phase 3: Verification

- [x] [serial] Run Cairn validation and proposal/design/tasks gates. r[cairn.supersede.hermes_skill.canonical_cairn]
- [x] [serial] Verify the live `cairn-workflow` skill loads and excludes transient secrets/progress. r[cairn.supersede.hermes_skill.redirect]
- [x] [serial] Sync/archive the change, rerun validation/Tracey/diff checks, commit, and push. r[cairn.supersede.hermes_skill.redirect]

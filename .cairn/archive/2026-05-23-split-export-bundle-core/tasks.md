# Tasks

- [x] [serial] Split export-bundle pure-core implementation into focused modules while preserving the `verified::content_addressed` facade. r[cli_maintainability.export_bundle_core_modules.focused_modules]
- [x] [serial] Keep public API, receipt hash material, schema constants, issue codes/messages, and fixture validation behavior unchanged. r[cli_maintainability.export_bundle_core_modules.behavior_preserved]
- [x] [serial] Add durable Tracey implementation and verification refs for the split. r[cli_maintainability.export_bundle_core_modules.focused_modules.narrow_bridges]
- [x] [serial] Run focused content-addressed core and export-bundle CLI fixture checks. r[cli_maintainability.export_bundle_core_modules.behavior_preserved.focused_checks]
- [x] [serial] Run full workspace checks, Tracey coverage, and flake confidence before archive/commit. r[cli_maintainability.export_bundle_core_modules.behavior_preserved.repository_checks]

# Design: Semantic-review core boundary split

Keep `verified::semantic_review::mod.rs` as the public facade for schema constants, public DTOs, and re-exports. Move the inline `#[cfg(test)]` module into `semantic_review/tests.rs` so semantic-review receipt, completeness, metrics-regression, and evidence-boundary regressions remain compiled only for tests but no longer dominate the facade review surface.

The split must preserve:

- public constants and type names;
- public function re-exports;
- existing issue codes, receipt hashes, and schema names;
- existing Tracey implementation and verification refs;
- existing focused and full core verification behavior.

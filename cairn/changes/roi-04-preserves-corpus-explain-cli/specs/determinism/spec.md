# Determinism Specification Delta

## ADDED Requirements

### Requirement: Preserves corpus has a read-only explain command
r[cairn.determinism.preserves_corpus_explain_cli.command] Cairn MUST provide a read-only command that explains Preserves corpus cases as deterministic operator evidence.

#### Scenario: Explain command is read-only and deterministic
r[cairn.determinism.preserves_corpus_explain_cli.read_only]
- GIVEN a Preserves corpus file
- WHEN an operator runs `cairn preserves-corpus explain --path <corpus.json>`
- THEN Cairn SHALL emit deterministic JSON with case ids, descriptions, expected hash metadata, issue codes, and receipt hash
- AND it SHALL NOT mutate the corpus file.

#### Scenario: Explain command has positive and negative tests
r[cairn.determinism.preserves_corpus_explain_cli.tests]
r[cairn.determinism.preserves_corpus_explain_cli.validation]
- GIVEN valid and malformed corpus fixtures
- WHEN focused CLI tests and Cairn lifecycle validation run
- THEN read-only behavior, malformed failures, duplicate-case issues, Tracey coverage, and diff checks SHALL pass.

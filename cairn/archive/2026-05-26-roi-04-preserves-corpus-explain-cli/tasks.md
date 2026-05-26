# Tasks

## 1. Explain command

- [x] I1 [serial] Add `preserves-corpus explain --path <corpus.json>` as a read-only deterministic command. r[cairn.determinism.preserves_corpus_explain_cli.command]
- [x] I2 [serial] Reuse corpus parsing/validation while preventing mutation paths. r[cairn.determinism.preserves_corpus_explain_cli.read_only]

## 2. Verification

- [x] V1 [serial] Add positive, malformed, duplicate-case, and no-mutation tests. r[cairn.determinism.preserves_corpus_explain_cli.tests]
- [x] V2 [serial] Run focused CLI tests, Cairn validation, Tracey, and diff checks. r[cairn.determinism.preserves_corpus_explain_cli.validation]

# Design: Preserves workflow test split

Create a dedicated integration test target or module, likely `tests/preserves_workflow_docs.rs`, containing documentation/runbook coverage assertions. Leave `tests/preserves_corpus.rs` focused on corpus command behavior.

Verification should prove both test targets run independently and all Tracey refs remain live.

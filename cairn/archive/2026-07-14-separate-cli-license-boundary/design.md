## Context

The existing functional-core / imperative-shell architecture is also the correct licensing seam. The pure core is consumed as a Rust library; the CLI owns filesystem, process, MCP, mutation, and presentation behavior.

## Decisions

### Preserve the permissive core

The workspace default remains `MIT OR Apache-2.0`, and `cairn-core` continues to inherit it. Copyable templates, examples, schemas, and generated lifecycle starter content remain permissive so using Cairn does not determine the consumer repository's license.

### Apply AGPL to the shell

`cairn-cli` overrides the workspace default with `AGPL-3.0-or-later`. The CLI can link the permissive core, while the inverse dependency remains forbidden by the architecture.

### Make file-level boundaries reviewable

A root license boundary document maps package and copied-content surfaces to complete checked-in license texts. Third-party dependencies retain upstream terms.

### Keep output claims narrow

Ordinary validation reports, receipts, and user-authored lifecycle content are not automatically covered merely because Cairn processed them. Content copied from shipped templates remains available under the template's permissive terms.

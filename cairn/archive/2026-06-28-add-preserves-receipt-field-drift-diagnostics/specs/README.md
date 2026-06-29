# add-preserves-receipt-field-drift-diagnostics specs

Adds `cairn.determinism.preserves_receipt_field_drift` requirements: a
read-only `cairn receipt-preserves-diff` command that maps a Preserves hash
delta between two receipt snapshots to sorted field paths and source
requirement IDs, satisfying the diagnostic-mapping clause of the
`format_strategy.preserves_candidate` promotion gate.

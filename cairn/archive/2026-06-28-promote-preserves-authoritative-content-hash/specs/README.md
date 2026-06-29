# promote-preserves-authoritative-content-hash specs

Adds `cairn.determinism.preserves_authoritative_content_hash` requirements: an
authoritative, additive `preserves_content_hash` field computed from canonical
Preserves bytes over receipt content (excluding hash fields), alongside the
unchanged JSON `receipt_hash`, plus a `cairn receipt verify-content-hash`
recheck. Depends on `add-preserves-receipt-field-drift-diagnostics`, which
satisfies the diagnostic-mapping clause of the `format_strategy.preserves_candidate`
promotion gate.

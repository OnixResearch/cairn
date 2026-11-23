# Tasks

- [x] [serial] [task:manifest] Add the versioned comparison manifest, pinned upstream identity, and declared semantic projection. r[cairn.trace_index.buoy_comparison.pinned_identity] r[cairn.trace_index.buoy_comparison.projection]
- [x] [serial] [task:fixtures] [after:manifest] Retain positive and negative fixture roots for clean, missing marker, duplicate marker, moved span, changed requirement body, and schema/version drift. r[cairn.trace_index.buoy_comparison.mutation]
- [x] [serial] [task:evaluator] [after:fixtures] Implement closed manifest validation, native observation comparison, report identity separation, and restated-observation rejection. r[cairn.trace_index.buoy_comparison.projection] r[cairn.trace_index.buoy_comparison.mutation]
- [x] [serial] [task:execution] [after:evaluator] Execute the ordinary trace-index commands on bounded temporary fixture copies and retain raw observations. r[cairn.trace_index.buoy_comparison.boundary]
- [x] [serial] [task:check] [after:execution] Add the dedicated pinned Nix comparison check and consumer-visible integration tests. r[cairn.trace_index.buoy_comparison.mutation]
- [x] [serial] [task:report] [after:check] Retain the comparison receipt, unsupported cases, known disagreements, and the measured reuse decision. r[cairn.trace_index.buoy_comparison.boundary]

## Validation and completion

- [x] [serial] [task:focused] [after:report] Run the focused comparison integration tests and the dedicated Nix check. r[cairn.trace_index.buoy_comparison.mutation]
- [x] [serial] [task:independence] [after:focused] Run existing trace-index checks unchanged and confirm they remain independent. r[cairn.trace_index.buoy_comparison.boundary]
- [x] [serial] [task:lifecycle] [after:independence] Run policy freshness and repository lifecycle checks, then sync and archive through native lifecycle. r[cairn.trace_index.buoy_comparison.projection]

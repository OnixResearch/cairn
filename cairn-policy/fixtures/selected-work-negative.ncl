# Negative selected-work fixture: the positive exported obligation section
# relabelled with a forged "satisfied" state must be refused rather than read
# as a satisfied obligation set.
let selected = import "../../contracts/selected-work.ncl" in
let positive = import "selected-work-positive.ncl" in
std.record.update "state" 'satisfied positive.exported_section
| selected.contracts.obligations

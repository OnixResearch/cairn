## MODIFIED Requirements

### Requirement: Requirement identity controls sync merge

r[cairn.sync_archive.delta_merge.identity] Cairn MUST resolve accepted and delta requirement blocks by one canonical bare requirement ID per block and MUST refuse a merge whose result would keep or create a duplicated requirement ID.

#### Scenario: Renamed modified requirement replaces by ID

r[cairn.sync_archive.delta_merge.identity.rename]
- GIVEN an accepted requirement and a MODIFIED operation share one requirement ID but use different headings
- WHEN Cairn applies the operation
- THEN the old block MUST be replaced by the new block
- AND the accepted spec MUST contain exactly one block with that requirement ID

#### Scenario: Duplicate identity blocks the plan

r[cairn.sync_archive.delta_merge.identity.duplicate]
- GIVEN accepted content or delta operations contain the same canonical requirement ID more than once
- AND no reviewed operation repairs every accepted duplicate
- WHEN Cairn builds the sync plan
- THEN the plan MUST report every conflicting location and MUST NOT write any destination

#### Scenario: Missing or multiple block identities fail

r[cairn.sync_archive.delta_merge.identity.malformed]
- GIVEN a requirement operation has no bare requirement ID or has more than one candidate bare requirement ID
- WHEN Cairn parses the delta
- THEN the operation MUST be rejected with a deterministic identity diagnostic

#### Scenario: Duplicated accepted ID is repaired by unique heading

r[cairn.sync_archive.delta_merge.identity.duplicate.repair]
- GIVEN an accepted spec contains one requirement ID in two blocks with distinct headings
- AND a MODIFIED operation carries a different requirement ID and the exact heading of one of those blocks
- WHEN Cairn applies the operation
- THEN it MUST replace that block and leave the remaining blocks unchanged
- AND it MUST refuse the merge when the heading matches more than one accepted block, when the merged document still contains a duplicated ID, or when the merge would create a new duplicated ID

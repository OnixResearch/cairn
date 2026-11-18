# Task-scoped MCP discovery delta

## ADDED Requirements

### Requirement: Explicit policy-owned tool profiles
r[cairn.mcp_discovery.profiles] The system MUST select bounded task-specific tool catalogs through explicit checked policy profiles while preserving the existing full-catalog default.

#### Scenario: A supported task profile selects its required tools
- GIVEN an admitted inspection, authoring, or verification profile
- WHEN Cairn lists tools
- THEN it returns the policy-ordered required tool set with an exact catalog identity

#### Scenario: Unknown profiles do not fall back silently
- GIVEN an unknown profile or invalid policy selection
- WHEN MCP startup admits the request
- THEN it fails before tool dispatch instead of returning an unrelated catalog

### Requirement: Exact strict schema lookup
r[cairn.mcp_discovery.lookup] The system MUST provide bounded lookup for one admitted tool descriptor without changing that tool's input schema, validation, or execution semantics.

#### Scenario: One descriptor matches the full catalog
- GIVEN a known tool admitted by the selected profile and current capability set
- WHEN CLI or MCP requests its descriptor
- THEN the schema and annotations equal the corresponding full-catalog descriptor

#### Scenario: Unknown or unavailable tools cannot expose executable authority
- GIVEN an unknown tool, a tool outside the selected profile, or a mutation tool without capability
- WHEN discovery or dispatch receives its name
- THEN it returns a typed unavailable result before product observation or execution

### Requirement: Stable catalog pagination
r[cairn.mcp_discovery.pages] The system MUST paginate tool descriptors under explicit named bounds and bind cursors to the catalog, profile, capability set, and ordering.

#### Scenario: Pages contain every admitted tool exactly once
- GIVEN an unchanged admitted catalog and valid continuations
- WHEN a client reads all pages
- THEN their ordered union equals the full selected catalog without duplicates

#### Scenario: Stale or invalid continuations cannot change the tool set
- GIVEN a changed profile, policy, capability set, or malformed cursor
- WHEN the client requests another page
- THEN the server rejects continuation rather than mixing catalog versions

#### Scenario: Paging is opt-in and terminal pages are nonempty
- GIVEN no bounded listing selection
- WHEN a client calls tools/list without a cursor
- THEN the full selected catalog retains the unpaged tools result with no nextCursor
- GIVEN explicit --paged-tools and a valid admitted catalog
- WHEN a client consumes its last page
- THEN the result contains descriptors and omits nextCursor, and continuations at or past the terminal position fail

#### Scenario: Listing requests respect named policy bounds
- GIVEN explicit --paged-tools and unchanged policy
- WHEN the client lists descriptors
- THEN each serialized result including nextCursor respects page-byte and entry bounds, every descriptor respects its bound, and oversized cursors or unknown request fields fail before product observation


### Requirement: Discovery never expands mutation authority
r[cairn.mcp_discovery.authority] The system MUST keep catalog selection and lookup read-only and preserve capability-first mutation denial and strict tool arguments.

#### Scenario: A mutation-enabled profile still needs explicit startup capability
- GIVEN a profile that can include authoring tools and explicit mutation capability
- WHEN the server admits its catalog
- THEN it exposes only the tools permitted by both facts and retains original execution preflight

#### Scenario: A discovered name does not authorize a denied call
- GIVEN absent mutation capability and a malformed mutation request
- WHEN dispatch evaluates the call under any profile
- THEN capability denial occurs before argument-dependent product effects with mutated as false

### Requirement: Host-specific discovery evidence
r[cairn.mcp_discovery.conformance] The system MUST compare the declared eager-schema and lazy-discovery presentations over retained fixtures, record the descriptors and discovery calls each presentation exposes, and keep live token accounting explicitly unavailable when no usage or exact tokenizer observation is supplied.

#### Scenario: Declared catalog strategies are compared on retained fixtures
- GIVEN the same fixture set except the declared catalog strategy
- WHEN both presentations are observed
- THEN it records the descriptors exposed, the discovery calls required, and unchanged outcomes

#### Scenario: Extra discovery costs remain visible
- GIVEN a lazy host that already avoids unused schemas or a profile that omits a needed diagnostic tool
- WHEN the benchmark evaluates the candidate
- THEN it retains extra calls and failures and withholds unsupported improvement claims

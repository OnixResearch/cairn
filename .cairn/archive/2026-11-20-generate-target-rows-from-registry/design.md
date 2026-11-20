## Context

`contracts/assurance-targets.ncl` is the consumer-facing authoring contract vendored by Octet. A target carries `requirement`, the six content-bound registry row fields, and a contract carries `specification {path, blake3}`. Both are currently copied from `cairn requirement-registry export`; the archived `add-assurance-targets-and-convergence` design names that copy as the remaining hand-maintained lock.

`crates/cairn-core/src/verified/assurance_targets.rs` already re-verifies every target's six fields and each contract's specification against the registry, so a derived row gains no new admission path. The change is authoring-side only: pure Nickel helpers, fixtures that prove them, and guidance.

## Decisions

### Decision: Select by requirement id, fail closed

**Choice:** `requirement_from_registry registry requirement_id` filters the export's `rows` on `requirement_id`. `specification_from_registry registry requirement_id` reads the same selected row's `specification_path` and `specification_blake3`. Both require exactly one match: zero or several rows fail with a contract error naming the id. Before filtering, both require `registry.schema_version` to equal `cairn.content-bound-requirement-registry.v1`, the identifier the Rust registry already admits, so a foreign document cannot be selected from.

**Rationale:** The registry is the authority for both values; a target that names an id the export does not contain exactly once must not silently bind another row or an empty record. Validating the schema first keeps the helper's domain identical to the Rust admission's domain.

### Decision: Return the contract, not a raw row

**Choice:** `requirement_from_registry` applies `RowContract` to the projected six fields and `specification_from_registry` applies the contract's `specification` shape. The helpers are exported beside `Row`, `Target`, `Contract`, `TargetsFile`, `GeneratedSources` and `project`.

**Rationale:** A derived value is held to the same shape as an authored one, so a registry export with a malformed digest or span fails at authoring time rather than at Rust admission.

### Decision: Rust admission stays unchanged

**Choice:** No Rust changes. `assurance_targets` still re-verifies all six fields and the contract specification against the registry for every target, whether the target was copied or derived by Nickel.

**Rationale:** The helper only writes the same six values a consumer would copy; the existing binding check is the gate. Adding a second admission path would weaken, not strengthen, the check.

### Decision: Byte-identical positive fixture

**Choice:** `fixtures/assurance-targets/registry-derived/` holds a synthetic registry export carrying the same two synthetic rows as the octet example, and a targets file that derives both targets and the contract specification from it. The flake check exports it, sorts with `jq -S` like every other fixture, and requires `cmp` equality with `fixtures/assurance-targets/octet-example/targets.json`. Three negatives (missing id, duplicate id, wrong schema) make `nickel export` fail and are wired into the existing negative loop.

**Rationale:** Byte equality proves the derivation changes no admitted value. The negatives are the three ways selection can fail closed.

## Risks / Trade-offs

- The synthetic registry export's envelope identities are placeholders; the helpers validate only the schema identifier and the selected row's shape, so the fixture documents that boundary.
- A consumer must re-export the registry and re-run Nickel to refresh a derived row; a stale checked-in export still fails the existing Rust binding check.

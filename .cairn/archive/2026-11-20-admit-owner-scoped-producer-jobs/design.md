# Design

## Context

The assurance-targets policy admits every foreign schema identifier that
convergence binds. Producer jobs are the one admitted identifier that is owned
by a foreign producer rather than by Cairn, and each producer owner authors its
own job contract. Today `schemas.octet_producer_job` hardcodes Octet's
`octet-mantle-producer-job/v1`, and the CLI requires it before any binding runs.

Mantle's verification bridge checks a producer job against its owner and
schema: `octet-job` requires owner `Octet`, and the WP8 `trellis-job` mode
requires owner `Trellis` and `trellis-mantle-producer-job/v1`. Cairn binds the
same `job.json`, so the job's `owner` is authoritative and must select the
admitted schema.

## Decisions

### Decision: One owner-scoped map, not a list of identifiers

**Choice:** `schemas.producer_jobs` is an owner-to-schema map with the admitted
entries `octet = "octet-mantle-producer-job/v1"` and
`trellis = "trellis-mantle-producer-job/v1"`.

**Rationale:** The map states the pair the job must present, not two unrelated
strings. A new producer owner is one reviewed policy entry; no comparison code
changes. A list of identifiers would force Cairn to guess which schema belongs
to which owner, and accepting any admitted schema for any owner would admit an
Octet-schema job that claims to be Trellis.

### Decision: Owner keys match without ASCII case

**Choice:** The job's `owner` value selects its entry with
`eq_ignore_ascii_case`; policy keys stay lower case.

**Rationale:** Owners author their jobs and write `owner = "Octet"` or
`owner = "Trellis"`, which is what Mantle requires. Matching exactly would
require either capitalised policy keys or a capitalisation change in the job,
and the job contract belongs to the producer. Case folding is limited to ASCII
and does not admit a different owner name.

### Decision: Three stable rejection codes, at both boundaries

**Choice:** The owner admission reports `convergence.missing_producer_owner`,
`convergence.unknown_producer_owner` and `convergence.producer_schema_mismatch`.
The CLI rejects a handoff with the same codes and exit status 1; the pure
evaluator marks the row `incomplete`, `unsupported` or `stale` respectively so a
state decision also fails closed.

**Rationale:** A missing owner is a missing input (`incomplete`). An owner the
policy does not admit is a value outside the admitted vocabulary
(`unsupported`). A `schema_version` that does not equal the schema admitted for
the owner is a binding mismatch (`stale`). Naming the codes and their states
keeps them stable for consumers and tests.

### Decision: Typed policy map, checked once at parse

**Choice:** The Nickel contract requires every map entry to be a non-empty
string. Rust rejects an empty map, an empty owner key or schema value, and two
keys that name the same owner without regard to ASCII case, all with
`assurance_targets.invalid_producer_jobs`.

**Rationale:** A map that admits nothing cannot bind any job, and case-duplicate
owners make admission depend on record iteration order. Both are policy defects,
and neither falls back to a default.

### Decision: Retain the pilot bytes and add a synthetic Trellis case

**Choice:** `fixtures/convergence/pilot/` is not edited. The synthetic CLI corpus
gains one Octet-owner job, one Trellis-owner job and three rejected jobs; the
core tests cover the same cases over constructed facts.

**Rationale:** The retained Octet corpus is exercised evidence and must stay
`current` to prove the map preserves existing admission. A synthetic Trellis case
proves the new owner binds without claiming an exercised Trellis realization;
the real WP8 run lands with Terrane in stage 2.

# Verification guidance

Guidance recommends checks from an explicit source comparison. It does not run checks, authorize mutation, or prove that omitted checks are safe.

## Request guidance

1. Prepare a source comparison under `contracts/verification-impact.ncl`.
2. Include fresh before and after trace indexes for the declared source projection.
3. Run the read-only command:

```text
cairn verification-impact guidance --root . --input comparison.json --change demo
```

The MCP tool is `cairn_verification_impact`. Its `input` property contains the same source comparison, not a file path.
Both transports return the same report for identical inputs and request arguments.

To request a producer plan, supply all three producer input paths:

```text
cairn verification-impact guidance --root . --input comparison.json \
  --contract contract.json --profiles profiles.json --applicability applicability.json
```

Guidance returns this command as a request:

```text
cairn check-evidence plan --root . --contract contract.json --profiles profiles.json --applicability applicability.json
```

Each action contains an exact `argv` array and an `arguments` object. The `command` string is for display only.
Guidance does not read the producer files or execute the action. It does not select a subset by rewriting the producer contract.
The operator must review the request. Original producer admission and full gates remain authoritative.
Partial producer requests and execution flags fail through both transports. Invalid guidance inputs return no actions.

## Optional structural observations

The CLI accepts `--evidence evidence.json`. MCP accepts the same object in its optional `evidence` property.
The object follows `structural_evidence` in `contracts/verification-impact.ncl`.
It binds the after-source identity, repository, target, features, pinned producer revision, and canonical receipt hash.
These bindings are declarations. They do not prove producer authenticity or that the producer observed the declared source.

Cairn reuses its existing `check_evidence` translator for `octet-acceptance-receipt/v1`.
The translator preserves native status and bounded diagnostic counts. Every Octet observation remains completion-ineligible.
Receipt, source, scope, and producer substitutions block admission. Observations never reduce the check selection.

The reviewed Valence revision is `e40c76b4d2070a29636e00c85c0dff93f03dba2f`.
That revision has no `valence.evidence-admission.v1` span contract.
Cairn's adoption consumer recognizes that schema for declared identity and roles, not source spans.
The separate stack-provenance sidecar does not fill this gap.

Without compatible span evidence, the report retains `declared_links_only` scope and reports `valence_span_contract_unavailable`.
Present Valence evidence blocks admission until a reviewed compatible span contract exists. Cairn does not silently discard unsupported evidence.
No producer pin changed for this feature.

## Selection and refresh limits

Deletion uses before-source links. Rename and copied-marker cases preserve changed paths.
Missing edges, mappings, macro coverage, or generated-source coverage retain broader default checks.
Cycles, stale indexes, and target or feature drift block admission. Mandatory checks remain in every valid selection.

The complete required set remains separate from the focused selection.
Changed whole-subject identity makes prior completion evidence stale, including source changes without marker links.
A current applicability result does not establish receipt authenticity, successful execution, or release eligibility.

The integration suite rebuilds indexes from fresh, complete Cairn source bytes in a declared one-file projection.
Its comparison appends a comment in memory. It does not modify the repository source.
This bounded replay does not establish coverage of the complete repository or measure a speed improvement.

<!-- r[verify cairn.verification_impact.interfaces] -->
<!-- r[verify cairn.verification_impact.conformance] -->

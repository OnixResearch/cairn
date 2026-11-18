# Opt-in profile only. This standalone export does not change existing gate policy.
# Proposed mappings are NOT approvals. Retain independently reviewed records separately.
let Profile = {
  id = "cairn.requirement-semantics.finite.v1",
  interpretation = "finite-enumeration.equality.nominal-domains.total-observables.single-step.v1",
  max_nodes = 4096, max_depth = 64, max_domain_values = 256, max_steps = 1000000,
} in
let val = fun domain_name value_text => { domain = domain_name, value = value_text } in
let lit = fun domain_name value_text => { kind = "literal", value = val domain_name value_text } in
let var = fun variable_name => { kind = "variable", name = variable_name } in
let obs = fun mapping_name arguments_list => { kind = "observable", mapping = mapping_name, arguments = arguments_list } in
let eq = fun left_side right_side => { kind = "equal", left = left_side, right = right_side } in
let truth = { kind = "bool", value = true } in
let yes = lit "bool" "true" in
let is_true = fun mapping_name arguments_list => eq (obs mapping_name arguments_list) yes in
let all = fun expressions_list => { kind = "and", expressions = expressions_list } in
let implies = fun premise_expr conclusion_expr => { kind = "implies", premise = premise_expr, conclusion = conclusion_expr } in
let each = fun variable_name domain_name expression_expr => { kind = "forall", variable = variable_name, domain = domain_name, expression = expression_expr } in
let exists = fun variable_name domain_name expression_expr => { kind = "exists", variable = variable_name, domain = domain_name, expression = expression_expr } in
let mapping = fun arguments_list result_name phase_name source_path projection_text => {
  arguments = arguments_list, result = result_name, phase = phase_name, source = source_path, projection = projection_text,
} in
let Boolean = { values = ["false", "true"] } in
let Identity = { values = ["a", "b", "absent"] } in
let Checks = { values = ["check-a", "check-b", "unknown"] } in
let Rows = { values = ["row-a", "row-b"] } in
let Completion = "crates/cairn-core/src/verified/agent_receipt/authoritative.rs" in
let Workflow = "crates/cairn-core/src/verified/workflow_collaboration_impl.rs" in
# Identity tokens are injective labels within a supplied finite example, not digest syntax checks.
# Row slots enumerate a bounded result universe; presence distinguishes absent slots.
let make = fun native_row domains_set observables_set activation_expr guarantee_expr supported_text remainder_text => {
  profile = Profile.id, proposed_by = "cairn-typed-requirements-author",
  requirement = native_row, domains = domains_set, observables = observables_set,
  assumptions = [], activation = activation_expr, guarantee = guarantee_expr,
  supported_portion = supported_text, informal_remainder = remainder_text,
} in
let Row0 = { "requirement_id" = "cairn.supersede.agent_run_receipts.verification_contract", "specification_path" = ".cairn/specs/agent-completion/spec.md", "specification_blake3" = "18b48ac73b3632a800d40d818697dc05e282383dbf9f149aa6f77a3f396fe405", "source_span" = { "start_byte" = 184, "end_byte" = 466, "start_line" = 11, "end_line" = 11 }, "requirement_body_blake3" = "6a30659535a7f87cce3625cab7db6bdc8ede8be27f95eed08d5ad5860bfebb8b", "row_blake3" = "cab928f04ff3d61d2ea9e2a2640262e2022dc3ba820d1949fad73d8d6dc977af" } in
let Row1 = { "requirement_id" = "cairn.supersede.agent_run_receipts.pass_only", "specification_path" = ".cairn/specs/agent-completion/spec.md", "specification_blake3" = "18b48ac73b3632a800d40d818697dc05e282383dbf9f149aa6f77a3f396fe405", "source_span" = { "start_byte" = 1992, "end_byte" = 2206, "start_line" = 38, "end_line" = 38 }, "requirement_body_blake3" = "7c55795bcf82118d164643e0ad3e57494d3eccc26cfd5300db11da82b44357d3", "row_blake3" = "34abbfe64cae739b75554f48d89d0e0bc19ca8e55d54cf51a9363ffe84a6f58d" } in
let Row2 = { "requirement_id" = "cairn.supersede.agent_run_receipts.freshness", "specification_path" = ".cairn/specs/agent-completion/spec.md", "specification_blake3" = "18b48ac73b3632a800d40d818697dc05e282383dbf9f149aa6f77a3f396fe405", "source_span" = { "start_byte" = 4080, "end_byte" = 4280, "start_line" = 74, "end_line" = 74 }, "requirement_body_blake3" = "be11d3a5ca69b23a147df33c11a591998537fc649cd8e8f9aed6801dbb07fe3f", "row_blake3" = "653914140db582ef1145f36f87c233a9ab57a310f715cadf8a23efa5f5333e5e" } in
let Row3 = { "requirement_id" = "cairn.supersede.agent_run_receipts.completion_authority", "specification_path" = ".cairn/specs/agent-completion/spec.md", "specification_blake3" = "18b48ac73b3632a800d40d818697dc05e282383dbf9f149aa6f77a3f396fe405", "source_span" = { "start_byte" = 7484, "end_byte" = 7737, "start_line" = 127, "end_line" = 127 }, "requirement_body_blake3" = "2b32b4728e4d48cd067823340e77bb5d92a6d1106f44747c2c9066c21a8d4568", "row_blake3" = "5427af464b1ebc00db599a9ec90cdee48d0ce7b4a0b2652c6dc1d134bd89451b" } in
let Row4 = { "requirement_id" = "cairn.verification_obligation_cob.claim_revision", "specification_path" = ".cairn/specs/verification-obligation-radicle-cob/spec.md", "specification_blake3" = "56240c57533326ccee1e6fe41c57408db9d47f24226ec22f3643f365eeff517f", "source_span" = { "start_byte" = 3392, "end_byte" = 3646, "start_line" = 65, "end_line" = 65 }, "requirement_body_blake3" = "97a773f0545b2e3aa12cd5b0cbdafe2698381256f3de1878a809332a381008b6", "row_blake3" = "1de40130cc0a88bb88288b16daa621de599b1508511648332e9e23ef11a3b666" } in

let IdentityContract = make Row0
  { identity = Identity, component = { values = ["repository", "registry_id", "registry_hash", "selection", "policy", "subject", "required_checks", "commands", "prerequisites"] }, bool = Boolean }
  {
    repository_contract_present = mapping [] "bool" "state" Completion "AgentVerificationContract deserialization succeeds for repository-owned contract supplied independently of result rows; false for missing/unparseable contract",
    repository_identity = mapping ["component"] "identity" "state" Completion "component selects exact AgentVerificationContract.repository_id, registry_id, registry_hash, selection_hash, policy_hash, final_subject_hash, or canonical required_checks/check_id set, command argv/hash map, prerequisites edge set; injectively label exact values",
    decision_identity = mapping ["component"] "identity" "state" Completion "same components in independently normalized repository decision inputs/report; registry/selection/policy/subject also compared to AgentExecutionResult fields; no result row may define repository_identity",
  }
  truth
  (all [is_true "repository_contract_present" [], each "component" "component" (eq (obs "repository_identity" [var "component"]) (obs "decision_identity" [var "component"]))])
  "Finite exact equality of independently supplied repository verification contract identities, including required-set, command and prerequisite identities. Missing repository contract is distinguished from self-authored passing rows."
  "Repository ownership/authenticity, canonicalization implementation, cryptographic hash syntax/collision resistance, arbitrary registry size, command execution and full diagnostic classes remain outside this finite interpretation."
in
let CoverageContract = make Row1
  { bool = Boolean, check = Checks, row = Rows, status = { values = ["passed", "failed", "blocked", "skipped", "deferred", "timed_out", "cancelled", "dry_run", "cleanup_failed", "infrastructure_error", "not_selected", "unknown"] } }
  {
    selected = mapping ["check"] "bool" "state" Completion "membership of check_id in AgentVerificationContract.required_checks; unknown sentinel is never selected",
    present = mapping ["row"] "bool" "state" Completion "whether result slot exists in supplied Vec<AgentExecutionResult>; two slots permit duplicate IDs and missing rows without collapsing them",
    row_check = mapping ["row"] "check" "state" Completion "AgentExecutionResult.check_id mapped injectively to selected IDs or unknown sentinel",
    row_status = mapping ["row"] "status" "state" Completion "AgentExecutionResult.status exact string; unsupported strings are rejected by adapter, never mapped to passed",
    eligible = mapping ["row"] "bool" "state" Completion "row satisfies command argv/hash, registry/selection/policy/subject equality, required evidence/artifact hash syntax and every prerequisite edge passing as checked by evaluate_authoritative_agent_completion; this is a supplied observation, not an uninterpreted predicate assumed true",
  }
  truth
  (all [
    each "row" "row" (implies (is_true "present" [var "row"]) (all [is_true "selected" [obs "row_check" [var "row"]], eq (obs "row_status" [var "row"]) (lit "status" "passed"), is_true "eligible" [var "row"]])),
    each "check" "check" (implies (is_true "selected" [var "check"]) (exists "row" "row" (all [
      is_true "present" [var "row"], eq (obs "row_check" [var "row"]) (var "check"),
      each "other" "row" (implies (all [is_true "present" [var "other"], eq (obs "row_check" [var "other"]) (var "check")]) (eq (var "other") (var "row"))),
    ]))),
  ])
  "Exactly one present eligible passed result per selected required check in the declared finite universe, with no unknown result rows. A duplicate passed row is not equivalent to one passed row."
  "The eligible observation summarizes existing native validation rather than proving it: prerequisite graph traversal, argv/digest validation and external evidence truth remain informal. The model covers at most two result slots and two known checks, not arbitrary cardinalities."
in
let FreshnessContract = make Row2
  { bool = Boolean, identity = Identity, row = Rows }
  {
    eligible = mapping ["row"] "bool" "state" Completion "membership of a supplied result row in completion-eligible required passed results; absent slots are false",
    result_subject = mapping ["row"] "identity" "state" Completion "AgentExecutionResult.subject_hash exact identity; absent slots may use absent token",
    final_subject = mapping [] "identity" "after" Completion "evaluate_authoritative_agent_completion current_subject_hash argument after repository mutation",
    contract_subject = mapping [] "identity" "state" Completion "AgentVerificationContract.final_subject_hash",
    receipt_subject = mapping [] "identity" "state" Completion "AgentAuthoritativeCompletionReport.final_subject_hash included in canonical_report_hash",
  }
  truth
  (all [eq (obs "contract_subject" []) (obs "final_subject" []), eq (obs "receipt_subject" []) (obs "final_subject" []), each "row" "row" (implies (is_true "eligible" [var "row"]) (eq (obs "result_subject" [var "row"]) (obs "final_subject" [])))])
  "Equality of selected finite result, contract, aggregate receipt and final post-mutation subject identities. A changed final subject distinguishes stale from current evidence."
  "Filesystem subject measurement, mutation ordering, command/evidence/artifact hash syntax, hash authenticity and arbitrary result universes are not established by token equality."
in
let AuthorityContract = make Row3
  { bool = Boolean, identity = Identity, check = Checks }
  {
    completion_claimed = mapping [] "bool" "state" Completion "consumer requests authoritative completion or represents an agent task as complete; a caller-supplied assertion, not model text interpreted as authority",
    receipt_present = mapping [] "bool" "state" Completion "a supplied AgentAuthoritativeCompletionReport exists, independently of process exit, task counters, checkboxes or model text",
    receipt_passing = mapping [] "bool" "state" Completion "AgentAuthoritativeCompletionReport.valid is true AND verdict is PASS AND classification is authoritative AND schema is current; adapter must not label legacy receipt passing",
    receipt_subject = mapping [] "identity" "state" Completion "AgentAuthoritativeCompletionReport.final_subject_hash, absent token when receipt absent",
    final_subject = mapping [] "identity" "after" Completion "evaluate_authoritative_agent_completion current_subject_hash after final mutation",
    selected = mapping ["check"] "bool" "state" Completion "membership of check_id in independently repository-selected AgentVerificationContract.required_checks",
    covered = mapping ["check"] "bool" "state" Completion "membership of check_id in AgentAuthoritativeCompletionReport.required_checks AND passed_checks, with no native coverage diagnostics; false if receipt absent",
  }
  (is_true "completion_claimed" [])
  (all [is_true "receipt_present" [], is_true "receipt_passing" [], eq (obs "receipt_subject" []) (obs "final_subject" []), each "check" "check" (implies (is_true "selected" [var "check"]) (is_true "covered" [var "check"]))])
  "Authoritative completion assertion requires a present current passing native receipt for the final subject and all selected finite checks; successful prose/process observations cannot replace these fields."
  "Receipt origin/authenticity, schema/receipt-hash validation implementation, wrapper honesty, policy correctness, arbitrary check sets and broader release authority remain outside the finite model."
in
let RevisionContract = make Row4
  { bool = Boolean, identity = Identity, operation = { values = ["operation-a", "operation-b"] }, lifecycle = { values = ["open", "discharged", "rejected", "invalidated"] } }
  {
    revision_before = mapping [] "identity" "before" Workflow "WorkflowState.claim_root.revision before resolve_field; injective token for the exact revision number, not arithmetic",
    revision_after = mapping [] "identity" "after" Workflow "WorkflowState.claim_root.revision after admitted resolve_field; a different exact revision has a different token",
    lifecycle_after = mapping [] "lifecycle" "after" Workflow "WorkflowState.lifecycle.status after resolve_field",
    operation_before = mapping ["operation"] "bool" "before" Workflow "membership of operation ID in WorkflowState.operations before admitted claim field revision",
    operation_after = mapping ["operation"] "bool" "after" Workflow "membership of the same operation ID in WorkflowState.operations after admitted claim field revision",
  }
  { kind = "not", expression = eq (obs "revision_before" []) (obs "revision_after" []) }
  (all [eq (obs "lifecycle_after" []) (lit "lifecycle" "open"), each "operation" "operation" (implies (is_true "operation_before" [var "operation"]) (is_true "operation_after" [var "operation"]))])
  "Single admitted claim revision changes the exact review epoch, resets lifecycle to open (so a prior discharge is not inherited), and preserves the selected finite operation history."
  "Current WorkflowState does not carry individual claim-root fields on every evidence link, finding and decision. That stronger prose portion is not established here; per-fact root binding, retained full claim payloads, authorized complete-basis revision admission, concurrent heads, numeric increment/overflow and arbitrary history remain informal."
in
let Contracts = [IdentityContract, CoverageContract, FreshnessContract, AuthorityContract, RevisionContract] in
{
  profile = Profile,
  contracts = Contracts,
  proposed_correspondence = std.array.map (fun contract => {
    requirement_id = contract.requirement.requirement_id,
    proposed_by = contract.proposed_by,
    rationale = contract.supported_portion ++ " INFORMAL REMAINDER: " ++ contract.informal_remainder,
  }) Contracts,
  authorized_reviewers = ["parent-independent-review-2026-11-18"],
  reviews = [],
} | {
  profile | { id | String, interpretation | String, max_nodes | Number, max_depth | Number, max_domain_values | Number, max_steps | Number },
  contracts | Array Dyn,
  proposed_correspondence | Array { requirement_id | String, proposed_by | String, rationale | String },
  authorized_reviewers | Array String,
  reviews | Array Dyn,
}

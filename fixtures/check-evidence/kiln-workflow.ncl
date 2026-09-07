# Cairn-owned native DTO fixture; kiln-core's original validator admits it.
# This records no real workflow execution.
let ContentIdentity = "0000000000000000000000000000000000000000000000000000000000000000" in
let reference = fun kind => { contract_version = 1, provider_namespace = "lattice", native_kind = kind, native_id = kind } in
let MaximumNodes = 8 in
let MaximumEdges = 16 in
let MaximumLogReferences = 8 in
let MaximumLogReadBytes = 4096 in
let MaximumTotalLogBytes = 16384 in
let linkage = {
  effect_id = "effect", attempt_id = "attempt", candidate_id = "candidate",
  pipeline_revision_id = "pipeline", workflow_policy_id = "policy",
  provider_profile = "lattice-v1", idempotency_key = ContentIdentity,
} in
{
  plan = linkage & {
    platform = "linux-x86_64", source = null,
    nodes = [{ id = "test", kind = "test", effect_class = "Test", adapter_kind = "local", operation_identity = ContentIdentity }],
    edges = [], controls = { cancel = true, pause = true, resume = true },
    bounds = { max_nodes = MaximumNodes, max_edges = MaximumEdges, max_log_references = MaximumLogReferences,
      max_log_read_bytes = MaximumLogReadBytes, max_total_log_bytes = MaximumTotalLogBytes },
  },
  observation = linkage & {
    workflow = reference "workflow", execution = reference "execution", sequence = 4,
    state = "Completed", terminal = true, successful = true, outcome = "Success",
    nodes = [{ node_id = "test", state = "Completed", terminal = true, successful = true,
      adapter = reference "adapter", receipt = reference "receipt" }],
    logs = [], receipt = reference "receipt",
  },
}

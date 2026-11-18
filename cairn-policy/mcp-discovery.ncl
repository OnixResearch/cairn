# r[impl cairn.mcp_discovery.profiles]
# r[impl cairn.mcp_discovery.pages]
let ReadTool = fun tool_id => { name = tool_id, requires_mutations = false } in
let MutationTool = fun tool_id => { name = tool_id, requires_mutations = true } in
let Membership = { name | String, requires_mutations | Bool } in
let Profile = { name | String, tools | Array Membership, required_routes | Array String } in
let Inspect = std.array.map ReadTool [
  "cairn_status", "cairn_list_content", "cairn_read_spec", "cairn_change_show",
  "cairn_context", "cairn_workflow_status", "cairn_workflow_instructions",
  "cairn_doctor", "cairn_gate_context", "cairn_change_graph",
  "cairn_readiness", "cairn_implementation_ready", "cairn_tool_describe",
] in
let AllRead = std.array.map ReadTool [
  "cairn_status", "cairn_list_content", "cairn_read_spec", "cairn_change_show",
  "cairn_context", "cairn_workflow_status", "cairn_workflow_instructions",
  "cairn_doctor", "cairn_validate", "cairn_gate", "cairn_gate_context",
  "cairn_change_graph", "cairn_agent_receipt_validate", "cairn_agent_completion_validate",
  "cairn_readiness", "cairn_verification_impact", "cairn_adoption_report",
  "cairn_drain_plan", "cairn_implementation_ready", "cairn_sync_plan",
  "cairn_archive_plan", "cairn_migrate_plan",
] in
let Mutations = std.array.map MutationTool [
  "cairn_init", "cairn_change_create", "cairn_sync_execute",
  "cairn_archive_execute", "cairn_migrate_execute",
] in
{
  schema_version = "cairn.mcp-discovery.v1",
  limits = {
    descriptor_bytes | Number = 65536,
    page_bytes | Number = 131072,
    page_entries | Number = 8,
    cursor_bytes | Number = 256,
    tool_name_bytes | Number = 128,
  },
  profiles | Array Profile = [
    {
      name = "full",
      tools = AllRead @ Mutations @ [ReadTool "cairn_tool_describe"],
      required_routes = ["cairn_doctor", "cairn_gate_context", "cairn_sync_plan", "cairn_archive_plan", "cairn_migrate_plan"],
    },
    {
      name = "inspect", tools = Inspect,
      required_routes = ["cairn_doctor", "cairn_gate_context", "cairn_workflow_instructions"],
    },
    {
      name = "author",
      tools = Inspect @ std.array.map ReadTool ["cairn_validate", "cairn_gate", "cairn_sync_plan", "cairn_archive_plan", "cairn_migrate_plan"] @ Mutations,
      required_routes = ["cairn_doctor", "cairn_gate_context", "cairn_workflow_instructions", "cairn_sync_plan", "cairn_archive_plan", "cairn_migrate_plan"],
    },
    {
      name = "verify",
      tools = std.array.map ReadTool [
        "cairn_status", "cairn_read_spec", "cairn_change_show", "cairn_context",
        "cairn_doctor", "cairn_validate", "cairn_gate", "cairn_gate_context",
        "cairn_change_graph", "cairn_agent_receipt_validate", "cairn_agent_completion_validate",
        "cairn_readiness", "cairn_verification_impact", "cairn_adoption_report",
        "cairn_drain_plan", "cairn_implementation_ready", "cairn_sync_plan",
        "cairn_archive_plan", "cairn_tool_describe",
      ],
      required_routes = ["cairn_doctor", "cairn_gate_context", "cairn_sync_plan", "cairn_archive_plan"],
    },
  ],
}

use super::*;

pub(super) fn run_mcp_json_rpc(
    catalog: &cairn_core::verified::mcp_discovery::ToolCatalog,
    paged_tools: bool,
) -> Result<(), Box<dyn Error>> {
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    for line in stdin.lock().lines() {
        let line = line?;
        if line.trim().is_empty() {
            continue;
        }
        let request: Value = serde_json::from_str(&line)?;
        let id = request.get("id").cloned().unwrap_or(Value::Null);
        let method = request["method"].as_str().unwrap_or_default();
        let response = match method {
            "initialize" => json!({
                "jsonrpc": "2.0", "id": id,
                "result": {"protocolVersion": MCP_PROTOCOL_VERSION, "serverInfo": {"name": "cairn", "version": env!("CARGO_PKG_VERSION")}, "capabilities": {"tools": {}}}
            }),
            "tools/list" => match catalog.list(request.get("params"), paged_tools) {
                Ok(result) => json!({"jsonrpc": "2.0", "id": id, "result": result}),
                Err(error) => {
                    json!({"jsonrpc": "2.0", "id": id, "error": {"code": -32602, "message": error.to_string()}})
                }
            },
            "tools/call" => {
                let params = &request["params"];
                let name = params["name"].as_str().unwrap_or_default();
                let arguments = params.get("arguments").unwrap_or(&Value::Null);
                match mcp_call_tool_in_catalog(name, arguments, catalog) {
                    Ok(result) => json!({"jsonrpc": "2.0", "id": id, "result": result}),
                    Err(error) => {
                        if let Some(
                            cairn_core::verified::mcp_discovery::DiscoveryError::Unavailable(tool),
                        ) = error.downcast_ref()
                        {
                            json!({"jsonrpc":"2.0", "id":id, "error":{
                                "code":-32000, "message":error.to_string(),
                                "data":{"code":"tool_unavailable", "tool":tool, "allowed":false, "mutated":false}
                            }})
                        } else {
                            json!({"jsonrpc": "2.0", "id": id, "error": {"code": -32000, "message": error.to_string()}})
                        }
                    }
                }
            }
            _ => {
                json!({"jsonrpc": "2.0", "id": id, "error": {"code": -32601, "message": format!("unknown method: {method}")}})
            }
        };
        writeln!(stdout, "{}", serde_json::to_string(&response)?)?;
        stdout.flush()?;
    }
    Ok(())
}

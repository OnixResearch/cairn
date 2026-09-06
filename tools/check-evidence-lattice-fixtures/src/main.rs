//! Disposable fixture producer, not an execution or completion producer.
//! Input is the pinned producer's exported request-valid.ncl. The original
//! producer library binds identities and validates every generated observation.
use lattice_workflow_contract::*;
use serde_json::{Value, json};
use std::{error::Error, fs};
const DIGITS: usize = 64;
const MAX_NODES: usize = 8;
const MAX_EDGES: usize = 16;
const MAX_LOG_REFERENCES: usize = 8;
const MAX_LOG_READ_BYTES: usize = 4096;
const MAX_TOTAL_LOG_BYTES: usize = 16384;
fn id(digit: char) -> Identity {
    Identity::new(format!("b3:{}", digit.to_string().repeat(DIGITS))).expect("identity")
}
fn main() -> Result<(), Box<dyn Error>> {
    let published_path = std::env::args()
        .nth(2)
        .ok_or("expected original producer fixture")?;
    let published: Value = serde_json::from_slice(&fs::read(published_path)?)?;
    let p: CapabilityProfile = serde_json::from_value(published["profile"].clone())?;
    let r: WorkflowRequest = serde_json::from_value(published["request"].clone())?;
    let a: WorkflowAccepted = serde_json::from_value(published["accepted"].clone())?;
    let o: ExecutionObservation = serde_json::from_value(published["observation"].clone())?;
    validate_observation(&p, &r, &a, &o).expect("published native positive");
    let cyclic: WorkflowRequest =
        serde_json::from_value(published["rejected"]["cyclic_request"].clone())?;
    assert!(validate_request(&p, &cyclic).is_err());
    for name in ["false_paused_success", "stale_execution"] {
        let bad: ExecutionObservation =
            serde_json::from_value(published["rejected"][name].clone())?;
        assert!(validate_observation(&p, &r, &a, &bad).is_err());
    }
    let source = std::env::args()
        .nth(1)
        .ok_or("expected exported native request")?;
    let mut request: WorkflowRequest = serde_json::from_slice(&fs::read(source)?)?;
    let profile: CapabilityProfile = serde_json::from_value(json!({
        "schema":PROFILE_SCHEMA,"graph_schema":GRAPH_SCHEMA,"platforms":["linux-x86_64"],
        "node_kinds":["build"],"effect_classes":["build"],"adapter_kinds":["local"],
        "controls":{"cancel":true,"pause":true,"resume":true},
        "limits":{"maximum_nodes":MAX_NODES,"maximum_edges":MAX_EDGES,"maximum_log_references":MAX_LOG_REFERENCES,"maximum_log_read_bytes":MAX_LOG_READ_BYTES,"maximum_total_log_bytes":MAX_TOTAL_LOG_BYTES},
        "non_claims":REQUIRED_NON_CLAIMS
    }))?;
    request.profile_identity = profile_identity(&profile);
    request.graph.identity = graph_identity(&request.graph);
    request.identity = request_identity(&request);
    let accepted = WorkflowAccepted {
        schema: ACCEPTED_SCHEMA.into(),
        request_identity: request.identity.clone(),
        idempotency_key: request.idempotency_key.clone(),
        workflow_identity: id('9'),
        execution_identity: id('a'),
    };
    let mut observation: ExecutionObservation = serde_json::from_value(json!({
        "schema":OBSERVATION_SCHEMA,"identity":id('0'),"request_identity":request.identity,
        "idempotency_key":request.idempotency_key,"workflow_identity":accepted.workflow_identity,"execution_identity":accepted.execution_identity,
        "sequence":7,"state":"completed","terminal":true,"successful":true,
        "nodes":[{"node_id":"build","state":"completed","terminal":true,"successful":true,"adapter_identity":id('b'),"receipt_identity":id('c')}],
        "logs":[{"node_id":"build","stream":"stdout","reference_identity":id('d'),"cursor":0,"bytes":32,"truncated":false,"gap_before":false,"retained":true}],
        "receipt_identity":id('e')
    }))?;
    observation.identity = observation_identity(&observation);
    validate_observation(&profile, &request, &accepted, &observation)
        .expect("original native validation");
    let positive =
        json!({"profile":profile,"request":request,"accepted":accepted,"observation":observation});
    let mut negatives: Vec<Value> = Vec::new();
    for (pointer, value) in [
        ("/successful", json!(false)),
        ("/request_identity", json!(id('f'))),
        ("/nodes/0/receipt_identity", Value::Null),
        ("/logs/0/bytes", json!(MAX_LOG_READ_BYTES + 1)),
        ("/terminal", json!(false)),
    ] {
        let mut bad = serde_json::to_value(&observation)?;
        *bad.pointer_mut(pointer).ok_or("pointer")? = value;
        let mut bad: ExecutionObservation = serde_json::from_value(bad)?;
        bad.identity = observation_identity(&bad);
        assert!(validate_observation(&profile, &request, &accepted, &bad).is_err());
        let mut bundle = positive.clone();
        bundle["observation"] = serde_json::to_value(bad)?;
        negatives.push(bundle);
    }
    let mut skipped = positive.clone();
    observation.nodes[0].state = NodeState::Skipped;
    observation.nodes[0].receipt_identity = None;
    observation.identity = observation_identity(&observation);
    validate_observation(&profile, &request, &accepted, &observation)
        .expect("skipped is natively successful");
    skipped["observation"] = serde_json::to_value(observation)?;
    println!(
        "{}",
        serde_json::to_string_pretty(
            &json!({"positive":positive,"skipped":skipped,"negative":negatives})
        )?
    );
    Ok(())
}

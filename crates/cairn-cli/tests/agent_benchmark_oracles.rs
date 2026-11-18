// r[verify cairn.agent_efficiency.outcomes]
// r[verify cairn.agent_efficiency.conformance]
use cairn_core::canonical_json_hash;
use serde_json::{json, Value};
use std::{
    error::Error,
    fs,
    io::{BufRead, BufReader, Write},
    path::PathBuf,
    process::{Child, Command, Stdio},
    sync::atomic::{AtomicU64, Ordering},
    time::{Duration, Instant},
};

static NEXT_ROOT: AtomicU64 = AtomicU64::new(0);
const SUBJECT: &str = "cccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccc";

struct FreshRoot(PathBuf);
impl FreshRoot {
    fn new() -> Result<Self, Box<dyn Error>> {
        let path = repository().join("target").join(format!(
            "benchmark-oracle-{}-{}",
            std::process::id(),
            NEXT_ROOT.fetch_add(1, Ordering::Relaxed)
        ));
        fs::create_dir(&path)?;
        Ok(Self(path))
    }

    fn observe(&self, args: &[&str]) -> Result<(Value, u64), Box<dyn Error>> {
        let started = Instant::now();
        let output = Command::new(env!("CARGO_BIN_EXE_cairn"))
            .current_dir(&self.0)
            .env(
                "CAIRN_POLICY",
                repository().join("cairn-policy/generated/cairn-policy.json"),
            )
            .args(args)
            .output()?;
        let elapsed = u64::try_from(started.elapsed().as_millis())?;
        assert!(
            output.status.success(),
            "{}",
            String::from_utf8_lossy(&output.stderr)
        );
        Ok((serde_json::from_slice(&output.stdout)?, elapsed))
    }

    fn interrupted_mcp(&self) -> Result<(Value, u64), Box<dyn Error>> {
        let started = Instant::now();
        let mut session = LiveSession(
            Command::new(env!("CARGO_BIN_EXE_cairn"))
                .current_dir(&self.0)
                .env(
                    "CAIRN_POLICY",
                    repository().join("cairn-policy/generated/cairn-policy.json"),
                )
                .arg("mcp")
                .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .stderr(Stdio::null())
                .spawn()?,
        );
        let stdin = session.0.stdin.as_mut().ok_or("MCP stdin")?;
        writeln!(stdin, r#"{{"jsonrpc":"2.0","id":1,"method":"tools/list"}}"#)?;
        stdin.flush()?;
        let stdout = session.0.stdout.take().ok_or("MCP stdout")?;
        let (send, receive) = std::sync::mpsc::channel();
        std::thread::spawn(move || {
            let mut line = String::new();
            let observed = BufReader::new(stdout).read_line(&mut line).map(|_| line);
            let _ = send.send(observed);
        });
        let line = receive.recv_timeout(Duration::from_secs(10))??;
        let startup: Value = serde_json::from_str(&line)?;
        assert_eq!(startup["id"], 1);
        assert_eq!(startup["error"], Value::Null);
        session.0.kill()?;
        let termination = session.0.wait()?;
        assert!(!termination.success());
        Ok((
            json!({
                "observation":"interrupted-native-mcp-session",
                "startup_response":startup,
                "termination":{"success":termination.success(),"exit_code":termination.code()}
            }),
            u64::try_from(started.elapsed().as_millis())?,
        ))
    }
}
impl Drop for FreshRoot {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

struct LiveSession(Child);
impl Drop for LiveSession {
    fn drop(&mut self) {
        let _ = self.0.kill();
        let _ = self.0.wait();
    }
}
fn repository() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}
fn diagnostics_contain(receipt: &Value, code: &str) -> bool {
    receipt["completion"]["diagnostics"]
        .as_array()
        .is_some_and(|rows| rows.iter().any(|row| row["code"] == code))
}
fn saved_run(
    case: &str,
    observed: &(Value, u64),
    verdict: &str,
    claimed: bool,
    cohort: &Value,
) -> Value {
    let call_id = canonical_json_hash(&json!({"case":case,"native_observation":observed.0}));
    json!({
        "run_id":canonical_json_hash(&json!({"case":case,"call":call_id})),
        "cohort":cohort,
        "trace":{
            "declared_calls":1,"complete":true,
            "calls":[{"call_id":call_id,"sequence":0,"usage":null,"content_identity":null}],
            "retry_count":0,"tool_call_count":1,"repeated_read_count":0,"duration_ms":observed.1
        },
        "oracle":{"verdict":verdict,"agent_claimed_completion":claimed}
    })
}

#[test]
fn native_fresh_root_oracles_prevent_claimed_or_structural_success_from_becoming_completion(
) -> Result<(), Box<dyn Error>> {
    let fixture = FreshRoot::new()?;
    let contract = fs::read(repository().join("fixtures/agent-completion/contract.json"))?;
    let results = fs::read(repository().join("fixtures/agent-completion/results.json"))?;
    fs::write(fixture.0.join("contract.json"), &contract)?;
    fs::write(fixture.0.join("results.json"), &results)?;
    let complete_args = [
        "agent-receipt",
        "complete",
        "--contract",
        "contract.json",
        "--results",
        "results.json",
        "--subject-hash",
        SUBJECT,
    ];
    let completed = fixture.observe(&complete_args)?;
    assert_eq!(completed.0["valid"], true);
    assert_eq!(completed.0["completion"]["verdict"], "PASS");
    assert_eq!(completed.0["checks_executed"], false);

    let stale = fixture.observe(&[
        "agent-receipt",
        "complete",
        "--contract",
        "contract.json",
        "--results",
        "results.json",
        "--subject-hash",
        "dddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddd",
    ])?;
    assert_eq!(stale.0["valid"], false);
    assert!(diagnostics_contain(
        &stale.0,
        "verification_contract.subject_stale"
    ));
    assert_eq!(
        stale.0["completion"]["passed_checks"],
        json!(["clippy", "tests"])
    );

    fs::write(fixture.0.join("results.json"), b"[]\n")?;
    let correctly_blocked = fixture.observe(&complete_args)?;
    assert_eq!(correctly_blocked.0["valid"], false);
    assert!(diagnostics_contain(
        &correctly_blocked.0,
        "verification_result.missing"
    ));

    let change = fixture.0.join(".cairn/changes/demo");
    fs::create_dir_all(&change)?;
    fs::write(
        change.join("tasks.md"),
        "# Tasks\n\n- [x] Claim complete without required artifacts or evidence.\n",
    )?;
    let failed = fixture.observe(&["gate", "tasks", "demo", "--root", "."])?;
    assert_eq!(failed.0["valid"], false);
    assert_eq!(failed.0["verdict"], "FAIL");

    let unavailable = fixture.observe(&[
        "readiness",
        "demo",
        "--root",
        ".",
        "--contract",
        "contract.json",
        "--results",
        "absent.json",
        "--subject-hash",
        SUBJECT,
    ])?;
    let completion_stage = unavailable.0["stages"]
        .as_array()
        .ok_or("stages")?
        .iter()
        .find(|stage| stage["stage"] == "completion")
        .ok_or("completion stage")?;
    assert_eq!(completion_stage["state"], "unavailable");
    assert_eq!(completion_stage["decision"]["evaluated"], false);
    let interrupted = fixture.interrupted_mcp()?;

    let mut binary = blake3::Hasher::new();
    binary.update_reader(fs::File::open(env!("CARGO_BIN_EXE_cairn"))?)?;
    let mut fixture_hash = blake3::Hasher::new();
    fixture_hash.update(b"cairn.agent-benchmark.native-oracle-fixture.v1\0");
    for bytes in [&contract, &results] {
        fixture_hash.update(&u64::try_from(bytes.len())?.to_le_bytes());
        fixture_hash.update(bytes);
    }
    let oracle = blake3::hash(include_bytes!("agent_benchmark_oracles.rs"))
        .to_hex()
        .to_string();
    let fixture_identity = fixture_hash.finalize().to_hex().to_string();
    let cohort = json!({
        "source_projection":canonical_json_hash(&json!({"fixture":fixture_identity,"oracle":oracle})),
        "product_revision":binary.finalize().to_hex().to_string(),
        "policy":completed.0["current_policy_hash"],"fixture":fixture_identity,"oracle":oracle,
        "model":canonical_json_hash(&json!({"kind":"no-model-native-command-replay"})),
        "agent_configuration":canonical_json_hash(&json!({"observer":"repository-owned-oracle-v1"})),
        "catalog":canonical_json_hash(&json!(["agent-receipt complete","gate tasks","readiness","mcp tools/list"])),
        "presentation":canonical_json_hash(&json!({"format":"native-json"})),
        "accounting":{"method":"provider_usage","usage_source":canonical_json_hash(&json!({"usage":"not-supplied"}))}
    });
    let observation = json!({
        "schema":"cairn.agent-benchmark.input.v1",
        "scenario":{"scope":"native-oracle-controls","mode":"representation_replay","repetitions":5,"max_calls_per_run":1},
        "runs":[
            saved_run("completed", &completed, "completed", true, &cohort),
            saved_run("expected-missing-check-block", &correctly_blocked, "correctly_blocked", false, &cohort),
            saved_run("false-stale-success", &stale, "rejected", true, &cohort),
            saved_run("invalid-checked-artifacts", &failed, "rejected", false, &cohort),
            saved_run("interrupted-native-mcp-session", &interrupted, "inconclusive", true, &cohort)
        ]
    });
    fs::write(
        fixture.0.join("observation.json"),
        serde_json::to_vec(&observation)?,
    )?;
    let report =
        fixture.observe(&["agent-benchmark", "validate", "--input", "observation.json"])?;
    assert_eq!(
        report.0["outcomes"],
        json!({
            "completed":1,"correctly_blocked":1,"false_completion":1,"failed":1,"incomplete":1
        })
    );
    assert_eq!(report.0["totals"]["complete_cost"], false);
    assert_eq!(report.0["tokens_per_correct_completion"], Value::Null);
    assert_eq!(report.0["tokens_per_correct_block"], Value::Null);
    Ok(())
}

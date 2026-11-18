// r[verify cairn.agent_efficiency.import]
// r[verify cairn.agent_efficiency.accounting]
// r[verify cairn.agent_efficiency.cohort]
// r[verify cairn.agent_efficiency.outcomes]
use serde_json::{json, Value};
use std::{
    error::Error,
    fs,
    path::{Path, PathBuf},
    process::{Command, Output},
    sync::atomic::{AtomicU64, Ordering},
    time::{SystemTime, UNIX_EPOCH},
};

static NEXT_FIXTURE: AtomicU64 = AtomicU64::new(0);

struct Fixture(PathBuf);

impl Fixture {
    fn new() -> Result<Self, Box<dyn Error>> {
        let nonce = SystemTime::now().duration_since(UNIX_EPOCH)?.as_nanos();
        let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../../target")
            .join(format!(
                "agent-benchmark-{}-{nonce}-{}",
                std::process::id(),
                NEXT_FIXTURE.fetch_add(1, Ordering::Relaxed)
            ));
        fs::create_dir_all(&path)?;
        Ok(Self(path))
    }

    fn write(&self, name: &str, value: &Value) -> Result<(), Box<dyn Error>> {
        fs::write(self.0.join(name), serde_json::to_vec(value)?)?;
        Ok(())
    }

    fn run(&self, args: &[&str]) -> Result<Output, Box<dyn Error>> {
        Ok(Command::new(env!("CARGO_BIN_EXE_cairn"))
            .current_dir(Path::new(env!("CARGO_MANIFEST_DIR")).join("../.."))
            .args(args)
            .arg("--root")
            .arg(&self.0)
            .output()?)
    }
}

impl Drop for Fixture {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

fn hex(digit: char) -> String {
    digit.to_string().repeat(64)
}

fn call(
    id: char,
    sequence: u64,
    uncached: u64,
    cached: u64,
    output: u64,
    reasoning: u64,
    inclusion: &str,
) -> Value {
    json!({
        "call_id":hex(id), "sequence":sequence,
        "usage":{"uncached_input":uncached,"cached_input":cached,"output":output,
            "reasoning":reasoning,"reasoning_inclusion":inclusion},
        "content_identity":null
    })
}

fn run(id: char, revision: char, verdict: &str, claimed: bool, calls: Vec<Value>) -> Value {
    let (retry_count, tool_call_count, repeated_read_count) = if calls.len() > 1 {
        (1, 2, 1)
    } else {
        (0, 1, 0)
    };
    json!({
        "run_id":hex(id),
        "cohort":{
            "source_projection":hex('a'),"product_revision":hex(revision),"policy":hex('b'),
            "fixture":hex('c'),"oracle":hex('d'),"model":hex('e'),
            "agent_configuration":hex('f'),"catalog":hex('1'),"presentation":hex('2'),
            "accounting":{"method":"provider_usage","usage_source":hex('3')}
        },
        "trace":{"declared_calls":calls.len(),"complete":true,"calls":calls,
            "retry_count":retry_count,"tool_call_count":tool_call_count,
            "repeated_read_count":repeated_read_count,"duration_ms":40},
        "oracle":{"verdict":verdict,"agent_claimed_completion":claimed}
    })
}

fn input(revision: char) -> Value {
    json!({
        "schema":"cairn.agent-benchmark.input.v1",
        "scenario":{"scope":"fresh-authoring","mode":"live_host","repetitions":2,"max_calls_per_run":3},
        "runs":[
            run('4', revision, "completed", true, vec![
                call('6',0,10,3,5,2,"additional_to_output"),
                call('7',1,4,1,6,0,"additional_to_output")
            ]),
            run('5', revision, "correctly_blocked", false, vec![
                call('8',0,8,2,4,0,"additional_to_output")
            ])
        ]
    })
}

fn comparison(baseline: Value, candidate: Value) -> Value {
    json!({
        "schema":"cairn.agent-benchmark.comparison.v1",
        "allowed_candidate_differences":["product_revision"],
        "baseline":baseline,"candidate":candidate
    })
}

fn accepted(output: &Output) -> Result<Value, Box<dyn Error>> {
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(output.stderr.is_empty());
    Ok(serde_json::from_slice(&output.stdout)?)
}

fn rejected(output: &Output, class: &str) {
    assert!(!output.status.success());
    assert!(output.stdout.is_empty());
    assert_eq!(
        String::from_utf8_lossy(&output.stderr).trim(),
        format!("error: {class}")
    );
}

#[test]
fn exact_accounting_classification_and_machine_framing() -> Result<(), Box<dyn Error>> {
    let fixture = Fixture::new()?;
    fixture.write("observation.json", &input('9'))?;
    let args = ["agent-benchmark", "validate", "--input", "observation.json"];
    let direct = fixture.run(&args)?;
    let report = accepted(&direct)?;
    assert_eq!(report["outcomes"]["completed"], 1);
    assert_eq!(report["outcomes"]["correctly_blocked"], 1);
    assert_eq!(report["outcomes"]["false_completion"], 0);
    assert_eq!(report["attempted_runs"], 2);
    assert_eq!(report["totals"]["uncached_input"], 22);
    assert_eq!(report["totals"]["cached_input"], 6);
    assert_eq!(report["totals"]["output"], 15);
    assert_eq!(report["totals"]["reasoning"], 2);
    assert_eq!(report["totals"]["reasoning_complete"], true);
    assert_eq!(report["totals"]["billed_output"], 17);
    assert_eq!(report["totals"]["billed_total"], 45);
    assert_eq!(report["totals"]["complete_cost"], true);
    assert_eq!(
        report["tokens_per_correct_completion"]["attempted_tokens"],
        45
    );
    assert_eq!(
        report["tokens_per_correct_completion"]["correct_completions"],
        1
    );
    assert_eq!(report["tokens_per_correct_block"]["attempted_tokens"], 45);
    assert_eq!(direct.stdout, fixture.run(&args)?.stdout);
    let machine = fixture.run(&[
        "--machine",
        "agent-benchmark",
        "validate",
        "--input",
        "observation.json",
    ])?;
    let envelope = accepted(&machine)?;
    assert_eq!(envelope["schema_version"], "cairn.cli-machine.v1");
    assert_eq!(envelope["command"], "agent-benchmark");
    assert_eq!(envelope["result"], report);
    assert_eq!(
        fs::read_dir(&fixture.0)?.count(),
        1,
        "validation must not write files"
    );
    Ok(())
}

#[test]
fn paired_live_comparison_preserves_completion_and_scopes_lower_cost() -> Result<(), Box<dyn Error>>
{
    let fixture = Fixture::new()?;
    let baseline = input('9');
    let mut candidate = input('0');
    candidate["runs"][0]["trace"]["calls"][0]["usage"]["uncached_input"] = json!(9);
    candidate["runs"][1]["trace"]["calls"][0]["usage"]["uncached_input"] = json!(7);
    fixture.write("comparison.json", &comparison(baseline, candidate))?;
    let report =
        accepted(&fixture.run(&["agent-benchmark", "compare", "--input", "comparison.json"])?)?;
    assert_eq!(report["comparable"], true);
    assert_eq!(report["baseline"]["totals"]["billed_total"], 45);
    assert_eq!(report["candidate"]["totals"]["billed_total"], 43);
    assert_eq!(report["baseline"]["outcomes"]["completed"], 1);
    assert_eq!(report["candidate"]["outcomes"]["completed"], 1);
    assert_eq!(report["verdict"], "improved");
    assert_eq!(
        report["scoped_token_change"]["baseline_attempted_tokens"],
        45
    );
    assert_eq!(
        report["scoped_token_change"]["candidate_attempted_tokens"],
        43
    );
    Ok(())
}

#[test]
fn lower_tokens_cannot_hide_false_completion_or_cohort_drift() -> Result<(), Box<dyn Error>> {
    let fixture = Fixture::new()?;
    let baseline = input('9');
    let mut candidate = input('0');
    candidate["runs"][0]["run_id"] = json!(hex('0'));
    candidate["runs"][1]["run_id"] = json!(hex('1'));
    candidate["runs"][0]["oracle"] = json!({"verdict":"rejected","agent_claimed_completion":true});
    candidate["runs"][0]["trace"]["calls"][0]["usage"] = json!({
        "uncached_input":2,"cached_input":0,"output":1,"reasoning":0,
        "reasoning_inclusion":"additional_to_output"
    });
    fixture.write(
        "comparison.json",
        &comparison(baseline.clone(), candidate.clone()),
    )?;
    let report =
        accepted(&fixture.run(&["agent-benchmark", "compare", "--input", "comparison.json"])?)?;
    assert_eq!(report["comparable"], true);
    assert_eq!(report["candidate"]["outcomes"]["false_completion"], 1);
    assert_eq!(report["candidate"]["outcomes"]["completed"], 0);
    assert!(
        report["candidate"]["totals"]["billed_total"]
            .as_u64()
            .unwrap()
            < 45
    );
    assert_eq!(report["verdict"], "withheld");
    assert!(report["scoped_token_change"].is_null());

    candidate["runs"][0]["cohort"]["oracle"] = json!(hex('7'));
    candidate["runs"][1]["cohort"]["oracle"] = json!(hex('7'));
    fixture.write("comparison.json", &comparison(baseline, candidate))?;
    let drift =
        accepted(&fixture.run(&["agent-benchmark", "compare", "--input", "comparison.json"])?)?;
    assert_eq!(drift["comparable"], false);
    assert_eq!(drift["verdict"], "withheld");
    assert!(drift["scoped_token_change"].is_null());
    assert!(drift["drift_fields"]
        .as_array()
        .is_some_and(|fields| !fields.is_empty()));
    let mut stale = input('0');
    stale["runs"][0]["cohort"]["source_projection"] = json!(hex('7'));
    stale["runs"][1]["cohort"]["source_projection"] = json!(hex('7'));
    fixture.write("comparison.json", &comparison(input('9'), stale))?;
    let drift =
        accepted(&fixture.run(&["agent-benchmark", "compare", "--input", "comparison.json"])?)?;
    assert_eq!(drift["comparable"], false);
    assert_eq!(drift["verdict"], "withheld");
    let mut stale = input('0');
    stale["runs"][0]["cohort"]["policy"] = json!(hex('7'));
    stale["runs"][1]["cohort"]["policy"] = json!(hex('7'));
    fixture.write("comparison.json", &comparison(input('9'), stale))?;
    let drift =
        accepted(&fixture.run(&["agent-benchmark", "compare", "--input", "comparison.json"])?)?;
    assert_eq!(drift["comparable"], false);
    assert_eq!(drift["verdict"], "withheld");
    assert!(drift["drift_fields"]
        .as_array()
        .is_some_and(|fields| fields.contains(&json!("policy"))));
    Ok(())
}

#[test]
fn missing_usage_and_incomplete_sequences_do_not_fabricate_cost() -> Result<(), Box<dyn Error>> {
    let fixture = Fixture::new()?;
    let mut observation = input('9');
    observation["runs"][0]["trace"]["calls"][0]["usage"] = Value::Null;
    fixture.write("observation.json", &observation)?;
    let report =
        accepted(&fixture.run(&["agent-benchmark", "validate", "--input", "observation.json"])?)?;
    assert_eq!(report["totals"]["complete_cost"], false);
    assert_eq!(report["runs"][0]["cost_available"], false);
    assert!(report["tokens_per_correct_completion"].is_null());

    observation["runs"][0]["trace"]["calls"]
        .as_array_mut()
        .unwrap()
        .pop();
    fixture.write("observation.json", &observation)?;
    rejected(
        &fixture.run(&["agent-benchmark", "validate", "--input", "observation.json"])?,
        "trace.incomplete",
    );
    observation["runs"][0]["trace"]["complete"] = json!(false);
    fixture.write("observation.json", &observation)?;
    rejected(
        &fixture.run(&["agent-benchmark", "validate", "--input", "observation.json"])?,
        "trace.incomplete",
    );
    Ok(())
}

#[test]
fn optional_reasoning_preserves_exact_inclusive_output_but_not_unknown_additional_cost(
) -> Result<(), Box<dyn Error>> {
    let fixture = Fixture::new()?;
    let mut inclusive = input('9');
    for run in inclusive["runs"].as_array_mut().unwrap() {
        for call in run["trace"]["calls"].as_array_mut().unwrap() {
            call["usage"]["reasoning_inclusion"] = json!("included_in_output");
        }
    }
    inclusive["runs"][0]["trace"]["calls"][0]["usage"]["reasoning"] = Value::Null;
    fixture.write("observation.json", &inclusive)?;
    let report =
        accepted(&fixture.run(&["agent-benchmark", "validate", "--input", "observation.json"])?)?;
    assert_eq!(report["totals"]["reasoning_complete"], false);
    assert_eq!(report["totals"]["complete_cost"], true);
    assert_eq!(report["totals"]["billed_total"], 43);
    assert_eq!(
        report["tokens_per_correct_completion"]["attempted_tokens"],
        43
    );

    let mut additional = input('9');
    additional["runs"][0]["trace"]["calls"][0]["usage"]["reasoning"] = Value::Null;
    fixture.write("observation.json", &additional)?;
    let report =
        accepted(&fixture.run(&["agent-benchmark", "validate", "--input", "observation.json"])?)?;
    assert_eq!(report["totals"]["reasoning_complete"], false);
    assert_eq!(report["totals"]["complete_cost"], false);
    assert!(report["tokens_per_correct_completion"].is_null());
    Ok(())
}

#[test]
fn strict_json_admission_rejects_invalid_duplicate_and_overflow() -> Result<(), Box<dyn Error>> {
    let fixture = Fixture::new()?;
    let limit = cairn_core::verified::agent_benchmark::BenchmarkPolicy::bundled()?
        .bounds
        .max_token_count;
    let mut observation = input('9');
    observation["scenario"]["max_calls_per_run"] = json!(7);
    observation["runs"][0]["trace"]["declared_calls"] = json!(7);
    observation["runs"][0]["trace"]["calls"] = json!(['6', '7', '8', '9', 'a', 'b', 'c']
        .into_iter()
        .enumerate()
        .map(|(sequence, id)| call(
            id,
            sequence as u64,
            limit,
            limit,
            limit,
            0,
            "additional_to_output"
        ))
        .collect::<Vec<_>>());
    fixture.write("observation.json", &observation)?;
    rejected(
        &fixture.run(&["agent-benchmark", "validate", "--input", "observation.json"])?,
        "accounting.overflow",
    );
    let mut mixed = input('9');
    mixed["runs"][0]["trace"]["calls"][1]["usage"]["reasoning_inclusion"] =
        json!("included_in_output");
    fixture.write("observation.json", &mixed)?;
    rejected(
        &fixture.run(&["agent-benchmark", "validate", "--input", "observation.json"])?,
        "accounting.invalid",
    );
    let mut observation = input('9');
    observation["runs"][0]["trace"]["calls"][1]["call_id"] = json!(hex('6'));
    fixture.write("observation.json", &observation)?;
    rejected(
        &fixture.run(&["agent-benchmark", "validate", "--input", "observation.json"])?,
        "trace.duplicate_call",
    );
    let mut observation = input('9');
    observation["schema"] = json!("cairn.agent-benchmark.input.v0");
    fixture.write("observation.json", &observation)?;
    rejected(
        &fixture.run(&["agent-benchmark", "validate", "--input", "observation.json"])?,
        "input.schema",
    );

    let mut observation = input('9');
    observation["runs"][0]["trace"]["calls"][0]["extra_field"] = json!("secret-do-not-echo");
    fixture.write("observation.json", &observation)?;
    rejected(
        &fixture.run(&["agent-benchmark", "validate", "--input", "observation.json"])?,
        "agent_benchmark.input.malformed",
    );
    let serialized = serde_json::to_string(&input('9'))?;
    let duplicated = serialized.replacen(
        "\"schema\":",
        "\"schema\":\"cairn.agent-benchmark.input.v1\",\"schema\":",
        1,
    );
    fs::write(fixture.0.join("observation.json"), duplicated)?;
    rejected(
        &fixture.run(&["agent-benchmark", "validate", "--input", "observation.json"])?,
        "agent_benchmark.input.malformed",
    );
    fs::write(
        fixture.0.join("observation.json"),
        b"secret-do-not-echo malformed JSON",
    )?;
    let output = fixture.run(&["agent-benchmark", "validate", "--input", "observation.json"])?;
    rejected(&output, "agent_benchmark.input.malformed");
    assert!(!String::from_utf8_lossy(&output.stderr).contains("secret-do-not-echo"));
    Ok(())
}

#[test]
fn named_options_and_capability_paths_fail_closed() -> Result<(), Box<dyn Error>> {
    let fixture = Fixture::new()?;
    fixture.write("observation.json", &input('9'))?;
    for args in [
        vec!["agent-benchmark", "validate"],
        vec!["agent-benchmark", "validate", "--input", ""],
        vec!["agent-benchmark", "validate", "--input", "--root"],
        vec![
            "agent-benchmark",
            "validate",
            "--input",
            "observation.json",
            "--input",
            "observation.json",
        ],
        vec!["agent-benchmark", "validate", "--bogus", "observation.json"],
        vec![
            "agent-benchmark",
            "validate",
            "observation.json",
            "--input",
            "observation.json",
        ],
    ] {
        let mut machine_args = vec!["--machine"];
        machine_args.extend(args);
        let output = fixture.run(&machine_args)?;
        let envelope: Value = serde_json::from_slice(&output.stdout)?;
        assert!(!output.status.success());
        assert!(output.stderr.is_empty());
        assert_eq!(envelope["schema_version"], "cairn.cli-machine.v1");
        assert_eq!(envelope["ok"], false);
        assert_eq!(envelope["exit_class"], "invalid_arguments");
        assert_eq!(envelope["result"], Value::Null);
    }
    for name in ["../observation.json", "/tmp/observation.json"] {
        rejected(
            &fixture.run(&["agent-benchmark", "validate", "--input", name])?,
            "agent_benchmark.input.unsafe_path",
        );
    }
    rejected(
        &fixture.run(&["agent-benchmark", "validate", "--input", "absent.json"])?,
        "agent_benchmark.input.unavailable",
    );
    fs::write(fixture.0.join("too-large.json"), vec![b' '; 1_048_577])?;
    rejected(
        &fixture.run(&["agent-benchmark", "validate", "--input", "too-large.json"])?,
        "agent_benchmark.input.oversize",
    );
    Ok(())
}

#[cfg(unix)]
#[test]
fn linked_leaf_parent_and_root_are_not_followed() -> Result<(), Box<dyn Error>> {
    use std::os::unix::fs::symlink;
    let fixture = Fixture::new()?;
    fixture.write("observation.json", &input('9'))?;
    fs::create_dir(fixture.0.join("nested"))?;
    fs::write(
        fixture.0.join("nested/input.json"),
        serde_json::to_vec(&input('9'))?,
    )?;
    symlink("observation.json", fixture.0.join("leaf.json"))?;
    symlink("nested", fixture.0.join("parent"))?;
    for name in ["leaf.json", "parent/input.json"] {
        rejected(
            &fixture.run(&["agent-benchmark", "validate", "--input", name])?,
            "agent_benchmark.input.unavailable",
        );
    }
    let root_link = fixture.0.join("linked-root");
    symlink(&fixture.0, &root_link)?;
    let result = Command::new(env!("CARGO_BIN_EXE_cairn"))
        .args([
            "agent-benchmark",
            "validate",
            "--input",
            "observation.json",
            "--root",
        ])
        .arg(&root_link)
        .output()?;
    rejected(&result, "agent_benchmark.root.unavailable");
    Ok(())
}

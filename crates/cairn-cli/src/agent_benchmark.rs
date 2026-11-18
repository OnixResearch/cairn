//! Bounded offline import for saved agent benchmark observations.
// r[impl cairn.agent_efficiency.import]
use std::{error::Error, io, path::Path};

use cairn_core::verified::agent_benchmark::{
    compare, validate, BenchmarkInput, BenchmarkPolicy, ComparisonInput,
};

use crate::support::{print_json, CapabilityRoot, CapabilityRootKind};

pub(crate) fn run(args: Vec<String>) -> Result<(), Box<dyn Error>> {
    let operation = match args.first().map(String::as_str) {
        Some("validate") => "validate",
        Some("compare") => "compare",
        _ => return Err("agent_benchmark.command.invalid".into()),
    };
    if args.len() < 3 || args.len() > 5 || !(args.len() - 1).is_multiple_of(2) {
        return Err("agent_benchmark.arguments.invalid".into());
    }
    let (mut input, mut root) = (None, None);
    for pair in args[1..].chunks_exact(2) {
        let value = pair[1].as_str();
        if value.is_empty() || value.starts_with('-') {
            return Err("agent_benchmark.arguments.invalid".into());
        }
        let slot = match pair[0].as_str() {
            "--input" => &mut input,
            "--root" => &mut root,
            _ => return Err("agent_benchmark.arguments.invalid".into()),
        };
        if slot.replace(value).is_some() {
            return Err("agent_benchmark.arguments.invalid".into());
        }
    }
    let input = input.ok_or("agent_benchmark.arguments.invalid")?;
    let policy = BenchmarkPolicy::bundled()?;
    let capability = CapabilityRoot::open_nofollow(
        CapabilityRootKind::Repository,
        Path::new(root.unwrap_or(".")),
    )
    .map_err(|_| "agent_benchmark.root.unavailable")?;
    let relative = capability
        .relative(Path::new(input))
        .map_err(|_| "agent_benchmark.input.unsafe_path")?;
    let bytes = capability
        .read_bounded(&relative, policy.max_input_bytes() as u64)
        .map_err(|error| match error.kind() {
            io::ErrorKind::InvalidData => "agent_benchmark.input.oversize",
            _ => "agent_benchmark.input.unavailable",
        })?;
    let report = match operation {
        "validate" => {
            let observation: BenchmarkInput =
                serde_json::from_slice(&bytes).map_err(|_| "agent_benchmark.input.malformed")?;
            serde_json::to_value(validate(&policy, &observation)?)?
        }
        "compare" => {
            let observation: ComparisonInput =
                serde_json::from_slice(&bytes).map_err(|_| "agent_benchmark.input.malformed")?;
            serde_json::to_value(compare(&policy, &observation)?)?
        }
        _ => unreachable!("operation checked above"),
    };
    print_json(report)
}

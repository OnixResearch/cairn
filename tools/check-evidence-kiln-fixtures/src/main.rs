//! Export original private Kiln test fixtures without rewriting their values.
//! The build script reads the exact published Git object. No upstream source
//! is checked into Cairn and no producer schema or implementation changes.
extern crate alloc;
use kiln_core::{domain::Outcome, types::*, workflow::*};
use std::{error::Error, process::Command};
const REVISION: &str = "5d99f4e9bb954743443e340baece31256b4c53f1";
mod producer {
    include!(concat!(env!("OUT_DIR"), "/producer-tests.rs"));
    pub fn export() -> serde_json::Value {
        let plan = plan();
        let observation = completed(&plan);
        assert_eq!(
            admit_workflow_observation(&plan, &observation),
            Ok(Outcome::Success)
        );
        let mut stale = observation.clone();
        stale.attempt_id = AttemptId::new("stale-attempt").expect("fixed fixture identity");
        assert!(admit_workflow_observation(&plan, &stale).is_err());
        let mut paused = observation.clone();
        paused.state = WorkflowState::Paused;
        assert!(admit_workflow_observation(&plan, &paused).is_err());
        serde_json::json!({
            "producer_revision": super::REVISION,
            "plan": plan, "observation": observation,
            "rejected": {"stale_attempt": stale, "false_paused_success": paused}
        })
    }
}
fn main() -> Result<(), Box<dyn Error>> {
    let root = std::env::args_os()
        .nth(1)
        .ok_or("explicit pinned source root required")?;
    let head = Command::new("git")
        .arg("-C")
        .arg(&root)
        .args(["rev-parse", "HEAD"])
        .output()?;
    if !head.status.success() || String::from_utf8(head.stdout)?.trim() != REVISION {
        return Err("Kiln fixture source revision mismatch".into());
    }
    let bytes = Command::new("git")
        .arg("-C")
        .arg(&root)
        .args([
            "show",
            &format!("{REVISION}:crates/kiln-core/src/workflow/tests.rs"),
        ])
        .output()?;
    if !bytes.status.success()
        || bytes.stdout != include_bytes!(concat!(env!("OUT_DIR"), "/producer-tests.rs"))
    {
        return Err("Kiln fixture source substitution".into());
    }
    println!("{}", serde_json::to_string_pretty(&producer::export())?);
    Ok(())
}

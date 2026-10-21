//! TEST ONLY: these deterministic keys are deliberately not real review approval.
// r[verify cairn.source_review.shell.valid]
// r[verify cairn.source_review.shell.invalid]
use artifact_auth_core::{AuthenticationPolicy, KeyCurrentness, TrustedKeyObservation};
use artifact_auth_ed25519::public_key_identity;
use cairn_core::source_review::{
    self, ClaimRoot, SourceReviewClaim, SourceReviewDisposition, SourceReviewSubject,
    SourceReviewVerificationPolicy,
};
use ed25519_dalek::{Signer, SigningKey};
use serde_json::{json, Value};
use std::{
    error::Error,
    fs,
    path::PathBuf,
    process::{Command, Output},
};

struct Fixture(PathBuf);
impl Drop for Fixture {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}
impl Fixture {
    fn run(&self, args: &[&str]) -> Result<Output, Box<dyn Error>> {
        Ok(Command::new(env!("CARGO_BIN_EXE_cairn"))
            .args(["source-review"])
            .args(args)
            .arg("--root")
            .arg(&self.0)
            .output()?)
    }
}
fn hash(value: &str) -> String {
    blake3::hash(value.as_bytes()).to_hex().to_string()
}

#[test]
fn operator_detached_signature_round_trip_and_fail_closed_cli() -> Result<(), Box<dyn Error>> {
    let fixture = Fixture(
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../../target")
            .join(format!("source-review-test-only-{}", std::process::id())),
    );
    fs::create_dir_all(&fixture.0)?;
    let signer = SigningKey::from_bytes(&[53; 32]);
    let identity = public_key_identity(signer.verifying_key().as_bytes());
    let subject = SourceReviewSubject::new(
        hash("TEST ONLY source archive"),
        Some("test-only-revision".to_owned()),
        None,
    )?;
    let context = artifact_auth_core::ArtifactRef {
        profile: "test-only-currentness.v1".to_owned(),
        algorithm: "blake3".to_owned(),
        digest_hex: hash("TEST ONLY currentness"),
    };
    let root = ClaimRoot {
        claim_identity: hash("test existing claim"),
        subject_root: hash("test existing root"),
        revision: 5,
    };
    let policy = SourceReviewVerificationPolicy {
        schema: source_review::POLICY_SCHEMA.to_owned(),
        subject: subject.clone(),
        claim_root: root.clone(),
        review_policy_blake3: hash("TEST ONLY policy"),
        authentication_policy: AuthenticationPolicy {
            schema: artifact_auth_core::POLICY_SCHEMA_V1.to_owned(),
            profile_id: source_review::PROFILE.to_owned(),
            threshold: 1,
            trusted_keys: vec![TrustedKeyObservation {
                producer_id: "test-only-reviewer".to_owned(),
                key_id: "test-only-key".to_owned(),
                key_identity: identity.clone(),
                allowed_purposes: vec![source_review::PURPOSE.to_owned()],
                generation: 1,
                currentness: KeyCurrentness::Current,
                currentness_ref: context.clone(),
            }],
        },
        verifier_context: context,
        author_key_blake3: vec![],
        exclude_authors: true,
        required_approvals: 1,
        non_claims: source_review::required_non_claims(),
    };
    let claim = SourceReviewClaim::new(
        subject,
        root,
        policy.review_policy_blake3.clone(),
        SourceReviewDisposition::Approved,
    )?;
    fs::write(fixture.0.join("policy.json"), serde_json::to_vec(&policy)?)?;
    fs::write(fixture.0.join("claim.json"), serde_json::to_vec(&claim)?)?;
    let public_hex = source_review::encode_hex(signer.verifying_key().as_bytes());
    let options = [
        "signing-request",
        "--claim",
        "claim.json",
        "--policy",
        "policy.json",
        "--producer",
        "test-only-reviewer",
        "--key-id",
        "test-only-key",
        "--generation",
        "1",
        "--public-key-hex",
        &public_hex,
    ];
    let request = fixture.run(&options)?;
    assert!(
        request.status.success(),
        "{}",
        String::from_utf8_lossy(&request.stderr)
    );
    fs::write(fixture.0.join("request.json"), request.stdout)?;
    let raw_options = options.into_iter().chain(["--raw"]).collect::<Vec<_>>();
    let raw = fixture.run(&raw_options)?;
    assert!(
        raw.status.success(),
        "{}",
        String::from_utf8_lossy(&raw.stderr)
    );
    let detached = source_review::encode_hex(&signer.sign(&raw.stdout).to_bytes());
    let assemble = fixture.run(&[
        "assemble",
        "--request",
        "request.json",
        "--signature-hex",
        &detached,
        "--policy",
        "policy.json",
    ])?;
    assert!(
        assemble.status.success(),
        "{}",
        String::from_utf8_lossy(&assemble.stderr)
    );
    fs::write(fixture.0.join("attachment.json"), &assemble.stdout)?;
    let verify_args = [
        "verify",
        "--attachment",
        "attachment.json",
        "--policy",
        "policy.json",
    ];
    let verified = fixture.run(&verify_args)?;
    assert!(
        verified.status.success(),
        "{}",
        String::from_utf8_lossy(&verified.stderr)
    );
    let report: Value = serde_json::from_slice(&verified.stdout)?;
    assert_eq!(report["report"]["outcome"], "accepted_manual_review");
    assert_eq!(
        report["report"]["approved_key_blake3"],
        json!([identity.digest_hex])
    );
    assert_eq!(
        report["attachment_artifact_blake3"],
        blake3::hash(&assemble.stdout).to_hex().as_str()
    );
    let mut tampered: Value = serde_json::from_slice(&assemble.stdout)?;
    tampered["reviews"][0]["signature_hex"] = json!("00".repeat(64));
    fs::write(
        fixture.0.join("attachment.json"),
        serde_json::to_vec(&tampered)?,
    )?;
    assert!(!fixture.run(&verify_args)?.status.success());
    fs::write(fixture.0.join("attachment.json"), assemble.stdout)?;
    let mut revoked = policy.clone();
    revoked.authentication_policy.trusted_keys[0].currentness = KeyCurrentness::Revoked;
    fs::write(fixture.0.join("policy.json"), serde_json::to_vec(&revoked)?)?;
    assert!(!fixture.run(&verify_args)?.status.success());
    let mut excluded = policy.clone();
    excluded.author_key_blake3.push(identity.digest_hex);
    fs::write(
        fixture.0.join("policy.json"),
        serde_json::to_vec(&excluded)?,
    )?;
    assert!(!fixture.run(&verify_args)?.status.success());
    fs::write(fixture.0.join("policy.json"), serde_json::to_vec(&policy)?)?;
    assert!(!fixture
        .run(&[
            "verify",
            "--attachment",
            "../attachment.json",
            "--policy",
            "policy.json"
        ])?
        .status
        .success());
    assert!(!fixture
        .run(&[
            "verify",
            "--attachment",
            "attachment.json",
            "--policy",
            "policy.json",
            "--policy",
            "policy.json"
        ])?
        .status
        .success());
    Ok(())
}

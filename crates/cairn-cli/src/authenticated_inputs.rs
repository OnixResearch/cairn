use std::fs;
use std::path::{Path, PathBuf};

use cairn_core::{
    authenticated_evidence_policy_from_value, authenticated_signature_message,
    evaluate_authenticated_evidence, AuthenticatedEvidenceInput, AuthenticatedEvidencePolicy,
    AuthenticatedEvidenceReport, CanonicalReceiptObservation, MeasuredEvidenceInput,
    PolicyExportObservation, SignatureVerificationObservation, AUTHENTICATED_EVIDENCE_SCHEMA_V1,
};
use ed25519_dalek::{Signature, Verifier, VerifyingKey};
use serde::Deserialize;
use serde_json::{json, Value};

const ED25519_PUBLIC_KEY_BYTES: usize = 32;
const ED25519_SIGNATURE_BYTES: usize = 64;
const HEX_PAIR_BYTES: usize = 2;
const NIBBLE_BITS: u8 = 4;
const HEX_ALPHA_OFFSET: u8 = 10;
#[cfg(test)]
const HEX_ALPHABET_LENGTH: usize = 16;
#[cfg(test)]
const LOW_NIBBLE_MASK: u8 = 0x0f;

#[derive(Debug, Clone)]
pub(crate) struct AuthenticationArtifact {
    pub(crate) role: &'static str,
    pub(crate) path: Option<PathBuf>,
}

#[derive(Debug, Clone)]
pub(crate) struct AuthenticationComponent {
    pub(crate) check: Value,
    pub(crate) value: Value,
    pub(crate) report: Option<AuthenticatedEvidenceReport>,
}

#[derive(Debug, Deserialize)]
struct AuthenticationManifest {
    schema_version: String,
    evidence_kind: String,
    canonicalization_profile: String,
    supplied_receipt_hash: String,
    measurements: Vec<DeclaredMeasurement>,
    declared_policy_export_blake3: String,
    required_parent_hashes: Vec<String>,
    valence_integrity_receipt_hash: String,
    signature: DeclaredSignature,
}

#[derive(Debug, Deserialize)]
struct DeclaredMeasurement {
    role: String,
    path: String,
    blake3: String,
}

#[derive(Debug, Deserialize)]
struct DeclaredSignature {
    algorithm: String,
    producer: String,
    key_id: String,
    purpose: String,
    signature_hex: String,
}

// r[impl cairn.stack_provenance_gates.authenticated_inputs.measurement]
// r[impl cairn.stack_provenance_gates.authenticated_inputs.receipt_identity]
// r[impl cairn.stack_provenance_gates.authenticated_inputs.signature_policy]
pub(crate) fn authentication_component(
    root: &Path,
    policy_value: Option<&Value>,
    evidence_kind: &str,
    authentication_path: Option<PathBuf>,
    artifacts: Vec<AuthenticationArtifact>,
    primary_receipt_role: &str,
) -> AuthenticationComponent {
    let Some(authentication_path) = authentication_path else {
        return skipped_component(evidence_kind);
    };
    let result = build_authenticated_report(
        root,
        policy_value,
        evidence_kind,
        &authentication_path,
        &artifacts,
        primary_receipt_role,
    );
    match result {
        Ok(report) => {
            let value = serde_json::to_value(&report).unwrap_or_else(|error| {
                json!({
                    "passed": false,
                    "shell_error": error.to_string(),
                })
            });
            AuthenticationComponent {
                check: json!({
                    "name": format!("{evidence_kind}_authentication"),
                    "passed": report.passed,
                    "source": "measured local bytes and configured Ed25519 trust policy",
                    "path": authentication_path,
                    "authentication_hash": report.authentication_hash,
                }),
                value,
                report: Some(report),
            }
        }
        Err(error) => AuthenticationComponent {
            check: json!({
                "name": format!("{evidence_kind}_authentication"),
                "passed": false,
                "source": "measured local bytes and configured Ed25519 trust policy",
                "path": authentication_path,
                "issue": error,
            }),
            value: json!({
                "evidence_kind": evidence_kind,
                "passed": false,
                "shell_error": error,
            }),
            report: None,
        },
    }
}

fn build_authenticated_report(
    root: &Path,
    policy_value: Option<&Value>,
    evidence_kind: &str,
    authentication_path: &Path,
    artifacts: &[AuthenticationArtifact],
    primary_receipt_role: &str,
) -> Result<AuthenticatedEvidenceReport, String> {
    let policy_value = policy_value.ok_or_else(|| "policy could not be loaded".to_string())?;
    let policy = authenticated_evidence_policy_from_value(policy_value)?;
    let manifest = read_manifest(authentication_path)?;
    if manifest.schema_version != AUTHENTICATED_EVIDENCE_SCHEMA_V1 {
        return Err(format!(
            "authentication manifest schema {} is unsupported",
            manifest.schema_version
        ));
    }
    if manifest.evidence_kind != evidence_kind {
        return Err(format!(
            "authentication manifest kind {} does not match {evidence_kind}",
            manifest.evidence_kind
        ));
    }
    let mut measurements = measure_artifacts(artifacts, &manifest.measurements)?;
    let valence_integrity_measurement =
        measure_valence_integrity_receipt(root, &policy, &manifest.valence_integrity_receipt_hash)?;
    let valence_integrity_receipt_hash = valence_integrity_measurement.measured_blake3.clone();
    measurements.push(valence_integrity_measurement);
    let receipt_path = artifacts
        .iter()
        .find(|artifact| artifact.role == primary_receipt_role)
        .and_then(|artifact| artifact.path.as_ref())
        .ok_or_else(|| format!("missing primary receipt role {primary_receipt_role}"))?;
    let (embedded_receipt_hash, recomputed_receipt_hash, canonical_receipt) =
        recompute_receipt_identity(receipt_path)?;
    if embedded_receipt_hash != manifest.supplied_receipt_hash {
        return Err(format!(
            "authentication manifest supplied receipt hash {} does not match embedded receipt hash {}",
            manifest.supplied_receipt_hash, embedded_receipt_hash
        ));
    }
    let supplied_receipt_hash = embedded_receipt_hash;
    let policy_export = measure_policy_export(root, &policy, evidence_kind, &manifest)?;
    let required_parent_hashes =
        derive_required_parent_hashes(&measurements, primary_receipt_role, &policy_export);
    let trusted_key = policy.trusted_producers.iter().find(|producer| {
        producer.producer == manifest.signature.producer
            && producer.key_id == manifest.signature.key_id
    });
    let public_key_blake3 = trusted_key
        .map(|producer| {
            blake3::hash(producer.public_key_hex.as_bytes())
                .to_hex()
                .to_string()
        })
        .unwrap_or_default();
    let mut input = AuthenticatedEvidenceInput {
        schema_version: manifest.schema_version.clone(),
        evidence_kind: manifest.evidence_kind.clone(),
        measurements,
        receipt: CanonicalReceiptObservation {
            profile: manifest.canonicalization_profile.clone(),
            supplied_receipt_hash,
            recomputed_receipt_hash: recomputed_receipt_hash.clone(),
        },
        policy_export,
        signature: SignatureVerificationObservation {
            algorithm: manifest.signature.algorithm.clone(),
            verified: false,
            producer: manifest.signature.producer.clone(),
            key_id: manifest.signature.key_id.clone(),
            purpose: manifest.signature.purpose.clone(),
            subject_hash: recomputed_receipt_hash,
            parent_hashes: manifest.required_parent_hashes.clone(),
            public_key_blake3,
        },
        required_parent_hashes,
        valence_integrity_receipt_hash,
    };
    let message = authenticated_signature_message(&input, &canonical_receipt);
    input.signature.verified = verify_signature(
        trusted_key.map(|producer| producer.public_key_hex.as_str()),
        &manifest.signature.signature_hex,
        message.as_bytes(),
    );
    Ok(evaluate_authenticated_evidence(&policy, &input))
}

fn read_manifest(path: &Path) -> Result<AuthenticationManifest, String> {
    let bytes = fs::read(path).map_err(|error| format!("read {}: {error}", path.display()))?;
    serde_json::from_slice(&bytes).map_err(|error| format!("parse {}: {error}", path.display()))
}

fn measure_artifacts(
    artifacts: &[AuthenticationArtifact],
    declared: &[DeclaredMeasurement],
) -> Result<Vec<MeasuredEvidenceInput>, String> {
    if artifacts.len() != declared.len() {
        return Err(format!(
            "authentication manifest declares {} measurements for {} supplied artifacts",
            declared.len(),
            artifacts.len()
        ));
    }
    let mut measurements = Vec::with_capacity(artifacts.len());
    for artifact in artifacts {
        let path = artifact
            .path
            .as_ref()
            .ok_or_else(|| format!("missing evidence path for role {}", artifact.role))?;
        let bytes = fs::read(path).map_err(|error| format!("read {}: {error}", path.display()))?;
        let actual_path = path.display().to_string();
        let declaration = declared
            .iter()
            .find(|entry| entry.role == artifact.role && entry.path == actual_path);
        measurements.push(MeasuredEvidenceInput {
            role: artifact.role.to_string(),
            path: actual_path,
            declared_blake3: declaration
                .map(|entry| entry.blake3.clone())
                .unwrap_or_default(),
            measured_blake3: blake3::hash(&bytes).to_hex().to_string(),
        });
    }
    Ok(measurements)
}

fn recompute_receipt_identity(path: &Path) -> Result<(String, String, String), String> {
    let bytes = fs::read(path).map_err(|error| format!("read {}: {error}", path.display()))?;
    let mut value: Value = serde_json::from_slice(&bytes)
        .map_err(|error| format!("parse receipt {}: {error}", path.display()))?;
    let object = value
        .as_object_mut()
        .ok_or_else(|| format!("receipt {} is not a JSON object", path.display()))?;
    let supplied = object
        .remove("receipt_hash")
        .and_then(|hash| hash.as_str().map(ToOwned::to_owned))
        .ok_or_else(|| format!("receipt {} has no receipt_hash self-hash", path.display()))?;
    let canonical_receipt = cairn_core::canonical_json(&value);
    let receipt_hash = blake3::hash(canonical_receipt.as_bytes())
        .to_hex()
        .to_string();
    Ok((supplied, receipt_hash, canonical_receipt))
}

fn measure_policy_export(
    root: &Path,
    policy: &AuthenticatedEvidencePolicy,
    evidence_kind: &str,
    manifest: &AuthenticationManifest,
) -> Result<PolicyExportObservation, String> {
    let export = policy
        .policy_exports
        .iter()
        .find(|export| export.evidence_kind == evidence_kind)
        .ok_or_else(|| format!("no configured policy export for {evidence_kind}"))?;
    let configured_path = PathBuf::from(&export.path);
    let path = if configured_path.is_absolute() {
        configured_path
    } else {
        root.join(configured_path)
    };
    let bytes = fs::read(&path)
        .map_err(|error| format!("read policy export {}: {error}", path.display()))?;
    Ok(PolicyExportObservation {
        path: export.path.clone(),
        declared_blake3: manifest.declared_policy_export_blake3.clone(),
        measured_blake3: blake3::hash(&bytes).to_hex().to_string(),
    })
}

fn derive_required_parent_hashes(
    measurements: &[MeasuredEvidenceInput],
    primary_receipt_role: &str,
    policy_export: &PolicyExportObservation,
) -> Vec<String> {
    let mut parent_hashes = measurements
        .iter()
        .filter(|measurement| measurement.role != primary_receipt_role)
        .map(|measurement| measurement.measured_blake3.clone())
        .collect::<Vec<_>>();
    parent_hashes.push(policy_export.measured_blake3.clone());
    parent_hashes.sort();
    parent_hashes.dedup();
    parent_hashes
}

fn measure_valence_integrity_receipt(
    root: &Path,
    policy: &AuthenticatedEvidencePolicy,
    declared_blake3: &str,
) -> Result<MeasuredEvidenceInput, String> {
    let configured_path = PathBuf::from(&policy.accepted_valence_integrity_receipt_path);
    let path = if configured_path.is_absolute() {
        configured_path
    } else {
        root.join(configured_path)
    };
    let bytes = fs::read(&path)
        .map_err(|error| format!("read Valence integrity receipt {}: {error}", path.display()))?;
    Ok(MeasuredEvidenceInput {
        role: "valence_integrity_receipt".to_string(),
        path: policy.accepted_valence_integrity_receipt_path.clone(),
        declared_blake3: declared_blake3.to_string(),
        measured_blake3: blake3::hash(&bytes).to_hex().to_string(),
    })
}

fn verify_signature(public_key_hex: Option<&str>, signature_hex: &str, message: &[u8]) -> bool {
    let Some(public_key_hex) = public_key_hex else {
        return false;
    };
    let Ok(public_key_bytes) = decode_fixed_hex::<ED25519_PUBLIC_KEY_BYTES>(public_key_hex) else {
        return false;
    };
    let Ok(signature_bytes) = decode_fixed_hex::<ED25519_SIGNATURE_BYTES>(signature_hex) else {
        return false;
    };
    let Ok(public_key) = VerifyingKey::from_bytes(&public_key_bytes) else {
        return false;
    };
    let signature = Signature::from_bytes(&signature_bytes);
    public_key.verify(message, &signature).is_ok()
}

fn decode_fixed_hex<const SIZE: usize>(value: &str) -> Result<[u8; SIZE], ()> {
    let expected_length = SIZE.checked_mul(HEX_PAIR_BYTES).ok_or(())?;
    if value.len() != expected_length {
        return Err(());
    }
    let mut bytes = [0_u8; SIZE];
    for (index, pair) in value.as_bytes().chunks_exact(HEX_PAIR_BYTES).enumerate() {
        let high = decode_nibble(pair[0]).ok_or(())?;
        let low = decode_nibble(pair[1]).ok_or(())?;
        bytes[index] = (high << NIBBLE_BITS) | low;
    }
    Ok(bytes)
}

fn decode_nibble(value: u8) -> Option<u8> {
    match value {
        b'0'..=b'9' => Some(value - b'0'),
        b'a'..=b'f' => Some(value - b'a' + HEX_ALPHA_OFFSET),
        _ => None,
    }
}

fn skipped_component(evidence_kind: &str) -> AuthenticationComponent {
    AuthenticationComponent {
        check: json!({
            "name": format!("{evidence_kind}_authentication"),
            "passed": true,
            "skipped": true,
            "source": "authentication manifest not supplied",
        }),
        value: json!({
            "evidence_kind": evidence_kind,
            "passed": true,
            "skipped": true,
        }),
        report: None,
    }
}

#[cfg(test)]
// r[verify cairn.stack_provenance_gates.authenticated_inputs.final_validation]
mod tests {
    use std::error::Error;

    use cairn_core::{
        authenticated_signature_message, AuthenticatedPolicyExport, AuthenticatedTrustedProducer,
        AUTHENTICATED_EVIDENCE_CANONICALIZATION_CORRECTED_V1,
        AUTHENTICATED_EVIDENCE_KIND_STACK_PROVENANCE,
        AUTHENTICATED_EVIDENCE_SIGNATURE_ALGORITHM_ED25519,
    };
    use ed25519_dalek::{Signer, SigningKey};

    use super::*;

    const TEST_SEED_BYTE: u8 = 7;
    const TEST_PURPOSE: &str = "onix.stack-provenance.release-evidence.v1";
    const TEST_POLICY_PATH: &str = "policy-export.json";
    const TEST_VALENCE_PATH: &str = "valence-integrity.json";

    #[test]
    fn shell_verifies_measured_bytes_and_rejects_post_signature_tampering(
    ) -> Result<(), Box<dyn Error>> {
        // r[verify cairn.stack_provenance_gates.authenticated_inputs.fixtures.positive]
        // r[verify cairn.stack_provenance_gates.authenticated_inputs.fixtures.negative]
        let root =
            std::env::temp_dir().join(format!("cairn-authenticated-inputs-{}", std::process::id()));
        let _ = fs::remove_dir_all(&root);
        fs::create_dir_all(&root)?;
        let receipt_path = root.join("receipt.json");
        let graph_path = root.join("graph.json");
        let policy_export_path = root.join(TEST_POLICY_PATH);
        let valence_path = root.join(TEST_VALENCE_PATH);

        let receipt_body = json!({"claim": "bounded"});
        let receipt_hash = cairn_core::canonical_json_hash(&receipt_body);
        let mut receipt = receipt_body;
        receipt["receipt_hash"] = json!(receipt_hash);
        fs::write(&receipt_path, serde_json::to_vec_pretty(&receipt)?)?;
        fs::write(&graph_path, b"{\"edges\":[]}")?;
        fs::write(&policy_export_path, b"{\"reviewed\":true}")?;
        fs::write(&valence_path, b"{\"profile\":\"corrected\"}")?;

        let policy_export_hash = file_hash(&policy_export_path)?;
        let valence_hash = file_hash(&valence_path)?;
        let signing_key = SigningKey::from_bytes(&[TEST_SEED_BYTE; ED25519_PUBLIC_KEY_BYTES]);
        let public_key_hex = encode_hex(signing_key.verifying_key().as_bytes());
        let policy = AuthenticatedEvidencePolicy {
            schema_version: AUTHENTICATED_EVIDENCE_SCHEMA_V1.to_string(),
            release_profile: "onix-release".to_string(),
            accepted_canonicalization_profiles: vec![
                AUTHENTICATED_EVIDENCE_CANONICALIZATION_CORRECTED_V1.to_string(),
            ],
            accepted_valence_integrity_receipt_path: TEST_VALENCE_PATH.to_string(),
            accepted_valence_integrity_receipt_hash: valence_hash.clone(),
            policy_exports: vec![AuthenticatedPolicyExport {
                evidence_kind: AUTHENTICATED_EVIDENCE_KIND_STACK_PROVENANCE.to_string(),
                path: TEST_POLICY_PATH.to_string(),
                digest_blake3: policy_export_hash.clone(),
                signature_purpose: TEST_PURPOSE.to_string(),
            }],
            trusted_producers: vec![AuthenticatedTrustedProducer {
                producer: "test-producer".to_string(),
                key_id: "test-key".to_string(),
                algorithm: AUTHENTICATED_EVIDENCE_SIGNATURE_ALGORITHM_ED25519.to_string(),
                public_key_hex: public_key_hex.clone(),
                allowed_purposes: vec![TEST_PURPOSE.to_string()],
                revoked: false,
            }],
        };
        let policy_value = json!({"authenticated_evidence_policy": policy});
        let receipt_measurement = measured(&receipt_path, "receipt")?;
        let graph_measurement = measured(&graph_path, "graph")?;
        let valence_measurement = MeasuredEvidenceInput {
            role: "valence_integrity_receipt".to_string(),
            path: TEST_VALENCE_PATH.to_string(),
            declared_blake3: valence_hash.clone(),
            measured_blake3: valence_hash.clone(),
        };
        let mut parent_hashes = vec![
            graph_measurement.measured_blake3.clone(),
            policy_export_hash.clone(),
            valence_hash.clone(),
        ];
        parent_hashes.sort();
        let message_input = AuthenticatedEvidenceInput {
            schema_version: AUTHENTICATED_EVIDENCE_SCHEMA_V1.to_string(),
            evidence_kind: AUTHENTICATED_EVIDENCE_KIND_STACK_PROVENANCE.to_string(),
            measurements: vec![
                receipt_measurement.clone(),
                graph_measurement.clone(),
                valence_measurement,
            ],
            receipt: CanonicalReceiptObservation {
                profile: AUTHENTICATED_EVIDENCE_CANONICALIZATION_CORRECTED_V1.to_string(),
                supplied_receipt_hash: receipt_hash.clone(),
                recomputed_receipt_hash: receipt_hash.clone(),
            },
            policy_export: PolicyExportObservation {
                path: TEST_POLICY_PATH.to_string(),
                declared_blake3: policy_export_hash.clone(),
                measured_blake3: policy_export_hash.clone(),
            },
            signature: SignatureVerificationObservation {
                algorithm: AUTHENTICATED_EVIDENCE_SIGNATURE_ALGORITHM_ED25519.to_string(),
                verified: false,
                producer: "test-producer".to_string(),
                key_id: "test-key".to_string(),
                purpose: TEST_PURPOSE.to_string(),
                subject_hash: receipt_hash.clone(),
                parent_hashes: parent_hashes.clone(),
                public_key_blake3: blake3::hash(public_key_hex.as_bytes()).to_hex().to_string(),
            },
            required_parent_hashes: parent_hashes.clone(),
            valence_integrity_receipt_hash: valence_hash.clone(),
        };
        let signature = signing_key.sign(
            authenticated_signature_message(
                &message_input,
                &cairn_core::canonical_json(&json!({"claim": "bounded"})),
            )
            .as_bytes(),
        );
        let manifest_path = root.join("authentication.json");
        fs::write(
            &manifest_path,
            serde_json::to_vec_pretty(&json!({
                "schema_version": AUTHENTICATED_EVIDENCE_SCHEMA_V1,
                "evidence_kind": AUTHENTICATED_EVIDENCE_KIND_STACK_PROVENANCE,
                "canonicalization_profile": AUTHENTICATED_EVIDENCE_CANONICALIZATION_CORRECTED_V1,
                "supplied_receipt_hash": receipt_hash,
                "measurements": [
                    {
                        "role": "receipt",
                        "path": receipt_path.display().to_string(),
                        "blake3": receipt_measurement.declared_blake3,
                    },
                    {
                        "role": "graph",
                        "path": graph_path.display().to_string(),
                        "blake3": graph_measurement.declared_blake3,
                    },
                ],
                "declared_policy_export_blake3": policy_export_hash,
                "required_parent_hashes": parent_hashes,
                "valence_integrity_receipt_hash": valence_hash,
                "signature": {
                    "algorithm": AUTHENTICATED_EVIDENCE_SIGNATURE_ALGORITHM_ED25519,
                    "producer": "test-producer",
                    "key_id": "test-key",
                    "purpose": TEST_PURPOSE,
                    "signature_hex": encode_hex(&signature.to_bytes()),
                },
            }))?,
        )?;
        let artifacts = || {
            vec![
                AuthenticationArtifact {
                    role: "receipt",
                    path: Some(receipt_path.clone()),
                },
                AuthenticationArtifact {
                    role: "graph",
                    path: Some(graph_path.clone()),
                },
            ]
        };
        let passing = authentication_component(
            &root,
            Some(&policy_value),
            AUTHENTICATED_EVIDENCE_KIND_STACK_PROVENANCE,
            Some(manifest_path.clone()),
            artifacts(),
            "receipt",
        );
        assert_eq!(
            passing.report.as_ref().map(|report| report.passed),
            Some(true)
        );

        fs::write(&graph_path, b"{\"edges\":[\"tampered\"]}")?;
        let failing = authentication_component(
            &root,
            Some(&policy_value),
            AUTHENTICATED_EVIDENCE_KIND_STACK_PROVENANCE,
            Some(manifest_path),
            artifacts(),
            "receipt",
        );
        let report = failing.report.ok_or("missing failure report")?;
        assert!(!report.passed);
        assert!(report
            .issues
            .iter()
            .any(|issue| { issue.code == "authenticated_inputs.measured_digest_mismatch" }));
        fs::remove_dir_all(root)?;
        Ok(())
    }

    fn measured(path: &Path, role: &str) -> Result<MeasuredEvidenceInput, Box<dyn Error>> {
        let digest = file_hash(path)?;
        Ok(MeasuredEvidenceInput {
            role: role.to_string(),
            path: path.display().to_string(),
            declared_blake3: digest.clone(),
            measured_blake3: digest,
        })
    }

    fn file_hash(path: &Path) -> Result<String, Box<dyn Error>> {
        Ok(blake3::hash(&fs::read(path)?).to_hex().to_string())
    }

    fn encode_hex(bytes: &[u8]) -> String {
        const HEX: &[u8; HEX_ALPHABET_LENGTH] = b"0123456789abcdef";
        let mut encoded = String::with_capacity(bytes.len() * HEX_PAIR_BYTES);
        for byte in bytes {
            encoded.push(HEX[usize::from(byte >> NIBBLE_BITS)] as char);
            encoded.push(HEX[usize::from(byte & LOW_NIBBLE_MASK)] as char);
        }
        encoded
    }
}

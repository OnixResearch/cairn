//! Root API controls for lifecycle-store opaque type compatibility.

const STORE_ID: &str = "alpha";
const REMOTE_HINT: &str = "https://example.invalid/alpha.git";
const CHECKOUT_PATH: &str = "/home/operator/alpha";
const SOURCE_MATERIAL: &[u8] = b"lifecycle wrapper compatibility";

fn projection() -> cairn_core::LifecycleStoreIdentityProjection {
    cairn_core::LifecycleStoreIdentityProjection {
        schema: cairn_core::LIFECYCLE_STORE_IDENTITY_SCHEMA_V1.to_string(),
        store_id: STORE_ID.to_string(),
        remote_hint: Some(REMOTE_HINT.to_string()),
        non_claims: cairn_core::LIFECYCLE_STORE_IDENTITY_NON_CLAIMS
            .iter()
            .map(|value| (*value).to_string())
            .collect(),
        source_identity: cairn_core::lifecycle_store_source_identity(SOURCE_MATERIAL),
    }
}

#[test]
fn opaque_root_types_remain_usable() {
    // r[verify cairn.lifecycle_stores.identity.valid]
    let admission = cairn_core::admit_lifecycle_store_identity(&projection());
    assert!(admission.valid, "{:?}", admission.diagnostics);
    let identity = admission.identity.expect("valid identity");
    let _: &cairn_core::LifecycleStoreId = &identity.store_id;
    let _: &cairn_core::LifecycleStoreRemoteHint =
        identity.remote_hint.as_ref().expect("remote hint");
    let _: &cairn_core::LifecycleStoreNonClaim =
        identity.non_claims.first().expect("required non-claim");
    let _: &cairn_core::LifecycleStoreBlake3Identity = &identity.store_identity;

    let current = cairn_core::empty_lifecycle_store_locator();
    let plan = cairn_core::plan_lifecycle_store_registration(&current, &identity, CHECKOUT_PATH);
    let application = cairn_core::apply_lifecycle_store_locator_plan(&current, &plan);
    assert!(application.accepted, "{:?}", application.diagnostics);
    let state = application.next_state.expect("accepted next state");
    let mapping = state.mappings.first().expect("registered mapping");
    let _: &cairn_core::LifecycleStoreCheckoutPath = &mapping.checkout_path;
    assert_eq!(mapping.store_id.as_str(), STORE_ID);
}

#[test]
fn opaque_root_types_reject_invalid_projection() {
    // r[verify cairn.lifecycle_stores.identity.invalid]
    let mut candidate = projection();
    candidate.store_id = "../escape".to_string();
    let report = cairn_core::admit_lifecycle_store_identity(&candidate);
    assert!(!report.valid);
    assert!(report.identity.is_none());
    assert!(report
        .diagnostics
        .iter()
        .any(|diagnostic| diagnostic.field == "store_id"));
}

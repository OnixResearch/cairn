//! Root API controls for lifecycle-store type compatibility.

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
    let _: &cairn_core::LifecycleStoreIdentityAdmission = &admission;
    assert!(admission.valid, "{:?}", admission.diagnostics);
    let identity = admission.identity.expect("valid identity");
    let _: &cairn_core::LifecycleStoreIdentity = &identity;
    let _: &cairn_core::LifecycleStoreId = &identity.store_id;
    let _: &cairn_core::LifecycleStoreRemoteHint =
        identity.remote_hint.as_ref().expect("remote hint");
    let _: &cairn_core::LifecycleStoreNonClaim =
        identity.non_claims.first().expect("required non-claim");
    let _: &cairn_core::LifecycleStoreBlake3Identity = &identity.store_identity;

    let current = cairn_core::empty_lifecycle_store_locator();
    let _: &cairn_core::LifecycleStoreLocatorState = &current;
    let plan = cairn_core::plan_lifecycle_store_registration(&current, &identity, CHECKOUT_PATH);
    let _: &cairn_core::LifecycleStoreLocatorPlan = &plan;
    let _: &cairn_core::LifecycleStoreLocatorLocalFacts = &plan.local_facts;
    let _: &cairn_core::LifecycleStoreLocatorPortableFacts = &plan.portable_facts;
    let _: cairn_core::LifecycleStoreLocatorOperation = plan.local_facts.operation;
    assert_eq!(
        cairn_core::lifecycle_store_locator_plan_hash(&plan),
        plan.plan_hash
    );
    assert_eq!(
        cairn_core::lifecycle_store_locator_portable_hash(&plan.portable_facts),
        plan.portable_hash
    );

    let locator_projection = cairn_core::lifecycle_store_locator_projection(&current);
    let _: &cairn_core::LifecycleStoreLocatorProjection = &locator_projection;
    let locator_admission = cairn_core::admit_lifecycle_store_locator(&locator_projection);
    let _: &cairn_core::LifecycleStoreLocatorAdmission = &locator_admission;
    assert!(
        locator_admission.valid,
        "{:?}",
        locator_admission.diagnostics
    );

    let application = cairn_core::apply_lifecycle_store_locator_plan(&current, &plan);
    let _: &cairn_core::LifecycleStoreLocatorApplication = &application;
    assert!(application.accepted, "{:?}", application.diagnostics);
    let state = application.next_state.expect("accepted next state");
    let mapping = state.mappings.first().expect("registered mapping");
    let _: &cairn_core::LifecycleStoreLocatorMapping = mapping;
    let _: &cairn_core::LifecycleStoreCheckoutPath = &mapping.checkout_path;
    assert_eq!(mapping.store_id.as_str(), STORE_ID);

    let persisted = cairn_core::lifecycle_store_locator_projection(&state);
    let mapping_projection = persisted.mappings.first().expect("registered projection");
    let _: &cairn_core::LifecycleStoreLocatorMappingProjection = mapping_projection;
}

#[test]
fn opaque_root_types_reject_invalid_projection() {
    // r[verify cairn.lifecycle_stores.identity.invalid]
    let mut candidate = projection();
    candidate.store_id = "../escape".to_string();
    let report = cairn_core::admit_lifecycle_store_identity(&candidate);
    assert!(!report.valid);
    assert!(report.identity.is_none());
    let diagnostic = report
        .diagnostics
        .first()
        .expect("invalid identifier diagnostic");
    let _: &cairn_core::LifecycleStoreDiagnostic = diagnostic;
    assert!(report
        .diagnostics
        .iter()
        .any(|diagnostic| diagnostic.field == "store_id"));
}

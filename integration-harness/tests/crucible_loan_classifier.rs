//! Integration test for `RandomForestClassifierSuggestor` driving a
//! synthetic loan-default decision through a real Convergence Engine.
//!
//! This is the cross-extension proof point for crucible's first
//! fact-emitting pack. It exercises:
//!
//! - the `linfa-trees`-backed RF training pipeline through
//!   `crucible::fixtures::loan_default`,
//! - the `ClassifierModel` trait via `RandomForestModel`,
//! - the generic `ClassifierSuggestor<M>` reading typed
//!   `ClassificationFeaturesPayload`s and emitting typed
//!   `ClassPredictionPayload`s,
//! - typed `ProvenanceSource::Crucible` provenance at the
//!   `ProposedFact` boundary,
//! - end-to-end Engine execution with promotion.

use std::sync::Arc;

use converge_kernel::{Budget, ContextKey, ContextState, Engine};
use converge_pack::ProposedFact;
use crucible::{
    ClassPredictionPayload, ClassificationFeaturesPayload, ClassifierModel,
    RandomForestClassifierSuggestor, RandomForestConfig, RandomForestModel, fixtures::loan_default,
};

const TRAINING_SAMPLES: usize = 400;
const DATA_SEED: u64 = 20_260_515;
const TRAIN_SEED: u64 = 11;

fn budget() -> Budget {
    Budget {
        max_cycles: 5,
        max_facts: 100,
    }
}

fn trained_random_forest() -> Arc<RandomForestModel> {
    let (features, labels) = loan_default(TRAINING_SAMPLES, DATA_SEED);
    let model = RandomForestModel::train(
        &RandomForestConfig {
            n_trees: 25,
            max_depth: Some(6),
            min_weight_split: 2.0,
            random_seed: TRAIN_SEED,
        },
        &features,
        &labels,
    )
    .expect("RF training on synthetic loan-default data should succeed");
    Arc::new(model)
}

/// High-risk applicant: low credit score, high DTI, low income,
/// short tenure, prior default. Should fall on the default side of
/// the decision boundary.
fn high_risk_applicant_features() -> Vec<f64> {
    vec![
        520.0,    // credit_score
        45_000.0, // annual_income
        0.72,     // debt_to_income
        90_000.0, // loan_amount
        1.0,      // employment_years
        1.0,      // has_prior_default
    ]
}

/// Low-risk applicant: high credit, low DTI, high income, long
/// tenure, no prior default.
fn low_risk_applicant_features() -> Vec<f64> {
    vec![
        780.0,     // credit_score
        180_000.0, // annual_income
        0.18,      // debt_to_income
        50_000.0,  // loan_amount
        15.0,      // employment_years
        0.0,       // has_prior_default
    ]
}

async fn classify(features: Vec<f64>, fact_id: &str) -> (usize, Vec<f64>, String) {
    let model = trained_random_forest();
    let suggestor = RandomForestClassifierSuggestor::new(
        "crucible.loan_default.random_forest",
        model,
        ContextKey::Seeds,
        ContextKey::Evaluations,
    );

    let mut engine = Engine::with_budget(budget());
    engine.register_suggestor(suggestor);

    let mut context = ContextState::new();
    context
        .add_proposal(ProposedFact::new(
            ContextKey::Seeds,
            fact_id,
            ClassificationFeaturesPayload::new(features),
            "integration-test",
        ))
        .expect("features should stage into Seeds");

    let result = engine.run(context).await.expect("engine should run");
    assert!(result.converged, "engine should converge");

    let evals = result.context.get(ContextKey::Evaluations);
    assert_eq!(evals.len(), 1, "expected exactly one classifier proposal");
    let pred = evals[0]
        .payload::<ClassPredictionPayload>()
        .expect("emitted fact should carry a typed ClassPredictionPayload");
    // Payload family string is the audit-friendly proxy for provenance
    // on a `ContextFact`: only crucible can mint
    // `crucible.classification.prediction` payloads.
    let family = evals[0].payload_family().to_string();

    // Audit contract: every Crucible prediction carries an
    // ExecutionIdentity recording producer + backend + runtime
    // config. Verified inline so a regression that drops the field
    // back to the v1 shape fails the test.
    let identity = pred.execution_identity();
    assert_eq!(
        identity.producer.name, "converge-crucible-models",
        "execution_identity producer should be the crucible crate"
    );
    assert_eq!(
        identity.backend, "linfa-trees-v0.8",
        "RF backend should track the workspace's linfa-trees pin"
    );
    assert!(
        !identity.runtime_config.is_empty(),
        "runtime config should carry the serialized RandomForestConfig"
    );

    (
        pred.predicted_class(),
        pred.class_probabilities().to_vec(),
        family,
    )
}

#[tokio::test]
async fn random_forest_classifier_suggestor_emits_typed_prediction_with_crucible_provenance() {
    let (class, probs, family) =
        classify(high_risk_applicant_features(), "loan-app-high-risk-001").await;

    // Structural contract: provenance is typed `crucible`, two-class
    // probability vector that sums to 1.0 within floating-point
    // tolerance.
    assert_eq!(
        family, "crucible.classification.prediction",
        "RF suggestor must stamp a crucible-family payload"
    );
    assert_eq!(
        probs.len(),
        2,
        "synthetic loan-default dataset has two classes"
    );
    let sum: f64 = probs.iter().sum();
    assert!(
        (sum - 1.0).abs() < 1e-9,
        "class probabilities should sum to 1.0, got {sum}"
    );

    // Semantic check: the high-risk applicant should be on the
    // default side. Bagged trees on 400 samples are not infallible,
    // but a well-trained RF should call this case correctly with
    // probability >= 0.5 on the default class.
    assert_eq!(
        class, 1,
        "high-risk applicant should be classified as default (1)"
    );
    let p_default = probs[1];
    assert!(
        p_default >= 0.5,
        "expected P(default) >= 0.5 for high-risk applicant, got {p_default}"
    );
}

#[tokio::test]
async fn random_forest_classifier_suggestor_classifies_low_risk_below_decision_boundary() {
    let (class, probs, family) =
        classify(low_risk_applicant_features(), "loan-app-low-risk-001").await;

    assert_eq!(family, "crucible.classification.prediction");
    assert_eq!(probs.len(), 2);
    let sum: f64 = probs.iter().sum();
    assert!((sum - 1.0).abs() < 1e-9);

    // The low-risk applicant should be classified as non-default.
    assert_eq!(
        class, 0,
        "low-risk applicant should be classified as non-default (0)"
    );
    let p_non_default = probs[0];
    assert!(
        p_non_default >= 0.5,
        "expected P(non-default) >= 0.5 for low-risk applicant, got {p_non_default}"
    );
}

#[tokio::test]
async fn random_forest_classifier_suggestor_ignores_non_features_payloads() {
    use converge_pack::TextPayload;

    let model = trained_random_forest();
    let suggestor = RandomForestClassifierSuggestor::new(
        "crucible.loan_default.random_forest",
        model,
        ContextKey::Seeds,
        ContextKey::Evaluations,
    );

    let mut engine = Engine::with_budget(budget());
    engine.register_suggestor(suggestor);

    let mut context = ContextState::new();
    // Seed with a TextPayload — should be ignored by the classifier
    // Suggestor since the payload type does not match.
    context
        .add_proposal(ProposedFact::new(
            ContextKey::Seeds,
            "not-a-feature-vector",
            TextPayload::new("a plain text seed that the RF suggestor cannot consume"),
            "integration-test",
        ))
        .expect("text payload should stage");

    let result = engine.run(context).await.expect("engine should run");
    assert!(result.converged);
    let evals = result.context.get(ContextKey::Evaluations);
    assert!(
        evals.is_empty(),
        "RF suggestor should emit nothing when no ClassificationFeaturesPayload is present"
    );
}

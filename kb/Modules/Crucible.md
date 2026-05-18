---
tags: [module, ml, training]
source: mixed
date: 2026-05-14
---
# Crucible

`crucible-models` owns the training pipeline and trained-artifact packs for
Converge agents.

The crate name on crates.io is `converge-crucible-models`; the library name
is `crucible`. The boundary with [[Modules/Prism]] is sharp: prism owns
closed-form, hand-authored inference; crucible owns every model that must be
fit to data. The training pipeline was lifted from prism into crucible on
2026-05-14, restoring this boundary after prism had drifted into owning the
fitting logic during the period when crucible was scaffolded but unbuilt.

## Owns

- Multi-format dataset ingestion via Polars (CSV / TSV / Parquet / Excel).
- Polars ⇄ `converge-storage::ObjectStore` bridge for `gs://`, `s3://`,
  `file://`, and MinIO-compatible backends (feature `storage`).
- Training pipeline agents: dataset, validation, feature engineering,
  hyperparameter search, fit, evaluation, registry, monitoring, deployment.
- Trained-artifact packs: Random Forests / gradient-boosted trees
  (`ensembles`), CART decision trees (`trees`), kernel SVMs (`svm`), and
  ANFIS via Burn (`neuro_fuzzy`). Implementation lands pack-by-pack; first
  pack is `ensembles::RandomForestModel` pulled by the loan-application
  showcase.
- Typed Crucible proposal provenance at the `ProposedFact` boundary.
- `crucible.suggestor.execute` tracing spans at training and inference
  suggestor boundaries.

## Public Surface

- `DatasetAgent`
- `DataValidationAgent`
- `FeatureEngineeringAgent`
- `HyperparameterSearchAgent`
- `ModelTrainingAgent`
- `ModelEvaluationAgent`
- `ModelRegistryAgent`
- `MonitoringAgent`
- `DeploymentAgent`
- `SampleInferenceAgent`
- `ProvenanceSource`, `CRUCIBLE_PROVENANCE`, `UnknownProvenanceSource`

## Planned Packs

- `ensembles` — Random Forest, gradient-boosted trees. First pack in flight.
- `trees` — CART decision trees with Gini / information gain.
- `svm` — kernel SVMs with linear, polynomial, and RBF kernels.
- `neuro_fuzzy` — ANFIS via Burn autodiff for learned membership-function
  parameters.

## Feature Flags

- `storage` enables the `converge-storage::ObjectStore` bridge.
- `excel` enables optional Excel ingestion through `calamine`.

## Continuous Learning Position

Crucible's `MonitoringAgent`, `DeploymentAgent`, and `ModelRegistryAgent` are
the substrate for the continuous-learning story. Today they run from a
training CLI; once a real retrain trigger pulls (drift signal, scheduled
cadence, HITL outcome accumulation), the training pipeline lifts into a
Formation. Full closed-loop "experience → drift → retrain → deploy" is a
deliberate vision target, not an early-release claim.

## Boundary

- `prism` for closed-form inference; never trains.
- `mnemos` for experience storage and historical recall; the eventual source
  of HITL-outcome labels and drift signal.
- `arbiter` for runtime policy decisions over crucible's predicted scores.
- `ferrox` for optimisation when the trained-score feeds an objective or
  constraint.

See also: [[Architecture/Pluralist Reasoning Substrate]],
[[Architecture/Expert Portfolio Architecture]],
[[Standards/Suggestor Contract]].

## Entry Points

- `crucible-models/README.md`
- `crucible-models/crates/crucible/src/lib.rs`
- `crucible-models/crates/crucible/src/training/mod.rs` — re-exports + all tests (split from the former `training.rs` on 2026-05-18)
- `crucible-models/crates/crucible/src/training/types.rs` — 14 structs, fact-payload impls, context helpers
- `crucible-models/crates/crucible/src/training/io.rs` — file I/O, DataFrame utilities, math helpers
- `crucible-models/crates/crucible/src/training/dataset.rs` — `DatasetAgent`
- `crucible-models/crates/crucible/src/training/features.rs` — `DataValidationAgent`, `FeatureEngineeringAgent`, `HyperparameterSearchAgent`
- `crucible-models/crates/crucible/src/training/pipeline.rs` — `ModelTrainingAgent`
- `crucible-models/crates/crucible/src/training/evaluation.rs` — `ModelEvaluationAgent`, `SampleInferenceAgent`, `ModelRegistryAgent`, `MonitoringAgent`, `DeploymentAgent`
- `crucible-models/crates/crucible/src/ingest.rs`
- `crucible-models/crates/crucible/src/storage.rs`
- `crucible-models/crates/crucible/src/provenance.rs`
- `crucible-models/crates/crucible/src/{ensembles,trees,svm,neuro_fuzzy}/`

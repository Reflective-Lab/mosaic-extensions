---
tags: [module, analytics, ml]
source: mixed
date: 2026-05-05
---
# Prism

`prism` owns analytics and ML pipeline capabilities for Converge agents.

It implements feature extraction, inference, training, monitoring, and analytic
pack suggestors on top of Polars and Burn.

## Owns

- Polars-based ingestion and feature extraction.
- Burn-based inference examples.
- Training pipeline agents.
- Analytic pack solvers and input/output types.
- Compile-fail tests that enforce pack/suggestor authority boundaries.
- Typed Prism proposal provenance at the `ProposedFact` boundary.
- `prism.suggestor.execute` tracing spans at analytics suggestor boundaries.

## Public Surface

- `FeatureAgent`
- `InferenceAgent`
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

## Packs

- `AnomalyDetectionPack`
- `ClassificationPack`
- `DescriptiveStatsPack`
- `ForecastingPack`
- `RankingPack`
- `RegressionPack`
- `SegmentationPack`
- `SimilarityPack`
- `TrendDetectionPack`

## Planned Fuzzy Inference

Fuzzy logic belongs in `prism` first as a reusable capability plus analytic
pack wrapper, not as a standalone extension. `prism::fuzzy` should own the
membership functions and inference engine. `FuzzyInferencePack` should expose
that capability to Converge formations as graded memberships, activated-rule
traces, confidence, and advisory proposals.

Use `arbiter` for hard policy, `ferrox` for hard constraints and optimization,
and `mnemos` for memory and historical grounding.

See also: [[Architecture/Expert Portfolio Architecture]]

## Feature Flags

- Default features are empty.
- `storage` enables optional `converge-storage` support.
- `excel` enables optional Excel ingestion through `calamine`.

## Entry Points

- `prism-analytics/README.md`
- `prism-analytics/crates/prism/src/lib.rs`
- `prism-analytics/crates/prism/src/engine.rs`
- `prism-analytics/crates/prism/src/training.rs`
- `prism-analytics/crates/prism/src/packs/`
- `prism-analytics/crates/prism/tests/`

See also: [[Architecture/Dependency Rules]]

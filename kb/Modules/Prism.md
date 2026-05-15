---
tags: [module, analytics, ml]
source: mixed
date: 2026-05-05
---
# Prism

`prism` owns closed-form analytics and inference for Converge agents.

It implements feature extraction, Burn-based inference over feature vectors,
fuzzy inference (Mamdani / Sugeno / Tsukamoto), and analytic pack suggestors on
top of Polars. Training-pipeline concerns — dataset loading, train/val split,
hyperparameter search, model fitting, registry, and deployment — live in
[[Modules/Crucible]]. The boundary is: prism never fits; crucible never owns
expert rules. The pipeline was lifted out of prism on 2026-05-14 to restore
this boundary after implementation had drifted while crucible was a stub.

## Owns

- Polars-based feature extraction and inference.
- Burn-based inference over pre-fit feature vectors.
- Fuzzy inference engine (Mamdani / Sugeno / Tsukamoto) and defuzzification.
- Analytic pack solvers and input/output types for closed-form packs.
- Compile-fail tests that enforce pack/suggestor authority boundaries.
- Typed Prism proposal provenance at the `ProposedFact` boundary.
- `prism.suggestor.execute` tracing spans at analytics suggestor boundaries.

## Public Surface

- `FeatureAgent`
- `InferenceAgent`

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
- `excel` enables optional Excel ingestion through `calamine` (kept for the
  feature-extraction path; ingestion of training corpora now lives in
  [[Modules/Crucible]]).

## Entry Points

- `prism-analytics/README.md`
- `prism-analytics/crates/prism/src/lib.rs`
- `prism-analytics/crates/prism/src/engine.rs`
- `prism-analytics/crates/prism/src/model.rs`
- `prism-analytics/crates/prism/src/fuzzy/`
- `prism-analytics/crates/prism/src/packs/`
- `prism-analytics/crates/prism/tests/`

See also: [[Architecture/Dependency Rules]]

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

## Fuzzy Inference

`prism::fuzzy` is built. `FuzzyInferencePack` ships and is available infra — do not propose rebuilding it.

The engine supports Mamdani, Sugeno, and Tsukamoto inference with configurable membership functions and defuzzification. Key types:

- `MembershipDegree` — `f64` in [0, 1] with `serde(transparent)`. The single typed float for all membership/confidence values in the fuzzy module. Do not use bare `f64` here.
- `MembershipFunction::evaluate()` returns `MembershipDegree`.
- `FuzzyInferenceOutput.confidence` and `.memberships` are `MembershipDegree`.

Use `arbiter` for hard policy, `ferrox` for hard constraints and optimization, and `mnemos` for memory and historical grounding.

See also: [[Architecture/Expert Portfolio Architecture]]

## Domain Primitives

`crates/prism/src/primitives.rs` — constrained types for analytics pack structs. Do not substitute raw floats or `usize`.

| Type | Constraint | Used in |
|---|---|---|
| `UnitFraction` | `f64` in [0, 1], custom `Deserialize` enforcement | forecast confidence, anomaly scores, classification probabilities |
| `ZScoreThreshold` | `f64` > 0, custom `Deserialize` enforcement | anomaly detection threshold |
| `NonZeroUsize` | re-exported from `std` | required-count fields across packs |

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

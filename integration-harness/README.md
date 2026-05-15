# Mosaic Integration Harness

Executable cross-extension checks for the extensions container.

This crate is intentionally not a workspace root and not a Converge
foundation crate. It is a product-side assembly harness that depends on
extension crates by path and verifies selected flows across repositories.

Current golden flow:

```text
Mnemos knowledge recall
  + Prism fuzzy risk signal
  -> Arbiter Cedar expense gate
```

Run it from the container root:

```sh
just integration-test
```

Optional solver bridge:

```text
Arbiter CedarAnalysisSuggestor
  -> Cedar/SymCC generated SMT
  -> Soter CVC5 FFI
  -> CedarAnalysisReport
```

Run it from this directory when the Soter CVC5 FFI build is available:

```sh
cargo test --all-targets --features soter-cvc5
```

The bridge is intentionally product-side assembly. Arbiter owns the Cedar
policy model and suggestor contract; Soter owns native CVC5 execution.

Crucible trained-classifier flow:

```text
crucible::fixtures::loan_default (synthetic dataset)
  -> crucible::RandomForestModel (linfa-trees bagging of CART)
  -> crucible::RandomForestClassifierSuggestor
     reads ClassificationFeaturesPayload from Seeds
     emits ClassPredictionPayload into Evaluations
```

Exercised by `tests/crucible_loan_classifier.rs`: three tests cover the
high-risk applicant (predicted default), the low-risk applicant
(predicted non-default), and the non-features payload rejection
(empty `Evaluations` when seeded with a `TextPayload`). The default
test set runs without any feature flag — no CVC5 dependency.


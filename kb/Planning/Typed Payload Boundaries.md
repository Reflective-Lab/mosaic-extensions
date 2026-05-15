---
tags: [planning, contracts, proposed-facts]
source: mixed
date: 2026-05-14
---
# Typed Payload Boundaries

Typed provenance is only the first half of the `ProposedFact` boundary.
The current Converge contract still stores proposal content as `String`
under broad `ContextKey`s, so many fact families are typed inside an
extension but informal at the shared context boundary.

## Current State

`converge-pack::ProposedFact` carries:

- `key: ContextKey`
- `id: ProposalId`
- `content: String`
- `confidence`
- `provenance: String`

The extensions now route proposal construction through typed
`ProvenanceSource` adapters, but payload typing is not enforced by
`ProposedFact` itself.

Many extensions already define Rust DTOs and parse/emit them locally:

- Arbiter: `DecideRequest`, `CedarAnalysisInput`, `CedarAnalysisReport`.
- Soter: `SmtQuery`, `SmtReport`.
- Ferrox: `LpRequest`, `MipRequest`, `CpSatRequest`, scheduling/job-shop/
  network-flow request and plan DTOs.
- Prism: feature, training, evaluation, monitoring, inference, and fuzzy
  analytic DTOs.

That is useful, but it is not yet a contract. The shared boundary still
relies on conventions such as id prefixes, selected `ContextKey`s, and
manual `serde_json::from_str::<T>(fact.content())` calls.

Some families remain intentionally loose today:

- diagnostics as JSON objects with `source` and `message`;
- Mnemos recall snippets with ad hoc `source`, `query`, `title`,
  `content`, and `score`;
- Arbiter budget, approval, compliance, and data-classification gates that
  inspect generic JSON fields or raw text.

## Conclusion

The concern is correct: typed `ProposedFact` construction is currently
partial. Provenance and some local payload DTOs are typed, but the kernel
cannot know that `ContextKey::Evaluations` contains a `CedarAnalysisReport`
rather than a Prism evaluation report, a Soter SMT report, or arbitrary JSON.

The next useful bar is not "make all facts generic over Rust types." That
would fight the dynamic convergence model. The next useful bar is a
schema-backed fact-family boundary.

## Recommended Contract

For reusable or high-risk fact families, define a small contract object:

```rust
pub trait FactFamily {
    const FAMILY: &'static str;
    const VERSION: u16;
    const INPUT_KEY: ContextKey;
    const OUTPUT_KEY: ContextKey;
    type Payload: serde::Serialize + serde::de::DeserializeOwned;

    fn id_prefix() -> &'static str;
    fn validate(payload: &Self::Payload) -> Result<(), FactPayloadError>;
}
```

Then add additive helpers around the existing string-backed contract:

```rust
fn propose_typed<F: FactFamily>(
    id_suffix: impl AsRef<str>,
    payload: &F::Payload,
    provenance: impl Provenance,
) -> Result<ProposedFact, FactPayloadError>;

fn parse_typed<F: FactFamily>(
    fact: &ContextFact,
) -> Result<F::Payload, FactPayloadError>;
```

This preserves the current wire format while making the payload contract
auditable and testable.

## Prioritization

Do this first for high-risk and cross-extension families:

1. Arbiter Cedar decisions and Cedar Analysis reports.
2. Soter SMT queries and reports.
3. Ferrox solver requests and plans.
4. Prism evaluation, monitoring, and fuzzy threshold outputs used by policy.
5. Mnemos formalized knowledge facts, not every recall snippet.

Leave low-risk diagnostics and human-readable notes loose until they become
promotion inputs.

## Acceptance Criteria

- A fact family declares its expected input/output `ContextKey`, id prefix,
  payload type, schema version, and validator.
- Construction and parsing go through typed helpers rather than raw
  `serde_json::to_string` / `serde_json::from_str` at every call site.
- Malformed payloads produce diagnostics or empty effects; they do not panic.
- Cross-extension tests assert that product-side fixtures satisfy the same
  payload contract used by suggestors.

This is an upstream Converge contract candidate only after at least two
extension families adopt the same shape.

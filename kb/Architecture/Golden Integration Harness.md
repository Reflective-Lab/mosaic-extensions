---
tags: [architecture, integration, testing]
source: mixed
date: 2026-05-13
---
# Golden Integration Harness

`integration-harness/` is the workspace-level executable check for flows that
cross extension repository boundaries.

It is not a Converge foundation crate and it is not an extension package. It is
a product-side assembly harness: it depends on extension crates by path, wires
their public APIs together, and verifies one concrete scenario.

## Current Golden Flow

```text
Mnemos knowledge recall
  + Prism fuzzy risk signal
  -> Arbiter Cedar expense gate
```

The first test seeds a Mnemos knowledge entry for expense approvals, runs a
Prism fuzzy-inference pack through the Converge engine, retrieves the knowledge
fixture through `KnowledgeRetrievalSuggestor`, and then maps the high-risk
signal into an Arbiter `DecideRequest`.

The Arbiter decision remains the authority boundary. Prism provides analytic
context, Mnemos provides recall context, and Cedar decides the policy outcome.

## Command

```bash
just integration-test
```

## Boundary

- The harness may depend on multiple extension crates.
- Extension crates must not depend on the harness.
- The harness should encode app-level golden paths, not reusable contracts.
- New reusable contracts still belong upstream only after cross-family reuse is
  real.

See also: [[Architecture/Runtime Assembly]],
[[Standards/Suggestor Contract]], [[Planning/MILESTONES]]

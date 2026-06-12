---
tags: [architecture, extensions, topology]
source: mixed
date: 2026-05-05
---
# Extension Topology

`/Users/kpernyer/dev/reflective/mosaic-extensions` is the canonical home
for reusable Mosaic extensions.

The dependency rule is one-way:

```text
converge contracts <- extensions <- products / deployments
```

Converge does not import extension repositories. Products, demos, and runtime
assemblies wire extension crates into runnable systems.

## Sub-Families

The container holds three kinds of member. All ride the same Converge
contracts and release checklist; the split is what each member *is for*,
not how it is built.

- **Reasoning extensions** answer an epistemic question and emit tiered
  evidence into the promotion path. Each has a `kb/Positioning.md` with its
  pitch and boundary one-liners.
- **Connectivity extensions** move and observe: source-shaped connector
  ports and vendor-hiding adapters. They carry no judgment of their own.
- **Assembly** is product-side verification across repositories — not an
  extension, intentionally not a workspace root, and path-dependent on its
  siblings by design.

## Canonical Homes

### Reasoning extensions

| Directory | Extension | Answers | Owns |
|---|---|---|---|
| `arbiter-policy/` | `arbiter` | Should this concrete request be allowed now? (`Decided`) | Policy engines, Cedar wiring, authorization gates, delegation checks, and policy suggestors. |
| `crucible-models/` | `crucible` | What does a model fitted to our data predict? | Training pipelines, trained-artifact packs, and classifier suggestors. |
| `ferrox-solvers/` | `ferrox` | What is the best feasible plan? (`Searched`) | Native optimization solver integrations and solver suggestors. |
| `mnemos-knowledge/` | `mnemos` | What do we already know? | Knowledge, recall, retrieval, memory, vector stores, and learning suggestors. |
| `prism-analytics/` | `prism` | What does the data say, closed-form? (`Observed`/`Argued`) | Closed-form analytics, feature extraction, inference, fuzzy logic, and analytic packs. |
| `soter-smt/` | `soter` | Can any modeled request violate this invariant? (`Searched`) | SMT query modeling, CVC5 native bindings, solver reports, and solver-backed suggestors. |

### Connectivity extensions

| Directory | Extension | Owns |
|---|---|---|
| `embassy-ports/` | `embassy` | External-party connector ports where source identity is part of the API. |
| `manifold-adapters/` | `manifold` | Generic provider, storage, vector, fetch, feed, and tool adapters where vendors should be hidden behind capabilities. |

### Assembly

| Directory | Owns |
|---|---|
| `integration-harness/` | Executable cross-extension checks; product-side assembly harness with path deps on sibling extensions. |

If the connectivity family later grows shared platform concerns (rate
limiting, credential handling), that is the point at which a separate
container earns its keep — not before.

## Port, Provider, Backend, Suggestor

- **Port:** trait plus request/response DTOs and semantic value types.
- **Provider / adapter:** concrete implementation of a port or Converge contract.
- **Backend / capability:** operational provider layer selected by requirements.
- **Suggestor:** purposeful Converge participant that reads context and proposes facts.

Ports may live in Converge when universal. Source-specific ports live in
extensions. Concrete providers and adapters live downstream from the contract
they implement.

## Promotion Rule

If an extension-local contract becomes universal across extension families,
promote the contract upstream to Converge. Keep implementation downstream.

Do not promote a contract only because one extension needs it. The promotion
test is cross-family reuse and foundation relevance.

See also: [[Architecture/Port Provider Boundary]], [[Architecture/Dependency Rules]]

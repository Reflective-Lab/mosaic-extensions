---
tags: [architecture, extensions, topology]
source: mixed
date: 2026-05-05
---
# Extension Topology

`/Users/kpernyer/dev/extensions` is the canonical home for reusable Converge
extensions.

The dependency rule is one-way:

```text
converge contracts <- extensions <- products / deployments
```

Converge does not import extension repositories. Products, demos, and runtime
assemblies wire extension crates into runnable systems.

## Canonical Homes

| Directory | Extension | Owns |
|---|---|---|
| `arbiter-policy/` | `arbiter` | Policy engines, Cedar wiring, authorization gates, delegation checks, and policy suggestors. |
| `atelier-showcase/` | `atelier` | Worked examples and cross-platform showcase material. |
| `embassy-ports/` | `embassy` | External-party connector ports where source identity is part of the API. |
| `ferrox-solvers/` | `ferrox` | Native optimization solver integrations and solver suggestors. |
| `manifold-adapters/` | `manifold` | Generic provider, storage, vector, fetch, feed, and tool adapters where vendors should be hidden behind capabilities. |
| `mnemos-knowledge/` | `mnemos` | Knowledge, recall, retrieval, memory, vector stores, and learning suggestors. |
| `prism-analytics/` | `prism` | Analytics, ML pipelines, feature extraction, inference, training, monitoring, and analytic packs. |

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

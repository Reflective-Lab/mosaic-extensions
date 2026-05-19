---
tags: [index]
source: mixed
date: 2026-05-05
---
# KB Entity Catalog

Curated catalog for the Converge extensions workspace.

## Extension Repositories

| Entity | Description | Location |
|---|---|---|
| Arbiter | Cedar policy engine and policy gate suggestors | `arbiter-policy/` |
| Crucible | Training pipelines, trained-artifact packs, and classifier suggestors | `crucible-models/` |
| Embassy | Source-specific connector ports where foreign-system identity is part of the contract | `embassy-ports/` |
| Ferrox | Native optimization solver integrations and solver suggestors | `ferrox-solvers/` |
| Manifold | Generic storage, vector, provider, and tool adapters | `manifold-adapters/` |
| Mnemos | Knowledge base, recall, retrieval, storage, and memory suggestors | `mnemos-knowledge/` |
| Prism | Closed-form analytics, feature extraction, inference, and fuzzy suggestors | `prism-analytics/` |
| Soter | SMT-backed safety evidence, CVC5 FFI, and solver-backed suggestors | `soter-smt/` |

## Cargo Packages

| Package | Repository | Role |
|---|---|---|
| `converge-arbiter-policy` | `arbiter-policy/` | Cedar PDP, policy decisions, delegation verification, flow gates |
| `converge-crucible-models` | `crucible-models/` | Training pipelines, trained-artifact packs, and classifier suggestors |
| `converge-embassy-pack` | `embassy-ports/` | Shared connector call context and provenanced observations |
| `converge-embassy-linkedin` | `embassy-ports/` | LinkedIn connector port and stub provider |
| `converge-ferrox-solver` | `ferrox-solvers/` | CP-SAT, LP, MIP, scheduling, routing, and job-shop suggestors |
| `converge-ferrox-server` | `ferrox-solvers/` | gRPC solver service |
| `converge-ferrox-ortools-sys` | `ferrox-solvers/` | OR-Tools native binding wrapper |
| `converge-ferrox-highs-sys` | `ferrox-solvers/` | HiGHS native binding wrapper |
| `converge-manifold-adapters` | `manifold-adapters/` | Object-store, experience-store, provider, search, feed, tool, and vector adapters |
| `converge-mnemos-knowledge` | `mnemos-knowledge/` | Knowledge base library, CLI, gRPC server, and suggestor adapters |
| `converge-prism-analytics` | `prism-analytics/` | Analytics packs, feature agents, inference agents, and fuzzy inference packs |
| `converge-soter-smt` | `soter-smt/` | SMT query/report types and solver-backed suggestors |
| `converge-soter-cvc5-sys` | `soter-smt/` | Native CVC5 FFI boundary |

## Architecture

- [Extension Topology](Architecture/Extension%20Topology.md)
- [Converge Boundary](Architecture/Converge%20Boundary.md)
- [Repository Map](Architecture/Repository%20Map.md)
- [Dependency Rules](Architecture/Dependency%20Rules.md)
- [Port Provider Boundary](Architecture/Port%20Provider%20Boundary.md)
- [Runtime Assembly](Architecture/Runtime%20Assembly.md)
- [Golden Integration Harness](Architecture/Golden%20Integration%20Harness.md)
- [Pluralist Reasoning Substrate](Architecture/Pluralist%20Reasoning%20Substrate.md)
- [Expert Portfolio Architecture](Architecture/Expert%20Portfolio%20Architecture.md)
- [SMT vs Interactive Theorem Proving](Architecture/SMT%20vs%20Interactive%20Theorem%20Proving.md)
- [Extraction Status](Architecture/Extraction%20Status.md)

## Modules

- [Arbiter](Modules/Arbiter.md)
- [Crucible](Modules/Crucible.md)
- [Embassy](Modules/Embassy.md)
- [Ferrox](Modules/Ferrox.md)
- [Manifold](Modules/Manifold.md)
- [Mnemos](Modules/Mnemos.md)
- [Prism](Modules/Prism.md)
- [Soter](Modules/Soter.md)

## Building

- [Developer Guide](Building/Developer%20Guide.md)
- [Getting Started](Building/Getting%20Started.md)
- [Release and Versioning](Building/Release%20and%20Versioning.md)

## Workflow

- [Daily Journey](Workflow/Daily%20Journey.md)
- [Git Strategy](Workflow/Git%20Strategy.md)
- [Working with Codex](Workflow/Working%20with%20Codex.md)

## Standards

- [Extension Standard](Standards/Extension%20Standard.md)
- [Suggestor Contract](Standards/Suggestor%20Contract.md)

## Planning

- [Milestones](Planning/MILESTONES.md)
- [Upstream Handoff](Planning/Upstream%20Handoff.md)
- [Next Steps](Planning/Next%20Steps.md)

## Ecosystem

- [Ecosystem](Ecosystem.md)

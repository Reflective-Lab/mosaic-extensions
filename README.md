# Mosaic Extensions

Reusable specialist capability families for the Reflective stack.

Mosaic keeps Converge small and stable while still letting formations use
strong policy engines, trained models, source-specific ports, optimization
solvers, generic providers, memory, analytics, and SMT evidence.

## Workspace Shape

This directory is a multi-repo container, not a single Cargo workspace. Enter an
extension directory before running Cargo commands.

| Directory | Owns |
|---|---|
| `arbiter-policy/` | Cedar policy gates, delegation checks, and authorization Suggestors |
| `crucible-models/` | Training pipelines, trained-artifact packs, and classifier Suggestors |
| `embassy-ports/` | Source-specific connector ports where external identity is part of the contract |
| `ferrox-solvers/` | Native optimization solvers and solver-backed Suggestors |
| `manifold-adapters/` | Generic storage, vector, provider, fetch, feed, search, LLM, and tool adapters |
| `mnemos-knowledge/` | Knowledge, recall, retrieval, storage, and memory Suggestors |
| `prism-analytics/` | Closed-form analytics, feature extraction, inference, fuzzy logic, and analytic packs |
| `soter-smt/` | SMT-backed safety evidence, CVC5 FFI, and solver Suggestors |

## Boundary

Dependency direction is one way:

```text
Converge contracts <- Mosaic extensions <- products / deployments
```

Converge owns the contract and promotion authority. Mosaic owns reusable
implementation-heavy specialists. Products, Runway deployments, Commerce Rails,
and customer systems assemble the specialists they need.

## Start Here

- `AGENTS.md` for agent entrypoint rules.
- `REVIEW-GUIDE.md` for code quality standards — types, smells, Converge compliance, and reference implementations.
- `kb/Home.md` for the extension knowledge base.
- `kb/Architecture/Extension Topology.md` for ownership and dependency flow.
- `kb/Standards/Suggestor Contract.md` for extension Suggestor rules.

## Commands

Use the root `Justfile` for cross-repo checks:

```bash
just check
just test
just fmt-check
just clippy
just status
```

Use a subproject `Justfile` for focused work:

```bash
cd manifold-adapters
just check
just test
just lint
```

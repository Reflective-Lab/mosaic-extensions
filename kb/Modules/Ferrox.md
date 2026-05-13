---
tags: [module, optimization, solvers]
source: mixed
date: 2026-05-05
---
# Ferrox

`ferrox` owns native optimization solver integrations and solver-backed
Converge suggestors.

It makes OR-Tools CP-SAT and HiGHS available to formations so LLM, policy,
knowledge, analytics, and mathematical optimization agents can participate in
the same convergence run.

## Owns

- CP-SAT problem types and suggestors.
- LP and MIP problem types and suggestors.
- Multi-agent task scheduling.
- Job-shop scheduling.
- Vehicle routing with time windows.
- Formation selection optimization.
- Native OR-Tools and HiGHS bindings.
- Optional gRPC solver service.
- Typed Ferrox proposal provenance at the `ProposedFact` boundary.
- `ferrox.suggestor.execute` tracing spans at solver suggestor boundaries.

## Current Crates

| Crate | Role |
|---|---|
| `ferrox-solver` | Library surface for solver suggestors and problem models. |
| `ferrox-server` | gRPC service wrapper. |
| `ferrox-ortools-sys` | OR-Tools native wrapper. |
| `ferrox-highs-sys` | HiGHS native wrapper. |

## Suggestor Families

- `GreedySchedulerSuggestor` and `CpSatSchedulerSuggestor`
- `GreedyJobShopSuggestor` and `CpSatJobShopSuggestor`
- `NearestNeighborSuggestor` and `CpSatVrptwSuggestor`
- `CpSatSuggestor`
- `GlopLpSuggestor`
- `HighsMipSuggestor`
- `CpSatFormationSuggestor`

## Feature Flags

- `ferrox-solver` default features are empty.
- `ortools` enables OR-Tools-backed CP-SAT functionality.
- `highs` enables HiGHS-backed MIP functionality.
- `full` enables both.
- `ferrox-server` defaults to `full`.

## Entry Points

- `ferrox-solvers/README.md`
- `ferrox-solvers/crates/ferrox/src/lib.rs`
- `ferrox-solvers/crates/ferrox/src/*/problem.rs`
- `ferrox-solvers/crates/ferrox/src/*/suggestor.rs`
- `ferrox-solvers/examples/*/src/main.rs`

See also: [[Architecture/Runtime Assembly]]

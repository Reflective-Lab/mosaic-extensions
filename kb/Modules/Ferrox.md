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
- Structured solver identity on solver outputs, including native backend,
  version, expected/actual checkout commit, source mode, build flags, runtime
  config, and Rust crate version.

## Current Crates

| Crate | Role |
|---|---|
| `ferrox-solver` | Library surface for solver suggestors and problem models. |
| `ferrox-server` | gRPC service wrapper. |
| `ferrox-ortools-sys` | OR-Tools native wrapper. All 56 unsafe blocks carry `// SAFETY:` comments. |
| `ferrox-highs-sys` | HiGHS native wrapper. All 13 unsafe blocks carry `// SAFETY:` comments. |

## Domain Types

`crates/ferrox/src/domain_types.rs` defines typed primitives shared across solver problem structs. Do not use raw `i32`/`usize`/`i64` for these concepts.

| Type | Wraps | Used in |
|---|---|---|
| `NodeId` | `i32` | `network_flow` arc/supply node indices |
| `Minutes` | `i64` | `scheduling` and `jobshop` time windows and durations |
| `TaskId` | `usize` | `scheduling` task identifiers |
| `AgentId` | `usize` | `scheduling` agent identifiers |
| `MachineId` | `usize` | `jobshop` machine identifiers |
| `JobId` | `usize` | `jobshop` job identifiers |
| `ProcessingTime` | `i64` | `jobshop` operation durations |

## Solve-Status Types

All seven problem types use typed enums for solver outcome — not `status: String`:
`LpSolveStatus`, `MipSolveStatus`, `CpSolveStatus`, `SchedulingSolveStatus`, `VrptwSolveStatus`, `JobShopSolveStatus`, `FlowSolveStatus`.

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

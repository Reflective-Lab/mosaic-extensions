---
tags: [planning, roadmap]
source: mixed
date: 2026-05-13
---
# Milestones

Integration-driven roadmap for the extensions workspace. The spine is the
in-flight `arbiter` + `prism` work; everything else is pulled by that spine
or by an explicit app need. Items here are scoped to be executable, not
architecturally aspirational.

See also: [[Planning/Next Steps]], [[Architecture/Extension Topology]],
[[Architecture/Dependency Rules]].

## Anchor

The active integration is analytics-informed Cedar policy. Arbiter has the
first `cedar-policy-symcc` preparation and solver-execution slices on top of
the 4.10 runtime upgrade, an explicit solver CI policy, and the first
product-side golden harness. Conditional invariant queries are the next Arbiter
assurance step. All short-term milestones below are scoped to make that spine
production-quality before new surface area is added.

Governing heuristic: **friction inside the active integration is the
backlog.** Missing types, missing test harnesses, missing trace fields,
and missing trait methods that show up while wiring `arbiter` + `prism`
are the work. Architectural symmetry is not a reason to build.

## Completed 2026-05-14

- Added `soter-smt` as the SMT-backed safety evidence extension with CVC5 FFI
  isolated in a sys crate, default fake-solver tests, and Formation-facing
  `soter.smt` discovery.

## Completed 2026-05-13

- Replaced string-literal provenance at `ProposedFact` construction sites
  with extension-local typed `ProvenanceSource` adapters for Arbiter,
  Ferrox, Prism, and Mnemos. Embassy and Manifold do not currently emit
  proposed facts.
- Added suggestor-boundary tracing spans for Arbiter, Ferrox, Prism, and
  Mnemos with structured provenance, suggestor name, context keys, and
  input count fields.
- Documented the workspace Suggestor contract: read/write boundaries,
  provenance and tracing requirements, side-effect rules, error handling,
  async span behavior, and test expectations.
- Added a root `integration-harness/` crate plus `just integration-test` for
  the first Arbiter + Prism + Mnemos golden flow: Mnemos recall and Prism fuzzy
  risk context feeding an Arbiter Cedar expense gate.
- Made Arbiter's CVC5 policy explicit: fake-solver SymCC tests are required in
  PR/push CI, while real CVC5 is a scheduled/manual smoke lane until
  conditional invariant queries encode actual Arbiter claims.

## Short-term (2-6 weeks)

- Add conditional Cedar Analysis queries for actual high-risk Arbiter claims.
  This is the missing step before real CVC5 can mean invariant assurance
  rather than solver-path smoke.
- Prove Soter's native CVC5 link path (`just deps`, `just check-cvc5`) and
  add a first CVC5-backed SMT-LIB solving slice.
- Extend the golden harness only when a second app-level path pulls on it.
  Keep it product-side and avoid turning it into a shared framework.

## Mid-term (2-4 months)

- Second integration, pulled by an app: either `arbiter` +
  `crucible-models` (trained classifier feeding policy context) or
  `ferrox-solvers` + `prism` (optimization with fuzzy-weighted
  objectives). Pick when an app pulls, not on architectural grounds.
- Make the convergence kernel's promotion rules explicit. Per
  `EvidenceTier`, document the quorum, conflict-resolution rule, and
  staleness window. Configurable per `ContextKey`. The kernel's
  promotion semantics are currently hand-wave; make them load-bearing.
- Workspace benchmark suite. Criterion benches for cold suggestor chain
  latency, fuzzy inference throughput at realistic rule-base size, and
  Cedar evaluation cost under production-scale policy counts. Targets
  later; baseline first so regressions surface.
- Public-API audit per crate. Anything not consumed by another crate or
  an app becomes `#[doc(hidden)]` or private. Shrinks contract surface
  and makes future refactors cheaper.

## Explicitly deferred

These are written down so they stop tempting future planning rounds.
Each has a clear re-open condition.

- **Lean / Coq / Agda formal verification layer.** Re-open only when a
  specific compliance or audit claim demands a checked proof artifact.
  Cedar Analysis and symbolic compilation cover near-term assurance
  needs. See LOG entry 2026-05-13.
- **Additional SMT backends beyond CVC5.** Re-open when an app needs a second
  solver. CVC5 now has a dedicated extension home in `soter-smt`.
- **Generalized `certus-*` claim-registry crate.** Re-open when a third
  consumer of the same claim machinery exists. Until then, registry
  types live inside the crate that uses them.
- **`mnemos` causal and meta-learning extensions.** Heavy, no current
  app pull.
- **Standalone fuzzy-logic extension.** Stays inside `prism-analytics`
  until a real DSL or runtime boundary exists. See LOG entry
  2026-05-08.

## Promotion criteria for new milestones

A candidate item enters Short-term only if it satisfies all of:

- An identified app or in-flight integration pulls on it.
- It can be scoped to ship in a single PR or a small sequence.
- It does not introduce a new public crate surface unless an existing
  crate already exposes a gap that cannot be closed in place.
- It respects the dependency direction
  (`converge contracts <- extensions <- products`).

Items that fail any of these belong in Deferred with a re-open condition,
not in Mid-term.

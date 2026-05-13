---
tags: [log]
source: mixed
date: 2026-05-05
---
# KB Mutation Log

## 2026-05-05

- Created the extensions KB for `/Users/kpernyer/dev/extensions`.
- Seeded architecture pages from the Converge and Organism KB boundary docs.
- Seeded module pages from each local README, Cargo manifest, and public API surface.
- Recorded extraction status, known version drift, and follow-up work.
- Standardized extension project hygiene expectations around `README.md`,
  `AGENTS.md`, `Justfile`, and GitHub community health files.

## 2026-05-06

- Renamed local checkout directories to the codename-topic convention:
  `arbiter-policy`, `atelier-showcase`, `embassy-ports`, `ferrox-solvers`,
  `manifold-adapters`, `mnemos-knowledge`, and `prism-analytics`.
- Updated workspace Markdown paths and cross-repo command examples to use the
  new local directory names while preserving crate and GitHub repository names.
- Added `kb/Building/Developer Guide.md`, adapting the Converge developer guide
  for the extensions container and covering all seven extension homes.
- Aligned planned extension release versions to the Converge `3.8.1` baseline:
  `arbiter-policy`, `atelier-showcase`, `embassy-ports`, `manifold-adapters`,
  `mnemos-knowledge`, and `prism-analytics` at `1.0.0`; `ferrox-solvers` at
  `0.4.1`.
- Added actual Cargo package names to release, developer-guide, and entity
  catalog tables.

## 2026-05-08

- Added `kb/Architecture/Expert Portfolio Architecture.md`, documenting why
  expectation-aware systems should be assembled from specialized reasoning
  experts rather than one model.
- Recorded the fuzzy-logic boundary decision: start with a `prism` analytic
  capability and pack wrapper, keep hard policy in `arbiter`, hard constraints
  and optimization in `ferrox`, memory and recall in `mnemos`, and defer a
  standalone fuzzy extension until a real DSL/runtime boundary exists.

## 2026-05-13

- Recorded the Arbiter Cedar-first assurance decision: use Cedar validation,
  runtime tests, and Cedar Analysis / symbolic compilation before adding a
  custom Lean, Coq, or Agda verification layer.
- Upgraded Arbiter's Cedar runtime dependency from the 2.4 line to the 4.10
  line, leaving `cedar-policy-symcc` integration as the next separate slice.
- Added the first optional SymCC-backed Arbiter analysis preparation slice:
  an expense schema artifact, symbolic compilation, pinned preparation hashes, and
  happy-path/negative/property tests. Solver execution and counterexample
  capture remain the next slice.
- Added solver-backed Arbiter analysis execution with per-environment statuses,
  CVC5 helper wiring, deterministic fake-solver tests, and counterexample
  capture. Real solver CI policy remains open.
- Tightened Arbiter HITL escalation semantics so denied requests become
  `Escalate` only when Cedar would allow the same request after explicit human
  approval; hard policy denials remain `Reject`.
- Added Arbiter Formation discovery metadata under the `arbiter.cedar`
  capability family and an explicit `CedarHitlGateSuggestor` registration
  surface for strict Cedar-backed HITL gates.
- Added Arbiter suggestor-boundary tracing spans under
  `arbiter.suggestor.execute`; workspace-wide tracing for every extension
  remains open.
- Added Arbiter's typed `ProvenanceSource` proposal-boundary adapter; broader
  workspace adoption or upstream `converge-pack` support remains a separate
  follow-on.
- Migrated Ferrox, Prism, and Mnemos proposal construction to typed
  extension-local `ProvenanceSource` adapters while preserving the current
  string-backed `converge-pack::ProposedFact` contract.
- Extended suggestor-boundary tracing from Arbiter to Ferrox, Prism, and
  Mnemos using extension-local `*.suggestor.execute` spans with typed
  provenance fields.
- Added `kb/Standards/Suggestor Contract.md`, making extension suggestor
  read/write, provenance, tracing, side-effect, async, error-handling, and
  test expectations explicit.
- Added `kb/Planning/MILESTONES.md` as the integration-driven roadmap for
  the workspace. Anchors short-term work to the `arbiter` + `prism` spine
  and the `cedar-policy-symcc` slice; records deferred items with explicit
  re-open conditions (Lean/Coq/Agda, SMT, `certus-*` registry crate,
  standalone fuzzy extension, mnemos causal extensions).

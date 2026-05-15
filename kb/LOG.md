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

## 2026-05-15

- Restored the prism / crucible boundary by lifting the training pipeline
  (`ingest`, `storage`, `training` modules + `DatasetAgent`,
  `HyperparameterSearchAgent`, `ModelTrainingAgent`, `ModelEvaluationAgent`,
  `ModelRegistryAgent`, `MonitoringAgent`, `DeploymentAgent`,
  `SampleInferenceAgent`, plus the supporting types) out of `prism-analytics`
  and into `crucible-models`. Prism is back to closed-form inference only.
  `reqwest`, `bincode`, and `converge-storage` deps dropped from prism along
  with the `storage` feature. Crucible gained typed `ProvenanceSource::Crucible`
  and `crucible.suggestor.execute` spans matching the workspace Suggestor
  Contract. BREAKING change at the prism crate boundary.
- Updated `kb/Modules/Prism.md` to reflect the closed-form-only surface;
  created `kb/Modules/Crucible.md` documenting the training pipeline,
  planned packs, feature flags, continuous-learning position, and
  cross-extension boundary.

## 2026-05-14

- Added `soter-smt` as the SMT-backed safety evidence extension home, including
  root catalog entries, repository-map ownership, CVC5 FFI ownership, and the
  `soter.smt` capability family.
- Added the feature-gated integration harness bridge from Arbiter's
  `CedarAnalysisSuggestor` through Cedar/SymCC generated SMT into Soter's CVC5
  FFI backend. This keeps Arbiter as the Cedar policy owner and Soter as the
  SMT execution owner.
- Recorded the Arbiter model-adequacy boundary: the first conditional Cedar
  claim now has review fixtures, boundary cases, and a mutant-policy negative
  case before solver output is trusted as useful evidence.
- Clarified that real CVC5 is exercised through Soter and the integration
  harness today; Arbiter's own local-`cvc5` smoke tests are implemented but
  still need a scheduled/manual CI gate with `CVC5` or `cvc5` on `PATH`.
- Added `kb/Architecture/Pluralist Reasoning Substrate.md` — long-form essay
  walking the eight extensions with mathematical grounding (Zadeh, Mamdani,
  Sugeno, Tsukamoto, Cortes–Vapnik, Breiman, Freund–Schapire, Jang, Finn,
  Nichol, Kirkpatrick, Shinn), per-crate business scenarios, CVC5 attribution
  (Stanford and University of Iowa), a deferred-Lean hint, and a Formation
  diagram traced through a €25,000-invoice decision.
- Audited the five fact-emitting extensions (arbiter, prism, mnemos, ferrox,
  soter) and confirmed full adherence to the typed `ProvenanceSource` and
  `<crate>.suggestor.execute` tracing-span contract. Crucible, manifold, and
  embassy are out of scope (no Suggestor implementations). Added
  `kb/Planning/Upstream Handoff.md` recording the three Converge and three
  Organism platform tasks needed to lift these conventions from
  code-review-enforced to platform-enforced contracts.
- Audited the `ProposedFact` payload boundary and recorded that typed
  provenance is only partial type safety: payloads still cross Converge as
  `String` content under broad `ContextKey`s. Added
  `kb/Planning/Typed Payload Boundaries.md` and updated the Suggestor
  Contract and upstream handoff with the next schema-backed fact-family bar.

## 2026-05-13

- Added `integration-harness/` and `just integration-test` for the first
  product-side golden flow across Mnemos, Prism, and Arbiter. Documented it in
  `kb/Architecture/Golden Integration Harness.md` and linked it from runtime
  assembly and the entity catalog.
- Marked the selected short-term items complete in `kb/Planning/MILESTONES.md`
  while keeping conditional Arbiter invariant queries as the next required
  step before real CVC5 becomes assurance evidence.
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

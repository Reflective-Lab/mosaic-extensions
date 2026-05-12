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

# Converge Extensions Workspace

This is the canonical agent entrypoint for
`/Users/kpernyer/dev/reflective/mosaic-extensions`.

This directory is a multi-repo container for reusable Mosaic extension
families. It is not a single Cargo workspace and it is not itself the Converge
foundation.

## Start Here

1. Read `README.md`.
2. Read `kb/Home.md`.
3. Follow one relevant KB link, usually `kb/Architecture/Extension Topology.md`.
4. Enter the specific extension directory before running Cargo commands.
5. Read that extension's `AGENTS.md`, `README.md`, and `Cargo.toml`.

Do not bulk-read the entire KB or every extension unless the task is explicitly
cross-cutting.

## Extension Repositories

| Directory | Owns |
|---|---|
| `arbiter-policy/` | Cedar policy gates, delegation checks, and authorization suggestors. |
| `crucible-models/` | Training pipelines, trained-artifact packs, and classifier suggestors. |
| `embassy-ports/` | Source-specific connector ports where the external system identity is part of the contract. |
| `ferrox-solvers/` | Native optimization solvers and solver-backed suggestors. |
| `manifold-adapters/` | Generic storage, vector, provider, and tool adapters. |
| `mnemos-knowledge/` | Knowledge, recall, retrieval, storage, and agentic memory suggestors. |
| `prism-analytics/` | Closed-form analytics, feature extraction, inference, fuzzy logic, and analytic packs. |
| `soter-smt/` | SMT-backed safety evidence, CVC5 FFI, and solver suggestors. |

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
cd arbiter-policy
just check
just test
just lint
```

## Code Quality

Read `REVIEW-GUIDE.md` before reviewing or writing extension code. It defines the
quality standards for this workspace: strong types over strings, no anonymous numeric
primitives, Converge compliance, production panic safety, and the reference
implementations to copy.

## Shared Infrastructure

Before adding a helper, check whether it already exists in the shared modules below.
Do not reinvent these.

| Location | What it provides |
|---|---|
| `embassy_pack::simple_id!` | Declarative macro — generates a newtype with `parse()`, `as_str()`, `Display`, `serde(transparent)`. Use for any source-specific identifier. |
| `embassy_pack::{SanctionsSubject, SanctionsHit, MatchType, SubjectType}` | Shared sanctions types. All sanctions ports (ofac-sls, eu-sanctions, commerce-csl) re-export from here. |
| `ferrox::domain_types::*` | `NodeId`, `Minutes`, `TaskId`, `AgentId`, `MachineId`, `JobId`, `ProcessingTime` — typed domain primitives for solver problem structs. |
| `arbiter::primitives::*` | `Confidence`, `CostUsd`, `ProposalCount`, `ProposalLimit`, `EpochSeconds` — constrained primitives for policy payload structs. |
| `prism::primitives::*` | `UnitFraction` [0,1], `ZScoreThreshold` >0 (both with `Deserialize` enforcement), re-exported `NonZeroUsize`. |
| `mnemos::math::cosine_similarity` | Canonical `cosine_similarity(a: &[f32], b: &[f32]) -> f32`. The only cosine implementation in mnemos. |
| `manifold::llm::retry` | `retry_with_backoff(max_retries, closure)` — shared exponential backoff (100ms × 2^attempt) used by all 6 LLM backends. |

## Rules

- Dependencies flow one way: `converge contracts <- extensions <- products`.
- Converge must not depend on any extension repository.
- Extension crates may depend on stable Converge contracts. Direct internal
  Converge dependencies are extraction debt unless explicitly documented.
- No `unsafe` code unless the crate is a native FFI wrapper whose entire reason
  to exist is to isolate the unsafe boundary.
- Keep heavy SDKs, native bindings, database drivers, and provider clients out
  of the Converge foundation.
- Preserve local changes you did not make. Several subprojects may have their
  own git state.
- Update `kb/` when architecture, boundaries, or extension ownership changes.

## Git Model

The parent folder is a workspace container. Check git status per extension:

```bash
git -C ferrox-solvers status --short --branch
```

Each named directory is an independent git repository. Check status per repo
before editing or committing.

Trunk branch: every extension uses `main` as the single trunk. There is no
`next` branch. Tag releases on `main`. Open PRs against `main`. (Migrated from
a `next`+`main` two-track model on 2026-05-28; bedrock-platform follows the
same rule.)

See also: `kb/Workflow/Git Strategy.md`.

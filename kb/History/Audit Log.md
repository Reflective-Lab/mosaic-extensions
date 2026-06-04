---
tags: [history, audit]
source: mixed
date: 2026-05-05
---
# Audit Log

## 2026-05-18 - Workspace Code Quality Sweep (Tiers 1–4)

Full idiomatic Rust audit across all 8 extension families. Findings fixed in four tiers:

**Tier 1 — Panic sites and non-deterministic hash**
- `partial_cmp().unwrap()` → `.total_cmp()` in mnemos (knowledge_base, batch), crucible (decision_tree, random_forest).
- Per-call `Regex::new(...)` → `LazyLock<Regex>` in arbiter (DataClassificationGateSuggestor).
- `DefaultHasher` → SHA-256 (first 8 bytes, same 16-hex format) in manifold (`contract.rs`) and embassy-pack (`content_hash`).
- Fallible constructors in manifold: `HttpFetchProvider::new()`, `HttpFeedProvider::new()`, `QwenVLReranker::new()` → `Result<Self, _>`; callers updated.

**Tier 2 — Type system**
- Solved-status strings (`status: String`) in all 7 ferrox problem types → typed enums: `LpSolveStatus`, `MipSolveStatus`, `CpSolveStatus`, `SchedulingSolveStatus`, `VrptwSolveStatus`, `JobShopSolveStatus`, `FlowSolveStatus`. gRPC boundary updated.
- Path fields in 9 crucible training structs: `String` → `PathBuf`.
- Sanctions types (`SanctionsSubject`, `SanctionsHit`, `MatchType`, `SubjectType`) moved from three port crates to `embassy-pack::sanctions`; port crates re-export with `pub use`.
- `MembershipDegree` newtype in prism fuzzy engine — replaces bare `f64` across `MembershipFunction`, `FuzzyRule`, `ActivatedRule`, `FuzzyInferenceOutput`. `serde(transparent)`, `clamp()` in constructor.

**Tier 3 — Anonymous numeric primitives, identifier validation, SAFETY comments**
- `arbiter::primitives`: `Confidence` [0,1], `CostUsd` ≥0, `ProposalCount`, `ProposalLimit`, `EpochSeconds` — applied to delegation.rs and payload structs.
- `prism::primitives`: `UnitFraction` [0,1], `ZScoreThreshold` >0 with custom `Deserialize` enforcement; `NonZeroUsize` re-export — applied to 6 analytics packs.
- `ferrox::domain_types`: `NodeId`, `Minutes`, `TaskId`, `AgentId`, `MachineId`, `JobId`, `ProcessingTime` — applied to network_flow, scheduling, jobshop problem/solver files.
- Embassy skeleton identifiers: format validation added to all 10 identifier newtypes using iterator char predicates (no regex dep).
- SAFETY comments on all 13 unsafe blocks in `highs-sys/src/lib.rs` and all 56 unsafe blocks in `ortools-sys/src/lib.rs`.

**Tier 4 — Structural / separation of concerns**
- `crucible/src/training.rs` (2268 lines) split into 7-module directory: `mod.rs`, `types.rs`, `io.rs`, `dataset.rs`, `features.rs`, `pipeline.rs`, `evaluation.rs`. All 118 tests pass.
- `mnemos/src/math.rs` added: canonical `cosine_similarity(a: &[f32], b: &[f32]) -> f32` — replaces 4 inline duplicates in knowledge_base, batch, embedding, meta.
- `embassy-pack::macros`: `simple_id!` declarative macro — replaces manual boilerplate in all 10 skeleton identifier crates.
- `manifold/src/llm/retry.rs`: `retry_with_backoff(max_retries, closure)` — replaces inline exponential backoff in 6 LLM backends (openai, gemini, mistral, openrouter, staik, kong).
- LinkedIn port (345 lines, well-sectioned): left as single file — splitting would add indirection without benefit.
- `EmbassySuggestor` blanket impl: deferred. Provider method names (`fetch`/`screen`/`validate`) and payload shapes are too varied across 12 ports; a new `EmbassyProvider` supertrait would be needed first.

**Artefacts created during sweep**
- `REVIEW-GUIDE.md` at workspace root — reusable 9-section quality reference wired into `AGENTS.md`, `README.md`, and `kb/Standards/Review Guide.md`.

Test counts at completion: 804+ tests green across the four Tier 3 repos; 276 + 149 + 218 + 118 across the four Tier 4 repos.

## 2026-05-05 - KB Creation Pass

Sources read:

- `/Users/kpernyer/dev/work/kb`
- `/Users/kpernyer/dev/work/converge/kb`
- `/Users/kpernyer/dev/work/organism/kb`
- Local extension READMEs, Cargo manifests, and public `lib.rs` files.

Key findings:

- The Converge KB already names `/Users/kpernyer/dev/reflective/mosaic-extensions` as the
  canonical home for reusable extensions.
- The current extension set is `arbiter`, `embassy`, `ferrox`, `manifold`,
  `mnemos`, and `prism`.
- The parent folder is not a git repo.
- `arbiter`, `ferrox`, `mnemos`, and `prism` have local `.git` directories.
- `embassy` and `manifold` appeared to be scaffolds without local `.git`
  directories.
- `manifold` did not have a README.
- Converge dependency versions differ across the extracted repos and need a
  release cleanup pass.

No cargo checks were run as part of this KB creation pass.

## 2026-05-05 - Extension Docs Standardization

Added or standardized:

- Root `AGENTS.md` and root cross-repo `Justfile`.
- Per-extension `AGENTS.md` and `Justfile`.
- Missing GitHub community health files for `embassy`, `ferrox`, and
  `manifold`.
- Richer README coverage for `arbiter`, `embassy`, `manifold`, `mnemos`, and
  `prism`; `ferrox` kept its long-form product README and gained a repository
  guide section.

Verified that each extension has `README.md`, `CHANGELOG.md`,
`CONTRIBUTING.md`, `CODE_OF_CONDUCT.md`, `SECURITY.md`, `LICENSE`, `Justfile`,
and `AGENTS.md`.

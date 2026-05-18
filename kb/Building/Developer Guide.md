---
tags: [building, guide, extensions]
source: mixed
date: 2026-05-06
---
# Developer Guide for Converge Extensions

This guide is the practical entry point for developers working in
`/Users/kpernyer/dev/reflective/stack/mosaic-extensions`, the reusable extension home for
Converge-adjacent capabilities.

It is adapted from the Converge 3.8.1 developer guide at
`/Users/kpernyer/dev/reflective/stack/bedrock-platform/converge/kb/Building/Developer Guide.md`, but this
page is for extension repositories. The parent folder is a multi-repo
container, not a Cargo workspace and not the Converge foundation.

Converge owns the universal contracts, convergence engine, proposal promotion,
semantic model, and run integrity. Extensions own implementation-heavy
suggestors, adapters, solvers, memory, analytics, policies, connector ports,
and showcase examples.

## Version Baseline

| Item | Value |
|---|---|
| Workspace container | `/Users/kpernyer/dev/reflective/stack/mosaic-extensions` |
| Foundation checkout | `/Users/kpernyer/dev/reflective/stack/bedrock-platform/converge` |
| Converge contract baseline | `3.8.1` |
| Rust edition | 2024 |
| MSRV | 1.94.0 |
| License | MIT |
| Dependency rule | `converge contracts <- extensions <- products / deployments` |
| Core rule | extensions propose or implement; Converge promotes and coordinates |

Start with [[Home]], [[Architecture/Extension Topology]], and
[[Architecture/Dependency Rules]] before changing boundaries.

## Start a Session

From the container:

```bash
cd /Users/kpernyer/dev/reflective/stack/mosaic-extensions
just status
```

Then enter the specific extension repository:

```bash
cd /Users/kpernyer/dev/reflective/stack/mosaic-extensions/<extension-dir>
cat AGENTS.md
cat README.md
cat Cargo.toml
just --list
```

Run Cargo commands only from an extension repo, never from the parent folder.
The parent `Justfile` is only a cross-repo command wrapper.

## Choose the Right Home

| If you are working on... | Use... | Main packages or crates |
|---|---|---|
| Cedar policy gates, delegation checks, authorization decisions | `arbiter-policy/` | `converge-arbiter-policy` / `arbiter` |
| trained models, training pipelines, classifier suggestors | `crucible-models/` | `converge-crucible-models` |
| source-specific connector ports where the external system identity matters | `embassy-ports/` | `converge-embassy-pack`, `converge-embassy-linkedin` |
| native optimization solvers, scheduling, routing, MIP, CP-SAT | `ferrox-solvers/` | `converge-ferrox-solver`, `converge-ferrox-server`, `converge-ferrox-*-sys` |
| generic provider, storage, vector, fetch, feed, LLM, embedding, and tool adapters | `manifold-adapters/` | `converge-manifold-adapters` |
| knowledge bases, recall, retrieval, storage, and agentic memory | `mnemos-knowledge/` | `converge-mnemos-knowledge` |
| closed-form analytics, features, inference, fuzzy logic, analytic packs | `prism-analytics/` | `converge-prism-analytics` |
| SMT-backed safety evidence, CVC5 FFI, solver-backed suggestors | `soter-smt/` | `converge-soter-smt`, `converge-soter-cvc5-sys` |

## Extension Versions

Each extension versions independently. The parent folder has no shared release.
All extension release lines target Converge `3.8.1`.

| Repository | Workspace Version | Converge Baseline | Cargo Packages |
|---|---:|---|---|
| `arbiter-policy/` | `2.0.0` | `3.8.1` | `converge-arbiter-policy` |
| `crucible-models/` | `0.2.0` | `3.8.1` | `converge-crucible-models` |
| `embassy-ports/` | `1.1.1` | `3.8.1` | `converge-embassy-pack`, `converge-embassy-linkedin` |
| `ferrox-solvers/` | `0.6.0` | `3.8.1` | `converge-ferrox-solver`, `converge-ferrox-server`, `converge-ferrox-ortools-sys`, `converge-ferrox-highs-sys` |
| `manifold-adapters/` | `1.1.1` | `3.8.1` | `converge-manifold-adapters` |
| `mnemos-knowledge/` | `1.2.0` | `3.8.1` | `converge-mnemos-knowledge` |
| `prism-analytics/` | `2.0.0` | `3.8.1` | `converge-prism-analytics` |
| `soter-smt/` | `0.2.0` | `3.8.1` | `converge-soter-smt`, `converge-soter-cvc5-sys` |

Extension repos target Converge `3.8.1` public contracts from crates.io.
Keep local `[patch.crates-io]` overrides out of extension repos unless the task
explicitly requires testing unpublished foundation changes.

## Use the Smallest Converge Contract

Extensions should depend on the narrowest stable Converge surface that fits the
job.

| Need | Start with |
|---|---|
| reusable suggestors, packs, and proposed facts | `converge-pack` |
| semantic value types shared with products | `converge-model` |
| provider capability routing and backend traits | `converge-provider` |
| storage contracts | `converge-storage` |
| experience-store contracts | `converge-experience` |
| embedding the engine in examples or tests | `converge-kernel` |
| typed remote runtime calls | `converge-client` |
| gRPC/protobuf wire types | `converge-protocol` |

Do not treat `converge-core` as the default downstream dependency. Direct
internal Converge dependencies are transitional extraction debt unless the
boundary docs explicitly document them.

## Add Extensions to a Product

Products, demos, and runtime assemblies wire extension crates into runnable
systems. Extensions should expose reusable libraries or optional service shells;
they should not become product runtimes themselves.

Typical package choices:

```toml
[dependencies]
arbiter = { package = "converge-arbiter-policy", version = "2.0.0" }
crucible = { package = "converge-crucible-models", version = "0.2.0" }
embassy-pack = { package = "converge-embassy-pack", version = "1.1.1" }
embassy-linkedin = { package = "converge-embassy-linkedin", version = "1.1.1" }
ferrox = { package = "converge-ferrox-solver", version = "0.6.0", default-features = false }
manifold = { package = "converge-manifold-adapters", version = "1.1.1" }
mnemos = { package = "converge-mnemos-knowledge", version = "1.2.0" }
prism = { package = "converge-prism-analytics", version = "2.0.0" }
soter = { package = "converge-soter-smt", version = "0.2.0", default-features = false }
```

Enable heavy features deliberately. Native solvers, database drivers, cloud
SDKs, model providers, and file-format stacks should stay behind feature flags
where the crate supports them.

## Cross-Repo Commands

The root `Justfile` can run the same recipe across every extension:

```bash
just repos
just status
just check
just test
just fmt-check
just clippy
just lint
just doc
```

Most recipes also accept one repository name:

```bash
just check ferrox-solvers
just test prism-analytics
just lint manifold-adapters
```

Focused work should normally run inside one extension:

```bash
cd /Users/kpernyer/dev/reflective/stack/mosaic-extensions/prism-analytics
just check
just test
just lint
```

## Feature-Specific Checks

Use the repo-specific `Justfile` when features or native prerequisites matter.

| Repository | Useful checks |
|---|---|
| `arbiter-policy/` | `just check`, `just test`, `just lint`, `just doc` |
| `crucible-models/` | `just check`, `just test`, `just lint`, `just doc` |
| `embassy-ports/` | `just check`, `just test`, `just lint`, `just doc` |
| `ferrox-solvers/` | `just check`, `just test`, `just deps`, `just test-full`, `just doc` |
| `manifold-adapters/` | `just check`, `just check-all`, `just test`, `just lint`, `just doc` |
| `mnemos-knowledge/` | `just check`, `just check-memory`, `just test`, `just lint`, `just doc` |
| `prism-analytics/` | `just check`, `just check-all`, `just test`, `just lint`, `just doc` |
| `soter-smt/` | `just check`, `just test`, `just lint`, `cargo test --features cvc5` |

Native solver work in Ferrox requires native dependency setup through `just deps`
or a narrower `just deps-ortools` / `just deps-highs`.

## Repository Guides

### Arbiter

Use `arbiter-policy/` for reusable policy mechanics: Cedar evaluation,
delegation verification, and authorization gates that participate in the
Converge loop.

Primary surface:

- `PolicyEngine`
- `PolicyGateSuggestor`
- `DelegationVerifySuggestor`
- `FlowGateSuggestor`
- rate, budget, approval, data-classification, and compliance gates

Entry points:

- `arbiter-policy/README.md`
- `arbiter-policy/crates/arbiter/src/lib.rs`
- `arbiter-policy/crates/arbiter/src/engine.rs`
- `arbiter-policy/crates/arbiter/src/suggestor.rs`
- `arbiter-policy/crates/arbiter/policies/*.cedar`

Converge owns the pack and gate contracts. Arbiter owns Cedar wiring, policy
decision behavior, delegation checks, and reusable policy suggestors. Product
repos own production policy bundles, keys, rollout controls, and audit
retention.

### Crucible

Use `crucible-models/` for trained models, training pipelines, classifier
suggestors, and typed prediction payloads.

Primary surface:

- `ClassifierModel`
- `RandomForestModel`
- `DecisionTreeClassifier`
- `ClassifierSuggestor`
- `ClassificationFeaturesPayload`
- `ClassPredictionPayload`

Workspace package names:

- `converge-crucible-models`

Entry points:

- `crucible-models/README.md`
- `crucible-models/crates/crucible/src/lib.rs`
- `crucible-models/crates/crucible/src/training.rs`
- `crucible-models/crates/crucible/src/{ensembles,trees}/`

Crucible owns fitting and trained artifacts. Prism owns closed-form rules and
hand-authored inference.

### Embassy

Use `embassy-ports/` when the external system identity is part of the semantic
contract. LinkedIn-shaped data belongs here because the source name, terms,
identity, and provenance are part of the API.

Current crates:

| Crate | Role |
|---|---|
| `embassy-pack` | shared `CallContext`, `Observation<T>`, and `content_hash` helper |
| `embassy-linkedin` | LinkedIn request/response types, provider trait, and stub provider |

Entry points:

- `embassy-ports/README.md`
- `embassy-ports/crates/pack/src/lib.rs`
- `embassy-ports/crates/linkedin/src/lib.rs`

Use Embassy for source-shaped contracts. Use Manifold when another vendor can
be swapped behind the same capability.

### Ferrox

Use `ferrox-solvers/` for mathematical optimization and native solver-backed
suggestors.

Current crates:

| Crate | Role |
|---|---|
| `ferrox-solver` | solver problem models and Converge suggestors |
| `ferrox-server` | gRPC solver service |
| `ferrox-ortools-sys` | OR-Tools native wrapper |
| `ferrox-highs-sys` | HiGHS native wrapper |

Feature flags:

- default features are empty
- `ortools` enables OR-Tools-backed CP-SAT functionality
- `highs` enables HiGHS-backed MIP functionality
- `full` enables both
- `ferrox-server` defaults to `full`

Entry points:

- `ferrox-solvers/README.md`
- `ferrox-solvers/crates/ferrox/src/lib.rs`
- `ferrox-solvers/crates/ferrox/src/*/problem.rs`
- `ferrox-solvers/crates/ferrox/src/*/suggestor.rs`
- `ferrox-solvers/examples/*/src/main.rs`

Keep unsafe native FFI isolated in the `*-sys` crates. Products decide whether
to embed `ferrox-solver` or run `ferrox-server`.

### Manifold

Use `manifold-adapters/` for generic adapter implementations where the vendor
should be hidden behind an interchangeable capability.

Current adapter families:

- object storage: local, S3, GCS
- experience stores: SurrealDB, LanceDB
- vector recall: LanceDB and vector helper surfaces
- LLM chat backends and model selection metadata
- search, fetch, feed, embedding, reranking, and tool adapters

Feature flags include:

- default: `object-local`
- `object-all`
- `all-storage`
- `llm-all`
- `registry`
- `search-all`
- `all-vector`
- `tools`

Entry points:

- `manifold-adapters/README.md`
- `manifold-adapters/crates/manifold/src/lib.rs`
- `manifold-adapters/crates/manifold/src/object_storage/mod.rs`
- `manifold-adapters/crates/manifold/src/experience/mod.rs`
- `manifold-adapters/crates/manifold/src/vector/mod.rs`

Manifold implements Converge contracts. It should not define new truth
semantics or own product runtime assembly, credentials, tenancy, or provider
selection policy.

### Mnemos

Use `mnemos-knowledge/` for knowledge bases, recall, retrieval, storage, and
agentic memory.

Primary surface:

- `KnowledgeBase`, entries, search options, and search results
- local storage and vector-style retrieval
- embeddings, including OpenAI embedding support
- markdown and rich-media ingestion
- causal, temporal, reflexion, skill, session, online, and meta-learning memory
- CLI and gRPC server surfaces
- `KnowledgeRetrievalSuggestor` and `KnowledgeStoreSuggestor`

Feature flags:

- default: `cli`, `grpc`
- `cli` enables the `mnemos` binary
- `grpc` enables the `mnemos-server` binary
- `memory-only` checks in-memory operation

Entry points:

- `mnemos-knowledge/README.md`
- `mnemos-knowledge/crates/mnemos/src/lib.rs`
- `mnemos-knowledge/crates/mnemos/src/core/knowledge_base.rs`
- `mnemos-knowledge/crates/mnemos/src/suggestor.rs`
- `mnemos-knowledge/crates/mnemos/src/ingest/`
- `mnemos-knowledge/crates/mnemos/src/agentic/`
- `mnemos-knowledge/crates/mnemos/proto/knowledge.proto`

Recall must not bypass Converge promotion. Mnemos retrieves and proposes;
Converge decides what becomes fact.

### Prism

Use `prism-analytics/` for closed-form analytics, feature extraction, fuzzy
logic, and inference packs.

Primary surface:

- `FeatureAgent`
- `InferenceAgent`
- Mamdani, Sugeno, and Tsukamoto fuzzy inference packs
- analytic packs for anomaly detection, classification, descriptive stats,
  forecasting, ranking, regression, segmentation, similarity, and trend
  detection

Feature flags:

- default features are empty
- `excel` enables Excel ingestion through `calamine`

Entry points:

- `prism-analytics/README.md`
- `prism-analytics/crates/prism/src/lib.rs`
- `prism-analytics/crates/prism/src/engine.rs`
- `prism-analytics/crates/prism/src/fuzzy/`
- `prism-analytics/crates/prism/src/packs/`
- `prism-analytics/crates/prism/tests/`

Prism outputs proposals and analytic evidence. Training, model rollout policy,
and fitted artifacts live in Crucible or product repositories.

### Soter

Use `soter-smt/` for SMT-backed safety evidence and CVC5 integration.

Primary surface:

- `SmtQuery`
- `SmtReport`
- `SmtSuggestor`
- fake in-process backend for deterministic CI
- CVC5 FFI backend behind the `cvc5` feature

Entry points:

- `soter-smt/README.md`
- `soter-smt/crates/soter/src/lib.rs`
- `soter-smt/crates/soter/src/cvc5.rs`
- `soter-smt/crates/cvc5-sys/src/lib.rs`

Soter searches or refutes bounded SMT claims. It does not own Arbiter policy
semantics; Arbiter owns Cedar policy models and invariant adequacy fixtures.

## Boundary Rules

These rules are part of the extension contract:

- Dependencies flow one way: `converge contracts <- extensions <- products`.
- Converge must not import extension repositories.
- Extension crates may depend on stable Converge contracts.
- Direct internal Converge dependencies are extraction debt unless explicitly
  documented.
- Generic swappable providers belong in Manifold.
- Source-shaped external contracts belong in Embassy.
- Product-specific runtime wiring, credentials, Docker, databases, and secrets
  belong in products or deployment repos.
- Extension suggestors return proposals or effects; they do not manufacture
  authoritative facts.
- No `unsafe` code unless the crate is a native FFI wrapper whose purpose is
  to isolate that boundary.
- Heavy SDKs, native bindings, database drivers, and provider clients stay out
  of the Converge foundation.

If an extension-local contract becomes universal across extension families,
promote the contract upstream to Converge and keep the implementation
downstream.

## Code Rules

Use the existing repo style first. Across extension repos:

- preserve `unsafe_code = "forbid"` except in Ferrox native `*-sys` crates
- prefer typed enums, newtypes, and validated domain types over stringly typed
  semantics
- use `serde(deny_unknown_fields)` for known config schemas unless openness is
  deliberate
- keep feature flags explicit for heavyweight optional dependencies
- keep tests close to public boundaries, wire formats, feature gates, and
  algorithmic invariants
- do not add compatibility shims for new behavior unless the repo already has a
  documented compatibility policy
- keep dependency versions in the workspace root and use `workspace = true`
  inside member crates where the repo follows that pattern
- update the extension README, module KB page, and release notes when public
  behavior changes

## Git Model

There is no root git repository at `/Users/kpernyer/dev/reflective/stack/mosaic-extensions`. Each
extension directory is independent.

Before editing an extension:

```bash
git -C /Users/kpernyer/dev/reflective/stack/mosaic-extensions/<extension-dir> status --short --branch
```

Keep topic branches scoped to one extension unless the work is a deliberate
coordinated release. Do not mix extraction cleanup, feature work, and version
bumping in one branch.

## Before Opening a PR

1. Confirm the change belongs in the chosen extension, not the Converge
   foundation or a product repo.
2. Check the dependency direction and remove accidental new internal Converge
   dependencies.
3. Update README, changelog, and KB pages when public behavior, boundaries, or
   ownership changes.
4. Run the narrowest useful check first.
5. Run `just lint` from the extension root.
6. Run feature-specific or release-grade gates when the change touches native
   bindings, provider adapters, public contracts, security, storage, wire
   formats, or release packaging.
7. Keep the extension repo's git state independently reviewable.

Further reading:

- [[Building/Getting Started]]
- [[Building/Release and Versioning]]
- [[Architecture/Extension Topology]]
- [[Architecture/Repository Map]]
- [[Architecture/Dependency Rules]]
- [[Architecture/Port Provider Boundary]]
- [[Architecture/Runtime Assembly]]
- [[Workflow/Git Strategy]]

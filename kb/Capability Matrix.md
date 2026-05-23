---
tags: [moc, capabilities, matrix]
source: mixed
date: 2026-05-21
---
# Capability Matrix

What downstream apps can pull from Mosaic, with the algorithm, a tagline, and why it matters.

This page exists because downstream apps consistently underestimate what is already in reach. Each module section lists its callable functions, the algorithm behind each, a one-line tagline, and a tight paragraph on what app-level problem it solves. A short "How to pull" line at the end of each section names the crate(s) and feature flag(s).

Scope note: this is a capability index, not a product story. Apps and atelier showcases are the only places that should speak concretely about domains; extensions stay capability-shaped.

See also: [[Ecosystem]], [[Architecture/Extension Topology]], [[Architecture/Pluralist Reasoning Substrate]], [[Architecture/Expert Portfolio Architecture]].

## Arbiter — policy as code

| Function | Algorithm | Tagline | Why it matters |
|---|---|---|---|
| `PolicyGateSuggestor` | Cedar policy evaluation | Allow/deny with an auditable reason | Every Converge run gets a real authz engine instead of ad-hoc `if`-trees. Cedar policies are reviewable artifacts; the gate emits a structured `PolicyDecision` you can route, log, or escalate. |
| `CedarAnalysisSuggestor` | Cedar SymCC → SMT (CVC5 via Soter) | Prove your policy can't do *that* | Turns a Cedar policy into a symbolic model and asks an SMT solver whether a bad outcome is reachable. Lets a product claim "expense > X without finance approval is unreachable" with searched evidence, not vibes. |
| `LocalCvc5AnalysisBackend` | External `cvc5` process driver | Bring-your-own solver | Apps that can't link CVC5 natively can still run analysis by pointing at a CVC5 binary on `PATH` — same `CedarAnalysisReport` shape, same `ExecutionIdentity`. |
| `DelegationVerifySuggestor` | Ed25519 signature verify + `nbf`/`exp` window | Cryptographic "who said you could?" | Apps can hand off authority across agents/services and prove the chain. Replaces homegrown JWT-ish tokens with a typed `Delegation` and time-bounded check. |
| `CedarHitlGateSuggestor` | Cedar with `human_approval_present` flip | Human-in-the-loop with a hard floor | Strict escalation: only requests Cedar would allow *with* a human approval ever route to HITL — everything else stays denied. No "ask a human and pray" backdoor. |
| `FlowGateSuggestor` | Cedar over flow context | Govern multi-step flows, not just calls | Lets a flow's state (origin, prior decisions, owner) participate in the policy. Apps stop reinventing flow-scoped permissioning. |
| `RateLimitGateSuggestor` | Token / window counters as Cedar inputs | Throttling with the same policy story | Rate is a policy concern, not a side-band middleware setting. Same audit trail, same suggestor shape. |
| `BudgetGateSuggestor` | `CostUsd` accumulator vs. Cedar limits | Don't let an agent expense the company | Hard cap on spend per actor/flow/context, enforced at the suggestor layer where every other gate lives. |
| `ApprovalGateSuggestor` | Cedar over approval-count threshold | "Two managers, or it doesn't ship" | Co-sign / N-of-M approval patterns without writing them into the app. |
| `DataClassificationGateSuggestor` | Cedar over classification tags | Keep PII out of the wrong context | One place to encode "no `secret` data into a public-tier LLM call." Stop relying on hope. |
| `ComplianceGateSuggestor` | Cedar over jurisdiction + obligation tags | One gate, many regimes | Treats GDPR/SOX/HIPAA-style obligations as policy data, not branching code. |

How to pull: `converge-arbiter-policy` (lib name `arbiter`). Cedar Analysis lives behind the `analysis` feature. See [[Modules/Arbiter]].

## Crucible — fit-to-data models

| Function | Algorithm | Tagline | Why it matters |
|---|---|---|---|
| `DatasetAgent` | Polars CSV/TSV/Parquet/Excel + ObjectStore (`gs://`, `s3://`, `file://`, MinIO) | One loader, every backend | Apps stop writing format/transport glue. Datasets read from a local file or a cloud bucket with the same call. |
| `DataValidationAgent` | Schema / null / type checks on Polars frames | Cheap failures, early | Catches dataset rot before it eats a training run. |
| `FeatureEngineeringAgent` | Polars column transforms | Repeatable feature steps | Encodes feature recipes as agent steps so retrains are reproducible. |
| `HyperparameterSearchAgent` | Grid / random search loop | Pick the knobs without writing the loop | Apps say "search these" and get back a chosen config + score. |
| `ModelTrainingAgent` → `ensembles` | Random Forest / gradient-boosted trees | The honest first pack for tabular | Strong baseline on tabular data — the workhorse the loan-application showcase pulls today. |
| `ModelTrainingAgent` → `trees` | CART with Gini / information gain | A model an underwriter can read | When a single decision tree's interpretability is the requirement, not accuracy. |
| `ModelTrainingAgent` → `svm` | Kernel SVM (linear / polynomial / RBF) | Tight margins on small data | Where you don't have enough rows for an ensemble but the boundary is non-linear. |
| `ModelTrainingAgent` → `neuro_fuzzy` | ANFIS via Burn autodiff | Learn the fuzzy rules from data | Pairs with Prism's hand-authored fuzzy engine: same shape, parameters learned instead of authored. |
| `ModelEvaluationAgent` | Holdout metrics + report | Score before you ship | Uniform evaluation surface across pack types. |
| `ModelRegistryAgent` | Versioned artifact registry | The model has a name and a hash | No "which pickle is in prod?" — registry entries are the answer. |
| `MonitoringAgent` | Drift / health signal collection | Notice when reality moves | Substrate for the closed-loop retrain story; emits the signal a Formation will eventually consume. |
| `DeploymentAgent` | Promote registry artifact to runtime | Ship the version you actually picked | Apps don't write the "copy artifact, swap pointer" dance. |
| `SampleInferenceAgent` | Single-row predict | Sanity-check before integration | The "is this thing wired up right?" agent. |

How to pull: `converge-crucible-models` (lib name `crucible`). Features: `storage` for the ObjectStore bridge, `excel` for `calamine` ingestion. Packs land pack-by-pack; `ensembles` is the first one in flight. See [[Modules/Crucible]].

## Embassy — named-source observation

| Function | Algorithm | Tagline | Why it matters |
|---|---|---|---|
| `embassy-linkedin` | LinkedIn profile lookup → `LinkedInProfile` | Treat LinkedIn as evidence, not a string | Source identity is part of the type. Compliance, terms, and provenance ride with the observation. |
| `embassy-sec-edgar` | EDGAR filings fetch + `LiveSecEdgarProvider` | US public-company truth | Apps that touch US public-co context get filings as typed `Observation<Filing>` records, not a scrape. The `live` feature resolves SEC metadata, fetches the primary document, and carries Item 1A for 10-K filings through the provider trait. |
| `embassy-bolagsverket` | Swedish company registry lookup | Authoritative SE entity data | KYC/onboarding in Sweden without reinventing the registry client. |
| `embassy-gleif` | LEI lookup + `LiveGleifProvider` | One global ID for legal entities | Cross-jurisdiction joins on counterparties — the only ID that actually federates. The `live` feature adds `LiveGleifProvider` calling the free public CC0 API at `api.gleif.org/api/v1/lei-records/{lei}`; no auth required. Default-features still ships the stub for tests and harnesses. |
| `embassy-vies` | EU VAT number validation + `LiveViesProvider` (SOAP) | Is this VAT number real, today | Stops invoice fraud at the boundary; carries the EU's own validation response. The `live` feature POSTs the SOAP envelope to `ec.europa.eu/taxation_customs/vies/services/checkVatService` (free, no auth) and returns a typed `VatValidation` with the `requestIdentifier` consultation number for proof. `MS_UNAVAILABLE` is mapped to its own typed status — distinct from "checked and not valid." |
| `embassy-ofac-sls` | OFAC SDN match (`Exact` / `Fuzzy` / `Alias`) + `LiveOfacSlsProvider` | Don't trade with that name | Three-mode sanctions match with confidence scoring; hit type drives the review path. The `live` feature adds `LiveOfacSlsProvider` which downloads the canonical SDN.CSV from US Treasury (no auth required) and screens by name. Verified against real SDN: `GAZPROM` → fuzzy hit on `GAZPROMBANK JOINT STOCK COMPANY`. |
| `embassy-eu-sanctions` | EU Consolidated List match + `LiveEuSanctionsProvider` | Same shape, EU jurisdiction | Reuses the shared `SanctionsSubject` / `SanctionsHit` vocabulary so apps unify hits across lists. The `live` feature defaults to OpenSanctions' mirror of the EU FSF (CC-BY 4.0, no auth); consumers with EU-Login override `LiveFetchOptions::sanctions_url` to point at the canonical endpoint. |
| `embassy-commerce-csl` | US Commerce BIS Denied / Entity List match + `LiveCommerceCslProvider` | Export-control screening | Closes the third leg of the standard US / EU / Commerce screening triad. The `live` feature defaults to the OpenSanctions mirror (CC-BY 4.0, no auth); consumers with a trade.gov api_key override `LiveFetchOptions::sanctions_url`. |
| `embassy-sam-gov` | SAM.gov contractor lookup + `LiveSamGovProvider` | Is this vendor allowed to take federal money | Procurement / vendor-onboarding flows that touch US federal context. The `live` feature calls `https://api.sam.gov/entity-information/v3/entities` and returns a typed `ContractorRegistration` with Active/Expired/Inactive/Submitted status, expiration date, and CAGE code. Requires `SAM_GOV_API_KEY` (free at `api.data.gov/signup/`); `from_env()` fails loud with a diagnostic that names the signup URL if the var is unset. |
| `embassy-usaspending` | Federal spend records + `LiveUsaspendingProvider` | Follow the money the gov already published | Open-data context for opportunity sizing, lobbying analysis, fraud signals. The `live` feature calls `https://api.usaspending.gov/api/v2/awards/{id}/` — no auth required (FFATA / DATA Act open-data mandate). |
| `embassy-ted` | EU TED procurement notices | EU public-tender feed | Live opportunities for B2G use cases. |
| `embassy-skatteverket` | Skatteverket public-tax query | Swedish tax-side counterparty signal | Pairs with Bolagsverket for SE due-diligence. |
| `embassy_pack::simple_id!` | Format-validated identifier newtypes | Parse, don't validate | All P1 skeletons (USPTO, Crunchbase, GitHub, PubMed, arXiv, OpenAlex, Wikidata, Companies House, SCB, EPO) get a typed ID at the boundary — invalid input never reaches the port. |
| `embassy_pack::content_hash` | Stable hash of observation payload | Cache and dedupe by content | Replay, idempotency, and audit equality across runs. |

Boundary reminder: Embassy ports are evidence-only. Anything that signs, moves money, or mutates CRM / HR / accounting state belongs in Reflective Commerce Rails, not here.

How to pull: each port is its own crate `converge-embassy-<source>` (lib name `embassy_<source>`); shared types live in `converge-embassy-pack` (`embassy_pack`). See [[Modules/Embassy]].

## Ferrox — optimization

| Function | Algorithm | Tagline | Why it matters |
|---|---|---|---|
| `CpSatSuggestor` | OR-Tools CP-SAT | Industrial-grade constraint solving | When the problem is "satisfy these and maximize that," CP-SAT outclasses anything you'll hand-roll. |
| `GlopLpSuggestor` | OR-Tools Glop LP | Linear programming on tap | Continuous-variable optimization in the same suggestor shape as everything else. |
| `HighsMipSuggestor` | HiGHS MIP | Mixed-integer when you need it | Strong open-source MIP, native bindings, no SaaS solver subscription. |
| `GreedySchedulerSuggestor` | Greedy heuristic over `(TaskId, AgentId, Minutes)` | Fast first answer | Always returns something quickly; great for warm starts and demos. |
| `CpSatSchedulerSuggestor` | CP-SAT scheduling model | Optimal multi-agent task scheduling | The "assign work across N agents under constraints" problem, solved properly. |
| `GreedyJobShopSuggestor` | Greedy heuristic | Cheap job-shop baseline | Sanity check + speed floor before paying for the CP-SAT run. |
| `CpSatJobShopSuggestor` | CP-SAT job-shop | Classical job-shop scheduling | Multi-machine, multi-operation scheduling — manufacturing / ops backbone. |
| `NearestNeighborSuggestor` | NN heuristic for VRPTW | Routes in milliseconds | Cheap routing answer when you don't have time for the CP-SAT solve. |
| `CpSatVrptwSuggestor` | CP-SAT VRPTW | Vehicle routing with time windows | The real deal for last-mile / field-service planning. |
| `CpSatFormationSuggestor` | CP-SAT over formation selection | Pick the agents themselves | Optimizes which Converge agents to assemble, not just what they decide. |
| Network flow | Min-cost / max-flow primitives over typed `NodeId` | Classical flows when you need them | Supply, transportation, assignment problems — no `i32` confusion. |
| `ferrox-server` | gRPC solver service | Solve from anywhere | Apps that can't link OR-Tools / HiGHS call the gRPC service instead. |

How to pull: `converge-ferrox-solver` (lib name `ferrox`); features `ortools`, `highs`, or `full`. gRPC: `converge-ferrox-server`. Native bindings: `converge-ferrox-ortools-sys`, `converge-ferrox-highs-sys`. See [[Modules/Ferrox]].

## Manifold — vendor-hidden capabilities

| Function | Algorithm | Tagline | Why it matters |
|---|---|---|---|
| `object_storage` | `Arc<dyn ObjectStore>` over local / S3 / GCS | One blob API, three clouds | Apps write once and switch backend with a feature flag. |
| `experience` | SurrealDB / LanceDB experience stores | Persistent agent experience | Stash and recall Converge experiences without picking a database in app code. |
| `vector` | LanceDB-backed vector recall | KNN that doesn't require a SaaS bill | Embedded ANN that ships in your binary. |
| `fetch::HttpFetchProvider` + `WebFetchRequest::with_body` | Generic HTTP fetch + POST body, fallible ctor | Provider-shaped curl (GET and POST) | The "go get a URL" capability behind a contract. Now supports POST with a body via `WebFetchMethod::Post` / `WebFetchRequest::with_body` (used by Embassy's VIES SOAP provider; equally useful for JSON-API POST endpoints). |
| `xml` (`xml` feature) | `extract_first_text` / `extract_all_texts` over `quick-xml` | Tag-shaped XML field grab without a schema | Local-name matching that ignores SOAP namespace prefixes. The right tool for SOAP responses and simple XML feeds; for schema-bound parsing reach for `quick-xml`'s serde integration directly. |
| `feed::HttpFeedProvider` | Streaming feed retrieval | RSS / Atom / anything-streaming | Background ingestion with the same provider shape. |
| `llm` | Chat adapters: openai, gemini, mistral, openrouter, staik, kong, anthropic (plus arcee, minmax, writer) | Swap the model without touching the app | Vendors behind one trait — change provider via config, not refactor. All backends use fallible REAL-by-default constructors (`try_new(key) -> BackendResult<Self>` and `from_env() -> BackendResult<Self>`) that reject empty / whitespace keys at construction. No silent fallback to a useless backend on missing credentials. |
| `llm::retry::retry_with_backoff` | 100 ms × 2^attempt backoff | One backoff, not seven | Single shared retry loop across every LLM backend; apps never re-implement exponential backoff. |
| `reranker::QwenVLReranker` | Qwen-VL reranking, fallible ctor | Better top-K with a real reranker | Pairs naturally with a vector recall step — rerank what matters before paying an LLM. |
| `contract::canonical_hash` | SHA-256, first 8 bytes → 16-hex | Stable content fingerprint | Toolchain-independent dedup / cache key — same input, same hash, forever. |

How to pull: `converge-manifold-adapters` (lib name `manifold`). Features: `object-local` (default), `object-s3`, `object-gcs`, `object-all`, `experience-surrealdb`, `experience-lancedb`, `vector-lancedb`, `all-storage`. See [[Modules/Manifold]].

## Mnemos — memory and recall

| Function | Algorithm | Tagline | Why it matters |
|---|---|---|---|
| `KnowledgeBase` + `KnowledgeRetrievalSuggestor` | Vector recall + scoring | RAG that's a Converge citizen | Recall participates in convergence — every hit is a fact with provenance, not opaque context-string. |
| `KnowledgeStoreSuggestor` | Ingest + embed + persist | Write side of the same contract | Apps round-trip insight back into the same store the rest of the system reads. |
| Markdown / rich-media ingestion | Multi-format chunkers | Stop writing PDF loaders | Drop docs in, get queryable knowledge out. |
| Embedding (incl. OpenAI) | Provider-shaped embedding | Pluggable vectorization | Switch embedder without touching the recall path. |
| `cosine_similarity` (`math.rs`) | Cosine over `&[f32]` | The single cosine | One canonical implementation — all of knowledge_base, batch, embedding, meta delegate here. No drift. |
| `agentic::causal` | Causal memory | Remember *because* | Stores why an event followed another — the substrate for "we did X because Y." |
| `agentic::temporal` | Time-indexed memory | Yesterday matters | Time-bucketed recall that lets policies and agents reason about ordering and recency. |
| `agentic::reflexion` | Reflexion-style self-critique memory | Learn from your own mistakes | Persists the lesson, not just the event — closes the experience loop. |
| `agentic::skills` | Skill memory | Reusable how-tos | Treats learned procedures as recallable artifacts. |
| `agentic::online` | Online learning hooks | Adapt without a retrain | Lightweight in-stream updates between full Crucible retrains. |
| `agentic::sessions` | Session memory | Conversation continuity, properly stored | Session state behind a contract — not stuffed into a vendor's session ID. |
| `agentic::meta` | Meta-memory over the other layers | Memory about memory | Routes recall across the causal / temporal / reflexion / skills layers. |
| Feedback / replay / batch / insight jobs | Background reprocessing | Mine your own history | Turn the experience log into structured insight on a schedule. |
| `mnemos-server` | gRPC service | Share memory across processes | Multi-service deployments share one knowledge store via gRPC. |

How to pull: `converge-mnemos-knowledge` (lib name `mnemos`). Features: `cli`, `grpc` (defaults), `memory-only`. See [[Modules/Mnemos]].

## Prism — closed-form analytics

| Function | Algorithm | Tagline | Why it matters |
|---|---|---|---|
| `FeatureAgent` | Polars-based feature extraction | Features without pandas envy | First-class feature pipelines in Rust, sharing the Crucible / Prism data substrate. |
| `InferenceAgent` | Burn-based inference over feature vectors | Run the model, no Python | Closed-form inference path for pre-fit vectors — millisecond latency, no Python in the deploy. |
| `AnomalyDetectionPack` | Z-score with typed `ZScoreThreshold` | Outliers, with a knob you can trust | Threshold is a typed primitive — no "did 2.0 mean stddev or percentile?" |
| `ClassificationPack` | Linear scoring + sigmoid | Probabilities you can route on | Outputs `UnitFraction` confidence — pair directly with Arbiter gates. |
| `DescriptiveStatsPack` | Mean / median / percentiles | The unglamorous one you actually need | Repeatable shape for the "tell me about this column" step every flow eventually wants. |
| `ForecastingPack` | Exponential smoothing (`alpha`, residual std → upper/lower band) | Forecast with honest confidence bands | Built-in CI bands — downstream policies can branch on the lower bound, not the point estimate. |
| `RankingPack` | Multi-criterion weighted score, per-signal `higher_is_better` | Top-K with reasons | Mixed-direction signals (higher revenue, lower risk) handled in one pass; optional top-K cutoff. |
| `RegressionPack` | Linear `wᵀx + b` over `Vec<Vec<f64>>` | Predict a number, not a story | Closed-form scoring for pre-fit weights; pair with Crucible to learn the weights. |
| `SegmentationPack` | k-means (max-iter, optional seed) | Clusters you can rerun | Deterministic with a seed — important for reproducibility and audit. |
| `SimilarityPack` | Cosine / Euclidean / Manhattan / Jaccard pairs | All four metrics, one contract | One pack covers the metric choice rather than four bespoke implementations. |
| `TrendDetectionPack` | Linear slope per window + sensitivity + changepoints | "Going up", with changepoints | Returns slope, direction, *and* changepoints — usable for alerting, not just dashboards. |
| `fuzzy::Mamdani` | Mamdani inference + centroid defuzz | Expert rules without a neural net | "If demand is high and stock is low then reorder is urgent" — author rules, get a number. |
| `fuzzy::Sugeno` | Sugeno (TSK) inference | Functional consequents | Linear-in-input consequents — bridge to control / pricing logic. |
| `fuzzy::Tsukamoto` | Tsukamoto inference | Monotonic consequents | When you need monotone outputs (risk, escalation level) — Tsukamoto gives them by construction. |
| `MembershipDegree` | `f64` in [0, 1], `serde(transparent)` | The single typed float | Stops bare `f64` from creeping into membership / confidence paths anywhere in the engine. |

Build state: `prism::fuzzy` is fully built infra. Do not propose rebuilding `FuzzyInferencePack`.

How to pull: `converge-prism-analytics` (lib name `prism`). Features: `excel`. See [[Modules/Prism]].

## Soter — searched evidence via SMT

| Function | Algorithm | Tagline | Why it matters |
|---|---|---|---|
| `SmtSuggestor` + `SmtQuery` / `SmtReport` | SMT query / response contract | SMT as a Converge fact | A Converge run can consume "no counterexample found" as evidence, side-by-side with policy, model, and analytics suggestors. |
| `Cvc5FfiBackend` | Native CVC5 FFI in `crates/cvc5-sys` | First-class SMT in-process | No subprocess, no `PATH` dependency — apps that need real CVC5 link it. |
| `ScriptedSmtBackend` (`fake-backend` feat.) | Scripted-response solver | Tests that don't need CVC5 | CI runs the same suggestor shape without the native dep. Gated behind a non-default `fake-backend` feature so a default-features build cannot fall through to a scripted answer in place of real evidence. |
| Stable query hashes | Content-addressed query identity | Replay any solver call | Audit and replay across solver upgrades — the query you ran is the query you ran. |
| Structured solver identity | Version + commit + source mode + configure flags + runtime opts | Reproduce yesterday's "unsat" | Records *exactly* which solver said what — survives CVC5 upgrades and rebuilds. |
| `ArbiterExpenseCommitInvariant` + `ArbiterExpensePolicyModel` | Abstract policy model + invariant query | Working cross-extension showcase | The wired example: Arbiter owns Cedar modeling, Soter owns SMT execution — exactly the cross-module shape apps should copy. |

Evidence boundary reminder: Soter results are `Searched` evidence. `unsat` means no counterexample for the *encoded* query, not a formal proof.

How to pull: `converge-soter-smt` (lib name `soter`); FFI behind the `cvc5` feature backed by `converge-soter-cvc5-sys`. See [[Modules/Soter]].

## How to use this page

- **Picking a capability:** scan the taglines first, then the "why it matters" column to confirm fit, then jump to the linked module page for surface and entry points.
- **When two modules look close:** the boundary rule is in [[Architecture/Port Provider Boundary]] (Embassy vs Manifold) and [[Modules/Prism]] (Prism vs Crucible: closed-form vs fit-to-data).
- **Reasoning portfolio view:** [[Architecture/Pluralist Reasoning Substrate]] and [[Architecture/Expert Portfolio Architecture]] explain how these capabilities combine inside a Formation.
- **App-pull discipline:** every lower-layer build needs a real app pull. If a capability above is missing for your app, that is a signal worth surfacing — not a license to grow the extension preemptively.

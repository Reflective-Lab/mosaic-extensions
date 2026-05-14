---
tags: [architecture, essay]
source: mixed
date: 2026-05-14
---
# The Mosaic Extensions: A Pluralist Reasoning Substrate for Formations

The dominant agent architecture of 2026 reduces a remarkable diversity of cognitive work to a single ingredient. Prompt in, token stream out. The substrate is one large language model; the differences between agents are differences of prompt and tool wiring. This works astonishingly well for tasks where the answer is well-typed by surrounding text. It fails — sometimes loudly, sometimes silently — at tasks where the answer is well-typed by a search space, a constraint set, a probability distribution, an authorisation lattice, or a body of formal mathematics.

A Formation is a different architectural commitment. A Formation is an assembly of specialised reasoning agents, called Suggestors, that operate around a shared Context. Each Suggestor produces typed `ProposedFact`s and writes them under a particular `ContextKey`. A Convergence Kernel arbitrates promotion. The agents do not negotiate in prose; they exchange structured proposals with provenance. The kernel does not assume the agents are interchangeable; it assumes they are specialised.

The Mosaic Extensions are eight Rust crates that supply Suggestor implementations grounded in eight different formal traditions. Each crate answers a specific epistemic question; together they cover the surface area of decisions a Formation must actually make. Nothing in the design forbids a large language model — adapters for those are explicit — but the LLM is one Suggestor among many, not the substrate.

This article walks each crate, names its mathematical grounding, gives a business scenario in which it earns its keep, and explains what kind of Formation question it answers that a foundation model cannot honestly answer alone.

## Ferrox-solvers — search under hard constraints

The first epistemic mode is search over a combinatorial space under hard constraints. The mathematics is linear programming (Dantzig's simplex, interior-point methods), mixed-integer programming (branch-and-bound, Gomory cuts), and constraint programming (arc consistency, conflict-driven clause learning). The backing libraries are OR-Tools and HiGHS — decades of refinement at Google Research and the University of Edinburgh respectively.

Ferrox exposes these as Suggestors over Formation-level problem shapes: vehicle routing with time windows, job-shop scheduling, allocation under budget caps, formation selection. The mathematics is not approximate. When HiGHS proves a problem infeasible, that is a theorem. When it returns an optimum, the duality certificate witnesses that no better solution exists.

**Business scenario.** A regional logistics operator must dispatch 800 packages across 60 vans before the 18:00 service-level deadline, respecting driver hours-of-service rules and cold-chain time windows for refrigerated goods. A planner's intuition produces a workable schedule. Ferrox produces the route plan that provably minimises total drive time subject to all constraints, with a duality certificate that no schedule with fewer driver-hours exists. The difference between "workable" and "optimal" at this scale is several percent of weekly fuel — a number the firm can put in a board report.

A foundation model can describe a routing problem and sketch a heuristic. It cannot return a certificate of optimality.

## Prism-analytics and crucible-models — statistical and fuzzy inference

The second epistemic mode is empirical generalisation from data — the territory of statistical learning theory and its descendants. Prism wraps Polars for tabular data manipulation and Burn for differentiable computation. It ships Suggestors for classification, regression, anomaly detection, segmentation, similarity, ranking, forecasting, and trend detection — each a wrapper around a learning algorithm with a published guarantee.

Prism also implements fuzzy inference. The foundations were laid by Zadeh (1965): a fuzzy set assigns each element a membership grade in [0, 1] rather than the crisp {0, 1} of classical set theory. Membership functions — triangular, trapezoidal, Gaussian — compose via t-norms (typically `min` or product) and t-conorms (`max` or probabilistic sum). Prism implements three inference families: Mamdani (1975) for full fuzzy-set output, Takagi–Sugeno (1985) for linear consequents, and Tsukamoto for monotonic consequents. Defuzzification by centroid, weighted average, or Tsukamoto projection returns the system to crisp numbers for downstream consumers.

Crucible-models complements Prism with trained-artifact packs: support vector machines (Cortes and Vapnik 1995), ensembles via bagging (Breiman 1996) and boosting (Freund and Schapire 1997), and ANFIS — Jang's 1993 Adaptive Neuro-Fuzzy Inference System, which tunes fuzzy rule parameters by gradient descent. ANFIS is the bridge between Prism's symbolic fuzzy rules and a learned model that respects them.

**Business scenario.** A retail bank's transaction-monitoring team must triage thousands of card events per hour. A brittle rule — "flag transactions over €5,000 abroad" — produces both false positives (a customer's holiday) and false negatives (a 49-transaction structuring pattern). Prism's anomaly detector quantifies how far this transaction sits from the customer's 18-month baseline. A fuzzy inference pack combines `amount_elevated`, `time_of_day_unusual`, and `merchant_risk_high` into a graded `review_urgency` score — `very_low`, `low`, `medium`, `high`, `critical` — that survives the next regulatory audit because every rule is named and every membership function is inspectable.

A foundation model can opine on whether a number looks anomalous. A trained anomaly detector, calibrated against a held-out distribution, can quantify the probability under the null and reject at a chosen false-positive rate.

## Arbiter-policy — authorisation as a decidable fragment

The third epistemic mode is policy: what is permitted, what is forbidden, what must be escalated. Arbiter implements a Policy Decision Point on AWS's Cedar — a domain-specific logic with a deliberately small evaluation model, a validator, and (Cedar 4.10) a symbolic compiler backed by `cedar-policy-symcc`. The compiler reduces Cedar policies to SMT and dispatches them to CVC5; the result is symbolic analysis over the full state space of a policy, not just runtime spot-checks.

The mathematics is first-order logic with finite quantification over typed entities. Cedar's evaluation is decidable; its symbolic compilation reduces meta-questions — *can these two policies ever simultaneously permit X and Y?* — to satisfiability over a tractable fragment. Arbiter exposes runtime policy gates as Suggestors and ships invariant fixtures — Gherkin scenarios paired with runtime regression tests and optional SymCC analysis queries — to assert that a policy class never reaches a forbidden state.

**Business scenario.** A pharmaceutical company encodes the access policy for its clinical-trial data room in Cedar. The runtime PDP denies a 02:00 access request from a contract data scientist with read-only permissions. That is the small win. The large win is that the symbolic compiler proves at deploy time — for every new policy bundle — that no combination of roles, delegations, and temporary grants permits export of patient-level data to a non-EU principal under any sequence of events. The runtime answer is "deny now." The symbolic answer is "this class of breach is impossible."

A foundation model can produce a plausible policy decision and articulate the principle behind it. It cannot prove that no combination of policies in the deployed bundle admits the forbidden state.

## Soter-SMT — bounded symbolic search

The fourth epistemic mode is bounded symbolic search — asking whether *any* model can be constructed that violates a stated invariant. The mathematics is Satisfiability Modulo Theories: classical SAT extended with decidable first-order theories of linear arithmetic, bit-vectors, strings, arrays, algebraic datatypes, and quantifiers. SMT is the formal calculus for *does there exist a counterexample to this property* and its negation, *is this property preserved across all reachable states*.

Soter ships native bindings to CVC5 — the fifth-generation Cooperating Validity Checker, maintained jointly by Clark Barrett's group at Stanford University and Cesare Tinelli's group at the University of Iowa, with a research lineage reaching back through CVC4, CVC3, CVC Lite, and the original CVC of the early 2000s. CVC5 consistently places among the strongest open-source SMT solvers in the international SMT-COMP competition across the linear arithmetic, bit-vector, strings, datatypes, and finite-fields divisions. It is built for industrial-scale problems: it accepts SMT-LIB 2 input, emits proof certificates in LFSC and Alethe formats, and supports incremental solving for interactive use.

Soter exposes a `SmtSuggestor` that reads an `SmtQuery` JSON object from the Context, runs CVC5 against the encoded problem, and writes back an `SmtReport` with a stable content hash of the SMT-LIB payload plus a solver status from `{sat, unsat, unknown, timeout, error}`. The unsafe native FFI is isolated in a small `cvc5-sys` crate so that the rest of Soter — query types, validation, suggestor wiring, Formation discovery — stays pure, safe Rust.

A crucial discipline runs through the design: SMT results are `Searched` evidence, not `Verified`. CVC5 is a fast and expressive search engine, but its proof terms are not independently checked by the kernel running the search. An `unsat` from CVC5 is strong evidence that no counterexample exists; it becomes formal proof only when an independent proof checker certifies the emitted artifact. That verifier tier is the natural home for Lean 4 (with its small, auditable kernel), Coq, or Agda. The Mosaic Extensions hold that tier in reserve as an explicitly deferred re-open: it will be added when a regulator, an auditor, or a safety case demands an independently-checked artifact rather than a high-quality solver result.

**Business scenario.** A neobank's compliance team has accumulated, over four years, a KYC and sanctions policy stack of 140 Cedar rules, 22 manual-override classes, and 6 delegation patterns. The new general counsel asks one question: *can any sequence of permitted overrides clear a sanctioned counterparty?* Manual review of the rule combinatorics is hopeless. Soter encodes the stack as an SMT problem with the negated invariant — *there exists a sequence of overrides that clears a sanctioned party* — and submits it to CVC5. The solver returns `unsat`: no such sequence exists, with a hashed certificate that pins the policy bundle, the schema, and the query. The same query is re-run on every policy change; any modification that admits a counterexample fails the build.

A foundation model can guess at counterexamples. CVC5 either produces one or proves none exists in the decidable fragment.

## Mnemos-knowledge — memory with structure

The fifth epistemic mode is memory — not storage, but typed recall under uncertainty. Mnemos ships a vector knowledge base (cosine similarity, HNSW indexing), but its more interesting surface is agentic memory primitives. The Reflexion loop (Shinn et al. 2023) is a self-critique pattern that updates an episodic store after each outcome. The skill library is a content-addressed cache of validated procedures. Causal memory tracks hyperedges between facts — a hypergraph rather than a simple DAG, because real causal claims are rarely binary.

Mnemos also implements meta-learning primitives: MAML (Finn et al. 2017) and Reptile (Nichol et al. 2018) for few-shot adaptation, and Elastic Weight Consolidation (Kirkpatrick et al. 2017) for online drift detection without catastrophic forgetting. The mathematics is a mix of gradient methods in learner parameter space and graph algorithms over the causal hypergraph.

**Business scenario.** A claims-handling team at a property insurer has fifteen years of resolved cases, adjuster notes, regulator letters, and reinsurer correspondence — a corpus that does not fit in any model's context window and whose value is precisely in its specificity. When a new claim arrives, Mnemos retrieves the three closest historical analogues with citation, surfaces the causal hyperedges that link similar fact patterns to past outcomes, and tracks (via the reflexion loop) which retrieved analogues the adjuster actually relied on. Retrieval quality compounds. After eighteen months the institutional memory of three retired senior adjusters is still in the system, not because the cases were summarised but because the right cases surface at the right time.

A foundation model has a context window. Mnemos has a memory architecture.

## Manifold-adapters and embassy-ports — typed boundaries

The sixth epistemic mode is integration — the place where the Formation meets external systems. Manifold-adapters supplies a capability registry: providers register the capabilities they implement (LLM completion, embedding, reranking, vector search, web search, graph search) with metadata on modality, data sovereignty, and latency SLA. The mathematics is straightforward but consequential: contract matching is subtyping over capability requirements, and the registry refuses to route to a provider that fails the contract.

Embassy-ports handles the symmetric problem at the source side. When an external system's identity is part of the contract — LinkedIn versus a private CRM, where calling them interchangeable would silently corrupt downstream reasoning — Embassy ships typed `Observation<T>` schemas with content-addressable hashing. Two requests to the same source with the same content produce the same hash; this is the lever that makes deduplication, caching, and replay tractable.

**Business scenario.** An EU-headquartered bank deploys an internal advisory agent. The regulator requires that any prompt containing customer-identifiable data is routed only to providers whose `data_sovereignty` includes the EU. A junior developer adds a US-only provider for cost reasons. The Manifold registry refuses the route at compile-time configuration check. No production incident, no breach notification, no fine. Separately, the marketing intelligence pipeline must never confuse a LinkedIn profile with a Twitter profile, because Embassy's typed `Observation<LinkedInProfile>` and `Observation<TwitterProfile>` are distinct types — the compiler enforces what a Slack message could not.

These crates do not reason. They make reasoning safe by typing the boundary.

## The Convergence Kernel above the eight

Above the eight extensions sits the Convergence Kernel supplied by the Converge platform. The Kernel owns three things the extensions do not: the run loop, the promotion authority, and the typed provenance vocabulary that lets agents disagree productively rather than overwriting each other. A `ProposedFact` carries its source as `ProvenanceSource::Arbiter`, `ProvenanceSource::Prism`, `ProvenanceSource::Soter`, and so on. The kernel can therefore distinguish *the LLM thinks*, *the policy engine has decided*, *the optimiser has proved*, and *the SMT solver found no counterexample*.

The kernel does not adjudicate truth. It adjudicates promotion: which proposals survive into the next context state, with what confidence, under what quorum rule. The mathematics here is small but load-bearing — a partial order over evidence tiers (`Observed`, `Decided`, `Searched`, `Argued`, `Verified`) and a configurable quorum per `ContextKey`.

## A Formation in motion

The diagram below shows a Formation that evaluates one decision — *should this €25,000 invoice be paid?* — with Suggestors drawn from each crate around a single shared Context.

```
                       +-----------------------------------+
                       |        Convergence Kernel         |
                       |  promotion | typed provenance DAG |
                       +-----------------+-----------------+
                                         ^
                                         | promote / reject
                                         v
                             +-----------+-----------+
                             |    Shared Context     |
                             |  Seeds / Strategies / |
                             |  Evaluations / etc.   |
                             +-----------+-----------+
                                         ^
                                         | typed ProposedFact
       +--------+--------+--------+------+------+--------+--------+
       |        |        |        |             |        |        |
       v        v        v        v             v        v        v
   +-------+ +------+ +-------+ +------+    +-------+ +------+ +------+
   | Prism | |Cruc. | |Arbiter| |Soter |    |Ferrox | |Mnemos| | LLM  |
   | fuzzy | | SVM /| | Cedar | | CVC5 |    | LP /  | |vector| | (via |
   | + ML  | | ANFIS| | + sym | | SMT  |    | MIP / | | + RL | |Manif)|
   +---+---+ +--+---+ +---+---+ +--+---+    +---+---+ +--+---+ +--+---+
       |        |        |        |             |        |        |
   Observed Observed  Decided  Searched     Searched  Argued   Argued
                                  |
                              (Verified tier: Lean / Coq / Agda
                               held in reserve, deferred)

           +--------------------+          +--------------------+
           | Manifold-adapters  |          |   Embassy-ports    |
           | capability routing |          | typed observations |
           +----------+---------+          +----------+---------+
                      |                               |
                      v                               v
                external providers              external sources
```

A concrete trace through this Formation: the invoice arrives via Embassy as a typed `Observation<InvoicePayload>` with content hash. Prism's fuzzy risk pack scores the vendor (new, no prior history → elevated risk band). Crucible's anomaly model checks the amount against the vendor-category baseline (within range, no flag). Arbiter's Cedar engine evaluates the runtime authorisation (supervisory authority + receipt gate passed → would allow if not for the risk score). Soter, on this deploy, has already proved that the policy bundle cannot admit a non-finance commit above €10,000 without explicit human approval. Ferrox is not involved here — no optimisation problem in this flow. Mnemos retrieves the three closest historical analogues for vendor-onboarding decisions in this category. The LLM, called via Manifold under the EU data-sovereignty contract, drafts the rationale prose for the human approver. The kernel promotes the consensus: *escalate for human approval, with the analogues and the proven invariant attached as evidence.*

## The Formation argument

Calling all of this "agentic AI" obscures the architecture. An agent is not a language model with tools bolted on. An agent is a Suggestor with a formal grounding and a typed proposal protocol. A Formation is a particular composition of Suggestors that converges on a decision in a particular domain.

The Mosaic Extensions exist so that the Formation builder has, at hand, the full reasoning surface: optimisation when the answer is constrained, statistics when it is uncertain, fuzzy logic when it is graded, policy when it is governed, SMT when it must be exhaustively searched, memory when it must persist, and integration when it must cross a boundary. Beyond them, the Verified tier — Lean, Coq, Agda — is held in reserve for the day a compliance claim demands an independently-checked proof artifact rather than a high-quality solver result.

Foundation models are an excellent ingredient. They are not, by themselves, the substrate. The Mosaic Extensions are the substrate.

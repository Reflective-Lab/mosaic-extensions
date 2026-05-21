---
tags: [ecosystem]
source: mixed
date: 2026-05-05
---
# Ecosystem

Extensions sit beside Converge, not inside it.

```text
products / deployments
        |
        v
extensions:
arbiter-policy, crucible-models, embassy-ports, ferrox-solvers,
manifold-adapters, mnemos-knowledge, prism-analytics, soter-smt
        |
        v
converge contracts, kernel, protocol, model, promotion authority
```

Converge is the foundation layer. It owns the engine, shared context,
proposal-to-fact promotion, invariants, canonical public crates, and transport
contracts.

Extensions are reusable capability families. They depend on Converge contracts
and external SDKs. They do not make Converge depend on them.

Products and deployment repositories assemble extensions into runnable systems.
They own secrets, processes, Docker, cloud topology, and environment selection.

## Extension Families

| Family | Purpose |
|---|---|
| Policy | [[Modules/Arbiter]] turns Cedar decisions into Converge-readable gates. |
| Models | [[Modules/Crucible]] owns trained-artifact packs and training pipelines. |
| Connectors | [[Modules/Embassy]] owns source-shaped ports such as LinkedIn. |
| Optimization | [[Modules/Ferrox]] owns solver-backed suggestors. |
| Generic adapters | [[Modules/Manifold]] owns swappable storage, vector, provider, and tool adapters. |
| Knowledge | [[Modules/Mnemos]] owns recall, memory, retrieval, and knowledge storage. |
| Analytics | [[Modules/Prism]] owns feature extraction, closed-form inference, fuzzy logic, and analytics packs. |
| SMT evidence | [[Modules/Soter]] owns CVC5-backed safety evidence and SMT suggestors. |

## What's Already in Reach

Downstream apps consistently underestimate what is already callable. The [[Capability Matrix]] enumerates every function across the eight modules with its algorithm, tagline, and why-it-matters paragraph — read it before reaching for a new build.

## Why This Shape

The extracted shape keeps Converge small enough to be a stable foundation while
still letting formations use strong domain capabilities. Extensions can move at
their own pace, carry heavy native dependencies, and expose clear optional
feature flags without forcing every Converge consumer to compile every adapter.

See also: [[Architecture/Converge Boundary]], [[Architecture/Extension Topology]]

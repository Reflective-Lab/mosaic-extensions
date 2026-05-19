---
tags: [architecture, smt, soter, verification, tradeoffs]
source: mixed
date: 2026-05-19
---
# SMT vs Interactive Theorem Proving

Why the Mosaic Extensions stop at SMT-backed symbolic analysis and hold the
interactive-theorem-proving tier (Lean / Coq / Agda) in reserve.

For the class of problems Soter and Arbiter actually serve — authorization
reasoning, arbiter validation, policy consistency, invariant checking,
constraint analysis, state-space exploration — an SMT-based approach is the
engineering sweet spot.

## What the current stack already gives us

- a formal policy language via Cedar
- symbolic compilation via `cedar-policy-symcc`
- SMT-LIB as an interchange format
- a strong solver via CVC5

That stack delivers:

- satisfiability checking
- counterexample generation
- reachability analysis
- policy equivalence
- invariant enforcement
- privilege escalation detection
- bounded formal verification

without the cost of full interactive theorem proving.

## Where Lean would actually matter

Interactive theorem proving becomes valuable when the requirement is:

- machine-checked mathematical proofs
- proving properties for all executions, unbounded
- verified compiler correctness
- verified kernels or cryptography
- higher-order logic
- dependent-type reasoning
- proof-carrying code
- human-guided theorem construction

That rigor is paid for in:

- dramatically higher development cost
- ongoing proof-maintenance burden
- specialist expertise
- slower iteration

The Mosaic Extensions defer this tier until a regulator, auditor, or safety
case demands an independently-checked artifact rather than a high-quality
solver result.

## SMT as the engineering optimum

A lot of industrial verification systems land exactly where Soter sits:

```
DSL / policy language
        ↓
symbolic encoding
        ↓
SMT solver
```

The same pattern appears across cloud authorization, static analyzers,
hardware verification, smart-contract analyzers, symbolic execution engines,
and compiler optimization validation. SMT delivers automation, speed, usable
counterexamples, and relatively low friction.

## Why SMT fits the Formation domain

SMT solvers are strongest when the domain is:

- finite-ish
- constraint-oriented
- rule-based
- decidable or semi-decidable
- expressible in first-order logic fragments

Authorization and policy systems fit this extremely well. Policies map
naturally to predicates, sets, relations, graph constraints, and quantifiers
over bounded domains. Cedar + CVC5 is therefore a natural architecture, not a
compromise.

## What we gain by not adopting Lean (yet)

We avoid:

- proof-engineering overhead
- interactive proof scripts
- theorem maintenance
- dependent-type complexity
- difficult onboarding
- long verification cycles

We keep:

- automation
- fast iteration
- debuggability
- counterexample-driven development

For a production system that ships, that is usually the better trade.

## Mental model — the rigor spectrum

```
Testing
   ↓
Property testing
   ↓
SMT-based verification          ← Mosaic Extensions live here
   ↓
Interactive theorem proving     ← held in reserve (Lean / Coq / Agda)
   ↓
Fully verified systems
```

SMT-backed symbolic analysis is real formal methods — just not the strongest
possible form. The tradeoff it wins on:

- scales better operationally
- developers can actually use it
- integrates cleanly into CI/CD
- produces actionable failures

That is why many successful systems stop there, and why the Mosaic Extensions
do too — until an app pulls the Verified tier into existence.

See also: [[Modules/Soter]], [[Modules/Arbiter]],
[[Architecture/Pluralist Reasoning Substrate]]

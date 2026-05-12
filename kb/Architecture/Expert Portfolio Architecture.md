---
tags: [architecture, orchestration, expert-systems, fuzzy]
source: codex
date: 2026-05-08
---
# Expert Portfolio Architecture

Advanced Converge systems should be assembled as a portfolio of specialized
reasoning systems, not as one oversized model that tries to own every kind of
judgment.

This is especially important for expectation-aware products, where the system
must reason about language, memory, emotional nuance, uncertainty, hard policy,
and operational feasibility at the same time. Those are different kinds of
truth. They deserve different experts.

## Decision

Model expectation cognition as an orchestrated portfolio of extension-backed
experts:

```text
signals
  -> semantic interpretation
  -> graded expectation state
  -> predictive scoring
  -> hard policy checks
  -> hard constraint solving
  -> decision policy / orchestration
  -> action, explanation, and feedback
```

The orchestrator should not ask every expert for a final answer. Each expert
should contribute a structured claim with evidence, confidence, and provenance.
The product or formation layer then decides which claims are advisory, which
are predictive, which are hard blockers, and which are optimization inputs.

## Why A Portfolio Is Right

Customer expectations are probabilistic, contextual, emotional, dynamic, and
socially constructed. No single reasoning technique handles all of that well.

LLMs are useful for semantic and social interpretation: tone, intent, implicit
expectations, culture, and prior experience. They can read a phrase such as
"genuinely Italian" as a signal about authenticity, simplicity, emotional fit,
and tolerance for novelty.

Fuzzy logic is useful for graded experience states: satisfied, delighted,
uncertain, disappointed, too slow, almost right, or high value but not premium.
It lets a formation represent partial membership instead of forcing early
binary decisions.

ML and analytics are useful for statistical patterns: which combinations
predict repeat purchase, churn, tolerance for delay, or segment-specific
preference.

Policy engines are useful for hard governance: authorization, compliance,
approval, budget, data classification, and flow gates.

Optimization solvers are useful for hard feasibility and tradeoffs: capacity,
schedule, routing, assignment, price, SLA, and resource constraints.

Memory and retrieval are useful for grounding: previous customer interactions,
feedback, learned preferences, institutional knowledge, and traceable context.

Together, these systems create artificial situation awareness: not one model
claiming authority, but multiple experts making bounded claims that can be
compared, constrained, promoted, or rejected.

## Extension Boundaries

`prism-analytics` should own fuzzy inference first as a reusable capability,
not only as one pack. The core surface should be `prism::fuzzy`, with
membership functions, linguistic variables, fuzzy rules, activated-rule traces,
and membership outputs. `prism::packs::fuzzy::FuzzyInferencePack` should wrap
that capability for Converge formations.

`arbiter-policy` remains the home for hard policy decisions: Cedar, gates,
delegation, authorization, compliance, approvals, budgets, and other decision
points where the answer must be allow, reject, review, or otherwise governed.
Fuzzy confidence should not become policy authority.

`ferrox-solvers` remains the home for hard constraints and optimization:
CP-SAT, LP, MIP, routing, scheduling, assignment, capacity, and proof-oriented
feasibility. Fuzzy scores may become objective weights or advisory inputs, but
the solver owns the feasible plan.

`mnemos-knowledge` remains the home for memory, recall, retrieval, and
historical grounding. It can supply prior customer expectations and feedback,
but it should not own the fuzzy inference engine.

A standalone fuzzy extension is premature. It becomes justified only if fuzzy
logic grows into a substantial runtime of its own: a versioned fuzzy rule DSL,
rule registry, editor, compiled/runtime rule engine, domain-independent
explainability traces, and multiple extension families depending on that
contract directly.

## First Practical Shape

Start with a Prism capability plus pack wrapper:

```text
prism::fuzzy
  FuzzyInferenceEngine
  MembershipFunction
  LinguisticVariable
  FuzzyRule
  FuzzyInferenceInput
  FuzzyInferenceOutput

prism::packs::fuzzy
  FuzzyInferencePack
```

The output should be structured and explainable:

```json
{
  "memberships": {
    "satisfied": 0.72,
    "delighted": 0.35,
    "disappointed": 0.18
  },
  "activated_rules": [
    "authenticity_high AND wait_time_medium => satisfaction_high"
  ],
  "confidence": 0.81
}
```

That output can then be consumed by a product formation, Arbiter policy gates,
Ferrox optimization models, or Mnemos feedback loops without making any one
extension the runtime assembler.

See also: [[Modules/Prism]], [[Modules/Arbiter]], [[Modules/Ferrox]],
[[Modules/Mnemos]], [[Architecture/Runtime Assembly]]

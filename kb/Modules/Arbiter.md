---
tags: [module, policy, cedar]
source: mixed
date: 2026-05-05
---
# Arbiter

`arbiter` owns policy enforcement as reusable Converge extension code.

It implements Cedar-based policy decisions and Converge suggestors that turn
authorization findings into context-visible effects.

Arbiter's assurance direction is Cedar-first: use Cedar validation, runtime
tests, and Cedar Analysis / symbolic compilation before adding a separate
Lean, Coq, or Agda verifier. Formal proof assistants remain deferred until a
policy claim cannot be handled by Cedar's own analysis stack.

HITL escalation is strict: Arbiter escalates only when Cedar would allow the
same request with `human_approval_present = true`; otherwise the denial stays a
reject.

Formations should discover Arbiter through its `arbiter.cedar` capability
catalog. The current Formation-facing entries are `arbiter.cedar.policy_gate`,
`arbiter.cedar.hitl_gate`, and `arbiter.cedar.analysis_evidence`.

Arbiter suggestor execution emits `arbiter.suggestor.execute` tracing spans
with provenance, suggestor name, context key, and input count fields.
The provenance value is derived from Arbiter's typed `ProvenanceSource`
adapter before crossing into `converge-pack::ProposedFact`'s string-backed
field.

## Owns

- Cedar policy engine wiring.
- Policy decision types.
- Ed25519 delegation tokens and verification.
- Flow authorization gates.
- Rate, budget, approval, data-classification, and compliance gates.
- Reference Cedar policies for expense approval, flow governance, and vendor
  selection.
- Cedar-first policy assurance fixtures and optional SymCC-backed analysis
  preparation/execution artifacts.
- Suggestor-boundary tracing spans for runtime provenance inspection.
- Typed provenance vocabulary at the Arbiter proposal boundary.

## Public Surface

- `PolicyEngine`
- `PolicyGateSuggestor`
- `DelegationVerifySuggestor`
- `FlowGateSuggestor`
- `CedarHitlGateSuggestor`
- `CedarAnalysisSuggestor` under the `analysis` feature
- `CedarAnalysisBackend` under the `analysis` feature
- `LocalCvc5AnalysisBackend` under the `analysis` feature
- `CedarAnalysisInput`, `CedarAnalysisPlan`, and `CedarAnalysisReport` under
  the `analysis` feature
- `RateLimitGateSuggestor`
- `BudgetGateSuggestor`
- `ApprovalGateSuggestor`
- `DataClassificationGateSuggestor`
- `ComplianceGateSuggestor`
- `PolicyDecision`
- `PolicyOutcome`
- `Delegation`
- `formation_capabilities()`
- `ProvenanceSource`

## Boundary

Converge owns the pack and gate contracts. Arbiter owns the Cedar
implementation and policy suggestor family.

Do not move product-specific policy bundles into Converge. Keep reusable
authorization mechanics here, and let products choose the policies they run.

## Entry Points

- `arbiter-policy/README.md`
- `arbiter-policy/crates/arbiter/src/lib.rs`
- `arbiter-policy/crates/arbiter/src/analysis.rs`
- `arbiter-policy/crates/arbiter/src/engine.rs`
- `arbiter-policy/crates/arbiter/src/formation.rs`
- `arbiter-policy/crates/arbiter/src/suggestor.rs`
- `arbiter-policy/crates/arbiter/invariants/*.feature`
- `arbiter-policy/crates/arbiter/policies/*.cedar`
- `arbiter-policy/crates/arbiter/tests/*.rs`

See also: [[Architecture/Dependency Rules]], [[Architecture/Runtime Assembly]]

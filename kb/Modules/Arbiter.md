---
tags: [module, policy, cedar]
source: mixed
date: 2026-05-05
---
# Arbiter

`arbiter` owns policy enforcement as reusable Converge extension code.

It implements Cedar-based policy decisions and Converge suggestors that turn
authorization findings into context-visible effects.

## Owns

- Cedar policy engine wiring.
- Policy decision types.
- Ed25519 delegation tokens and verification.
- Flow authorization gates.
- Rate, budget, approval, data-classification, and compliance gates.
- Reference Cedar policies for expense approval, flow governance, and vendor
  selection.

## Public Surface

- `PolicyEngine`
- `PolicyGateSuggestor`
- `DelegationVerifySuggestor`
- `FlowGateSuggestor`
- `RateLimitGateSuggestor`
- `BudgetGateSuggestor`
- `ApprovalGateSuggestor`
- `DataClassificationGateSuggestor`
- `ComplianceGateSuggestor`
- `PolicyDecision`
- `PolicyOutcome`
- `Delegation`

## Boundary

Converge owns the pack and gate contracts. Arbiter owns the Cedar
implementation and policy suggestor family.

Do not move product-specific policy bundles into Converge. Keep reusable
authorization mechanics here, and let products choose the policies they run.

## Entry Points

- `arbiter-policy/README.md`
- `arbiter-policy/crates/arbiter/src/lib.rs`
- `arbiter-policy/crates/arbiter/src/engine.rs`
- `arbiter-policy/crates/arbiter/src/suggestor.rs`
- `arbiter-policy/crates/arbiter/policies/*.cedar`
- `arbiter-policy/crates/arbiter/tests/*.rs`

See also: [[Architecture/Dependency Rules]], [[Architecture/Runtime Assembly]]

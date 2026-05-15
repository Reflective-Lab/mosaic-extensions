---
tags: [planning, contracts, proposed-facts]
source: mixed
date: 2026-05-14
---
# Typed Payload Boundaries

Typed provenance was only the first half of the `ProposedFact` boundary.
The shared Converge contract now requires typed in-process payloads as well:
`ProposedFact::new` accepts `FactPayload + PartialEq`, not semantic strings.
Serialization is allowed at borders only.

## Current State

`converge-pack::ProposedFact` carries:

- `key: ContextKey`
- `id: ProposalId`
- typed payload erased behind a `FactPayload` boundary
- `confidence`
- uniform `Provenance`

The payload schema identity is the frozen `(family, version)` tuple declared
by the payload type. `ContextKey` routes the fact instance; it is not the
payload schema.

Migrated families include:

- Arbiter: `DecideRequest`, `CedarAnalysisInput`, `CedarAnalysisReport`.
- Soter: `SmtQuery`, `SmtReport`.
- Ferrox: `LpRequest`, `MipRequest`, `CpSatRequest`, scheduling/job-shop/
  network-flow request and plan DTOs.
- Prism: feature vectors and generic pack inputs/plans.
- Mnemos: `KnowledgeHitPayload`.
- Crucible: training-plan, dataset-split, model metadata, evaluation,
  inference, data-quality, feature-spec, registry, monitoring, and deployment
  payloads.

Generic `PackSuggestor` usage is typed through `PackInputPayload` and
`PackPlanPayload`. Domain-specific Suggestors should still prefer
domain-specific payloads.

## Conclusion

The old string-content bypass is gone from the migrated Converge/extension
fact path. A `ContextKey::Evaluations` fact may still contain different
semantic families, but consumers must ask for the family they understand with
`payload::<T>()` or `require_payload::<T>()`.

Unknown wire `(family, version)` fails closed at the border registry.

## Execution Identity

Converge now owns a shared `ExecutionIdentity` contract and companion
`ExecutionIdentityEvidence` payload. This was promoted because Soter and
Ferrox independently needed the same audit shape for native/backend execution
identity.

- Soter embeds `ExecutionIdentity` directly in `SmtReport`.
- Ferrox solver-owned plans use `ExecutionIdentity` for their `solver_identity`
  field.
- Ferrox CP-SAT formation emits `converge.execution_identity.evidence` beside
  generic `FormationPlan` output instead of polluting `FormationPlan`.

Arbiter and Prism should adopt the same contract when Cedar analysis reports,
policy-model checks, model inference, or analytic engines need replayable
producer/backend/build/runtime identity.

## Prioritization

The next useful bar is narrower than another payload migration:

1. Add payload registries at every real border: CLI fixtures, storage/replay,
   HTTP/gRPC, NATS/Lattice, and audit export.
2. Register only the payload families that border is allowed to accept.
3. Add contract tests that unknown families/versions fail closed.
4. Replace old snapshots through a deliberate rewrite or an explicit
   storage-border decoder that emits named payload families.

## Acceptance Criteria

- In-process facts are constructed with named payload types.
- Reads use `payload::<T>()` / `require_payload::<T>()`.
- Diagnostics use `DiagnosticPayload`; human text uses `TextPayload`.
- Serialization uses `WireProposedFact` / `WireContextFact` at borders only.
- Generic pack flows use `PackInputPayload` / `PackPlanPayload`, not opaque
  legacy JSON.

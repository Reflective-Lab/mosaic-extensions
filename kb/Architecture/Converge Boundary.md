---
tags: [architecture, boundary]
source: mixed
date: 2026-05-05
---
# Converge Boundary

This workspace exists because Converge should not become a warehouse for every
useful capability.

## Converge Owns

- The convergence engine and run lifecycle.
- Shared context, proposals, facts, invariants, and promotion authority.
- Canonical public contracts such as pack authoring, kernel embedding, model
  types, client/protocol surfaces, and provider capability contracts.
- Runtime-neutral semantics that every Converge deployment can reasonably need.

## Extensions Own

- Heavy implementation dependencies.
- Domain-pack suggestor families.
- Native solver bindings.
- Concrete storage, vector, and external SDK adapters.
- Source-specific connector contracts.
- Capability implementations that are reusable but not foundational.

## Products Own

- Runtime assembly.
- Secrets and credentials.
- Operational topology.
- Environment selection.
- User-facing workflows and product-specific policy.

## The Practical Test

Ask three questions before moving code into Converge:

1. Would a minimal generic Converge deployment need this concept?
2. Is this a universal contract, not a provider implementation?
3. Does Converge need to enforce the invariant itself?

If the answer is no, keep it in an extension or product layer.

See also: [[Architecture/Runtime Assembly]], [[Architecture/Dependency Rules]]

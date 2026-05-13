---
tags: [standard, suggestors, convergence]
source: mixed
date: 2026-05-13
---
# Suggestor Contract

This contract applies to extension implementations of
`converge_pack::Suggestor`. Converge owns the trait and promotion authority.
Extensions own specialized implementations that read context and propose
effects.

## Core Rule

A suggestor is an advisory participant. It may read declared context and emit
`AgentEffect` proposals. It must not directly promote facts, mutate the
convergence kernel, or bypass policy gates.

## Context Access

- `dependencies()` must name the context keys the suggestor needs for normal
  operation.
- `accepts(ctx)` must be cheap, deterministic, and side-effect free.
- `execute(ctx)` may read only the context it needs for the declared behavior.
- A suggestor should skip work when its intended output already exists.
- Missing or malformed input should produce `AgentEffect::empty()` or a
  diagnostic proposal; it should not panic.

## Effects

- Suggestors emit proposed facts or proposed plans through `AgentEffect`.
- Proposed facts must carry typed extension-local provenance before crossing
  into the current string-backed `ProposedFact` contract.
- Confidence values are advisory. They are not proof and are not promotion.
- Formal verification, solver execution, Cedar decisions, analytics signals,
  and knowledge recall must remain distinguishable in emitted content and
  provenance.

## Side Effects

Suggestors should be pure with respect to Converge state. External I/O is
allowed only when it is the explicit purpose of the suggestor and is documented
by the extension surface.

Allowed examples:

- `mnemos` storing a promoted evaluation in a configured knowledge base.
- `prism` reading a configured dataset path or writing declared model artifacts.
- `ferrox` calling an embedded/native solver backend for a seed request.
- `arbiter` evaluating a configured Cedar policy engine.

Requirements for external I/O:

- no hidden credentials or undeclared network calls,
- bounded timeouts or caller-controlled limits when the backend can block,
- idempotent behavior where repeated convergence runs may re-enter,
- deterministic diagnostics for common failure modes,
- no product-specific routing or deployment decisions inside the extension.

## Tracing

Every suggestor boundary should create a structured tracing span named:

```text
<extension>.suggestor.execute
```

Required fields:

- `provenance`
- `suggestor`
- `input_key`
- `output_key`
- `input_count`

Async suggestors that await while inside the span should use
`tracing::Instrument` rather than holding a non-`Send` entered span guard
across `.await`.

## Error Handling

- Do not panic for bad user, context, policy, solver, model, or knowledge input.
- Prefer diagnostic proposals when the failure is useful to the convergence
  loop.
- Prefer `AgentEffect::empty()` when the suggestor simply has nothing relevant
  to contribute.
- Native or remote backend errors must not poison later suggestors in the same
  formation.

## Tests

Every suggestor family should have focused tests for:

- happy-path proposal emission,
- malformed input,
- skip behavior when output already exists,
- negative or infeasible cases for the domain,
- provenance on emitted proposals,
- confidence semantics where confidence is meaningful,
- property tests when the suggestor encodes a reusable invariant.

Compile-fail tests are appropriate when Rust types enforce a boundary, such as
preventing packs or suggestors from emitting promoted facts directly.

## Ownership Boundary

Converge decides promotion. Extensions propose. Products assemble and configure.

If a suggestor needs a new universal contract to behave safely, promote the
contract upstream only after a second extension family or app needs it. Until
then, keep implementation-local helper types inside the extension that uses
them.

See also: [[Extension Standard]], [[Architecture/Runtime Assembly]],
[[Architecture/Dependency Rules]]

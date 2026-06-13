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
- When an output proposal is derived from a specific input `ContextFact`, the
  Suggestor must preserve that fact's `SubjectRef` with
  `ProvenanceSource::proposed_fact_for(...)` or
  `ProposedFact::with_subject_from(...)`. The extension treats the ref as
  opaque app-owned correlation metadata; it must not infer Helm readiness,
  app authority, or domain state from the subject string.
- The Suggestor must override `Suggestor::provenance()` to return its
  crate's canonical `*_PROVENANCE.as_str()`. The engine middleware emits
  a uniform `suggestor.execute` span carrying that string in the
  `provenance` field. **An empty `provenance()` on a Suggestor that
  emits proposals is a contract violation** — log-query auditors should
  flag any `suggestor.execute` span with `provenance=""` that produced
  proposals. Filter / observer Suggestors that never emit proposals are
  the only legitimate consumers of the default empty `provenance()`.
- Reusable or high-risk proposed fact families should expose a typed or
  schema-backed payload boundary: expected context key, id prefix, payload
  DTO, version, and validator. A broad `ContextKey` plus arbitrary JSON is
  acceptable for diagnostics and notes, but not for facts used as promotion,
  policy, solver, or CI evidence.
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

The engine middleware emits the canonical span name `suggestor.execute`
once per `Suggestor::execute` call. Field ownership is split:

**Engine-emitted (always present):**

- `provenance` — from `Suggestor::provenance()`
- `suggestor` — from `Suggestor::name()`

**Extension-emitted (added by the suggestor body):**

- `input_key` — primary context key consumed
- `output_key` — primary context key emitted
- `input_count` — number of input facts considered

Per-crate `<extension>.suggestor.execute` spans are deprecated; rely on
the engine's canonical span and add the extension-emitted fields inside
it. Async suggestors that await while inside the span should use
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
- subject pass-through when the input fact carries `SubjectRef`,
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

## Trait Stability Policy

The `Suggestor` trait is a load-bearing contract for every downstream
extension. The following rules are non-negotiable:

- **New methods MUST ship with default implementations.** Adding a required
  method is a breaking change that forces every downstream implementor to
  update in lockstep. Default impls preserve source compatibility.
- **Existing method signatures MUST NOT change** within a Converge MAJOR
  version. Adding parameters, changing return types, or tightening trait
  bounds breaks every implementor silently or noisily.
- **Removing a method** (or marking it required after it was defaulted) is
  reserved for the next MAJOR version and must be staged through a release
  cycle of `#[deprecated]`.
- **Sealing the trait** (preventing external impls) is explicitly rejected.
  The whole mosaic-extensions architecture depends on external crates
  implementing `Suggestor`.

When in doubt, prefer adding a new free function or a sibling trait over
extending `Suggestor` itself. The trait surface should grow slowly.

See also: [[Extension Standard]], [[Architecture/Runtime Assembly]],
[[Architecture/Dependency Rules]]

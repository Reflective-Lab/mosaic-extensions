---
tags: [planning, handoff]
source: mixed
date: 2026-05-14
---
# Upstream Handoff: Typed Provenance, Payloads, and Suggestor Tracing

The eight Mosaic Extensions have converged on two conventions that are
currently enforced by code review rather than by the Converge foundation
or the Organism platform: typed provenance at proposal construction and
structured suggestor tracing. A third boundary is now visible but not yet
landed: typed or schema-backed payload contracts for reusable fact families.
This document collects the upstream work needed to lift these conventions into
enforced contracts so the next extension does not re-invent the pattern.

See also: [[Standards/Suggestor Contract]], [[Planning/MILESTONES]],
[[Architecture/Pluralist Reasoning Substrate]].

## What we landed at the extension level

All five fact-emitting extensions — `arbiter-policy`, `prism-analytics`,
`mnemos-knowledge`, `ferrox-solvers`, `soter-smt` — now:

- define a crate-local `ProvenanceSource` enum and a canonical
  `*_PROVENANCE: &str` constant, and route
  `ProposedFact::new(..., provenance)` through it instead of raw string
  literals;
- wrap `Suggestor::execute` in a `<crate>.suggestor.execute` tracing
  span carrying structured fields (provenance, suggestor name, context
  keys, input count).

Three extensions are out of scope for this contract because they do not
implement Suggestors:

- `crucible-models` — trained-artifact pack crate; no fact emission.
- `manifold-adapters` — provider and capability registry; integration
  surface only.
- `embassy-ports` — typed `Observation<T>` contract layer; emits
  pre-structured observations, not proposed facts.

Adherence across the five in-scope extensions was confirmed by audit on
2026-05-14. Of those, `ferrox-solvers` and `soter-smt` already use a
`suggestor_span()` helper function rather than inline
`tracing::info_span!`. That helper is the prototype of the engine
middleware described in **Converge task 2** below.

The same audit found that proposal payloads are still only partially typed.
Most extensions define Rust DTOs internally, but `ProposedFact` stores
`content: String` and broad `ContextKey`s do not identify which DTO is present.
See [[Typed Payload Boundaries]] for the payload-contract follow-up.

The Suggestor Contract standard
(`kb/Standards/Suggestor Contract.md`) records the read/write
boundaries, no-panic, no-undeclared-I/O, and cancellation-safety rules
these implementations rely on.

## Task → Converge platform

1. **Add a `Provenance` trait** in the contract layer
   (`converge-pack` or `converge-core`):

   ```rust
   pub trait Provenance: Debug + Send + Sync + 'static {
       fn as_str(&self) -> &'static str;
       fn kind(&self) -> ProvenanceKind;
   }
   ```

   Add an additive constructor
   `ProposedFact::with_typed_provenance(impl Provenance, ...)`. The
   existing `&str` overload stays for now — migration is incremental
   and non-breaking.

2. **Suggestor execution middleware in the Engine.** The engine emits a
   `suggestor.execute` tracing span around every `Suggestor::execute`
   call, carrying the suggestor name and provenance automatically.
   Once shipped, extensions delete their hand-rolled spans. Removes
   the per-crate copy of `tracing::info_span!("…suggestor.execute", …)`
   and guarantees coverage.

3. **Promote the Suggestor Contract** from
   `mosaic-extensions/kb/Standards/Suggestor Contract.md` into
   Converge's `kb/Standards/`. Cancellation-safety, no-panic,
   declared-dependencies, and error-handling rules become platform
   expectations rather than workspace convention.

4. **Add schema-backed fact-family helpers.** Keep `ProposedFact` wire
   compatibility, but add additive construction/parsing helpers that bind
   a payload type to a family id, schema version, expected context keys,
   id prefix, and validator. This should start as an extension-local
   convention and move upstream only after at least two extension families
   adopt the same shape.

## Task → Organism platform

1. Implement the new `Provenance` trait in organism packs (one variant
   per pack family).
2. Once Converge ships the middleware, delete any hand-rolled spans in
   organism pack suggestors.
3. Audit organism's existing suggestor implementations against the
   elevated Suggestor Contract.

## Suggested ordering

| Step | Owner | Blocking |
|------|-------|----------|
| Converge task 1 (trait) | Converge | nothing — purely additive |
| At least three extensions migrate | Mosaic | Converge task 1 |
| Converge task 2 (middleware) | Converge | adoption evidence |
| Extensions drop hand-rolled spans | Mosaic | Converge task 2 |
| Organism task 1 (trait adoption) | Organism | Converge task 1 |
| Converge task 3 (contract promotion) | Converge | any time |
| Payload-family helper prototype | Mosaic | one high-risk family |
| Converge task 4 (payload helpers) | Converge | adoption by two families |
| Organism task 3 (contract audit) | Organism | Converge task 3 |

## Acceptance criteria

- **Converge task 1** is done when a Mosaic extension can replace its
  crate-local `ProvenanceSource::as_str() -> &str` with
  `impl Provenance for ProvenanceSource` and call
  `ProposedFact::with_typed_provenance(prov, …)` without any
  string-literal fallback.
- **Converge task 2** is done when the engine emits
  `suggestor.execute` spans without any extension code calling
  `tracing::info_span!` directly.
- **Converge task 3** is done when the upstream
  `kb/Standards/Suggestor Contract.md` exists and Mosaic extensions
  cite the upstream version rather than the workspace copy.
- **Converge task 4** is done when a Mosaic extension can emit and read a
  `ProposedFact` through a fact-family contract without manually spelling
  the id prefix, context keys, schema version, serializer, or validator at
  every call site.

## Open questions

- Should `ProvenanceKind` carry an evidence-tier hint (`Observed`,
  `Decided`, `Searched`, `Argued`, `Verified`), or stay neutral and
  leave evidence-tier mapping to the kernel?
- Do middleware-emitted spans need an opt-out per-suggestor, or is the
  span always-on with the suggestor free to add child spans?
- Does the `Provenance` trait need a version field for forward
  compatibility, or is `&'static str` stable enough?
- Should fact-family schemas be Rust-only DTO contracts first, or should
  Converge also emit JSON Schema artifacts for product fixtures and
  non-Rust integrations?

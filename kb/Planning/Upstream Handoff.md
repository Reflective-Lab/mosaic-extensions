---
tags: [planning, handoff]
source: mixed
date: 2026-05-14
---
# Upstream Handoff: Typed Provenance, Payloads, and Suggestor Tracing

The Mosaic Extensions converged on three conventions: typed provenance at
proposal construction, typed `FactPayload` payloads in process, and structured
suggestor tracing. Typed payloads have now moved into the Converge contract;
the remaining upstream work is about platform-owned provenance/tracing
middleware and border registries.

See also: [[Standards/Suggestor Contract]], [[Planning/MILESTONES]],
[[Architecture/Pluralist Reasoning Substrate]].

## What we landed at the extension level

Fact-emitting extensions — `arbiter-policy`, `prism-analytics`,
`mnemos-knowledge`, `ferrox-solvers`, `soter-smt`, and `crucible-models` —
now:

- define a crate-local `ProvenanceSource` enum and a canonical
  `*_PROVENANCE: &str` constant, and route
  `ProposedFact::new(..., provenance)` through it instead of raw string
  literals;
- wrap `Suggestor::execute` in a `<crate>.suggestor.execute` tracing
  span carrying structured fields (provenance, suggestor name, context
  keys, input count).
- emit and read named `FactPayload` types instead of semantic strings.

Two extensions are out of scope for this contract because they do not
implement Suggestors:

- `manifold-adapters` — provider and capability registry; integration
  surface only.
- `embassy-ports` — typed `Observation<T>` contract layer; emits
  pre-structured observations, not proposed facts.

Adherence across the five in-scope extensions was confirmed by audit on
2026-05-14. Of those, `ferrox-solvers` and `soter-smt` already use a
`suggestor_span()` helper function rather than inline
`tracing::info_span!`. That helper is the prototype of the engine
middleware described in **Converge task 2** below.

Payload typing is no longer just convention: `ProposedFact::new` requires
`FactPayload + PartialEq`, and generic pack execution uses
`PackInputPayload` / `PackPlanPayload`. See [[Typed Payload Boundaries]].

The Suggestor Contract standard
(`kb/Standards/Suggestor Contract.md`) records the read/write
boundaries, no-panic, no-undeclared-I/O, and cancellation-safety rules
these implementations rely on.

## Task → Converge platform

1. **Add a `Provenance` trait** in the contract layer
   (`converge-pack` or `converge-core`) if we want stronger provenance
   typing than the current uniform `Provenance` value:

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

4. **Add border payload registries.** `WireProposedFact` and
   `WireContextFact` are the public contract. Each border should register
   only the families it is allowed to accept and fail closed on unknown
   `(family, version)` tuples. This should start as an extension-local
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
| Border payload registry prototype | Mosaic | typed payload families landed |
| Converge task 4 (border registries) | Converge | adoption by two borders |
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
- **Converge task 4** is done when a Mosaic border can materialize
  `WireProposedFact` / `WireContextFact` through a registered allow-list of
  payload families and reject unknown families/versions.

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

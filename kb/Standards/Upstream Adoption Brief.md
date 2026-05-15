---
tags: [standard, adoption-brief]
source: mixed
date: 2026-05-15
---
# Upstream Adoption Brief — ProvenanceSource + Engine Middleware + ExecutionIdentity

This brief is for the four platforms that depend on Converge — the
foundation itself, plus Organism, Axiom, and Helms — explaining what
shipped in `converge-pack` + `converge-core` and what each platform
needs to do to adopt the workspace-standard fact-emission contract.

The mosaic extensions (arbiter, prism, mnemos, ferrox, crucible,
soter) finished their migration to this contract on 2026-05-15.
Reference implementations live in those six crates.

## What's new in Converge

### 1. `converge_pack::ProvenanceSource` trait

```rust
pub trait ProvenanceSource: Copy + Send + Sync + 'static {
    fn as_str(&self) -> &'static str;
    fn proposed_fact<T>(
        self,
        key: ContextKey,
        id: impl Into<ProposalId>,
        payload: T,
    ) -> ProposedFact
    where
        T: FactPayload + PartialEq;
}
```

Replaces the per-crate `ProvenanceSource` enum pattern that
duplicated an N-variant enum + `FromStr` + `Display` +
`UnknownProvenanceSource` + `ALL` array in every fact-emitting
crate. Each consumer now declares only its own marker:

```rust
use converge_pack::ProvenanceSource;

pub struct Organism;
impl ProvenanceSource for Organism {
    fn as_str(&self) -> &'static str { "organism" }
}
pub const ORGANISM_PROVENANCE: Organism = Organism;
```

Three lines + a const. No enum that lists every sibling.

### 2. `Suggestor::provenance()` default method

```rust
trait Suggestor {
    // ... existing methods unchanged
    fn provenance(&self) -> &'static str { "" }
}
```

Default returns the empty string for non-fact-emitting Suggestors.
Fact-emitting Suggestors override:

```rust
fn provenance(&self) -> &'static str { ORGANISM_PROVENANCE.as_str() }
```

### 3. Engine middleware in `converge-core::Engine::execute_agents`

Wraps every `Suggestor::execute` call in a uniform
`suggestor.execute` tracing span. Field provenance is split into
two layers:

**Engine-emitted (always present, set by middleware):**

- `suggestor` — `Suggestor::name()`
- `provenance` — `Suggestor::provenance()`

**Extension-emitted (added by the suggestor inside its span):**

- `input_key` — primary context key consumed
- `output_key` — primary context key emitted
- `input_count` — number of input facts considered

This split is canonical. The full required-field list lives in
`kb/Standards/Suggestor Contract.md` ("Tracing"); the brief only
names the engine-emitted half. No more per-crate
`<crate>.suggestor.execute` helpers. The engine emits the canonical
span exactly once per Suggestor call. Filter log queries by the
`provenance` field, not by span name prefix.

### 4. `ExecutionIdentity` (already shipped, now broadly adopted)

Every fact-emitting payload should carry an `ExecutionIdentity` so
audit and replay can answer *which library version, with which
runtime config, produced this fact*. See
`crucible::types::ClassPredictionPayload` and
`ferrox::solver_identity` for reference shapes.

## What each platform needs to do

### Converge (the foundation)

- [x] `ProvenanceSource` trait shipped in `converge-pack::fact`.
- [x] Engine middleware shipped in `converge-core::engine::execute_agents`.
- [ ] Tag a release that exposes both. Today they live in your local
  WIP tree alongside other in-flight platform work.
- [ ] Update Converge's own `kb/Standards/` to point at the trait
  and middleware as the canonical pattern; the mosaic
  `kb/Standards/Suggestor Contract.md` is the working source.

### Organism (`bedrock-platform/organism`)

- [ ] Declare an `Organism` ZST + `ORGANISM_PROVENANCE` const that
  implements `converge_pack::ProvenanceSource`.
- [ ] For every fact-emitting Suggestor (organism pack), override
  `Suggestor::provenance()` to return `ORGANISM_PROVENANCE.as_str()`.
- [ ] Drop any per-crate `suggestor_span` helper. The engine emits
  the canonical span.
- [ ] Add an `execution_identity` field to every typed payload
  emitted by an organism pack. See *Backend String Format* and
  *Runtime Config Encoding* standards.
- [ ] Use `proposed_fact` from the trait at all fact-emission sites:
  `ORGANISM_PROVENANCE.proposed_fact(key, id, payload)`.

### Axiom-truth (`bedrock-platform/axiom`)

- [ ] Declare an `Axiom` (or `AxiomTruth`) ZST + const implementing
  `converge_pack::ProvenanceSource`.
- [ ] Truth parsing emits `ProposedFact`s; route them through
  `AXIOM_PROVENANCE.proposed_fact(...)`.
- [ ] Add `execution_identity` to every emitted payload — at
  minimum the axiom-truth crate version; if a particular parser
  uses a versioned external grammar / model, populate it as the
  backend.

### Helms (`bedrock-platform/helms`)

Helms's seed-gen produces parquet, not `ProposedFact`s, so the
trait does not apply to seed-gen output. However:

- [ ] If any helms component emits Convergence facts (today, none
  do), follow the Organism pattern above.
- [ ] Helms's parquet output should still carry a workspace-standard
  identity envelope — recommended: emit a sidecar JSON manifest
  containing `producer = "helms-<binary>-v<MAJOR>.<MINOR>"` and the
  seed-gen runtime config. This is a follow-on convention; the
  current `ExecutionIdentity` shape covers it directly when helms
  starts emitting facts.

## Reference migrations (mosaic-extensions)

| Extension | Commit | Lines removed |
|---|---|---|
| `crucible-models` | `2c2935d` | -108 |
| `soter-smt` | `6874609` | -58 |
| `mnemos-knowledge` | `d211638` | -105 |
| `ferrox-solvers` | `665c86f` | -102 |
| `arbiter-policy` | `2cc4c49` | -95 |
| `prism-analytics` | `ea4cda4` | -108 |

Each migration:

1. Deletes the per-crate `ProvenanceSource` enum, `UnknownProvenanceSource`,
   `FromStr`, `Display`, and `ALL` array.
2. Replaces them with a single ZST + const + trait impl (≈ 15 lines).
3. Keeps the canonical const name (`*_PROVENANCE`) so every call
   site reads the same after migration.
4. Updates `lib.rs` re-exports.

Reading any one of these six diffs is enough to see the pattern.

## Conventions

- **Backend string format**: `<package>-v<MAJOR>.<MINOR>` (e.g.
  `linfa-trees-v0.8`, `cvc5-v1.0`). See
  `kb/Standards/Backend String Format.md`.
- **runtime_config encoding**: JSON of the typed config struct via
  `serde_json::to_string(&config)`. See
  `kb/Standards/Runtime Config Encoding.md`.

## Adoption checklist (per platform)

- [ ] Local `ProvenanceSource` enum replaced with ZST + const.
- [ ] Local `UnknownProvenanceSource`, `FromStr`, `Display`, `ALL`
  array deleted.
- [ ] `lib.rs` re-exports updated.
- [ ] `Suggestor::provenance()` overridden on every fact-emitting
  Suggestor.
- [ ] Local `suggestor_span` helper deleted (or kept `pub(crate)`
  during transitional cleanup).
- [ ] `execution_identity` carried in every emitted typed payload.
- [ ] Backend-string format and runtime_config encoding adopted.
- [ ] `cargo test`, `cargo clippy -D warnings`, `cargo fmt --check`
  all green.

## Why this matters

Before this contract, every fact-emitting extension carried a
duplicated 7-or-8-variant enum that listed every sibling extension
by name, a duplicated span helper, and ad-hoc identity strings.
Adding a new extension meant updating every existing extension's
enum.

After this contract, a new extension declares its own three-line
provenance and inherits the rest. Audit, log filtering, and
identity reasoning all read uniformly across platforms.

This is the contract that should be the same in Converge, Organism,
Axiom, Helms, and downstream applications.

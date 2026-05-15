---
tags: [standard]
source: mixed
date: 2026-05-15
---
# Runtime Config Encoding

The `runtime_config` field in
[`converge_pack::ExecutionIdentity`](https://github.com/Reflective-Lab/converge/blob/main/crates/pack/src/fact.rs)
carries the parameters that the producer used at runtime — solver
timeout, model hyperparameters, retrieval cutoff, and so on. Audit
and replay queries need to compare these across extensions without
parsing per-crate folklore.

## Format

JSON serialization of the typed config struct, via the canonical
helper on `ExecutionIdentity`:

```rust
let runtime_config = ExecutionIdentity::runtime_config_from_typed(&typed_config);
// or, builder-style on an existing identity:
let identity = ExecutionIdentity::non_native(/* ... */)
    .with_runtime_config_typed(&typed_config);
```

- `typed_config` is the crate's own `Config` struct (e.g.
  `RandomForestConfig`, `SmtBudget`, `CpSatSettings`).
- The struct derives `Serialize` via `serde`.
- JSON keys are the struct field names; values are the field
  values.
- The helper panics if `T`'s `Serialize` impl is malformed
  (non-finite floats, non-string map keys); for all workspace
  config structs this is unreachable. A panic here means the
  caller's config struct is broken — fix that, do not paper over
  with `unwrap_or_default()`.

Direct `serde_json::to_string(&typed_config).unwrap_or_default()` is
deprecated: it silently masks bugs in `Serialize` impls. Use the
canonical helper.

## Why JSON

- **Cross-extension comparable.** Any auditor can decode the field
  with `serde_json::from_str` without knowing the producer's Rust
  source.
- **Self-describing.** Field names carry meaning; positional
  encodings do not.
- **Stable.** Adding new fields to a config struct is backward
  compatible at the JSON layer (old consumers ignore unknown
  fields if they want, or error if they're strict).
- **Already a workspace dep.** No new infrastructure.

## Examples

### Crucible RandomForestConfig

```json
{
  "n_trees": 25,
  "max_depth": 6,
  "min_weight_split": 2.0,
  "random_seed": 11
}
```

### Ferrox CP-SAT runtime

```json
{
  "time_limit_seconds": 10,
  "num_search_workers": 8,
  "log_search_progress": false
}
```

### Soter SMT query envelope

```json
{
  "logic": "QF_LIA",
  "timeout_ms": 30000,
  "produce_proofs": true
}
```

## What this is *not*

- Not a place to store the typed payload itself. The payload goes
  in the `ProposedFact.payload` slot. `runtime_config` is the
  producer's *configuration*, not its *output*.
- Not a place for secrets. If a config struct ever grows a secret
  field, redact it before serialising.
- Not a stable schema across major versions. When a config struct
  changes shape (renamed field, removed field), the JSON shape
  changes with it. Bump the producer's `MAJOR` version
  accordingly; the audit record then reads correctly because
  `ExecutionIdentity.producer.version` records the producer
  version that emitted that JSON.

## When the config is empty

Some producers have no runtime config worth recording (a
closed-form inference pack with no parameters). Emit an empty JSON
object:

```json
{}
```

Not the empty string, not `null` — a literal `{}`. Audit queries
can then `serde_json::from_str` without special-casing.

## Migration note

Extensions that landed before this standard sometimes encoded
`runtime_config` as a free-form string (e.g. `"timeout_ms=10000"`).
Migrate to JSON as the typed config struct stabilises. Don't churn
just for the format change; do it the next time the config evolves.

---
tags: [standard]
source: mixed
date: 2026-05-15
---
# Backend String Format

The `backend` field in
[`converge_pack::ExecutionIdentity`](https://github.com/Reflective-Lab/converge/blob/main/crates/pack/src/fact.rs)
identifies the library, solver, or model that produced a fact. To
keep audit and replay queries uniform across the workspace, every
extension follows the same format.

## Format

```
<package>-v<MAJOR>.<MINOR>
```

- `<package>` — lowercase, hyphen-separated package or library
  name. No spaces, no underscores.
- `v` — literal lowercase letter.
- `<MAJOR>.<MINOR>` — the major and minor version of the package,
  no patch version. Patch versions drift too often to be useful
  audit signal; include the patch only when a specific patch
  changes behaviour materially.

## Examples

| Extension | Backend |
|---|---|
| Ferrox CP-SAT | `cp-sat-v9.15` |
| Ferrox HiGHS | `highs-v1.14` |
| Ferrox GLOP | `glop-v9.15` |
| Soter CVC5 | `cvc5-v1.0` (or the FFI-reported version) |
| Crucible Random Forest | `linfa-trees-v0.8` |
| Crucible Decision Tree | `linfa-trees-v0.8` |
| Crucible ANFIS (planned) | `burn-v0.20` |
| Prism (closed-form, no backend) | omitted; producer crate is the identity |

## Native vs non-native backends

When the backend is a native library exposed via FFI (CVC5,
OR-Tools, HiGHS), pair the backend string with a
`NativeExecutionIdentity` carrying:

- `version` — the actual linked-library version (not the Rust
  binding's version).
- `source_url` — upstream source-of-truth URL.
- `expected_commit` — pinned commit from the workspace's native
  dependency manifest.
- `actual_commit` — commit reported by the FFI at runtime.
- `source_mode` — how the native library was acquired (e.g.
  `pinned-commit`, `external-root`).

When the backend is pure Rust with no native dependency (linfa,
polars-only inference), use
`ExecutionIdentity::non_native(producer_name, producer_version, backend, runtime_config)`
without a `NativeExecutionIdentity`.

When the producer has no meaningful backend (closed-form inference
where the crate version is the identity), use
`ExecutionIdentity::unspecified(producer_name, producer_version)`.

## What this is *not*

- Not a wire format or schema — `ExecutionIdentity` is the schema;
  the backend string is one field.
- Not a substitute for the producer name; the producer is always
  the Rust crate that owns the Suggestor.
- Not a stable contract for `<patch>` versions; include patch only
  when a patch materially changes behaviour.

## When to deviate

Don't, unless an external system you're integrating with imposes
its own identifier scheme. In that case document the deviation
inline at the call site and reference this standard.

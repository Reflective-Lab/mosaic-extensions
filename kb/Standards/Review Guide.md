---
tags: [standard, quality, review]
source: mixed
date: 2026-05-18
---
# Review Guide

The workspace code quality standard. Covers the smells to look for during review,
the patterns to copy, and the Converge compliance rules that apply across all
eight extension families.

The authoritative file is `REVIEW-GUIDE.md` at the workspace root.

## Themes

### 1. Strings with Semantics
A `String` field that has a constrained format or a fixed vocabulary is a
missing newtype or enum. The canonical signal: a `match` on `.as_str()`, or a
field comment listing allowed values.

Reference implementations: `Lei` (gleif), `VatNumber` (vies), `OrgNumber`
(bolagsverket), `FormType::Other(String)` (sec-edgar).

### 2. Anonymous Numeric Primitives
A `f64`/`f32` with a `[0,1]` or `[0,∞)` range enforced only by a downstream
`clamp()` or `validate()` call. A `usize` validated `>= 1` in a validator instead
of `NonZeroUsize`. Money as `f64`. Time units only in the field name.

The `MembershipDegree` newtype pattern is the canonical fix for any `[0,1]` float.

### 3. Property Test Smell
A property test that validates a range constraint is a sign the type should own
the constraint. Once the type's constructor enforces the invariant, the test
deletes itself.

### 4. Reinventing Existing Functionality
`.partial_cmp(...).unwrap_or(Equal)` → `.total_cmp()`. `DefaultHasher` for
content fingerprinting → SHA-256. Duplicate boilerplate across trait implementors
→ a generic helper in the pack crate.

### 5. Separation of Concerns
A Converge port has four modules: `types.rs`, `provider.rs`, `suggestor.rs`,
`provenance.rs`. The suggestor is a Port, not an engine — business logic belongs
in a domain struct the suggestor calls.

### 6. Converge Compliance
No `unwrap`/`expect` in non-test library code. No `#[allow(dead_code)]`. No
`String` error types in public APIs. `serde(deny_unknown_fields)` on every
`FactPayload`. Thread context; do not construct it.

### 7. Production Panic Safety
Library functions reachable in production must never panic on reachable inputs.
Static regex → `LazyLock`. Float sorting → `.total_cmp()`. Constructors that can
fail → `Result<Self, _>`. FFI unsafe blocks → `// SAFETY:` comment on every one.

## Reference: soter/cvc5-sys

The reference FFI implementation: every `unsafe` block has a SAFETY comment; C
status is mapped to a Rust enum at the boundary; raw types are never made public.

## Quick Checklist

See `REVIEW-GUIDE.md` section 9 for the full yes/no checklist covering types,
tests, duplication, Converge compliance, FFI, and separation of concerns.

See also: [[Standards/Suggestor Contract]], [[Standards/Extension Standard]],
[[Architecture/Dependency Rules]]

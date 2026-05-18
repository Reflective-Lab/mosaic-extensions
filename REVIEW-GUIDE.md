# Code Review Guide

This guide captures the quality standards for this workspace. It is organized around smells —
patterns that signal something is wrong — and reference implementations — patterns to copy.

The philosophy behind every rule: **types do the work, tests verify behaviour, Converge is the
contract**. If a type cannot be wrong, you need fewer tests. If you implement the Converge
contract correctly, you need no workarounds.

---

## 1. Strings with Semantics

**The smell:** a `String` or `&str` field carries a value with constrained meaning — an identifier
with a documented format, an enum with a fixed vocabulary, a hash with a known prefix. Any
non-empty string passes the type checker. Typos, transposed fields, and renamed values are
invisible until runtime.

**How it shows up:**

- A `match` arm on `.as_str()` — `"optimal"`, `"deploy"`, `"healthy"` — is a string with
  semantics. If the string changes, the match silently becomes dead code.
- A field comment that lists the allowed values — `// "quote" | "spend" | "contract"` — means
  the vocabulary belongs in an enum, not a comment.
- A field named `*_id`, `*_code`, `*_type`, `*_status`, `*_action` that is `String`.
- Path fields stored as `String`, converted via `Path::new(&string)` at every use site.
- An error variant `NotFound(String)` where the string is always produced by `.to_string()` on a
  structured value (e.g. a `Uuid`).
- A hardcoded string literal in production code — `"cedar"`, `"knowledge-base"`, `"stub_gleif"` —
  with no named constant.

**The fix:**

```rust
// Before — silent failure if the vocabulary changes
pub status: String

// After — exhaustive match, rename-safe, self-documenting
pub enum SolveStatus { Optimal, Feasible, Infeasible, Unbounded, Error }
#[serde(rename_all = "snake_case")]
pub status: SolveStatus
```

For identifiers with a documented format:

```rust
pub struct Lei(String);
impl Lei {
    pub fn parse(s: &str) -> Result<Self, LeiError> {
        // enforce mod-97 check, length, character set
    }
    pub fn as_str(&self) -> &str { &self.0 }
}
```

For semi-open vocabularies (known set + future extension):

```rust
pub enum ComplianceFramework { Soc2, Iso27001, Gdpr, Other(String) }
pub enum FormType { Form10K, Form10Q, Form8K, Other(String) }
```

**Reference implementations to copy:**

- `gleif/src/types.rs` — `Lei` with full mod-97 check-digit validation
- `vies/src/types.rs` — `VatNumber` with country-code membership check and GR/EL normalization
- `bolagsverket/src/types.rs` — `OrgNumber` with Luhn check-digit validation
- `sec-edgar/src/types.rs` — `FormType::Other(String)` for a semi-open vocabulary; `Cik` with
  zero-padding normalization; `AccessionNumber` with hyphen normalization

**Extra scrutiny:** identifier types in embassy. Every identifier that has a published format
(`QId`, `Pmid`, `ArxivId`, `EpoNumber`, `CompanyNumber`, `OrgSlug`) should validate that format
in `parse`. A newtype that only checks non-empty is better than a bare `String` but is not done.

---

## 2. Anonymous Numeric Primitives

**The smell:** a `f64`, `f32`, `i64`, `usize` field that carries an implicit constraint — a range,
a unit, a domain — that the type does not enforce. The constraint lives in a comment, a
downstream `validate()` call, a `clamp()` call, or a property test.

**How it shows up:**

- A field comment `// 0.0 to 1.0` or `// must be positive` next to a bare `f64`.
- A `validate()` method whose entire body is range checks on primitive fields.
- A `clamp()` call scattered across multiple files all clamping the same field.
- Two fields of the same type in one struct with different semantics — `count: usize` and
  `limit: usize`, or `total: f64` and `cap: f64` — that could be accidentally transposed.
- `time_limit_seconds: f64` (use named constants for defaults; consider `Duration`).
- `usize` validated as `>= 1` anywhere → `NonZeroUsize`.
- `i64` representing euros/cents that admits negative values.
- `f32` `temperature` / `top_p` with no range constraint sent to an API.
- A magic literal `60.0` or `0.001` with no named constant.

**The fix:**

```rust
// Before — [0,1] range enforced by scattered clamp() calls
pub confidence: f64

// After — invariant is in the type; no clamp() at callsites
pub struct MembershipDegree(f64);
impl MembershipDegree {
    pub fn new(v: f64) -> Self { Self(v.clamp(0.0, 1.0)) }
    pub fn value(self) -> f64 { self.0 }
}
pub confidence: MembershipDegree
```

```rust
// Before — semantics lost, fields transposable
pub tail: i32, pub head: i32

// After — indices are not interchangeable
pub struct NodeId(pub i32);
pub tail: NodeId, pub head: NodeId
```

For counts that must be non-zero:

```rust
// Before
pub top_k: Option<usize>  // validated >= 1 in validate()

// After
pub top_k: Option<NonZeroUsize>
```

**Money is never `f64`.** Store as `i64` in the smallest unit (micros, cents) with the unit in
the type name. `f64` for money fails round-trip through JSON.

```rust
// Reference: usaspending/src/types.rs
pub total_obligated_micros: i64  // USD micro-dollars; avoids float round-trip
```

**Units in field names are a smell.** `duration_min: i64`, `latency_ms: u64`,
`timeout_seconds: f64` — the unit is only in the name. A newtype (`Minutes(i64)`,
`LatencyMs(u64)`) makes the unit impossible to ignore and prevents passing seconds where
milliseconds are expected.

---

## 3. Property Test Smells

**The smell:** a property test that validates an invariant the *type* should own. If you find
yourself generating random inputs to assert that a value stays in range, round-trips through
parsing, or maintains a structural constraint — the type is missing that constraint.

**Concrete examples of this smell:**

```rust
// Test is checking what the type should guarantee
#[test]
fn success_ratio_is_in_bounds(v in 0.0f64..=1.0) {
    let report = EvaluationReport::new(v);
    assert!((0.0..=1.0).contains(&report.success_ratio));
}
// Fix: struct SuccessRatio(f64) with new(v: f64) -> Self { Self(v.clamp(0.0, 1.0)) }
// Then the test is trivially true and can be deleted.
```

```rust
// Test is checking a construction invariant
#[test]
fn delegation_respects_amount_cap(amount in 0i64..1000, cap in 0i64..1000) {
    let token = DelegationToken::new(amount.min(cap), cap);
    assert!(token.amount <= token.cap);
}
// Fix: DelegationToken::new() rejects amount > cap at construction.
// The property test disappears; the constraint is structural.
```

**The useful inversion:** a property test that validates `MembershipFunction::evaluate(x) ∈ [0,1]`
for all `x` is a *sign you are about to add a `MembershipDegree` type*. Once the type exists and
its constructor does the clamping, the test becomes redundant. The test is a stepping stone, not
a destination.

**Property tests that are genuinely valuable** (do not remove):

- Round-trip through serialization for any type that crosses a JSON boundary.
- Commutativity / associativity properties for solver objectives.
- "Parse of valid value succeeds; parse of invalid value fails" for newtypes with format
  validation — these test the parser logic, not the range constraint.

---

## 4. Reinventing Existing Functionality

**The smell:** an implementation of something that already exists in `std`, the workspace, or a
well-established crate. The cost is not just code size — it is divergence, where two
implementations of the same thing drift apart over time.

**What to look for:**

- **`.partial_cmp(...).unwrap_or(Equal)`** anywhere in the codebase. Replace with `.total_cmp()`
  (stable since Rust 1.62). It handles NaN deterministically and needs no `unwrap_or`.
- **Hand-rolled argmax** using a `for` loop and a mutable max-index variable. Replace with
  `.iter().enumerate().max_by(|(_, a), (_, b)| a.total_cmp(b))`.
- **Manual accumulation** (`total += v; count += 1`) where `Iterator::sum` and `.count()` work.
- **Exponential backoff** duplicated across files. Extract once; import everywhere.
- **`DefaultHasher` for content fingerprinting.** Not stable across compiler versions or process
  invocations. Use SHA-256 (already in the dependency graph via `sha2`).
- **The same formula in two modules.** Cosine similarity, Luhn check digit, z-score computation —
  if it appears twice, one copy will drift. Extract to a shared location.
- **Structurally identical functions that differ only in the type parameter.** Three "find latest
  payload by iteration" functions, eight "has payload for iteration" functions — generics or a
  macro, not copy-paste.
- **Boilerplate `Suggestor::execute` bodies.** The seed-iterate → dispatch → confidence →
  proposals loop is the same in every port. A generic helper in the pack crate should own it;
  each port provides only the payload construction closure.

**Before reaching for a hand-rolled implementation, check:**

| What you need | What already exists |
|---|---|
| Total-order float comparison | `f32::total_cmp`, `f64::total_cmp` |
| Stable content hash | `sha2`, `blake3` |
| Exponential backoff | `tower::retry` or a shared workspace helper |
| FNV hash | `fnv` crate |
| Non-empty string validation | A newtype with a `parse` function |
| Language detection | `whatlang`, `lingua` |

---

## 5. Separation of Concerns

**The smell:** a single file or type doing more than one thing. The test: can you describe what
this module does in one noun phrase? If the answer requires "and", it is doing too much.

**The standard module split for a Converge port:**

```
crates/<port>/src/
  types.rs       — domain entity types and identifier newtypes
  provider.rs    — Provider trait + stub implementation
  suggestor.rs   — Suggestor trait implementation (thin — delegates to provider)
  provenance.rs  — provenance constant
  live.rs        — optional; live Provider implementation
```

Every file that does not follow this split is a candidate for refactoring. If `lib.rs` contains
types, provider, and suggestor all together, it was written under time pressure and needs to be
split.

**For larger modules (training pipelines, knowledge bases, LLM backends):**

- Wire types are not domain types. Parse wire format at the boundary; pass domain types through.
- HTTP transport is not response parsing. A function that sends a request and also parses the
  response body is two functions.
- Retry logic is not business logic. Extract it; then the business logic is a pure function of
  request → response.
- A 2000+ line file is a refactoring target regardless of how well-structured the code inside it
  is. The file length alone creates merge-conflict risk and makes navigation slow.

**The suggestor is a Port, not an engine.** If business logic lives in `execute()`, extract it
to a domain struct that the suggestor calls. The suggestor should be thin — it maps Converge
context to domain inputs, calls the domain logic, and maps results back to proposals.

---

## 6. Converge Compliance

**The contract is absolute.** No workarounds. No shims. No conditional compilation that patches
over a Converge trait boundary. If something is not possible within the contract, either the
contract needs to evolve or the feature does not ship.

**Checklist:**

- **No `unwrap()` or `expect()` in non-test, non-`main` library code** unless the invariant is
  genuinely unrepresentable as a type and is documented with a comment explaining why the panic
  is impossible.
- **`LazyLock` for static regex / expensive construction** that would otherwise panic in a
  `Default::default()` body. Construct once at process start; never per-request.
- **`#[allow(...)]` attributes** — each one is a debt. Suppress a Clippy lint only if fixing it
  would make the code worse; document why. `#[allow(dead_code)]` is never acceptable — either
  wire the code in or delete it.
- **`impl Default` on types that have no meaningful default** — if `Default::default()` produces
  a value that is not valid for normal use, remove the impl and require callers to be explicit.
  A `CallContext::default()` that silently discards all correlation state is the canonical
  violation of this rule.
- **No `String` error types.** Every `Result<_, String>` in a public API is a `thiserror` enum
  waiting to be born. Callers cannot pattern-match on a string; they can on an enum variant.
- **`serde(deny_unknown_fields)` on every `FactPayload` struct.** If the payload schema changes
  and a consumer is running an old binary, fail loud. Silently ignoring unknown fields masks
  version skew.
- **No `#[cfg(feature = ...)]` that works around a Converge trait bound** instead of implementing
  it. A feature gate is for *optional capability* (e.g. a live provider that requires API keys),
  not for making a type conditionally compliant with a trait.
- **Thread context, do not construct it.** A suggestor that calls `CallContext::default()`
  instead of threading the engine-supplied context is discarding tracing state. The context flows
  in; use it.

---

## 7. Production Panic Safety

**The rule:** a library function reachable in production must never panic on reachable inputs.

**The canonical violations:**

```rust
// Panics if the C allocator returns null — rare but possible in constrained environments
Client::builder().build().expect("failed to build client")

// Panics on NaN — NaN is reachable from degenerate embedding vectors
values.sort_by(|a, b| a.partial_cmp(b).unwrap())

// Panics if the regex literal is ever edited to be invalid
static PATTERN: &str = r"(?P<oops"; // compile-time mistake, runtime panic
Regex::new(PATTERN).expect("valid regex")

// Panics if an internal shape assumption is ever violated
array.into_shape((rows, cols)).expect("shape known correct")
```

**The fixes:**

- Constructors that call fallible operations → return `Result<Self, _>`.
- Float sorting → `.total_cmp()`, no `unwrap()`.
- Static regex → `LazyLock<Regex>` initialized once; the `expect` inside `LazyLock::new` is
  acceptable because it runs exactly once at startup, not on every request.
- Shape/index assumptions → use `debug_assert!` + `return Err(...)` on the failure path, not
  `expect`.

**FFI specifically:**

- Every `unsafe` block must have a `// SAFETY:` comment that names the invariant it relies on.
- A non-null pointer returned from a C allocation function must be wrapped in `NonNull<T>` or
  checked before use.
- C integer status codes must be mapped to a Rust enum at the FFI boundary — never passed
  through as raw integers into business logic.
- Non-`try_*` convenience variants that call `expect()` on the `try_*` result are a trap. Mark
  them `#[doc(hidden)]` or remove them; they are only safe in examples and tests.

**Reference: `soter/cvc5-sys/src/lib.rs`** — every `unsafe` block has a SAFETY comment; C status
is mapped to `SmtStatus` immediately at the boundary; `RawSolveResult` is never made public.
This is the model for all sys crates.

---

## 8. Converge Runtime and Kernel Addendum

These checks came out of reviewing Converge itself. Apply them whenever a change touches
`converge-core`, `converge-pack`, `converge-kernel`, `converge-runtime`, or public payload
contracts.

### Logical Time, Not Wall Time

**The smell:** core replay, provenance, facts, proposals, or deterministic tests call
`SystemTime::now()`, `Uuid::new_v4()`, thread RNGs, or process-random hashers.

Converge has its own clock. Core semantics should use the engine/logical clock
(Lamport-style) and pass that timestamp through promotion. Wall-clock time belongs at runtime
boundaries, logs, transport metadata, and operator-facing telemetry, not in replayable semantic
state.

Review questions:

- Does this timestamp affect fact identity, provenance, replay, hashing, or deterministic tests?
  If yes, it must come from the Converge logical clock.
- Does this ID need to be reproducible in tests or replay? If yes, use a deterministic counter or
  engine-supplied identifier, not wall time or random UUIDs.
- Is wall-clock time only used for transport/logging/operator observability? Then it may stay at
  the runtime boundary.

### Stable Hashing and Deterministic Ordering

**The smell:** `DefaultHasher`, map iteration order, `partial_cmp(...).unwrap_or(...)`, or
floating-point sort keys without deterministic tie-breakers.

`DefaultHasher` is intentionally not a stable content fingerprint. It can change across process
starts, compiler versions, and platforms. Use an explicit stable hash such as SHA-256 or BLAKE3
for replay, cache keys, provenance, or content addressing.

For scores and costs:

- use `f32::total_cmp` / `f64::total_cmp`
- add deterministic tie-breakers (`id`, provider name, model name, index) after score comparison
- reject or type-wrap non-finite inputs at the boundary when NaN/inf has no domain meaning

### Honest Optional Features

**The smell:** a feature flag exists, but the heavy dependency is unconditional; or a feature
compiles only in the default path and fails under `--all-features`.

Optional capability means both the module and its dependency graph are optional. A feature like
`metrics`, `telemetry`, `nats`, `gcp`, `wasm`, or `grpc` should own the `dep:*` entries that make
it compile.

Review commands for runtime feature work:

```bash
cargo check -p converge-runtime --no-default-features
cargo check -p converge-runtime
cargo check -p converge-runtime --all-features
just size-audit
```

If `--all-features` fails, do not call the feature healthy. Either fix it, quarantine it, or
document the failure as known drift with a follow-up.

### Blast Radius Is a Review Surface

**The smell:** a small runtime feature drags in Wasmtime/Cranelift, OpenTelemetry, cloud SDKs,
NATS, duplicate transport stacks, or database clients for consumers that did not ask for them.

Review dependency changes with the same seriousness as code changes:

- check `cargo tree -p <crate>` before and after
- measure artifact size with `just size-audit` when runtime/kernel packaging is touched
- remove stale workspace dependencies; a workspace dependency no crate uses is still confusing
  surface area
- keep dev/test-only crates in `[dev-dependencies]`
- never add inline versions in member `Cargo.toml`; use workspace dependencies

### Honest Runtime Failures

**The smell:** an endpoint, stream, or RPC returns success for work it did not actually perform.

Runtime shells must fail honestly:

- HTTP handlers for unsupported paths should return `501 Not Implemented`, not placeholder
  success
- gRPC methods without a backing store should return `Status::unimplemented`
- SSE/control streams should emit an error event or fail startup cleanly, not panic or report
  success
- startup paths should return `Result` errors instead of `expect`

Fake success is worse than a missing feature because it corrupts operator and client state.

### Schema Strictness

Known schemas should reject unknown fields. Use `serde(deny_unknown_fields)` for:

- `FactPayload` structs
- runtime/storage config structs
- wire-facing request/response DTOs where the extension point is not explicitly open

When a schema intentionally has an open extension point, make it obvious with a typed
`metadata`, `extensions`, or `serde_json::Value` field and validate the closed fields around it.

---

## 9. Reference Implementations

These are the patterns to copy, not invent around.

### Money
```rust
// usaspending/src/types.rs
pub total_obligated_micros: i64  // USD; integer micro-dollars avoids float round-trip
```

### Identifier newtype with format validation
```rust
// gleif/src/types.rs
pub struct Lei(String);
impl Lei {
    pub fn parse(s: &str) -> Result<Self, LeiError> { /* mod-97 check */ }
    pub fn as_str(&self) -> &str { &self.0 }
}
impl fmt::Display for Lei { ... }
```

### Semi-open vocabulary
```rust
// sec-edgar/src/types.rs
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FormType {
    Form10K, Form10Q, Form8K, Form4, FormDef14A,
    Other(String),
}
```

### Enum status with is_success
```rust
// ortools-sys/src/lib.rs
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MinCostFlowStatus { Optimal = 0, Feasible = 1, Infeasible = 2, Unbalanced = 3, BadResult = 4, NotSolved = 6 }
impl MinCostFlowStatus {
    pub fn is_success(self) -> bool { matches!(self, Self::Optimal | Self::Feasible) }
}
```

### Membership degree (range-constrained float)
```rust
// Pattern to adopt in prism
pub struct MembershipDegree(f64);
impl MembershipDegree {
    pub fn new(v: f64) -> Self { Self(v.clamp(0.0, 1.0)) }
    pub fn value(self) -> f64 { self.0 }
    pub fn zero() -> Self { Self(0.0) }
    pub fn one() -> Self { Self(1.0) }
}
```

### FFI block with SAFETY comment
```rust
// soter/cvc5-sys/src/lib.rs — the reference for all sys crates
// SAFETY: self.ptr is a live, non-null Cvc5Handle owned by this struct;
// the input slice is valid for the duration of this call.
unsafe { cvc5_assert_formula(self.ptr.as_ptr(), term) };
```

### Static regex (no per-request construction, no runtime panic risk)
```rust
use std::sync::LazyLock;
static SSN_PATTERN: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\b\d{3}-\d{2}-\d{4}\b").expect("SSN regex is valid"));
```

### Integer round-trip for durations at JSON boundaries
```rust
// When Duration does not round-trip cleanly through JSON (FactPayload requirement):
pub struct TimeoutMs(pub u64);  // unit is in the type name, not just the field name
pub const DEFAULT_TIMEOUT_MS: TimeoutMs = TimeoutMs(5_000);
pub const MAX_TIMEOUT_MS: TimeoutMs = TimeoutMs(60_000);
```

---

## 10. Quick Review Checklist

Run through this when reviewing a PR or auditing a crate.

**Types**
- [ ] Any `String`/`&str` field that has a constrained format or vocabulary → newtype or enum
- [ ] Any `f64`/`f32` with a `[0,1]` or `[0,∞)` constraint → newtype with constructor
- [ ] Any two fields of the same primitive type that have different semantics → different newtypes
- [ ] Any `usize` validated `>= 1` → `NonZeroUsize`
- [ ] Any `f64` representing money → `i64` in smallest unit
- [ ] Any `String` representing a path → `PathBuf`
- [ ] Any `String` error type in a public API → `thiserror` enum

**Tests**
- [ ] Does a property test validate a range constraint? → the type should own it
- [ ] Does a property test validate a construction invariant? → the constructor should enforce it
- [ ] After adding a newtype, does an existing property test become trivially true? → delete it

**Duplication**
- [ ] Does this formula / algorithm appear elsewhere in the workspace? → extract to shared location
- [ ] Are there 3+ structurally identical functions differing only in type? → generics or macro
- [ ] Is the same boilerplate in every implementor of a trait? → helper in the trait's crate

**Converge**
- [ ] Any `unwrap()` / `expect()` outside tests? → `Result`, `LazyLock`, `debug_assert`, or `unreachable!`
- [ ] Any `#[allow(dead_code)]`? → wire it in or delete it
- [ ] Any `#[allow(clippy::...)]`? → fix the underlying issue
- [ ] Any `impl Default` on a type with no meaningful default? → remove, require explicit construction
- [ ] `CallContext::default()` inside an `execute()` body? → thread the context from the caller
- [ ] `serde(deny_unknown_fields)` on every `FactPayload`? → add it if missing
- [ ] Core/replay/provenance path uses `SystemTime::now()`, random UUIDs, thread RNG, or
      `DefaultHasher`? → use logical time, deterministic IDs, explicit seeds, or stable hashing
- [ ] Float sorting has no deterministic tie-breaker? → add `total_cmp` plus id/name/index tie-break
- [ ] Runtime endpoint reports success for unsupported work? → return `501` / `Unimplemented`

**Packaging / features**
- [ ] Optional runtime feature owns its heavy `dep:*` dependencies?
- [ ] `cargo check -p converge-runtime --no-default-features` passes?
- [ ] `cargo check -p converge-runtime --all-features` passes or known drift is documented?
- [ ] `just size-audit` was run for runtime/kernel packaging changes?
- [ ] Test-only dependency is in `[dev-dependencies]`?
- [ ] Workspace dependency is still used by at least one crate?

**FFI**
- [ ] Every `unsafe` block has a `// SAFETY:` comment?
- [ ] C status codes mapped to a Rust enum at the boundary?
- [ ] Non-`try_*` panic variants removed or `#[doc(hidden)]`?

**Separation of concerns**
- [ ] Can you describe what this module does in one noun phrase without "and"?
- [ ] Does the suggestor contain business logic? → extract to a domain struct
- [ ] Does this file exceed 500 lines? → candidate for splitting
- [ ] Does a port crate mix wire types, domain types, HTTP logic, and trait implementations in
      one file? → split into `types.rs`, `provider.rs`, `suggestor.rs`, `provenance.rs`

---
tags: [standard, runtime, connections, real-vs-mock, anti-theatre]
source: mixed
date: 2026-05-21
---
# Real-by-Default Connections

REAL connections are the default. Mocks are opt-in. Silent fallback to fake is theatre.

## Why this standard exists

Reflective Labs is racing LLM-only competitors who claim outcomes they cannot defend. We win by shipping outcomes that prove end-to-end — every layer real, every connection live. We lose every time a demo "succeeds" because a hidden fake produced a plausible result. The user should never have to ask *"did you actually call the LLM?", "did the FFI link?", "did the Embassy port hit LinkedIn?"* If the answer to any of those is "no, the fake responded" without explicit opt-in, that is the bug.

When the system proves end-to-end with REAL calls, the user can feel good. Until then, every silent mock is a hidden lie.

## The rule

### 1. Provider, port, and FFI constructors default to REAL

Every Mosaic provider, Embassy port, FFI backend (CVC5, OR-Tools, HiGHS), and Manifold adapter:

- Returns the REAL implementation when no override is specified.
- Uses a **fallible constructor** (`-> Result<Self, Error>`) so missing API key / unreachable host / missing PATH binary surfaces immediately.
- Never silently substitutes a fake when configuration is missing.

If a mock or stub is needed, it must be one of:

- A separate explicit constructor: `::mock()`, `::with_fake_backend()`, `::with_stub_provider()`.
- Selected by an explicit flag in the input config struct: `RuntimeConfig { mock: true, .. }`.
- Behind a non-default feature flag that the consumer chose.

### 2. Compile-time feature flags for native deps remain acceptable

Heavy native deps (CVC5, OR-Tools, HiGHS, LanceDB, SurrealDB, S3, GCS) may sit behind cargo features. The feature gates *what is built*; once built, the runtime behavior defaults to REAL.

Acceptable today: `cvc5`, `ortools`, `highs`, `experience-lancedb`, `vector-lancedb`, `object-s3`, `object-gcs`, `experience-surrealdb`.

What is NOT acceptable: a default-features build that **silently runs a fake** in place of the missing native dep. A default build must produce either a REAL implementation or a loud compile / runtime error naming the missing feature.

### 3. CLI policy — REAL default, `--mock-ok` opt-in

Atelier scenarios and any other Mosaic-aware CLI tool:

- Default invocation uses REAL connections for every provider / port it touches.
- Provides `--mock-ok` (or named-mock flag like `--mock-llm`, `--mock-embassy`) for offline / CI runs.
- Missing API key in REAL mode → exit non-zero, name the env var, name the provider that needed it.
- README documents both modes and when each is appropriate.

### 4. App policy — REAL default, env-var mock opt-in

Marquee apps (Tauri + server runtimes):

- Default startup uses REAL connections.
- Mock mode is opt-in via env var (recommended pattern: `<APP>_MOCK=1` for app-scoped, `MOSAIC_MOCK=1` for workspace-wide) or explicit user-visible config setting.
- **The app logs its mode on startup.** No silent mode.
- Missing API key in REAL mode is a fast-fail with a clear error message naming the missing variable; the app does not boot in degraded "everything works but it's all fake" state.

### 5. Test policy

- **Unit tests:** mocks are fine. That is what unit tests are for.
- **Integration tests:** REAL is required. An "integration test" that hits a fake backend is a unit test in disguise.
- **Atelier scenarios:** REAL by default; `--mock-ok` only for explicitly offline cases (e.g. CI runners with no outbound network).

### 6. Loud failure beats silent fallback

When a REAL connection cannot be made:

- Name the **exact missing thing**: env var name, PATH binary, network destination, file path.
- Error at the **boundary** (the constructor), not three layers up where the error becomes opaque.
- **Do not** retry-into-fake.
- **Do not** log-and-continue with a mocked response.
- Application error messages should suggest the fix: "set `OPENAI_API_KEY` in your env" / "install `cvc5` and ensure it is on `PATH`" / "build with `--features cvc5`."

## Anti-patterns

- Constructor returns `Self` (infallible) and silently uses a stub when the env var is missing.
- Default cargo feature set chooses a fake backend for a native dep that wasn't enabled.
- "Test mode" that's auto-detected from `cargo test` and silently flips to mock at runtime.
- Atelier scenario that runs offline by default and hides a `MOCK_LLM=1` in `.env`.
- App that boots in mock mode unless told otherwise.
- Catch-log-continue patterns that mask missing API keys.
- A scenario README that claims "end-to-end" with a `FakeSmtBackend` in the default build path.

## Audit obligation

When you touch a provider / port / FFI / scenario / app, verify it matches this standard. If it doesn't, fix it in the same session — do not file a "should be REAL by default" issue. (See the no-delegation-ping-pong rule.)

## Current known violations (rolling list)

The first-pass audit found these. Each entry must be **fixed-or-deleted**, not preserved. When a violation closes, remove the entry and note the fix commit in `LOG.md`.

- **`soter-smt`** — `default = []` in `crates/soter/Cargo.toml`. Default-features builds do *not* link CVC5 and therefore `Cvc5FfiBackend` is unavailable; `FakeSmtBackend` is re-exported in the default build (`lib.rs:19 pub use backend::{FakeSmtBackend, SmtBackend}`). A consumer can build with default features and silently call `FakeSmtBackend::unsat().solve(...)` getting fake `unsat` results. Two acceptable fixes: (a) make `default = ["cvc5"]` so default builds require CVC5 native; or (b) gate `FakeSmtBackend` re-export behind a non-default `fake-backend` feature so default builds have no silent-fake foot-gun. (b) is the less-invasive fix.
- **`manifold-adapters` LLM ctors** — `OpenAiBackend::new(api_key: impl Into<String>) -> Self` is infallible and accepts any string including empty / whitespace (`crates/manifold/src/llm/openai.rs:32`). Need a per-backend audit confirming a fallible `from_env()` (or equivalent) is the recommended ctor and that empty / placeholder keys produce a loud error before the first request. Same audit applies to the other six LLM backends (gemini, mistral, openrouter, staik, kong, anthropic, plus arcee / minmax / writer).
- **Embassy ports — stub vs concrete defaults** — every Embassy crate exposes both a stub provider and a concrete provider. Need a per-port audit confirming: (a) the concrete provider is the documented default, (b) the stub is opt-in via an explicit constructor or feature, (c) the concrete ctor is fallible and surfaces missing-credential / unreachable-host errors at construction. Starting candidates for audit: `ofac-sls`, `gleif`, `linkedin`, `sec-edgar`, `bolagsverket`.

When fixing a violation, also confirm: any atelier scenario or marquee app that consumes the affected crate is not relying on the silent-fake path. If it is, surface that as an in-session fix.

## Cross-references

- `~/dev/reflective/stack/atelier-showcase/kb/Home.md` — scenarios must default to REAL; this Standard is binding on every v1.1.0 scenario.
- `~/dev/reflective/marquee-apps/kb/Home.md` — apps must default to REAL; mock is env-var-opt-in.
- `~/dev/reflective/marquee-apps/kb/mosaic-pull-matrix.md` — a pull does not count as "actively pulled" if the call lands on a fake under the default config.
- `~/dev/reflective/marquee-apps/shoal-meta/kb/portfolio-stretch.md` — the pull-rate metric excludes calls that resolve to fakes.

See also: [[Backend String Format]], [[Runtime Config Encoding]], [[Extension Standard]], [[Suggestor Contract]], [[Review Guide]].

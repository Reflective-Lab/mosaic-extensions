---
tags: [module, providers, adapters, storage]
source: mixed
date: 2026-05-05
---
# Manifold

`manifold` owns generic adapter implementations for Converge contracts.

Use it when the vendor should be hidden behind an interchangeable capability or
storage contract.

## Owns

- Object-store adapter builders.
- Experience-store adapters.
- Vector-store adapters.
- Future generic provider, fetch, feed, search, LLM, embedding, and tool
  adapters.

## Current Surface

| Module | Role |
|---|---|
| `object_storage` | Builds `Arc<dyn ObjectStore>` from `StorageConfig` for local, S3, or GCS backends. |
| `experience` | SurrealDB and LanceDB experience-store implementations. |
| `vector` | LanceDB-backed vector recall adapter. |
| `fetch` | `HttpFetchProvider` — fallible constructor (`::new() -> Result`), generic HTTP fetch. POST is supported via `WebFetchMethod::Post` and `WebFetchRequest::with_body` for SOAP envelopes and JSON-API POST endpoints. |
| `xml` (feature) | `extract_first_text` / `extract_all_texts` over quick-xml — tag-shaped, namespace-prefix-stripping element-text grabs. Right size for SOAP responses and simple XML feeds; not for schema-bound deserialization. |
| `feed` | `HttpFeedProvider` — fallible constructor, streaming feed retrieval. |
| `llm` | Chat LLM adapters: openai, gemini, mistral, openrouter, staik, kong, anthropic. All share `llm::retry::retry_with_backoff` — do not inline backoff loops. |
| `llm::retry` | `retry_with_backoff(max_retries, closure) -> Result` — 100ms × 2^attempt backoff, `RetryOutcome::Retry`/`Fail`/`Success`. Single shared implementation for all LLM backends. |
| `reranker` | `QwenVLReranker` — fallible constructor. |
| `contract` | `canonical_hash` — SHA-256 content fingerprint (first 8 bytes, 16-hex output). Stable across toolchain versions. |

## Feature Flags

- Default: `object-local`.
- `object-s3` and `object-gcs` enable cloud object storage.
- `object-all` enables local, S3, and GCS.
- `experience-surrealdb` enables the SurrealDB experience store.
- `experience-lancedb` enables the LanceDB experience store.
- `vector-lancedb` enables the LanceDB vector recall adapter.
- `all-storage` enables all storage-related adapters.

## Boundary

If the caller can ask for a capability without naming the vendor, it belongs
here. If the source identity is part of the type system, use [[Modules/Embassy]].

## Entry Points

- `manifold-adapters/crates/manifold/src/lib.rs`
- `manifold-adapters/crates/manifold/src/object_storage/mod.rs`
- `manifold-adapters/crates/manifold/src/experience/mod.rs`
- `manifold-adapters/crates/manifold/src/vector/mod.rs`

See also: [[Architecture/Port Provider Boundary]]

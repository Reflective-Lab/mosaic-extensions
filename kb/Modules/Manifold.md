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
| `object_storage` | Builds `Arc<dyn ObjectStore>` from `StorageConfig` for local, S3, or GCS backends depending on enabled features. |
| `experience` | SurrealDB and LanceDB implementations of Converge experience-store contracts. |
| `vector` | LanceDB-backed vector recall adapter and re-exported vector capability types. |

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

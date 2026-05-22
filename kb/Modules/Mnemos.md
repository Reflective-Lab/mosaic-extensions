---
tags: [module, knowledge, memory]
source: mixed
date: 2026-05-05
---
# Mnemos

`mnemos` owns knowledge, recall, retrieval, storage, and agentic memory as a
Converge extension.

It can be used as a library, CLI, gRPC server, or Converge suggestor family.

## Owns

- `KnowledgeBase`, entries, search options, and search results.
- Vector-backed retrieval, BM25 lexical recall, RRF hybrid ranking, and storage
  backend.
- Embedding support, including OpenAI embeddings.
- Markdown and rich-media ingestion.
- Agentic memory: causal, temporal, reflexion, skills, online learning, and
  session memory.
- Feedback, replay, batch learning, and insight jobs.
- gRPC server/client surfaces.
- `KnowledgeRetrievalSuggestor` and `KnowledgeStoreSuggestor`.
- Typed Mnemos proposal provenance at the `ProposedFact` boundary.
- `mnemos.suggestor.execute` tracing spans at knowledge suggestor boundaries.

## Feature Flags

- Default: `cli` and `grpc`.
- `cli` enables the `mnemos` binary.
- `grpc` enables the `mnemos-server` binary.
- `memory-only` is available for in-memory operation.

## Boundary

Converge owns the shared proposal and fact contract. Mnemos owns knowledge
storage and recall mechanisms that can participate in the convergence loop.

Do not move every knowledge store into Converge. Keep the foundation contract
small and let products choose whether recall is embedded or remote.

## Shared Math

`crates/mnemos/src/math.rs` — canonical `pub(crate) fn cosine_similarity(a: &[f32], b: &[f32]) -> f32`. This is the single cosine implementation; knowledge_base, batch, embedding, and meta all delegate here. Do not add another.

## Retrieval

Hybrid retrieval is opt-in through `SearchOptions::hybrid(...)` and the gRPC
`SearchRequest.hybrid` field. Mnemos ranks vector similarity and BM25 lexical
matches separately, then merges ranks with Reciprocal Rank Fusion:
`sum(1 / (60 + rank))`, using 1-based ranks. This keeps acronym, exact-term,
and rare-keyword recall separate from semantic similarity without calibrating
raw BM25 scores against vector similarities.

Category and tag filters are applied before ranking. Retrieval remains recall
only; Converge still owns proposal promotion.

## Entry Points

- `mnemos-knowledge/README.md`
- `mnemos-knowledge/crates/mnemos/src/lib.rs`
- `mnemos-knowledge/crates/mnemos/src/math.rs`
- `mnemos-knowledge/crates/mnemos/src/core/knowledge_base.rs`
- `mnemos-knowledge/crates/mnemos/src/suggestor.rs`
- `mnemos-knowledge/crates/mnemos/src/ingest/`
- `mnemos-knowledge/crates/mnemos/src/agentic/`
- `mnemos-knowledge/crates/mnemos/proto/knowledge.proto`

See also: [[Architecture/Runtime Assembly]]

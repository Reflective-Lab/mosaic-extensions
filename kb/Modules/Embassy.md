---
tags: [module, connectors]
source: mixed
date: 2026-05-05
---
# Embassy

`embassy` owns source-specific connector ports.

Use it when an external system's identity is part of the semantic contract and
cannot be hidden behind a generic provider capability.

## Owns

- Cross-port call context and provenance helpers.
- Source-specific request/response DTOs.
- Source-shaped provider traits.
- Stub or concrete providers for those source-shaped traits.

## Current Crates

| Crate | Role |
|---|---|
| `embassy-pack` | Shared `CallContext`, `Observation<T>`, and `content_hash` helper. |
| `embassy-linkedin` | `LinkedInProvider`, `LinkedInGetRequest`, `LinkedInProfile`, `LinkedInGetResponse`, and `StubLinkedInProvider`. |

## Boundary

LinkedIn belongs here because `LinkedInProfile` is not a generic retrieval
record. The foreign service identity carries business meaning, provenance,
terms, and compliance constraints.

Generic web fetch, search, feed retrieval, LLM chat, and embedding providers
belong in [[Modules/Manifold]] unless their source identity is the core meaning
of the API.

## Entry Points

- `embassy-ports/README.md`
- `embassy-ports/crates/pack/src/lib.rs`
- `embassy-ports/crates/linkedin/src/lib.rs`

See also: [[Architecture/Port Provider Boundary]]

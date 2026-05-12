---
tags: [architecture, connectors, providers]
source: mixed
date: 2026-05-05
---
# Port Provider Boundary

Use this page when deciding whether code belongs in [[Modules/Embassy]] or
[[Modules/Manifold]].

## Embassy

Use Embassy when the external party is semantically part of the API.

Examples:

- LinkedIn profile lookup.
- Professional-network company or person enrichment.
- A source-specific business connector whose request/response shape carries
  source terms, identity, compliance constraints, provenance, or rate-limit
  semantics.

The contract may say names like `LinkedInProfile`, `StripeCustomer`, or
`PatentFamily` because those names are part of the meaning.

## Manifold

Use Manifold when the provider should be interchangeable behind a generic
capability.

Examples:

- Object storage.
- Vector recall.
- Web fetch.
- Web search.
- Feed retrieval.
- LLM chat.
- Embedding generation.

The contract should say what the capability does, not which vendor backs it.

## Decision Checks

| Question | If yes | If no |
|---|---|---|
| Must callers name the foreign system to understand the data? | Embassy | Manifold |
| Could two vendors plausibly stand in for each other? | Manifold | Embassy |
| Does the request/response type carry source-specific legal or provenance semantics? | Embassy | Manifold |
| Is this a generic operational backend selected by requirements? | Manifold | Embassy or product layer |

See also: [[Architecture/Extension Topology]], [[Modules/Embassy]], [[Modules/Manifold]]

---
tags: [module, connectors]
source: mixed
date: 2026-05-05
---
# Embassy

`embassy` owns source-specific connector ports.

Use it when an external system's identity is part of the semantic contract and
cannot be hidden behind a generic provider capability.

Embassy observes named sources. It does not own operating authority for
Reflective Labs, customers, partners, or product domains.

## Owns

- Cross-port call context and provenance helpers.
- Source-specific request/response DTOs.
- Source-shaped provider traits.
- Stub or concrete providers for those source-shaped traits.

## Does Not Own

- Reflective billing, subscriptions, entitlements, partner payouts, revenue
  share, refunds, disputes, or marketplace terms.
- Customer CRM, accounting, HR, support, signing, identity, payroll, commerce,
  or ERP writeback policy.
- Domain workflows, Truth catalogs, projections, irreversible approval
  placement, or business command contracts.
- Credentials, tenant secret storage, runtime deployment, or product topology.

## Current Crates

| Crate | Role |
|---|---|
| `embassy-pack` | Shared `CallContext`, `Observation<T>`, and `content_hash` helper. |
| `embassy-linkedin` | `LinkedInProvider`, `LinkedInGetRequest`, `LinkedInProfile`, `LinkedInGetResponse`, and `StubLinkedInProvider`. |

## Boundary

LinkedIn belongs here because `LinkedInProfile` is not a generic retrieval
record. The foreign service identity carries evidence meaning, provenance,
terms, and compliance constraints.

Generic web fetch, search, feed retrieval, LLM chat, and embedding providers
belong in [[Modules/Manifold]] unless their source identity is the core meaning
of the API.

If a connector can act with business authority, the owning business layer must
own the command surface. Embassy may provide source-faithful observations that
inform the action, but it must not own the action itself.

## Entry Points

- `embassy-ports/README.md`
- `embassy-ports/crates/pack/src/lib.rs`
- `embassy-ports/crates/linkedin/src/lib.rs`

See also: [[Architecture/Port Provider Boundary]]

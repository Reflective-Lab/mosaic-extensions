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

### Shared infrastructure

| Crate | Provides |
|---|---|
| `embassy-pack` | `CallContext`, `Observation<T>`, `content_hash`; `SanctionsSubject`, `SanctionsHit`, `MatchType`, `SubjectType` (shared across sanctions ports); `simple_id!` macro for identifier newtypes. |

### P0 ports — fully shaped, live on crates.io

| Crate | Source |
|---|---|
| `embassy-linkedin` | LinkedIn professional profile lookup. |
| `embassy-sec-edgar` | SEC EDGAR company filings (US public companies). |
| `embassy-bolagsverket` | Bolagsverket Swedish company registry. |
| `embassy-gleif` | GLEIF LEI entity data (legal entity identifiers). |
| `embassy-vies` | EU VAT Information Exchange System (VAT number validation). |
| `embassy-ofac-sls` | US Treasury OFAC Specially Designated Nationals sanctions list. |
| `embassy-eu-sanctions` | EU Consolidated Sanctions List. |
| `embassy-commerce-csl` | US Commerce BIS Denied Parties / Entity List. |
| `embassy-sam-gov` | SAM.gov federal contractor and grantee registry. |
| `embassy-usaspending` | USASpending.gov federal spend data. |
| `embassy-ted` | TED EU public procurement notices. |
| `embassy-skatteverket` | Skatteverket Swedish tax public query surface. |

### Skeleton ports — typed identifier + stub provider

Expand each when an app pulls on the domain. Do not grow the entity shape preemptively.

| Crate | Identifier type |
|---|---|
| `embassy-uspto` | `PatentNumber` |
| `embassy-crunchbase` | `OrganizationId` |
| `embassy-github` | `OrgSlug` |
| `embassy-pubmed` | `Pmid` |
| `embassy-arxiv` | `ArxivId` |
| `embassy-openalex` | `OpenAlexId` |
| `embassy-wikidata` | `QId` |
| `embassy-companies-house` | `CompanyNumber` |
| `embassy-scb` | `TableId` |
| `embassy-epo` | `EpoNumber` |

All skeleton identifier types are generated with the `embassy_pack::simple_id!` macro and include format validation. Use `parse()` at the boundary; do not accept raw strings into port logic.

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

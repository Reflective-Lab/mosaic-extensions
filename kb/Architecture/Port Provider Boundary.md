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
- A source-specific evidence connector whose request/response shape carries
  source terms, identity, compliance constraints, provenance, or rate-limit
  semantics.

The contract may say names like `LinkedInProfile`, `SecCompanyFiling`, or
`PatentFamily` because those names are part of the meaning.

Embassy is evidence-oriented. It can report what a named source said, with
provenance and source-specific semantics. It must not become the owner of
operating business authority.

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

## Operating Authority

Use the owning business layer, not Mosaic, when an integration changes business
state or creates commercial, legal, customer, employee, partner, or financial
consequence.

Examples:

- Reflective Labs billing, subscriptions, marketplace terms, entitlements,
  partner payouts, revenue sharing, refunds, and disputes.
- Customer CRM, ERP, accounting, HR, support, signing, identity, or commerce
  writeback.
- Escrow release, payroll mutation, contract execution, access grant, or any
  irreversible action that needs approval and audit.

These surfaces require command contracts, idempotency, replay protection,
ledger or ledger-like records, tenant scoping, policy gates, and audit events.
They belong in Reflective Commerce Rails, a customer app or engagement, or a
product domain such as Tally. Mosaic specialists can supply evidence, policy,
providers, memory, analytics, and solvers, but Mosaic does not own the business
consequence.

## Decision Checks

| Question | If yes | If no |
|---|---|---|
| Must callers name the foreign system to understand the data? | Embassy | Manifold |
| Could two vendors plausibly stand in for each other? | Manifold | Embassy |
| Does the request/response type carry source-specific legal or provenance semantics? | Embassy | Manifold |
| Is this a generic operational backend selected by requirements? | Manifold | Embassy or product layer |
| Does this action change business state or create operating authority? | Product, customer, or Reflective business layer | Embassy or Manifold |

See also: [[Architecture/Extension Topology]], [[Modules/Embassy]], [[Modules/Manifold]]

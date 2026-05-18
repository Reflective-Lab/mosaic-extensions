---
tags: [architecture, runtime]
source: mixed
date: 2026-05-05
---
# Runtime Assembly

Extensions are reusable parts. They are not the product runtime.

## Who Assembles

Products, demos, Runway, or deployment repositories assemble extensions into a
running system. They own:

- which extensions are enabled,
- which providers are selected,
- secrets and credentials,
- Docker and process topology,
- cloud resources,
- policy bundles,
- product-specific routing,
- environment-specific defaults.

Business operating rails also live above extensions. Reflective-owned commerce
systems, customer-owned writeback surfaces, and product-domain command
contracts assemble Mosaic specialists but do not live in Mosaic.

## What Extensions Expose

Extensions should expose clean library surfaces:

- suggestors that can be registered in a Converge engine,
- providers or adapters that implement a known contract,
- feature flags for heavy or optional backends,
- request/response types with clear provenance and validation.

Suggestors must stay advisory: they read declared context, emit proposals, and
leave promotion to Converge. See [[Standards/Suggestor Contract]] for the
extension-side execution contract.

Connectors must also stay below operating authority. An Embassy port can say
what SEC EDGAR, Bolagsverket, or LinkedIn reported. It does not decide to
charge a customer, grant an entitlement, advance an opportunity, release a
payout, or write back into a customer system. Those decisions belong in the
business layer that bears the consequence.

## Current Examples

- `ferrox-server` is a service wrapper for solver capabilities, but the product
  still chooses whether to run it or embed `ferrox-solver`.
- `mnemos` exposes a CLI and gRPC server, but products decide whether recall is
  embedded, remote, or disabled.
- `manifold` builds concrete storage and vector adapters from config, but
  deployment code decides the storage URI and credentials.
- `soter` exposes SMT query/report types and solver-backed suggestors, but
  products decide which invariants need SMT evidence and when native CVC5 runs
  in CI.
- `integration-harness/` is the container's product-side executable assembly
  check. It wires Arbiter, Prism, and Mnemos through public APIs without making
  any extension depend on another extension.

See also: [[Architecture/Converge Boundary]], [[Building/Getting Started]],
[[Standards/Suggestor Contract]], [[Architecture/Golden Integration Harness]]

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

## What Extensions Expose

Extensions should expose clean library surfaces:

- suggestors that can be registered in a Converge engine,
- providers or adapters that implement a known contract,
- feature flags for heavy or optional backends,
- request/response types with clear provenance and validation.

## Current Examples

- `ferrox-server` is a service wrapper for solver capabilities, but the product
  still chooses whether to run it or embed `ferrox-solver`.
- `mnemos` exposes a CLI and gRPC server, but products decide whether recall is
  embedded, remote, or disabled.
- `manifold` builds concrete storage and vector adapters from config, but
  deployment code decides the storage URI and credentials.

See also: [[Architecture/Converge Boundary]], [[Building/Getting Started]]

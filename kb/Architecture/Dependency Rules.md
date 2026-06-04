---
tags: [architecture, dependencies]
source: mixed
date: 2026-05-05
---
# Dependency Rules

Dependencies flow toward the foundation:

```text
products / deployments -> extensions -> converge contracts
```

The reverse edge is forbidden. Converge must not import `arbiter`, `embassy`,
`ferrox`, `manifold`, `mnemos`, or `prism`.

## Rules

1. Keep extension crates outside the Converge repository.
2. Prefer canonical public Converge crates for normal extension development.
3. Treat direct `converge-core` and other internal-crate dependencies as
   transitional extraction debt unless the boundary doc explicitly says they
   are allowed.
4. Keep native SDKs, vendor clients, solver bindings, database drivers, and
   optional adapters out of the Converge foundation.
5. Promote only reusable contracts upstream. Keep concrete implementations
   downstream.
6. Use Cargo feature flags for heavy adapters and native bindings.
7. Do not make one extension silently become the runtime assembler for another.
   Products or Runtime Runway own assembly.

## Current Transitional Edges

Some extracted repositories still reference unreleased or internal Converge
crates through path patches:

- `arbiter` patches `converge-core` and `converge-pack`.
- `ferrox` path-depends on `converge-pack`, `converge-model`,
  `converge-core`, and `converge-provider`.
- `manifold` patches `converge-core`, `converge-experience`, and
  `converge-storage`.
- `mnemos` patches `converge-pack`.
- `prism` patches Converge platform crates and uses `converge-pack` as its
  primary contract.

The goal is not to pretend those edges do not exist. The goal is to keep them
visible, reduce them as public contracts stabilize, and avoid adding new
internal edges casually.

See also: [[Architecture/Converge Boundary]], [[Architecture/Repository Map]]

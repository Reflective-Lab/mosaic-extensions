---
tags: [architecture, crates]
source: mixed
date: 2026-05-05
---
# Repository Map

The extensions folder is a multi-repo container, not a single Cargo workspace.
Run Cargo commands from the individual extension directory.

## Repositories

| Repository | Workspace Members | Primary Surface | Notes |
|---|---|---|---|
| `arbiter-policy/` | `crates/arbiter` | Cedar policy engine and suggestors | Extracted from `converge/crates/policy`; branch `main`; local git repo. |
| `atelier-showcase/` | `crates/atelier-domain`, `crates/example-*`, `crates/organism-domain` | Worked examples and showcase material | Local checkout for the `Reflective-Lab/atelier-showcase` repo. |
| `embassy-ports/` | `crates/pack`, `crates/linkedin` | Connector contracts and source-specific ports | Extracted connector-port repo. |
| `ferrox-solvers/` | `crates/ferrox`, `crates/ferrox-server`, `crates/ortools-sys`, `crates/highs-sys` | Solver suggestors and gRPC service | Existing public repo with examples and native bindings. |
| `manifold-adapters/` | `crates/manifold` | Generic storage, vector, and experience adapters | Generic adapter home for storage, vector, provider, and tool surfaces. |
| `mnemos-knowledge/` | `crates/mnemos` | Knowledge base library, CLI, server, and suggestors | Extracted from `converge/crates/knowledge`; branch `main`; local git repo. |
| `prism-analytics/` | `crates/prism` | Analytics packs and ML suggestors | Extracted from `converge/crates/analytics`; branch `main`; local git repo. |

## Toolchain Baseline

- Rust edition: 2024.
- Rust version floor: 1.94.0.
- Lints: `unsafe_code = "forbid"` and clippy pedantic with workspace-level exceptions.
- Dependency versions live at each repository workspace root and are referenced
  with `workspace = true` inside member crates.

## Local Path Assumption

Converge platform dependencies resolve from crates.io. Local Converge path
patches are not part of the default repo shape.

`atelier-showcase` keeps an Organism development override that resolves from
`/Users/kpernyer/dev/extensions/atelier-showcase` to
`/Users/kpernyer/dev/work/organism`.

See also: [[Building/Getting Started]], [[Building/Release and Versioning]]

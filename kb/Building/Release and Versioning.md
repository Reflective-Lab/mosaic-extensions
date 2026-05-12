---
tags: [building, release]
source: mixed
date: 2026-05-06
---
# Release and Versioning

Each extension repository versions independently. The parent `extensions`
folder does not define a shared release.

## Planned Extension Versions

All extension release lines target Converge `3.8.1`.

| Repository | Workspace Version | Converge Baseline | Cargo Packages |
|---|---:|---|---|
| `arbiter-policy/` | `1.0.0` | `3.8.1` | `converge-arbiter-policy` |
| `atelier-showcase/` | `1.0.0` | `3.8.1` | `converge-atelier-domain`, `organism-domain`, example packages |
| `embassy-ports/` | `1.0.0` | `3.8.1` | `converge-embassy-pack`, `converge-embassy-linkedin` |
| `ferrox-solvers/` | `0.4.1` | `3.8.1` | `converge-ferrox-solver`, `converge-ferrox-server`, `converge-ferrox-ortools-sys`, `converge-ferrox-highs-sys` |
| `manifold-adapters/` | `1.0.0` | `3.8.1` | `converge-manifold-adapters` |
| `mnemos-knowledge/` | `1.0.0` | `3.8.1` | `converge-mnemos-knowledge` |
| `prism-analytics/` | `1.0.0` | `3.8.1` | `converge-prism-analytics` |

## Converge Version Baseline

The extracted repositories target Converge `3.8.1` crates from crates.io.
Release checks that resolve Converge dependencies require the foundation crates
to be published at that version.

## Release Checklist

1. Confirm the extension has a README, license, changelog, and security docs
   when it is intended for publication.
2. Run `cargo fmt --check`.
3. Run `cargo clippy --all-targets --all-features -- -D warnings` where native
   dependencies permit it.
4. Run `cargo test --all-targets --all-features` where native dependencies
   permit it.
5. Verify local path patches do not leak into a publishable crate
   unintentionally.
6. Tag releases in the individual repository, not the parent folder.

See also: [[Architecture/Extraction Status]]

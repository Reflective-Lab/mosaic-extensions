---
tags: [building, release]
source: mixed
date: 2026-05-06
---
# Release and Versioning

Each extension repository versions independently. The parent `extensions`
folder does not define a shared release.

## Planned Extension Versions

Current local extension lines target Converge `3.9.1`.

| Repository | Workspace Version | Converge Baseline | Cargo Packages |
|---|---:|---|---|
| `arbiter-policy/` | `2.0.1` | `3.9.1` | `converge-arbiter-policy` |
| `crucible-models/` | `0.3.0` | `3.9.1` | `converge-crucible-models` |
| `embassy-ports/` | `1.3.0` | `3.9.1` | `converge-embassy-pack`, source-specific `converge-embassy-*` ports |
| `ferrox-solvers/` | `0.7.1` | `3.9.1` | `converge-ferrox-solver`, `converge-ferrox-server`, `converge-ferrox-ortools-sys`, `converge-ferrox-highs-sys` |
| `manifold-adapters/` | `1.1.1` | `3.9.1` | `converge-manifold-adapters` |
| `mnemos-knowledge/` | `1.2.2` | `3.9.1` | `converge-mnemos-knowledge` |
| `prism-analytics/` | `2.0.1` | `3.9.1` | `converge-prism-analytics`, `converge-fuzzy-inference` |
| `soter-smt/` | `0.2.2` | `3.9.1` | `converge-soter-smt`, `converge-soter-cvc5-sys` |

## Converge Version Baseline

The extracted repositories target Converge `3.9.1` contracts locally. Release
checks that resolve Converge dependencies from crates.io require the foundation
crates to be published at the matching version or for local patches to be
removed before packaging.

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

---
tags: [planning]
source: mixed
date: 2026-05-06
---
# Next Steps

Open follow-up work for the extensions workspace after the extraction.

## Repository Hygiene

- Decide whether `embassy` and `manifold` are standalone repos, then initialize
  and configure remotes if yes.
- Add or confirm remotes for `arbiter`, `mnemos`, and `prism`.
- Decide whether the parent `extensions` folder should remain only a local
  container or become a meta repo.
- Add CI to extracted repos that will be pushed.

## Version and Dependency Cleanup

- Keep all extension release lines pinned to the Converge `3.8.1` contract
  baseline.
- Reduce direct `converge-core` dependencies where public contracts now exist.
- Remove local path patches when the target Converge crates are published.
- Document any intentional internal Converge dependency in the relevant module
  page.

## Capability Extraction

- Move the next source-specific connector into `embassy` when its source
  identity is part of the contract.
- Move generic provider/tool adapters into `manifold` when they are swappable
  behind capability requirements.
- Keep product-specific orchestration out of extension crates.

## Validation

- Run `cargo check` for each extension.
- Run feature-specific checks for `ferrox`, `manifold`, `mnemos`, and `prism`.
- Add compile-fail or integration tests when a boundary claim is meant to be
  enforced by Rust types.

See also: [[Architecture/Extraction Status]]

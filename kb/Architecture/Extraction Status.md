---
tags: [architecture, extraction, status]
source: mixed
date: 2026-05-05
---
# Extraction Status

Observed during KB creation on 2026-05-05.

## Current State

| Extension | State |
|---|---|
| `arbiter` | Extracted policy functionality from Converge into a standalone local git repo. Working tree was clean. No remote was reported by `git remote -v`. |
| `embassy` | New scaffold for source-specific connector ports. Contains `embassy-pack` and `embassy-linkedin`. No local `.git` directory was observed. |
| `ferrox` | Existing solver extension repo with GitHub remote `Reflective-Lab/ferrox-solvers`. Working tree had a local `Cargo.toml` modification at inspection time. |
| `manifold` | New scaffold for generic adapters. Contains the `manifold` crate and storage/vector/experience modules. No local `.git` directory was observed. README and GitHub docs were added on 2026-05-05. |
| `mnemos` | Extracted knowledge functionality from Converge into a standalone local git repo. Working tree was clean. No remote was reported by `git remote -v`. |
| `prism` | Extracted analytics functionality from Converge into a standalone local git repo. Working tree had local manifest edits at inspection time. No remote was reported by `git remote -v`. |

## Known Follow-ups

- Decide whether `embassy` and `manifold` should be initialized as git repos or
  folded into another repository before first commit.
- Reconcile Converge version floors across the extracted repos.
- Reduce or document remaining direct internal Converge dependencies.
- Run `cargo check` per extension after extraction cleanup.
- Add CI and remotes for extracted repos that are meant to be published.

See also: [[Planning/Next Steps]], [[Building/Release and Versioning]]

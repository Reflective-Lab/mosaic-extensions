---
tags: [workflow, git]
source: mixed
date: 2026-05-05
---
# Git Strategy

There is no single git repository at `/Users/kpernyer/dev/reflective/stack/mosaic-extensions`.
Treat each extension directory as its own repository or scaffold.

## Rules

1. Do not assume a root `git status` represents all extensions.
2. Run `git -C <extension-dir> status --short --branch` before editing a repo.
3. Keep topic branches scoped to one extension unless the change is a deliberate
   coordinated release.
4. Do not mix extraction cleanup, feature work, and version bumping in one
   branch.
5. Do not initialize or publish a scaffold without deciding repository ownership
   and remote naming first.

## Observed Repos

At KB creation time:

- `arbiter`, `ferrox`, `mnemos`, and `prism` had local `.git` directories.
- `ferrox` had a GitHub remote at `git@github.com:Reflective-Lab/ferrox-solvers.git`.
- `embassy` and `manifold` did not have local `.git` directories.

## Multi-Repo Changes

For a coordinated extraction cleanup:

1. Create a branch in each affected repo.
2. Keep commits independently reviewable.
3. Run checks per repo.
4. Record the cross-repo rationale in this KB or the relevant issue.

See also: [[Architecture/Extraction Status]]

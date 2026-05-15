---
tags: [building]
source: mixed
date: 2026-05-05
---
# Getting Started

Run commands from an individual extension repository. The parent
`/Users/kpernyer/dev/extensions` folder is not a root Cargo workspace.

## Prerequisites

- Rust 1.94.0 or newer.
- Local Converge checkout at `/Users/kpernyer/dev/work/converge`.
- Native build prerequisites for Ferrox when enabling OR-Tools or HiGHS.

## Basic Checks

```bash
cd /Users/kpernyer/dev/extensions/arbiter-policy
cargo check

cd /Users/kpernyer/dev/extensions/crucible-models
cargo check

cd /Users/kpernyer/dev/extensions/embassy-ports
cargo check

cd /Users/kpernyer/dev/extensions/ferrox-solvers
cargo check

cd /Users/kpernyer/dev/extensions/manifold-adapters
cargo check

cd /Users/kpernyer/dev/extensions/mnemos-knowledge
cargo check

cd /Users/kpernyer/dev/extensions/prism-analytics
cargo check

cd /Users/kpernyer/dev/extensions/soter-smt
cargo check
```

To check all current extension workspaces:

```bash
for repo in arbiter-policy crucible-models embassy-ports ferrox-solvers manifold-adapters mnemos-knowledge prism-analytics soter-smt; do
  (cd "/Users/kpernyer/dev/extensions/$repo" && cargo check)
done
```

## Feature-Specific Checks

```bash
cd /Users/kpernyer/dev/extensions/ferrox-solvers
cargo check -p ferrox-solver --features full

cd /Users/kpernyer/dev/extensions/manifold-adapters
cargo check -p manifold --features all-storage

cd /Users/kpernyer/dev/extensions/prism-analytics
cargo check -p prism --features excel

cd /Users/kpernyer/dev/extensions/soter-smt
cargo check -p soter --features cvc5

cd /Users/kpernyer/dev/extensions/mnemos-knowledge
cargo check -p mnemos --no-default-features --features memory-only
```

## Dependency Sources

Extension repos depend on published Converge crates from crates.io. Keep local
`[patch.crates-io]` overrides out of the repos unless a task explicitly
requires testing unpublished foundation changes.

## First Pages to Read

1. [[Architecture/Extension Topology]]
2. [[Architecture/Repository Map]]
3. The module page for the repo you are changing
4. [[Architecture/Dependency Rules]]

See also: [[Workflow/Daily Journey]]

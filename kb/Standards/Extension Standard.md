---
tags: [standard, extensions]
source: mixed
date: 2026-05-05
---
# Extension Standard

Use this as the minimum bar for a reusable Converge extension repository.

## Required Shape

- `README.md` with purpose, boundary, layout, status, build command, and
  license.
- `AGENTS.md` as the agent entrypoint for local rules, commands, and boundaries.
- `Justfile` as the local command surface for build, check, test, format,
  clippy, docs, and status.
- `Cargo.toml` workspace root with edition, rust-version, lints, dependency
  versions, and feature policy.
- Member crates under `crates/`.
- `LICENSE`, `CHANGELOG.md`, `CONTRIBUTING.md`, `SECURITY.md`, and
  `CODE_OF_CONDUCT.md` for publishable repos.
- `unsafe_code = "forbid"`.
- Rust edition 2024 and rust-version 1.94.0 unless there is a documented reason
  to differ.

## Boundary Statement

Every extension README should answer:

- What does this extension own?
- Which Converge contracts does it depend on?
- What must stay in Converge?
- What must stay in products or deployments?
- What feature flags enable heavy or optional dependencies?

Suggestor-bearing extensions should also link to [[Suggestor Contract]] and
state which suggestors perform external I/O, native solver calls, or policy
evaluation.

## Command Surface

Every extension `Justfile` should expose:

- `build`
- `build-release`
- `check`
- `test`
- `test-all`
- `fmt`
- `fmt-check`
- `clippy`
- `lint`
- `doc`
- `doc-open`
- `status`
- `clean`

Native or feature-heavy extensions should add explicit feature recipes such as
`check-all`, `test-full`, or `deps`.

## KB Discipline

Every `kb/` page must have frontmatter:

```yaml
---
tags: [...]
source: human | llm | mixed
---
```

When a page becomes stale, update it in place and link it from [[Home]] and
[[INDEX]] if it is a durable entity.

See also: [[Suggestor Contract]], [[Architecture/Dependency Rules]],
[[Building/Release and Versioning]]

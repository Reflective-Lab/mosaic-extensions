---
tags: [workflow, codex]
source: mixed
date: 2026-05-05
---
# Working with Codex

Codex should treat this folder as a multi-repo workspace.

## Read First

1. `kb/Home.md`
2. `kb/Architecture/Extension Topology.md`
3. The module page for the extension being changed
4. The extension README and `Cargo.toml`

Do not bulk-read every extension unless the task is explicitly cross-cutting.

## Before Editing

Run status inside the target extension:

```bash
git -C /Users/kpernyer/dev/extensions/<extension-dir> status --short --branch
```

If there are local changes, preserve them unless the user explicitly asks to
revert them.

## During Work

- Prefer existing crate patterns over new abstractions.
- Keep boundary changes reflected in `kb/Architecture/*`.
- Keep public API changes reflected in the module page.
- Use `rg` for code searches.
- Run checks from the target extension root.

## After Work

Update `kb/LOG.md` only for durable KB changes. Do not use the KB as a scratch
pad for ordinary command output.

See also: [[Workflow/Daily Journey]], [[Workflow/Git Strategy]]

---
tags: [workflow, cheat-sheet]
source: mixed
date: 2026-05-05
---
# Daily Journey

This folder is a container for multiple extension repositories. Start by
choosing the extension you are touching.

## Morning

1. Read [[Home]].
2. Read [[Architecture/Extension Topology]].
3. Read the relevant module page.
4. Check status inside the target extension repo.

```bash
git -C /Users/kpernyer/dev/extensions/<extension-dir> status --short --branch
```

If the target folder is not a git repo yet, decide whether it should be
initialized before doing substantive work.

## Working

- Run Cargo commands from the extension repository.
- Keep unrelated extension changes in separate commits and branches.
- Update the module page when an extension's public surface or boundary changes.
- Update [[Architecture/Dependency Rules]] when a new Converge dependency edge
  appears.
- Update [[Planning/Next Steps]] when extraction follow-up changes.

## End of Session

1. Run the narrowest relevant check for the extension.
2. Record important durable knowledge in `kb/`.
3. Update [[LOG]] if the KB changed.
4. Update the extension README if the public usage story changed.

See also: [[Workflow/Git Strategy]], [[Workflow/Working with Codex]]

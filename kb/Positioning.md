---
tags: [positioning, pitch, reasoning-extensions]
source: llm
date: 2026-06-12
---
# Positioning

The combined elevator pitch for the reasoning extensions. Each extension
carries its own chapter in its `kb/Positioning.md` (ferrox, arbiter, soter,
prism, crucible, mnemos); this is the container-level synthesis. The
sub-family split is defined in [[Architecture/Extension Topology]].

## The Reasoning Extensions — Six Minds, One Governed Truth

Modern AI has a missing middle. On one side, an LLM: fluent, creative,
persuadable, and unable to prove anything. On the other, production systems
that need answers which are optimal, authorized, remembered, measured,
learned, or proven. The Mosaic reasoning extensions are that middle — six
pure-Rust extensions to the Converge platform, each answering one epistemic
question with an honest label on how it knows:

| Extension | Question | How it knows |
|---|---|---|
| Mnemos | What do we already know? | recall with provenance |
| Prism | What does the data say? | closed-form, auditable |
| Crucible | What does our fitted model predict? | trained opinion |
| Ferrox | What is the best feasible plan? | exact optimization |
| Arbiter | Is this allowed, right now? | deterministic policy |
| Soter | Can *any* case violate this rule? | symbolic counterexample search |

Together they wrap an LLM in everything it constitutively lacks. The LLM
formulates — it turns messy human intent into models, queries, and claims.
The extensions then do what no language model can: Mnemos remembers across
sessions, Prism computes instead of estimating, Crucible brings judgment
fitted to your data, Ferrox returns provably optimal plans via CP-SAT and
HiGHS, Arbiter holds a prompt-injection-immune Cedar veto, and Soter
exhausts the space of possibilities through SMT to find the counterexample
nobody wrote a test for.

The discipline that binds them: **no extension promotes its own truth.**
Every answer lands as tiered evidence — `Observed`, `Argued`, `Searched`,
`Decided`, with `Verified` reserved for checked proof — carrying typed
provenance into the Converge promotion path, where the platform, not the
component, decides what becomes fact.

The result is a cognitive architecture, not a toolbox: language and intent
from the LLM; memory, perception, learned prediction, exact planning, hard
boundaries, and adversarial proof-search from the reasoning extensions; and
one governed path from proposal to fact. In an era where generating
plausible answers is free, this is the machine that makes them trustworthy.

## Per-Extension Chapters

- `ferrox-solvers/kb/Positioning.md` — exact optimization (OR-Tools CP-SAT,
  GLOP, min-cost flow; HiGHS MIP/LP)
- `arbiter-policy/kb/Positioning.md` — Cedar authorization and the runtime →
  symbolic assurance lane
- `soter-smt/kb/Positioning.md` — SMT counterexample search and why formal
  methods matter more than ever
- `prism-analytics/kb/Positioning.md` — closed-form analytics and fuzzy
  inference, full algorithm catalog
- `crucible-models/kb/Positioning.md` — trained models and the training
  pipeline, full model catalog
- `mnemos-knowledge/kb/Positioning.md` — hybrid retrieval and agentic
  memory, full capability catalog

## See Also

- [[Architecture/Pluralist Reasoning Substrate]] — the eight reasoning modes
  in motion
- [[Architecture/Expert Portfolio Architecture]] — why formations use
  specialized reasoning experts
- [[Capability Matrix]] — every callable function across the modules

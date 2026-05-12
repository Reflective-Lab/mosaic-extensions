---
tags: [moc]
source: mixed
date: 2026-05-05
---
# Converge Extensions

Knowledge base for `/Users/kpernyer/dev/extensions`, the reusable extension
home for Converge-adjacent capabilities.

This workspace exists to make Converge right-shaped: Converge keeps the
universal contracts, engine, promotion authority, and semantic model; extension
repositories own implementation-heavy suggestors, adapters, solvers, memory,
analytics, policy engines, and source-specific connector ports.

**Start here:** [[Ecosystem]], then [[Architecture/Extension Topology]] and
[[Architecture/Converge Boundary]].

**Meta:** [[INDEX]] - entity catalog | [[LOG]] - mutation log

## Architecture

- [[Architecture/Extension Topology]] - canonical homes and dependency flow
- [[Architecture/Converge Boundary]] - what stays in Converge and what belongs here
- [[Architecture/Repository Map]] - workspace and crate layout
- [[Architecture/Dependency Rules]] - import rules, transitional deps, and promotion rules
- [[Architecture/Port Provider Boundary]] - Embassy vs Manifold decision rule
- [[Architecture/Runtime Assembly]] - how products wire extensions into systems
- [[Architecture/Expert Portfolio Architecture]] - why formations use specialized reasoning experts
- [[Architecture/Extraction Status]] - current extraction state and known gaps

## Modules

- [[Modules/Arbiter]] - Cedar policy gates and authorization suggestors
- [[Modules/Embassy]] - source-specific connector ports
- [[Modules/Ferrox]] - OR-Tools and HiGHS solver suggestors
- [[Modules/Manifold]] - generic provider, storage, and tool adapters
- [[Modules/Mnemos]] - knowledge, recall, memory, and retrieval suggestors
- [[Modules/Prism]] - analytics, ML, feature, and training suggestors

## Building

- [[Building/Developer Guide]] - full developer guide for all extension repos
- [[Building/Getting Started]] - local checks, toolchain, and feature flags
- [[Building/Release and Versioning]] - crate versions, path patches, and release notes

## Workflow

- [[Workflow/Daily Journey]] - daily operating loop for this multi-repo folder
- [[Workflow/Git Strategy]] - individual repo discipline inside the container
- [[Workflow/Working with Codex]] - what Codex should read and update

## Standards

- [[Standards/Extension Standard]] - minimum shape for new extension repos and crates

## Planning

- [[Planning/Next Steps]] - work still needed after extraction

## Experiments

- [[Experiments/INDEX]] - experiment catalog
- [[Experiments/LOG]] - experiment mutation log

## History

- [[History/Audit Log]] - observed state and KB creation notes

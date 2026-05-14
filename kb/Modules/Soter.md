---
tags: [module, smt, cvc5]
source: mixed
date: 2026-05-14
---
# Soter

`soter` owns SMT-backed safety evidence as reusable Converge extension code.

It implements SMT query/report types, a native CVC5 FFI boundary, fake solver
backends for CI, and solver-backed suggestors that emit searched evidence into
the convergence loop.

Soter exists because Arbiter needs a place for conditional invariant search
without turning Arbiter into a general SMT crate and without introducing a
proof-assistant tier before a checked-proof app need exists.

## Owns

- SMT query and report vocabulary.
- Stable query hashes for replay/audit.
- SMT statuses: `sat`, `unsat`, `unknown`, `timeout`, `error`.
- Native CVC5 FFI in `crates/cvc5-sys`.
- Solver-backed suggestors.
- Formation capability descriptors under `soter.smt`.
- Typed `soter` provenance at the proposal boundary.

## Public Surface

- `SmtQuery`
- `SmtReport`
- `SmtStatus`
- `SmtBackend`
- `FakeSmtBackend`
- `SmtSuggestor`
- `ProvenanceSource`
- `formation_capabilities()`
- `Cvc5FfiBackend` behind the `cvc5` feature

## Evidence Boundary

Soter results are `Searched` evidence.

`sat` usually means a counterexample was found for invariant-violation
queries. `unsat` means no counterexample exists for the encoded SMT query.
Neither status is a formal proof unless an independent proof checker verifies
an emitted artifact.

## Entry Points

- `soter-smt/README.md`
- `soter-smt/AGENTS.md`
- `soter-smt/crates/soter/src/lib.rs`
- `soter-smt/crates/cvc5-sys/build.rs`
- `soter-smt/kb/Home.md`

See also: [[Modules/Arbiter]], [[Modules/Ferrox]],
[[Architecture/Runtime Assembly]]

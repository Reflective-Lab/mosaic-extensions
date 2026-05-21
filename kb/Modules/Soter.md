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
- Structured solver identity for replay/audit across native CVC5 versions,
  checkout commits, source modes, configure flags, and runtime query options.
- SMT statuses: `sat`, `unsat`, `unknown`, `timeout`, `error`.
- Native CVC5 FFI in `crates/cvc5-sys`.
- Solver-backed suggestors.
- Typed abstract Arbiter invariant fixtures for selected high-risk policy
  claims.
- Formation capability descriptors under `soter.smt`.
- Typed `soter` provenance at the proposal boundary.

## Public Surface

- `SmtQuery`
- `SmtReport`
- `SmtStatus`
- `SmtBackend`
- `ScriptedSmtBackend` (behind the non-default `fake-backend` feature)
- `ArbiterExpenseCommitInvariant`
- `ArbiterExpensePolicyModel`
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

The first Arbiter fixture is the non-finance high-value expense commit
invariant. The strict abstraction is expected to return `unsat`; an
intentionally broken abstraction is expected to return `sat` with a
counterexample. This fixture is searched evidence over a generated abstraction,
not full Cedar semantics.

The integration harness also wires Soter's `Cvc5FfiBackend` into Arbiter's
actual Cedar/SymCC path behind the `soter-cvc5` feature. That path runs
`CedarAnalysisSuggestor` over real Arbiter policy/schema inputs and uses Soter
only as the CVC5 execution backend. This is the preferred cross-extension
shape: Arbiter owns Cedar modeling; Soter owns SMT execution.

## Entry Points

- `soter-smt/README.md`
- `soter-smt/AGENTS.md`
- `soter-smt/crates/soter/src/lib.rs`
- `soter-smt/crates/cvc5-sys/build.rs`
- `soter-smt/kb/Home.md`

See also: [[Modules/Arbiter]], [[Modules/Ferrox]],
[[Architecture/Runtime Assembly]]

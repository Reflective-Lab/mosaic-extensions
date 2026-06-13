#![cfg(feature = "soter-cvc5")]

use std::{
    io,
    pin::Pin,
    task::{Context as TaskContext, Poll},
};

use arbiter::{
    CedarAnalysisBackend, CedarAnalysisError, CedarAnalysisExecutionStatus, CedarAnalysisInput,
    CedarAnalysisQuery, CedarAnalysisReport, CedarAnalysisSuggestor, EXPENSE_APPROVAL_POLICY,
    EXPENSE_APPROVAL_SCHEMA, execute_analysis_with_solver,
};
use cedar_policy_symcc::solver::{Decision, DecisionWithModel, Solver, SolverError};
use converge_kernel::{Budget, ContextKey, ContextState, Engine};
use converge_pack::{ProposedFact, SubjectRef};
use soter::{Cvc5FfiBackend, SmtBackend, SmtQuery, SmtReport, SmtStatus};
use tokio::io::AsyncWrite;

const INVARIANT_ID: &str = "expense.non_finance_commit.high_value";

#[derive(Debug, Clone)]
struct SoterCvc5SymccSolver {
    backend: Cvc5FfiBackend,
    buffer: SmtLibBuffer,
    timeout_ms: u64,
}

impl Default for SoterCvc5SymccSolver {
    fn default() -> Self {
        Self {
            backend: Cvc5FfiBackend,
            buffer: SmtLibBuffer::default(),
            timeout_ms: 5_000,
        }
    }
}

impl SoterCvc5SymccSolver {
    fn take_query(&mut self, produce_model: bool) -> Result<SmtQuery, SolverError> {
        let mut smtlib = self.buffer.take_utf8()?;
        smtlib.push_str("(check-sat)\n");

        let mut query = SmtQuery::new(format!("{INVARIANT_ID}.cedar-symcc"), smtlib);
        query.timeout_ms = self.timeout_ms;
        query.produce_model = produce_model;
        query.produce_unsat_core = !produce_model;
        Ok(query)
    }

    async fn solve(&mut self, produce_model: bool) -> Result<SmtReport, SolverError> {
        let query = self.take_query(produce_model)?;
        self.backend
            .solve(&query)
            .await
            .map_err(|err| SolverError::Solver(err.to_string()))
    }
}

impl Solver for SoterCvc5SymccSolver {
    fn smtlib_input(&mut self) -> &mut (dyn AsyncWrite + Unpin + Send) {
        &mut self.buffer
    }

    async fn enable_models(&mut self) -> Result<(), SolverError> {
        self.buffer.write_line("(set-option :produce-models true)");
        Ok(())
    }

    async fn check_sat(&mut self) -> Result<Decision, SolverError> {
        Ok(decision_from_report(&self.solve(false).await?))
    }

    async fn check_sat_with_model(&mut self) -> Result<DecisionWithModel, SolverError> {
        decision_with_model_from_report(&self.solve(true).await?)
    }
}

#[derive(Debug, Clone, Default)]
struct SmtLibBuffer {
    bytes: Vec<u8>,
}

impl SmtLibBuffer {
    fn write_line(&mut self, line: &str) {
        self.bytes.extend_from_slice(line.as_bytes());
        self.bytes.push(b'\n');
    }

    fn take_utf8(&mut self) -> Result<String, SolverError> {
        let bytes = std::mem::take(&mut self.bytes);
        String::from_utf8(bytes)
            .map_err(|err| SolverError::Solver(format!("SymCC emitted non-UTF8 SMT-LIB: {err}")))
    }
}

impl AsyncWrite for SmtLibBuffer {
    fn poll_write(
        mut self: Pin<&mut Self>,
        _cx: &mut TaskContext<'_>,
        buf: &[u8],
    ) -> Poll<Result<usize, io::Error>> {
        self.bytes.extend_from_slice(buf);
        Poll::Ready(Ok(buf.len()))
    }

    fn poll_flush(self: Pin<&mut Self>, _cx: &mut TaskContext<'_>) -> Poll<Result<(), io::Error>> {
        Poll::Ready(Ok(()))
    }

    fn poll_shutdown(
        self: Pin<&mut Self>,
        _cx: &mut TaskContext<'_>,
    ) -> Poll<Result<(), io::Error>> {
        Poll::Ready(Ok(()))
    }
}

fn decision_from_report(report: &SmtReport) -> Decision {
    match report.status {
        SmtStatus::Sat => Decision::Sat,
        SmtStatus::Unsat => Decision::Unsat,
        SmtStatus::Unknown | SmtStatus::Timeout | SmtStatus::Error => Decision::Unknown,
    }
}

fn decision_with_model_from_report(report: &SmtReport) -> Result<DecisionWithModel, SolverError> {
    match report.status {
        SmtStatus::Sat => Ok(DecisionWithModel::Sat {
            model: report.model.clone().ok_or_else(|| {
                SolverError::Solver("CVC5 returned sat without a model".to_string())
            })?,
        }),
        SmtStatus::Unsat => Ok(DecisionWithModel::Unsat),
        SmtStatus::Unknown | SmtStatus::Timeout => Ok(DecisionWithModel::Unknown),
        SmtStatus::Error => Err(SolverError::Solver(
            report
                .diagnostics
                .clone()
                .unwrap_or_else(|| "CVC5 FFI backend returned error".to_string()),
        )),
    }
}

#[derive(Debug, Clone, Copy, Default)]
struct SoterCvc5AnalysisBackend;

#[async_trait::async_trait]
impl CedarAnalysisBackend for SoterCvc5AnalysisBackend {
    fn name(&self) -> &'static str {
        "soter-cvc5-ffi"
    }

    async fn analyze(
        &self,
        input: &CedarAnalysisInput,
    ) -> Result<CedarAnalysisReport, CedarAnalysisError> {
        let input = input.clone();
        tokio::task::spawn_blocking(move || {
            let runtime = tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .map_err(|err| CedarAnalysisError::SolverInit(err.to_string()))?;

            runtime.block_on(async {
                execute_analysis_with_solver(&input, SoterCvc5SymccSolver::default()).await
            })
        })
        .await
        .map_err(|err| CedarAnalysisError::SolverInit(err.to_string()))?
    }
}

fn budget() -> Budget {
    Budget {
        max_cycles: 3,
        max_facts: 50,
    }
}

fn expense_claim_input() -> CedarAnalysisInput {
    CedarAnalysisInput::new(
        INVARIANT_ID,
        CedarAnalysisQuery::ExpenseNonFinanceHighValueCommitDenied,
        EXPENSE_APPROVAL_POLICY,
        EXPENSE_APPROVAL_SCHEMA,
    )
}

#[tokio::test]
async fn soter_cvc5_backend_executes_actual_cedar_symcc_claim() {
    let report = SoterCvc5AnalysisBackend
        .analyze(&expense_claim_input())
        .await
        .expect("Soter CVC5 should execute the Cedar SymCC invariant");

    assert_eq!(report.status, CedarAnalysisExecutionStatus::NoViolation);
    assert_eq!(
        report.plan.query,
        CedarAnalysisQuery::ExpenseNonFinanceHighValueCommitDenied
    );
    assert_eq!(report.checks.len(), report.plan.request_env_count());
    assert!(
        report
            .checks
            .iter()
            .all(|check| check.status == CedarAnalysisExecutionStatus::NoViolation)
    );
}

#[tokio::test]
async fn cedar_analysis_suggestor_emits_soter_cvc5_report() {
    let subject = SubjectRef::parse("helm://policy-invariants/expense-non-finance-commit")
        .expect("subject ref should parse");
    let mut engine = Engine::with_budget(budget());
    engine.register_suggestor(CedarAnalysisSuggestor::new(SoterCvc5AnalysisBackend));

    let mut context = ContextState::new();
    context
        .add_proposal(
            ProposedFact::new(
                ContextKey::Seeds,
                "cedar-analysis-input",
                expense_claim_input(),
                "integration-test",
            )
            .with_subject(subject.clone()),
        )
        .expect("analysis input should stage");

    let result = engine.run(context).await.expect("engine should run");
    assert!(result.converged);

    let evaluations = result.context.get(ContextKey::Evaluations);
    assert_eq!(evaluations.len(), 1);
    assert_eq!(evaluations[0].subject(), Some(&subject));

    let report = evaluations[0]
        .require_payload::<CedarAnalysisReport>()
        .expect("report should be a typed Cedar analysis payload");
    assert_eq!(report.status, CedarAnalysisExecutionStatus::NoViolation);
    assert_eq!(report.plan.invariant_id, INVARIANT_ID);
    assert_eq!(
        report.plan.query,
        CedarAnalysisQuery::ExpenseNonFinanceHighValueCommitDenied
    );
}

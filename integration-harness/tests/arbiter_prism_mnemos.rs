use std::sync::Arc;

use arbiter::{
    ContextIn, DecideRequest, EXPENSE_APPROVAL_POLICY, PolicyEngine, PolicyOutcome, PrincipalIn,
    ResourceIn,
};
use converge_core::{AuthorityLevel, FlowAction, FlowPhase};
use converge_kernel::{Budget, ContextKey, ContextState, Engine};
use converge_pack::{
    DomainId, GateId, Pack, PackInputPayload, PackPlanPayload, PackSuggestor, PolicyVersionId,
    ProposedFact, ResourceKind, TextPayload,
};
use mnemos::{
    KnowledgeBase, KnowledgeBaseConfig, KnowledgeEntry, KnowledgeHitPayload,
    KnowledgeRetrievalSuggestor,
};
use prism::FuzzyInferencePack;

fn budget() -> Budget {
    Budget {
        max_cycles: 5,
        max_facts: 100,
    }
}

fn fuzzy_expense_risk_input() -> serde_json::Value {
    serde_json::json!({
        "inputs": {
            "amount_pressure": 0.86
        },
        "variables": [
            {
                "name": "amount_pressure",
                "sets": [
                    {
                        "name": "high",
                        "function": {
                            "kind": "right_shoulder",
                            "start": 0.5,
                            "end": 0.9
                        }
                    }
                ]
            },
            {
                "name": "expense_risk",
                "sets": [
                    {
                        "name": "high",
                        "function": {
                            "kind": "right_shoulder",
                            "start": 0.6,
                            "end": 0.9
                        }
                    }
                ]
            }
        ],
        "rules": [
            {
                "id": "high-amount-pressure",
                "if": {
                    "op": "is",
                    "variable": "amount_pressure",
                    "set": "high"
                },
                "then": {
                    "variable": "expense_risk",
                    "set": "high"
                }
            }
        ]
    })
}

fn risk_confidence(plan: &PackPlanPayload) -> f64 {
    assert_eq!(plan.pack, "fuzzy-inference");
    assert_eq!(
        plan.plan["activated_rules"][0]["id"],
        "high-amount-pressure"
    );

    let membership = plan.plan["memberships"]["expense_risk.high"]
        .as_f64()
        .expect("risk membership should be numeric");
    let confidence = plan.plan["confidence"]
        .as_f64()
        .expect("risk confidence should be numeric");

    assert!((membership - confidence).abs() < 1e-9);
    confidence
}

fn non_finance_expense_commit(amount: i64) -> DecideRequest {
    DecideRequest {
        principal: PrincipalIn {
            id: "agent:operations-supervisor".into(),
            authority: AuthorityLevel::Supervisory,
            domains: vec![DomainId::new("operations")],
            policy_version: Some(PolicyVersionId::new("expense_v1")),
        },
        resource: ResourceIn {
            id: "expense:golden-flow-001".into(),
            resource_type: Some(ResourceKind::new("expense")),
            phase: Some(FlowPhase::Commitment),
            gates_passed: Some(vec![
                GateId::new("receipt"),
                GateId::new("manager_approval"),
            ]),
        },
        action: FlowAction::Commit,
        context: Some(ContextIn {
            commitment_type: Some("expense".into()),
            amount: Some(amount),
            human_approval_present: Some(true),
            required_gates_met: Some(true),
        }),
        delegation_b64: None,
    }
}

#[tokio::test]
async fn golden_flow_uses_recall_and_fuzzy_risk_before_cedar_gate() {
    let recall_query = "high value expense finance manager approval receipt commit";
    let tempdir = tempfile::tempdir().expect("tempdir should be available");
    let config = KnowledgeBaseConfig::default()
        .with_path(tempdir.path().join("knowledge.db").to_string_lossy())
        .with_dimensions(64)
        .without_learning();
    let kb = Arc::new(
        KnowledgeBase::with_config(config)
            .await
            .expect("knowledge base should open"),
    );
    kb.add_entry(
        KnowledgeEntry::new(
            "expense approval gate",
            format!(
                "{recall_query}: High value expense commits require the finance domain, receipt gate, manager approval, and explicit approval."
            ),
        )
        .with_category("policy")
        .with_tags(["arbiter", "expense", "finance"]),
    )
    .await
    .expect("knowledge fixture should be stored");
    assert!(
        !kb.search_simple(recall_query, 1)
            .await
            .expect("knowledge preflight search should succeed")
            .is_empty(),
        "Mnemos preflight search should find the fixture"
    );

    let mut engine = Engine::with_budget(budget());
    engine.register_suggestor(PackSuggestor::new(
        FuzzyInferencePack,
        ContextKey::Seeds,
        ContextKey::Strategies,
    ));
    engine.register_suggestor(KnowledgeRetrievalSuggestor::new(kb).with_max_results(1));

    let mut context = ContextState::new();
    context
        .add_proposal(ProposedFact::new(
            ContextKey::Seeds,
            "expense-risk-input",
            PackInputPayload::new(FuzzyInferencePack.name(), fuzzy_expense_risk_input()),
            "integration-test",
        ))
        .expect("risk input should stage");
    context
        .add_proposal(ProposedFact::new(
            ContextKey::Seeds,
            "expense-policy-query",
            TextPayload::new(recall_query),
            "integration-test",
        ))
        .expect("knowledge query should stage");

    let result = engine.run(context).await.expect("engine should run");
    assert!(result.converged);

    let hypotheses = result.context.get(ContextKey::Hypotheses);
    let knowledge_hit = hypotheses
        .iter()
        .filter_map(|fact| fact.payload::<KnowledgeHitPayload>())
        .find(|payload| payload.content.contains("High value expense commits"))
        .expect("Mnemos should retrieve the policy fixture");

    let identity = &knowledge_hit.execution_identity;
    assert_eq!(
        identity.producer.name, "converge-mnemos-knowledge",
        "execution_identity producer should be the mnemos crate"
    );
    assert_eq!(
        identity.backend, "mnemos-knowledge-base-v1",
        "Mnemos backend should track the workspace's knowledge-base pin"
    );
    // Replay must be able to reconstruct WHICH query and HOW MANY hits were
    // requested — those are the variables that change retrieval output.
    // Asserting only `!is_empty()` would silently pass through any change
    // that drops the query text (e.g. privacy redaction) or the cutoff.
    let rc: serde_json::Value = serde_json::from_str(&identity.runtime_config)
        .expect("runtime_config must be valid JSON per the Runtime Config Encoding standard");
    assert_eq!(
        rc.get("query").and_then(serde_json::Value::as_str),
        Some(recall_query),
        "runtime_config.query must record the actual query string used for retrieval"
    );
    assert_eq!(
        rc.get("max_results").and_then(serde_json::Value::as_u64),
        Some(1),
        "runtime_config.max_results must record the cutoff (set via `with_max_results(1)` above)"
    );

    let strategies = result.context.get(ContextKey::Strategies);
    assert_eq!(strategies.len(), 1);
    let confidence = risk_confidence(
        strategies[0]
            .require_payload::<PackPlanPayload>()
            .expect("fuzzy pack should emit typed pack plan payload"),
    );
    assert!(confidence >= 0.8);

    let amount = if confidence >= 0.8 { 8_400 } else { 4_000 };
    let decision = PolicyEngine::from_policy_str(EXPENSE_APPROVAL_POLICY)
        .expect("expense policy should parse")
        .evaluate(&non_finance_expense_commit(amount))
        .expect("policy evaluation should succeed");

    assert_eq!(decision.outcome, PolicyOutcome::Reject);
}

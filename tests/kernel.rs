use codex_orchestrator_kernel::*;
use std::fs;

fn envelope(mode: Mode) -> Envelope {
    Envelope {
        schema_version: 1,
        task_language: "fr".into(),
        repository: "example".into(),
        repository_version: "abc123".into(),
        policy_version: "v1".into(),
        codex_version: "0.144.4".into(),
        worktree_state: "clean".into(),
        changed_paths: vec!["README.md".into()],
        estimated_scope: "bounded".into(),
        blast_radius: "local".into(),
        reversible: true,
        requested_side_effects: false,
        permissions: "default".into(),
        verifier_available: true,
        root_judge_available: true,
        quota_signal: None,
        latency_signal_ms: None,
        expected_savings_pct: Some(20.0),
        remaining_context_tokens: None,
        remaining_budget_tokens: None,
        category: "docs".into(),
        category_validated: true,
        mode,
        child_run: false,
    }
}

#[test]
fn strict_policy_rejects_duplicate_and_unknown_fields() {
    assert!(Policy::parse(DEFAULT_POLICY).is_ok());
    assert!(Policy::parse("schema_version=1\nschema_version=1").is_err());
    assert!(Policy::parse(&format!("{DEFAULT_POLICY}\nunknown=true")).is_err());
}

#[test]
fn decision_is_direct_first_and_locks_unproven_execution() {
    let policy = Policy::parse(DEFAULT_POLICY).unwrap();
    let observed = decide(&policy, &envelope(Mode::Observe));
    assert_eq!(observed.route, Route::DirectNative);
    assert!(observed.reason_codes.contains(&"SHADOW_ONLY"));

    let assisted = decide(&policy, &envelope(Mode::Assist));
    assert_eq!(assisted.route, Route::RecommendDispatch);

    let executed = decide(&policy, &envelope(Mode::Execute));
    assert_eq!(executed.route, Route::DirectNative);
    assert!(executed.reason_codes.contains(&"ACTUATOR_NOT_VALIDATED"));
}

#[test]
fn repository_evidence_overrides_low_risk_wording() {
    let policy = Policy::parse(DEFAULT_POLICY).unwrap();
    let mut input = envelope(Mode::Assist);
    input.changed_paths = vec!["src/auth/session.rs".into()];
    input.root_judge_available = false;
    let assignment = decide(&policy, &input);
    assert_eq!(assignment.risk, Risk::High);
    assert_eq!(assignment.route, Route::Blocked);
    assert!(assignment.reason_codes.contains(&"ROOT_JUDGE_UNAVAILABLE"));
}

#[test]
fn child_and_missing_benefit_never_dispatch() {
    let policy = Policy::parse(DEFAULT_POLICY).unwrap();
    let mut child = envelope(Mode::Assist);
    child.child_run = true;
    assert_eq!(decide(&policy, &child).route, Route::DirectNative);

    let mut unknown = envelope(Mode::Assist);
    unknown.expected_savings_pct = None;
    assert_eq!(decide(&policy, &unknown).route, Route::DirectNative);
}

#[test]
fn ledger_replay_is_idempotent_and_run_isolated() {
    let root = tempfile::tempdir().unwrap();
    let event = Event {
        schema_version: 1,
        event_id: "e1".into(),
        run_id: "r1".into(),
        kind: "decision".into(),
        repository_digest: "d1".into(),
        timestamp_ms: 1,
    };
    append_event(root.path(), &event).unwrap();
    append_event(root.path(), &event).unwrap();
    assert_eq!(replay(root.path(), "r1").unwrap(), vec![event.clone()]);

    fs::create_dir_all(root.path().join("runs/r2")).unwrap();
    fs::write(root.path().join("runs/r2/events.jsonl"), "{truncated").unwrap();
    assert!(replay(root.path(), "r2").is_err());
    assert_eq!(replay(root.path(), "r1").unwrap().len(), 1);

    let mut traversal = event.clone();
    traversal.run_id = "..".into();
    assert!(append_event(root.path(), &traversal).is_err());
    assert!(replay(root.path(), "../r1").is_err());
    assert!(replay(root.path(), "").is_err());
}

#[test]
fn proof_is_bound_to_risk_and_current_digest() {
    let low = vec![
        Proof {
            claim: "targeted_check".into(),
            required: true,
            status: ProofStatus::Passed,
            repository_digest: "d1".into(),
            scope: Risk::Low,
        },
        Proof {
            claim: "diff_inspection".into(),
            required: true,
            status: ProofStatus::Passed,
            repository_digest: "d1".into(),
            scope: Risk::Low,
        },
    ];
    assert!(can_complete("d1", Risk::Low, &low));
    assert!(!can_complete("d2", Risk::Low, &low));
    assert!(!can_complete("d1", Risk::High, &low));

    let duplicates = vec![low[0].clone(), low[0].clone()];
    assert!(!can_complete("d1", Risk::Low, &duplicates));

    let mut unknown = low.clone();
    unknown[1].claim = "arbitrary".into();
    assert!(!can_complete("d1", Risk::Low, &unknown));
}

#[test]
fn promotion_fails_closed_on_missing_or_regressed_metrics() {
    let policy = Policy::parse(DEFAULT_POLICY).unwrap();
    let passing = CategoryMetrics {
        accepted: 20,
        critical_false_negatives: 0,
        classification_accuracy: Some(0.92),
        trivial_overhead_pct: Some(1.0),
        complex_savings_pct: Some(18.0),
        acceptance_not_inferior_to_root: true,
        acceptance_not_inferior_to_subagents: true,
        rework_regression: false,
        incident_regression: false,
        stale_proof_completions: 0,
    };
    assert_eq!(
        validate_category(&policy, &passing),
        ValidationOutcome::Validated
    );

    let missing = CategoryMetrics {
        complex_savings_pct: None,
        ..passing.clone()
    };
    assert_eq!(
        validate_category(&policy, &missing),
        ValidationOutcome::NoProvenAdvantage
    );

    let critical_miss = CategoryMetrics {
        critical_false_negatives: 1,
        ..passing
    };
    assert_eq!(
        validate_category(&policy, &critical_miss),
        ValidationOutcome::NoProvenAdvantage
    );
}

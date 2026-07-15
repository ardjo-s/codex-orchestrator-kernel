use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::fs::{self, File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

pub const DEFAULT_POLICY: &str = include_str!("../policies/default.toml");

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Policy {
    pub schema_version: u32,
    pub limits: Limits,
    pub promotion: Promotion,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Limits {
    pub assist_context_bytes: usize,
    pub max_repairs: u8,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Promotion {
    pub min_accepted: u32,
    pub max_critical_false_negatives: u32,
    pub min_classification_accuracy: f64,
    pub max_trivial_overhead_pct: f64,
    pub min_complex_savings_pct: f64,
}

impl Policy {
    pub fn parse(input: &str) -> Result<Self, String> {
        let policy: Self = toml::from_str(input).map_err(|error| error.to_string())?;
        if policy.schema_version != 1 {
            return Err(format!(
                "unsupported policy schema {}",
                policy.schema_version
            ));
        }
        if policy.limits.assist_context_bytes == 0 || policy.limits.max_repairs > 1 {
            return Err("invalid safety limits".into());
        }
        Ok(policy)
    }
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Mode {
    Observe,
    Assist,
    Execute,
    Govern,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Risk {
    Low,
    Normal,
    High,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Route {
    DirectNative,
    RecommendDispatch,
    Blocked,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Envelope {
    pub schema_version: u32,
    pub task_language: String,
    pub repository: String,
    pub repository_version: String,
    pub policy_version: String,
    pub codex_version: String,
    pub worktree_state: String,
    pub changed_paths: Vec<String>,
    pub estimated_scope: String,
    pub blast_radius: String,
    pub reversible: bool,
    pub requested_side_effects: bool,
    pub permissions: String,
    pub verifier_available: bool,
    pub root_judge_available: bool,
    pub quota_signal: Option<f64>,
    pub latency_signal_ms: Option<u64>,
    pub expected_savings_pct: Option<f64>,
    pub remaining_context_tokens: Option<u64>,
    pub remaining_budget_tokens: Option<u64>,
    pub category: String,
    pub category_validated: bool,
    pub mode: Mode,
    #[serde(default)]
    pub child_run: bool,
}

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct Assignment {
    pub schema_version: u32,
    pub route: Route,
    pub risk: Risk,
    pub root_authority: &'static str,
    pub roles: Vec<&'static str>,
    pub mode: Mode,
    pub enforcement: &'static str,
    pub reason_codes: Vec<&'static str>,
    pub proof_obligations: Vec<&'static str>,
    pub alternatives: Vec<&'static str>,
    pub escalation_triggers: Vec<&'static str>,
    pub human_gate: bool,
    pub confidence: &'static str,
}

pub fn decide(policy: &Policy, envelope: &Envelope) -> Assignment {
    let risk = classify_risk(envelope);
    let mut reasons = Vec::new();

    if envelope.schema_version != 1 {
        reasons.push("UNSUPPORTED_ENVELOPE");
        return blocked(envelope.mode, risk, reasons);
    }
    if envelope.child_run {
        reasons.push("REENTRANT_CHILD");
        return direct(envelope.mode, risk, reasons);
    }
    if risk == Risk::High && !envelope.root_judge_available {
        reasons.push("ROOT_JUDGE_UNAVAILABLE");
        return blocked(envelope.mode, risk, reasons);
    }
    if envelope.requested_side_effects {
        reasons.push("HUMAN_AUTHORITY_REQUIRED");
        return blocked(envelope.mode, risk, reasons);
    }
    if matches!(envelope.mode, Mode::Execute | Mode::Govern) {
        reasons.push("ACTUATOR_NOT_VALIDATED");
        return direct(envelope.mode, risk, reasons);
    }
    if envelope.mode == Mode::Observe {
        reasons.push("SHADOW_ONLY");
        return direct(envelope.mode, risk, reasons);
    }
    if !envelope.category_validated {
        reasons.push("CATEGORY_NOT_VALIDATED");
        return direct(envelope.mode, risk, reasons);
    }
    if !envelope.verifier_available {
        reasons.push("VERIFIER_UNAVAILABLE");
        return direct(envelope.mode, risk, reasons);
    }
    match envelope.expected_savings_pct {
        Some(value) if value >= policy.promotion.min_complex_savings_pct && risk != Risk::High => {
            reasons.push("MEASURED_BENEFIT");
            Assignment {
                schema_version: 1,
                route: Route::RecommendDispatch,
                risk,
                root_authority: "configured_root",
                roles: vec!["bounded_worker", "independent_verifier"],
                mode: envelope.mode,
                enforcement: "advisory",
                reason_codes: reasons,
                proof_obligations: proof_for(risk),
                alternatives: vec!["DIRECT_NATIVE"],
                escalation_triggers: vec!["PROOF_FAILED", "SCOPE_EXPANDED", "RISK_CHANGED"],
                human_gate: false,
                confidence: "measured",
            }
        }
        Some(_) => {
            reasons.push("BENEFIT_BELOW_THRESHOLD");
            direct(envelope.mode, risk, reasons)
        }
        None => {
            reasons.push("BENEFIT_UNKNOWN");
            direct(envelope.mode, risk, reasons)
        }
    }
}

fn classify_risk(envelope: &Envelope) -> Risk {
    let critical = [
        "auth",
        "payment",
        "migration",
        "security",
        "public-api",
        "secret",
    ];
    if envelope.changed_paths.iter().any(|path| {
        critical
            .iter()
            .any(|term| path.to_lowercase().contains(term))
    }) {
        Risk::High
    } else if envelope.changed_paths.len() > 4 || !envelope.reversible {
        Risk::Normal
    } else {
        Risk::Low
    }
}

fn proof_for(risk: Risk) -> Vec<&'static str> {
    match risk {
        Risk::Low => vec!["targeted_check", "diff_inspection"],
        Risk::Normal => vec!["targeted_tests", "static_policy_checks", "diff_inspection"],
        Risk::High => vec![
            "positive_negative_cases",
            "domain_invariants",
            "independent_review",
            "human_gate_if_configured",
        ],
    }
}

fn direct(mode: Mode, risk: Risk, reasons: Vec<&'static str>) -> Assignment {
    Assignment {
        schema_version: 1,
        route: Route::DirectNative,
        risk,
        root_authority: "configured_root",
        roles: Vec::new(),
        mode,
        enforcement: "advisory",
        reason_codes: reasons,
        proof_obligations: proof_for(risk),
        alternatives: Vec::new(),
        escalation_triggers: vec!["PROOF_FAILED", "SCOPE_EXPANDED", "RISK_CHANGED"],
        human_gate: risk == Risk::High,
        confidence: "deterministic",
    }
}

fn blocked(mode: Mode, risk: Risk, reasons: Vec<&'static str>) -> Assignment {
    let mut assignment = direct(mode, risk, reasons);
    assignment.route = Route::Blocked;
    assignment.enforcement = "hard";
    assignment.human_gate = true;
    assignment
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Event {
    pub schema_version: u32,
    pub event_id: String,
    pub run_id: String,
    pub kind: String,
    pub repository_digest: String,
    pub timestamp_ms: u128,
}

pub fn state_root() -> Result<PathBuf, String> {
    if let Ok(path) = std::env::var("PLUGIN_DATA") {
        return Ok(PathBuf::from(path));
    }
    if let Ok(path) = std::env::var("CODEX_HOME") {
        return Ok(PathBuf::from(path).join("orchestrator"));
    }
    std::env::var("HOME")
        .map(|home| PathBuf::from(home).join(".codex/orchestrator"))
        .map_err(|_| "PLUGIN_DATA, CODEX_HOME, and HOME are unset".into())
}

pub fn new_run_id() -> String {
    format!("{}-{}", now_ms(), std::process::id())
}

pub fn append_event(root: &Path, event: &Event) -> Result<(), String> {
    if event.schema_version != 1 || !valid_id(&event.run_id) {
        return Err("invalid event".into());
    }
    let run_dir = root.join("runs").join(&event.run_id);
    fs::create_dir_all(&run_dir).map_err(|error| error.to_string())?;
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(run_dir.join("events.jsonl"))
        .map_err(|error| error.to_string())?;
    let mut line = serde_json::to_vec(event).map_err(|error| error.to_string())?;
    line.push(b'\n');
    file.write_all(&line).map_err(|error| error.to_string())?;
    file.sync_data().map_err(|error| error.to_string())
}

pub fn replay(root: &Path, run_id: &str) -> Result<Vec<Event>, String> {
    let file = File::open(root.join("runs").join(run_id).join("events.jsonl"))
        .map_err(|error| error.to_string())?;
    let mut seen = HashSet::new();
    let mut events = Vec::new();
    for line in BufReader::new(file).lines() {
        let line = line.map_err(|error| error.to_string())?;
        let event: Event = serde_json::from_str(&line).map_err(|error| error.to_string())?;
        if event.schema_version != 1 {
            return Err("unsupported event schema".into());
        }
        if seen.insert(event.event_id.clone()) {
            events.push(event);
        }
    }
    Ok(events)
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum ProofStatus {
    Planned,
    Passed,
    Failed,
    Unavailable,
    Stale,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Proof {
    pub claim: String,
    pub required: bool,
    pub status: ProofStatus,
    pub repository_digest: String,
    pub scope: Risk,
}

pub fn can_complete(current_digest: &str, risk: Risk, proofs: &[Proof]) -> bool {
    let minimum = proof_for(risk).len();
    proofs.iter().filter(|proof| proof.required).count() >= minimum
        && proofs.iter().filter(|proof| proof.required).all(|proof| {
            proof.status == ProofStatus::Passed
                && proof.repository_digest == current_digest
                && proof.scope == risk
        })
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CategoryMetrics {
    pub accepted: u32,
    pub critical_false_negatives: u32,
    pub classification_accuracy: Option<f64>,
    pub trivial_overhead_pct: Option<f64>,
    pub complex_savings_pct: Option<f64>,
    pub acceptance_not_inferior_to_root: bool,
    pub acceptance_not_inferior_to_subagents: bool,
    pub rework_regression: bool,
    pub incident_regression: bool,
    pub stale_proof_completions: u32,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ValidationOutcome {
    Validated,
    NoProvenAdvantage,
}

pub fn validate_category(policy: &Policy, metrics: &CategoryMetrics) -> ValidationOutcome {
    let comparable = metrics.classification_accuracy.is_some()
        && metrics.trivial_overhead_pct.is_some()
        && metrics.complex_savings_pct.is_some();
    let passes = comparable
        && metrics.accepted >= policy.promotion.min_accepted
        && metrics.critical_false_negatives <= policy.promotion.max_critical_false_negatives
        && metrics.classification_accuracy.unwrap_or_default()
            >= policy.promotion.min_classification_accuracy
        && metrics.trivial_overhead_pct.unwrap_or(f64::INFINITY)
            <= policy.promotion.max_trivial_overhead_pct
        && metrics.complex_savings_pct.unwrap_or(f64::NEG_INFINITY)
            >= policy.promotion.min_complex_savings_pct
        && metrics.acceptance_not_inferior_to_root
        && metrics.acceptance_not_inferior_to_subagents
        && !metrics.rework_regression
        && !metrics.incident_regression
        && metrics.stale_proof_completions == 0;
    if passes {
        ValidationOutcome::Validated
    } else {
        ValidationOutcome::NoProvenAdvantage
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct HookInput {
    pub session_id: String,
    pub hook_event_name: String,
    #[serde(default)]
    pub stop_hook_active: bool,
}

pub fn user_prompt_output(
    input: &HookInput,
    assist_context: Option<&str>,
    cap: usize,
) -> serde_json::Value {
    if input.hook_event_name != "UserPromptSubmit" {
        return serde_json::json!({"continue": true});
    }
    match assist_context.filter(|context| !context.is_empty() && context.len() <= cap) {
        Some(context) => serde_json::json!({
            "hookSpecificOutput": {
                "hookEventName": "UserPromptSubmit",
                "additionalContext": context
            }
        }),
        None => serde_json::json!({"continue": true}),
    }
}

pub fn stop_output(input: &HookInput, mandatory_proof_open: bool) -> serde_json::Value {
    if mandatory_proof_open && !input.stop_hook_active {
        serde_json::json!({
            "decision": "block",
            "reason": "Mandatory proof is still open for this Kernel-controlled run. Run the declared verifier or report BLOCKED."
        })
    } else if mandatory_proof_open {
        serde_json::json!({
            "continue": true,
            "systemMessage": "Orchestrator proof remains open; continuation already attempted. Report BLOCKED instead of looping."
        })
    } else {
        serde_json::json!({"continue": true})
    }
}

pub fn handshake_event(input: &HookInput) -> Event {
    let run_id: String = input
        .session_id
        .chars()
        .map(|character| {
            if character.is_ascii_alphanumeric() || matches!(character, '-' | '_') {
                character
            } else {
                '_'
            }
        })
        .collect();
    Event {
        schema_version: 1,
        event_id: format!("hook-{run_id}-{}", now_ms()),
        run_id,
        kind: "hook_handshake".into(),
        repository_digest: "not-collected".into(),
        timestamp_ms: now_ms(),
    }
}

fn valid_id(value: &str) -> bool {
    !value.is_empty()
        && value != "."
        && value != ".."
        && value
            .chars()
            .all(|character| character.is_ascii_alphanumeric() || matches!(character, '-' | '_'))
}

fn now_ms() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis()
}

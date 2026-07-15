use codex_orchestrator_kernel::*;

fn input(event: &str, active: bool) -> HookInput {
    HookInput {
        session_id: "session-1".into(),
        hook_event_name: event.into(),
        stop_hook_active: active,
    }
}

#[test]
fn direct_shadow_adds_no_model_context() {
    assert_eq!(
        user_prompt_output(&input("UserPromptSubmit", false), None, 64),
        serde_json::json!({"continue": true})
    );
}

#[test]
fn assist_context_is_bounded() {
    let short = user_prompt_output(&input("UserPromptSubmit", false), Some("use explorer"), 64);
    assert_eq!(
        short["hookSpecificOutput"]["hookEventName"],
        "UserPromptSubmit"
    );
    let long = user_prompt_output(
        &input("UserPromptSubmit", false),
        Some("this is too long"),
        4,
    );
    assert_eq!(long, serde_json::json!({"continue": true}));
}

#[test]
fn stop_continues_at_most_once() {
    assert_eq!(
        stop_output(&input("Stop", false), true)["decision"],
        "block"
    );
    let second = stop_output(&input("Stop", true), true);
    assert_eq!(second["continue"], true);
    assert!(second.get("decision").is_none());
}

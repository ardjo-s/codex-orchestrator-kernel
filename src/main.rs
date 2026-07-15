use codex_orchestrator_kernel::{
    DEFAULT_POLICY, Envelope, HookInput, Policy, append_event, decide, handshake_event, replay,
    state_root, stop_output, user_prompt_output,
};
use serde::de::DeserializeOwned;
use serde_json::json;
use std::io::{self, Read};

fn main() {
    if let Err(error) = run() {
        eprintln!("codex-orchestrator: {error}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), String> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let policy = Policy::parse(DEFAULT_POLICY)?;
    match args.as_slice() {
        [command] if command == "decide" => {
            let envelope: Envelope = read_stdin()?;
            print_json(&decide(&policy, &envelope))
        }
        [command, path] if command == "policy-check" => {
            let input = std::fs::read_to_string(path).map_err(|error| error.to_string())?;
            Policy::parse(&input)?;
            println!("OK");
            Ok(())
        }
        [command, event] if command == "hook" && event == "user-prompt-submit" => {
            let input: HookInput = read_stdin()?;
            let root = state_root()?;
            append_event(&root, &handshake_event(&input))?;
            let context = std::env::var("CODEX_ORCHESTRATOR_ASSIST_CONTEXT").ok();
            print_json(&user_prompt_output(
                &input,
                context.as_deref(),
                policy.limits.assist_context_bytes,
            ))
        }
        [command, event] if command == "hook" && event == "stop" => {
            let input: HookInput = read_stdin()?;
            let open = std::env::var("CODEX_ORCHESTRATOR_OPEN_PROOF").as_deref() == Ok("1");
            print_json(&stop_output(&input, open))
        }
        [command] if command == "doctor" => {
            let root = state_root()?;
            print_json(&json!({
                "status": "configured",
                "policy_schema": policy.schema_version,
                "state_root": root,
                "hook_activity": if root.join("runs").exists() { "verified-active" } else { "not-observed" },
                "execute_mode": "locked",
                "trust": "unknown"
            }))
        }
        [command, run_id] if command == "replay" => print_json(&replay(&state_root()?, run_id)?),
        [command, flag] if command == "benchmark" && flag == "--dry-run" => print_json(&json!({
            "status": "NO_PROVEN_ADVANTAGE",
            "lanes": ["native-root", "native-subagents", "kernel-assist"],
            "protocol": {
                "isolated_codex_exec_json": true,
                "randomized_lane_order": true,
                "separate_calibration_and_held_out": true,
                "full_task_tree_cost": true,
                "missing_metrics_are_non_comparable": true
            }
        })),
        _ => Err("usage: codex-orchestrator <decide|policy-check PATH|hook user-prompt-submit|hook stop|doctor|replay RUN_ID|benchmark --dry-run>".into()),
    }
}

fn read_stdin<T: DeserializeOwned>() -> Result<T, String> {
    let mut input = String::new();
    io::stdin()
        .read_to_string(&mut input)
        .map_err(|error| error.to_string())?;
    serde_json::from_str(&input).map_err(|error| error.to_string())
}

fn print_json(value: &impl serde::Serialize) -> Result<(), String> {
    println!(
        "{}",
        serde_json::to_string(value).map_err(|error| error.to_string())?
    );
    Ok(())
}

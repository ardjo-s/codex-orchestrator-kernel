---
name: orch-auto
description: Use measured, direct-first Codex orchestration guidance without changing the configured root model.
argument-hint: "[observe|assist]"
---

# Orchestrator Auto

Use the installed `codex-orchestrator` binary to classify the current phase from repository evidence.

1. Build the versioned envelope required by `codex-orchestrator decide`.
2. Keep `DIRECT_NATIVE` as the default.
3. Treat `RECOMMEND_DISPATCH` as advisory. Use only configured native subagents that satisfy the returned semantic roles.
4. Keep the configured root responsible for scope, integration, verification, and final synthesis.
5. Never claim `COMPLETE` for a Kernel-controlled run while mandatory proof is open or stale.

V0.1 does not actuate model selection. `execute` and `govern` remain locked until a documented native actuator and held-out benchmark both validate the category.

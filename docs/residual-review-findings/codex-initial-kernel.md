# Residual Review Findings

Source: ce-code-review autofix run `20260715-125243-6143d605` on `codex/initial-kernel`.

## Residual Review Findings

- P1 `src/main.rs:30` — Native prompt hook is disconnected from the decision engine ([#9](https://github.com/ardjo-s/codex-orchestrator-kernel/issues/9)).
- P1 `src/main.rs:41` — Stop proof state is global rather than session-ledger derived ([#3](https://github.com/ardjo-s/codex-orchestrator-kernel/issues/3)).
- P1 `src/main.rs:9` — Hook parse and persistence failures do not fail open ([#8](https://github.com/ardjo-s/codex-orchestrator-kernel/issues/8)).
- P1 `src/lib.rs:189` — Critical risk classification trusts incomplete caller evidence ([#5](https://github.com/ardjo-s/codex-orchestrator-kernel/issues/5)).
- P1 `src/lib.rs:250` — Event schema cannot reconstruct assignments and proof state ([#1](https://github.com/ardjo-s/codex-orchestrator-kernel/issues/1)).
- P1 `src/lib.rs:305` — Conflicting duplicate event IDs are silently discarded ([#11](https://github.com/ardjo-s/codex-orchestrator-kernel/issues/11)).
- P1 `src/lib.rs:106` — Assignment capsule omits required correlation and digests ([#7](https://github.com/ardjo-s/codex-orchestrator-kernel/issues/7)).
- P1 `src/lib.rs:35` — Policy validation permits relaxed hard promotion gates ([#2](https://github.com/ardjo-s/codex-orchestrator-kernel/issues/2)).
- P1 `src/main.rs:46` — Doctor invents configured and verified-active states ([#6](https://github.com/ardjo-s/codex-orchestrator-kernel/issues/6)).
- P1 `src/main.rs:58` — Benchmark command cannot run or summarize paired tasks ([#4](https://github.com/ardjo-s/codex-orchestrator-kernel/issues/4)).
- P1 `adapters/ai-dev-stack/README.md:3` — ai-dev-stack overlay has no executable schema or exporter ([#12](https://github.com/ardjo-s/codex-orchestrator-kernel/issues/12)).
- P1 `skills/orch-auto/SKILL.md:13` — Semantic native agent resolution is absent ([#10](https://github.com/ardjo-s/codex-orchestrator-kernel/issues/10)).

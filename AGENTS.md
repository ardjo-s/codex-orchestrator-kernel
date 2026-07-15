# Project Agent Instructions

## Purpose

This repository owns the public, model-neutral Codex Orchestrator Kernel and its native plugin adapter.

## Source of truth

1. `README.md`
2. `policies/default.toml`
3. `docs/architecture.md`
4. `docs/benchmark.md`

## Commands

```sh
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test
codex-orchestrator benchmark --dry-run
```

## Safety gates

- Never hard-code private providers, model names, credentials, or personal paths in the public core.
- Never claim native model actuation or hard Codex enforcement without a documented and tested surface.
- Keep `execute` and `govern` locked until both the actuator and held-out category gates pass.
- Publishing releases, changing external accounts, or relaxing a hard policy requires explicit owner approval.

## Local conventions

- Direct native execution is the fallback.
- Missing metrics are unknown, never zero.
- Prefer one boring crate and standard library code over new services or frameworks.
- Add one focused runnable test for non-trivial logic.

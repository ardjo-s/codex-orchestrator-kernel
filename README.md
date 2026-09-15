# Codex Orchestrator Kernel

A deterministic, direct-first policy layer for Codex.

The Kernel does **not** replace Codex routing and does not spend an LLM call choosing another LLM. It emits a local, explainable recommendation, records replayable proof state, and enables orchestration only when paired evaluation shows a net win over native root-only and native-subagent baselines.

## Current status

Experimental v0.1 tracer bullet:

- `observe` and `assist` are available.
- `execute` and `govern` are intentionally locked.
- Any supported Codex configuration can fall back safely to `DIRECT_NATIVE`.
- Native hooks are assistive guardrails, not a complete enforcement boundary.
- No category has yet earned a `VALIDATED` claim.

## Install from source

```sh
cargo install --path .
codex-orchestrator doctor
```

The Codex plugin lives at the repository root and is listed by `.agents/plugins/marketplace.json`. Hooks must be reviewed and trusted by the operator before Codex runs them.

## Use

Validate the default policy:

```sh
codex-orchestrator policy-check policies/default.toml
```

Request a decision by piping a complete envelope:

```sh
codex-orchestrator decide < examples/envelope.json
```

Inspect the benchmark protocol without spending tokens:

```sh
codex-orchestrator benchmark --dry-run
```

## Why direct-first?

Native Codex is already the cheapest correct route for many tasks. The Kernel recommends a bounded worker only when the category is validated, a verifier exists, authority permits it, and expected full-tree token savings clear policy. Missing evidence routes direct; it is never counted as zero cost.

See [architecture](docs/architecture.md), [benchmark contract](docs/benchmark.md), and [installation](docs/install.md).

## Development

```sh
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test
```

## License

MIT

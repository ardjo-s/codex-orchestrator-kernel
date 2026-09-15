# Installation

## Binary

```sh
cargo install --path .
codex-orchestrator doctor
```

Codex Desktop may not inherit the shell `PATH`. Before enabling hooks, verify the command from a minimal environment or install it at a stable location available to the app. The plugin includes a Windows command override.

## Plugin

The repository is a local marketplace through `.agents/plugins/marketplace.json`:

```sh
codex plugin marketplace add ardjo-s/codex-orchestrator-kernel
codex plugin add codex-orchestrator-kernel@codex-orchestrator-kernel
codex plugin list
```

The installed skill is `codex-orchestrator-kernel:orch-auto`. Review and trust its command hooks, then restart the app after marketplace/plugin changes.

`doctor` reports hook activity from a last-seen handshake as `verified-active` or `not-observed`. It reports trust as `unknown` unless Codex exposes a stable machine-readable trust API.

## Uninstall

```sh
codex plugin remove codex-orchestrator-kernel@codex-orchestrator-kernel
codex plugin marketplace remove codex-orchestrator-kernel
```

Remove the installed binary, then delete the Kernel state directory if desired. The Kernel does not rewrite user-owned Codex configuration.

Building from source requires Rust 1.85 or newer.

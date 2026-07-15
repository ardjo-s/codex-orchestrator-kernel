# Installation

## Binary

```sh
cargo install --path .
codex-orchestrator doctor
```

Codex Desktop may not inherit the shell `PATH`. Before enabling hooks, verify the command from a minimal environment or install it at a stable location available to the app. The plugin includes a Windows command override.

## Plugin

The repository is a local marketplace through `.agents/plugins/marketplace.json`. Add the repository as a marketplace in Codex, install `codex-orchestrator-kernel`, then review and trust its command hooks. Restart the app after marketplace/plugin changes.

`doctor` reports hook activity from a last-seen handshake as `verified-active` or `not-observed`. It reports trust as `unknown` unless Codex exposes a stable machine-readable trust API.

## Uninstall

Disable/remove the plugin, remove the installed binary, then delete the Kernel state directory if desired. The Kernel does not rewrite user-owned Codex configuration.

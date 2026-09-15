# Architecture

The Kernel is a local decision and evidence engine. It never calls a model to choose a model.

```mermaid
flowchart LR
  A[Codex task] --> B[Plugin skill or hook]
  B --> C[Deterministic Kernel]
  C --> D{Benefit proven?}
  D -- no --> E[DIRECT_NATIVE]
  D -- yes --> F[RECOMMEND_DISPATCH]
  E --> G[Per-run JSONL evidence]
  F --> G
  G --> H{Proof current?}
  H -- yes --> I[Kernel COMPLETE]
  H -- no --> J[REPAIR or BLOCKED]
```

## Boundaries

- The configured Codex root remains the judge and synthesizer.
- Semantic roles are host-resolved; the core knows no model-family ranking.
- V0.1 recommends but does not actuate a selected native subagent.
- A Stop hook may request one continuation, but it cannot guarantee Codex completion behavior. `stop_hook_active` prevents loops.
- State is partitioned under `runs/<run-id>/events.jsonl`; prompt bodies and diffs are not stored.

## Modes

- `observe`: records the counterfactual locally and adds no model context.
- `assist`: may add a byte-capped actionable recommendation.
- `execute`: parsed but locked until a documented actuator and category evaluation pass.
- `govern`: parsed but locked with `execute`.

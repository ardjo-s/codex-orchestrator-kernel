# Benchmark contract

The Kernel is better than native Codex only if a preregistered category passes a paired held-out evaluation against both relevant baselines.

## Lanes

1. Native root-only.
2. Native subagents.
3. Kernel assist, enacted by the declared harness.

Runs use isolated `codex exec --json` worktrees pinned to the same task revision, repository commit, Codex version, effective config, model/effort, cache condition, verifier, and judging method. Lane order is randomized. Calibration tasks and held-out tasks are separate. Desktop shadow observations cannot promote a category.

## Full-tree cost

Count hook/capsule overhead, input/output/reasoning/cache dimensions exposed by Codex, subagent work, verifier work, retries, and failed attempts. Missing dimensions make the comparison non-comparable.

## Gates

- zero critical false negatives;
- non-inferior accepted outcome rate against both relevant native baselines;
- at most 2% median trivial-task token overhead;
- at least 15% median medium/complex token savings;
- at least 90% classification accuracy;
- no rework, incident, or stale-proof regression;
- p90 token/latency and worst-category results remain within preregistered bounds.

`VALIDATED` requires every applicable gate. Otherwise the result is `NO_PROVEN_ADVANTAGE`, and execute mode stays locked.

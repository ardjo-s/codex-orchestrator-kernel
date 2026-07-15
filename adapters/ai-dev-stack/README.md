# ai-dev-stack adapter

`ai-dev-stack` remains the source of its private provider and model policy. An adapter should export only semantic capabilities and hard boundaries into a Kernel overlay.

The exporter must reject duplicate source keys, credentials, absolute personal paths, unknown roles, and attempts to relax public hard constraints. Absence of this adapter leaves the standalone Kernel operational in direct-native mode.

The current `ai-dev-stack` routing manifest must resolve its duplicate `lanes.build.cursor` key before a runtime exporter is enabled. That repair belongs in a separate narrow change because the working manifest already contains unrelated edits.

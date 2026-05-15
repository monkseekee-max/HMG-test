<!-- hmg:autonomous-memory:start -->
## HMG Autonomous Memory Policy
When the `hmg` MCP server is available, use it autonomously as the coding-agent long-term memory layer. Do not wait for the user to explicitly ask for HMG before using memory.

At task start:
- Call `hmg/memory_agent_brief` to retrieve branch-aware current status, handoff notes, decisions, root causes, constraints, risks, and next steps.
- Use `domain_pack_id = "software-engineering"`.
- Infer scope from the current repository and git branch when possible; default to tenant `tenant-acme`, workspace `platform`, and the current repository name unless the user or project context says otherwise.

Before editing code:
- Recall prior decisions, root causes, rejected approaches, and governance notes related to the files or symbols being changed.
- Prefer narrow branch/task context over broad recall.

When durable information appears:
- Call `hmg/memory_handoff` or `hmg/memory_memorize` for decisions, root causes, constraints, validation outcomes, risks, and next steps.
- Do not memorize ephemeral command output, secrets, raw tokens, or noisy intermediate observations.

When memory is stale or unsafe:
- Use `hmg/memory_correct` instead of writing conflicting facts.
- Use `hmg/memory_govern` for sensitive, unsafe, or audit-only memory.

At task end:
- Call `hmg/memory_handoff` with what changed, why, validation run, remaining risks, and next steps.
<!-- hmg:autonomous-memory:end -->

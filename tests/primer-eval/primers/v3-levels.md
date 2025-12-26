---
name: "V3: Constraint Levels"
version: "3.0"
tokens: 95
description: Adds constraint level definitions
tags: [constraints, levels, moderate]
---

This project uses ACP (AI Context Protocol)—a structured cache of codebase metadata (symbols, constraints, call graphs, domains) queryable via CLI instead of scanning files.

BEFORE editing any file: acp constraints <path>

Constraint levels:
  frozen → NEVER modify
  restricted → explain changes, wait for approval

Annotations in code: @acp:lock, @acp:ref, @acp:style, @acp:hack

To explore: acp query symbol|file|domain|callers <n>
To learn more: acp knowledge "question"

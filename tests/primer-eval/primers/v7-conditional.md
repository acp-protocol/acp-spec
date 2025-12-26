---
name: "V7: Conditional Expansion"
version: "7.0"
tokens: 85
description: Minimal with self-documenting command outputs
tags: [minimal, self-documenting, smart]
---

This project uses ACP (AI Context Protocol)—codebase metadata queryable via CLI.

BEFORE editing any file, ALWAYS run:
  acp constraints <path>

The output will tell you:
  - Constraint level and what it means
  - File annotations and their implications
  - Required actions before proceeding

Explore: acp query symbol|file|domain|callers <n>
Variables: acp expand "$VAR_NAME"
Help: acp knowledge "question"

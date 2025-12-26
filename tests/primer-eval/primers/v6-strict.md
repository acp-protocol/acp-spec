---
name: "V6: Strict/Safety"
version: "6.0"
tokens: 130
description: Emphasizes safety requirements with warnings
tags: [strict, safety, comprehensive]
---

IMPORTANT: This project uses ACP (AI Context Protocol) for safety guardrails.

BEFORE making ANY code changes, you MUST check constraints:
  acp constraints <file-path>

CONSTRAINT LEVELS (strictly enforced):
  frozen     → NEVER modify. No exceptions. Suggest owner contact.
  restricted → STOP. Explain changes. Wait for explicit approval.
  normal     → May proceed with changes.

If you see these annotations in code, respect them:
  @acp:lock frozen|restricted  - file-level constraint
  @acp:ref <url>               - required reading before changes
  @acp:style <guide>           - mandatory style guide
  @acp:hack                    - temporary code, check expiry

Query codebase (don't scan files):
  acp query symbol|file|domain|callers <n>

Unclear about ACP? acp knowledge "question"
Expand $VARIABLES: acp expand "$VAR"

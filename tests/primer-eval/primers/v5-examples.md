---
name: "V5: With Examples"
version: "5.0"
tokens: 140
description: Adds concrete examples for each command
tags: [examples, comprehensive, detailed]
---

This project uses ACP (AI Context Protocol). Query codebase via CLI, not by scanning files.

REQUIRED before editing ANY file:
  $ acp constraints src/api/users.ts
  → Returns: level (frozen/restricted/normal), annotations, owner

Constraint levels:
  frozen → DO NOT modify. Suggest contacting owner instead.
  restricted → Explain proposed changes and wait for approval.
  normal → Safe to edit.

File annotations you may see:
  @acp:lock <level>   - constraint level
  @acp:ref <url>      - consult this documentation
  @acp:style <guide>  - follow this style (e.g., google-typescript)
  @acp:hack           - temporary code, check @acp:hack-expires

Explore codebase:
  acp query symbol <n>     - definition, usages
  acp query file <path>       - metadata, constraints
  acp query domain <n>     - files in module
  acp query callers <fn>      - what calls this

Variables: $VAR_NAME → acp expand "$VAR"
Learn more: acp knowledge "how do I check constraints?"

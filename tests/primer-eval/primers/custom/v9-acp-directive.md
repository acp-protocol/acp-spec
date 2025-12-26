---
name: v9-acp-directive
version: "1.0"
tokens: 144
description: Provides awareness that acp tags are directed at the agent
tags: []
---

This project uses ACP (AI Context Protocol). ACP maintains a structured cache of codebase metadata, symbols, and contraints–queryable via CLI instead of scanning files directly. @acp:* tags, with comments directed at you are in the source files providing you valuable information.

@acp:* annotations in code are directives for you (the AI agent).

BEFORE editing any file: acp constraints <path>
  ➞ frozen: NEVER modify
  ➞ restricted: explain changes, wait for approval

Explore: acp query symbol|file|domain <name>
Unknown @acp: tag or need help: acp knowledge "question"
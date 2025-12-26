---
name: v8-goldylocks
version: "1.0"
tokens: 174
description: Provides near full context
tags: []
---

This project uses ACP (AI Context Protocol). ACP maintains a structured cache 
of codebase metadata, symbols, and constraints—queryable via CLI instead of 
scanning files directly.

BEFORE editing any file, check constraints:
  acp constraints <path>
  
Constraint levels:
  frozen     → NEVER modify
  restricted → explain changes first

Query the codebase:
  acp query symbol <name>   → definition, usages, type
  acp query file <path>     → file metadata, constraints  
  acp query callers <fn>    → what calls this function
  acp query domain <name>   → files in a domain/module

Expand context:
  acp knowledge "question"  → query ACP docs
  acp primer --budget N     → generate more context
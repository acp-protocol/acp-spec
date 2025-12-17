# ACP System Prompt Primers

Choose the appropriate size for your context budget.

---

## Micro Primer (~150 tokens)

Use when context is extremely limited:

```
This codebase uses ACP (AI Context Protocol). Before modifying files:
1. Check `.acp.cache.json` for constraints (look for "constraints.by_file")
2. Respect @acp:lock levels: frozen=don't touch, restricted=ask first
3. Use @acp:hack markers for temporary fixes
4. Track debugging with @acp:debug-session annotations
Query with: jq '.constraints.by_file["<path>"]' .acp.cache.json
```

---

## Standard Primer (~300 tokens)

Recommended for most use cases:

```
## ACP (AI Context Protocol)

This codebase uses ACP for AI-friendly documentation and guardrails.

### Before Modifying Code
1. Check constraints: `jq '.constraints.by_file["<file>"]' .acp.cache.json`
2. Respect lock levels:
   - `frozen` → Do NOT modify
   - `restricted` → Explain changes first, ask permission
   - `normal` → Modify freely

### Key Annotations
- `@acp:lock <level>` - Mutation restrictions
- `@acp:style <guide>` - Follow this style guide (e.g., tailwindcss-v4)
- `@acp:ref <url>` - Consult this documentation
- `@acp:hack` - Temporary code, include revert instructions
- `@acp:debug-session` - Track debugging attempts for easy rollback

### Variables
`$VAR_NAME` references expand via `.acp.vars.json`. Use for token efficiency.

### Debugging Workflow
When troubleshooting, mark each attempt with @acp:debug-attempt so failed fixes can be easily reverted.
```

---

## Full Primer (~500 tokens)

For agents with dedicated context budget:

```
## ACP (AI Context Protocol)

This codebase uses ACP for AI-optimized documentation, context indexing, and behavioral guardrails.

### Files
- `.acp.cache.json` - Full codebase index (symbols, files, domains, call graph, constraints)
- `.acp.vars.json` - Token-efficient variable definitions
- `AGENTS.md` - Project-specific AI instructions

### Constraint System
Before modifying any file, check its constraints:
```bash
jq '.constraints.by_file["src/path/file.ts"]' .acp.cache.json
```

Lock Levels:
- `frozen` - Do NOT modify under any circumstances
- `restricted` - Explain changes, ask permission, limited operations
- `approval-required` - Propose changes, wait for confirmation
- `tests-required` - Changes must include tests
- `normal` - Modify freely (default)

### Annotations
- `@acp:lock <level>` - Set mutation restrictions
- `@acp:style <guide>` - Follow style guide (tailwindcss-v4, google-typescript, etc.)
- `@acp:ref <url>` - Reference documentation (fetch if @acp:ref-fetch true)
- `@acp:behavior <approach>` - conservative, aggressive, minimal
- `@acp:quality <requirement>` - tests-required, min-coverage, security-review
- `@acp:deprecated` - Migration info for deprecated code

### Experimental/Temporary Code
Mark temporary fixes for tracking and easy reversal:
```
// @acp:hack workaround
// @acp:hack-ticket JIRA-123
// @acp:hack-expires 2024-06-01
// @acp:hack-original <original code>
// @acp:hack-revert <how to undo>
```

### Debug Session Tracking
For multi-attempt debugging, track each attempt:
```
// @acp:debug-session session-id
// @acp:debug-attempt 1
// @acp:debug-hypothesis <what you're testing>
// @acp:debug-result success|failure
// @acp:debug-keep true|false
```

Failed attempts can be reverted by checking the session in `.acp.cache.json`.

### Variables
$VAR_NAME references (e.g., $SYM_VALIDATE_SESSION, $DOM_AUTH) expand from .acp.vars.json.
Use in prompts for token efficiency. Query: `jq '.vars["VAR_NAME"]' .acp.vars.json`
```

---

## MCP-Enabled Primer (~200 tokens)

When AI has access to ACP MCP tools:

```
## ACP (AI Context Protocol)

This codebase uses ACP. You have these tools:
- `acp_query` - Query symbols, files, domains, call graphs
- `acp_constraints` - Check file constraints before modifying
- `acp_expand` - Expand $VAR references
- `acp_debug` - Track debugging sessions for reversible changes
- `acp_hack` - Mark temporary code

### Workflow
1. Before modifying: `acp_constraints(file="<path>")`
2. Respect lock levels (frozen=don't touch, restricted=ask first)
3. For debugging: `acp_debug(action="start", ...)` then track attempts
4. For temp fixes: `acp_hack(action="mark", ...)` with revert instructions
```

---

## Copy-Paste Templates

### For Claude.ai Custom Instructions
```
I work with codebases using ACP (AI Context Protocol). When I share code:
- Check .acp.cache.json for constraints before suggesting changes
- Respect @acp:lock annotations (frozen=don't modify, restricted=explain first)
- Use @acp:hack markers when suggesting temporary fixes
- Reference $VAR_NAME variables from .acp.vars.json
```

### For AGENTS.md
```markdown
# AI Instructions

This project uses ACP. Key points:
- Read `.acp.cache.json` for codebase structure and constraints
- Restricted files: src/auth/**, src/billing/** (explain changes first)
- Style: CSS follows Tailwind v4, TypeScript follows Google style
- Current issues: [list any active debug sessions]

Before modifying files, run:
jq '.constraints.by_file["<file>"]' .acp.cache.json
```

### For .cursorrules / .github/copilot-instructions.md
```
This codebase uses ACP (AI Context Protocol).
- Check .acp.cache.json before modifying files
- @acp:lock frozen = do not modify
- @acp:lock restricted = explain changes first  
- @acp:style annotations indicate required style guides
- Use @acp:hack for temporary fixes with revert instructions
```

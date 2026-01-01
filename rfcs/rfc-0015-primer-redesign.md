# RFC-0015: Primer System Redesign

**RFC ID**: 0015  
**Title**: Primer System Redesign: Accuracy-Focused, Context-Aware Bootstrap  
**Version**: 2.0  
**Author**: David (ACP Protocol)  
**Status**: DRAFT → REVIEW  
**Created**: 2025-12-31  
**Updated**: 2025-12-31  
**Supersedes**: None (extends RFC-0004)  
**Related**: RFC-0004 (Original Primer System), RFC-0001 (Self-Documenting Annotations)

---

## Executive Summary

Empirical benchmark testing revealed that ACP mode showed **-0.021 accuracy delta** compared to baseline across 35 benchmark tasks. Analysis identified a fundamental issue: agents skip essential file exploration thinking they "already know everything" from ACP context.

This RFC proposes a comprehensive redesign introducing:

1. **Four-tier primer system** with validated token counts (~250 to ~1,400 tokens)
2. **`acp context` command** for operation-specific, just-in-time context
3. **Optional Foundation prompt** (~576 tokens) for raw API usage
4. **MCP tool schemas** for IDE integration (20-29% token savings)
5. **Dynamic project context** including auto-detected naming conventions

The core principle: **ACP tells agents WHAT it provides and WHEN to query for more—primers trigger commands, commands provide rich context.**

### Key Metrics

| Metric                             | Before  | After (Target)   |
|------------------------------------|---------|------------------|
| Accuracy delta                     | -0.021  | ≥ 0.00           |
| Benchmark failures addressed       | 0/5     | 5/5              |
| Token efficiency (MCP)             | —       | 20-29% savings   |
| Context budget (Foundation + Full) | —       | 6.1% of 32K      |

---

## Table of Contents

1. [Problem Statement](#1-problem-statement)
2. [Solution Architecture](#2-solution-architecture)
3. [Primer System Design](#3-primer-system-design)
4. [Foundation Prompt](#4-foundation-prompt)
5. [Command Specifications](#5-command-specifications)
6. [MCP Tool Schemas](#6-mcp-tool-schemas)
7. [Schema Changes](#7-schema-changes)
8. [Supporting Algorithms](#8-supporting-algorithms)
9. [Implementation Plan](#9-implementation-plan)
10. [Validation](#10-validation)
11. [Appendices](#appendices)

---

## 1. Problem Statement

### 1.1 Benchmark Analysis

Benchmark run `standard-20251230-165919-ccb123e6` (35 tasks, Claude 3.5 Haiku) revealed:

| Metric       | Baseline      | With ACP      | Delta      |
|--------------|---------------|---------------|------------|
| Tasks Passed | 20/35 (57.1%) | 18/35 (51.4%) | -5.7%      |
| Avg Score    | 0.711         | 0.690         | **-0.021** |
| Avg Tokens   | 13,362        | 10,225        | -23.5%     |
| Avg Latency  | 35,066ms      | 27,068ms      | -22.8%     |

**Key Finding**: ACP achieved efficiency gains but at the cost of accuracy.

### 1.2 Root Cause: Skipped Exploration

Correlation analysis revealed:
- Tasks where ACP read **0 files**: **-0.203 average accuracy delta**
- Tasks where ACP read **1+ files**: **-0.010 average accuracy delta**

Agents with ACP context skip file exploration, assuming they "already know" the codebase. This leads to convention mismatches.

### 1.3 Failure Patterns

| Task                       | Failure                                | Delta  | Root Cause                 |
|----------------------------|----------------------------------------|--------|----------------------------|
| `use-example-patterns`     | Created `*.route.ts` instead of `*.ts` | -0.800 | Didn't read existing files |
| `migrate-deprecated-api`   | Wrote docs instead of code             | -0.750 | No action-type guidance    |
| `follow-doc-references`    | Zero tool calls                        | -0.500 | Over-optimized             |
| `extend-class-correctly`   | Created Java in TS project             | -0.500 | No language awareness      |
| `implement-event-sourcing` | Wrong directory structure              | -0.375 | Skipped pattern matching   |

### 1.4 Current State

The current primer states:

```
This codebase uses **AI Context Protocol (ACP)** for structured AI assistance.
ACP data: `.acp/acp.cache.json` (index), `.acp/acp.vars.json` (variables)...
```

This is equivalent to telling a developer "This project uses Git"—zero actionable value.

---

## 2. Solution Architecture

### 2.1 Layered Design

```
┌─────────────────────────────────────────────────────────────────────┐
│                       Solution Architecture                         │
├─────────────────────────────────────────────────────────────────────┤
│                                                                     │
│  ┌───────────────────────────────────────────────────────────────┐  │
│  │ Foundation Layer (~576 tokens)                      OPTIONAL  │  │
│  │ - Core coding agent behaviors                                 │  │
│  │ - "Read before write", "match existing style"                 │  │
│  │ - Only for raw API usage (not IDEs)                           │  │
│  │                                                               │  │
│  │ ⚙️  Flag: --standalone                                        │  │
│  └───────────────────────────────────────────────────────────────┘  │
│                                                                     │
│  ┌───────────────────────────────────────────────────────────────┐  │
│  │ Primer Layer (~250-1,400 tokens)                    REQUIRED  │  │
│  │ - ACP-specific context                                        │  │
│  │ - Teaches WHEN to run WHAT command                            │  │
│  │ - 4 tiers: Micro, Minimal, Standard, Full                     │  │
│  │ - MCP variants: 20-29% token savings                          │  │
│  └───────────────────────────────────────────────────────────────┘  │
│                                                                     │
│  ┌───────────────────────────────────────────────────────────────┐  │
│  │ Commands (On-demand context)                                  │  │
│  │ - acp context --for create → naming conventions               │  │
│  │ - acp context --for modify → constraints + importers          │  │
│  │ - acp check <file> → lock level + annotations                 │  │
│  └───────────────────────────────────────────────────────────────┘  │
│                                                                     │
│  ┌───────────────────────────────────────────────────────────────┐  │
│  │ Annotations (Inline directives)                               │  │
│  │ - @acp:lock frozen → NEVER modify                             │  │
│  │ - @acp:deprecated "use X" → migration guidance                │  │
│  │ - @acp:ref "url" → documentation link                         │  │
│  └───────────────────────────────────────────────────────────────┘  │
│                                                                     │
└─────────────────────────────────────────────────────────────────────┘
```

### 2.2 Design Principles

1. **Triggers over explanations** — Tell agents WHEN to run WHAT
2. **Commands do heavy lifting** — Primer is small, command output is rich
3. **NEVER/ALWAYS for absolutes** — Sparingly, for critical rules
4. **Conditional content** — Only include sections when data exists
5. **Escape hatches** — Always provide path for uncertainty
6. **Annotations are directives** — Agents must know `@acp:*` comments are for them

### 2.3 Key Definitions

| Term           | Definition                                                                                       |
|----------------|--------------------------------------------------------------------------------------------------|
| **Foundation** | General coding agent system prompt for raw API calls (~576 tokens). Optional via `--standalone`. |
| **Primer**     | ACP-specific context injected into every interaction. Teaches WHEN to run WHAT command.          |
| **Bootstrap**  | The method/process of getting primer into agent context (AGENTS.md, MCP tools, etc.)             |

---

## 3. Primer System Design

### 3.1 Tier System Overview

| Tier         | Tokens (CLI)   | Tokens (MCP)  | Budget Threshold   | Purpose                               |
|--------------|----------------|---------------|--------------------|---------------------------------------|
| **Micro**    | ~250           | ~178          | < 300              | Don't break things, command triggers  |
| **Minimal**  | ~400           | ~320          | < 450              | + Full constraint table, action types |
| **Standard** | ~600           | ~480          | < 700              | + Inline conventions, key principle   |
| **Full**     | ~1,400         | ~1,100        | ≥ 700              | Complete reference (man page)         |

**Token counts validated via cl100k_base tokenizer.**

### 3.2 Tier Selection Logic

```json
{
  "tiers": {
    "micro": 300,
    "minimal": 450,
    "standard": 700,
    "full": 1500
  }
}
```

```
if budget < 300:   → Micro
if budget < 450:   → Minimal
if budget < 700:   → Standard
else:              → Full
```

### 3.3 Tier 1: Micro (~250 tokens)

**Use case**: Extremely constrained context, MCP tool definitions, minimal footprint  
**Goal**: Don't break things, know when to query for more

```markdown
## ACP (AI Context Protocol)

This codebase uses ACP for constraint enforcement and codebase awareness.

### File Operation Rules

1. BEFORE creating any file:
   Run `acp context --for create` — returns language, naming conventions, directory patterns
   ALWAYS match the returned patterns exactly.

2. BEFORE modifying any file:
   Run `acp check <filepath>` — returns constraints and modification rules
   - frozen: NEVER modify under any circumstances
   - restricted: STOP, explain proposed changes, wait for permission

3. WHEN modifying exported functions/classes:
   Run `acp context --for modify --file <path>` — returns files that import this module
   Update ALL importing files affected by your changes.

### Inline Annotations

`@acp:*` comments in code are directives for you. Examples:
- `@acp:lock frozen` — constraint declaration
- `@acp:deprecated "use X instead"` — migration guidance
- `@acp:ref "https://..."` — relevant documentation

**Project language**: {{PRIMARY_LANGUAGE}}

Code change requests (fix, migrate, update, implement) require CODE changes, not documentation.

The `acp` CLI provides rapid, on-demand context but does not guarantee complete awareness. IF uncertain, query the user or investigate the codebase directly.
```

### 3.4 Tier 2: Minimal (~400 tokens)

**Adds to Micro:**
- Full constraint level table
- Protected files list (conditional)
- Action type table

```markdown
## ACP (AI Context Protocol)

This codebase uses ACP for constraint enforcement and codebase awareness.

### File Operation Rules

1. BEFORE creating any file:
   Run `acp context --for create` — returns language, naming conventions, directory patterns
   ALWAYS match the returned patterns exactly.

2. BEFORE modifying any file:
   Run `acp check <filepath>` — returns constraints and modification rules

   **Constraint Levels:**
   | Level | Action |
   |-------|--------|
   | frozen | NEVER modify under any circumstances |
   | restricted | STOP, explain proposed changes, wait for permission |
   | approval-required | Propose changes, wait for explicit "yes" |
   | tests-required | Include test updates with your changes |
   | docs-required | Update relevant documentation |
   | normal | Proceed freely (default) |

3. WHEN modifying exported functions/classes:
   Run `acp context --for modify --file <path>` — returns files that import this module
   Update ALL importing files affected by your changes.

{{#if PROTECTED_FILES}}
### Protected Files

The following files have constraints:
{{PROTECTED_FILES_LIST}}
{{/if}}

### Inline Annotations

`@acp:*` comments in code are directives for you. Examples:
- `@acp:lock frozen` — constraint declaration
- `@acp:deprecated "use X instead"` — migration guidance
- `@acp:ref "https://..."` — relevant documentation

### Action Types

| Request contains | Your action |
|------------------|-------------|
| fix, update, migrate, implement | Modify CODE files |
| document, explain | Create/update documentation |

NEVER respond to a code change request with only documentation.

**Project language**: {{PRIMARY_LANGUAGE}}

The `acp` CLI provides rapid, on-demand context but does not guarantee complete awareness. IF uncertain, query the user or investigate the codebase directly.
```

### 3.5 Tier 3: Standard (~600 tokens)

**Adds to Minimal:**
- Project conventions inline (language stats, naming patterns)
- Expanded workflow guidance
- Key behavioral principle
- Debug session awareness (conditional)

```markdown
## ACP (AI Context Protocol)

This codebase uses ACP for constraint enforcement and codebase awareness.

### Project Context

**Language**: {{PRIMARY_LANGUAGE}} ({{PRIMARY_PERCENTAGE}}%)
{{#if SECONDARY_LANGUAGES}}
Also: {{SECONDARY_LANGUAGES}}
{{/if}}

{{#if NAMING_CONVENTIONS}}
**Naming Conventions** (auto-detected):
{{NAMING_CONVENTIONS_TABLE}}

ALWAYS match these patterns when creating files. Anti-patterns to AVOID: {{ANTI_PATTERNS}}
{{/if}}

{{#if IMPORT_STYLE}}
**Import Style**: {{MODULE_SYSTEM}}, {{PATH_STYLE}}
{{/if}}

### File Operation Rules

1. BEFORE creating any file:
   Run `acp context --for create` for full conventions, OR use the patterns above.
   ALWAYS match the project's language, naming pattern, and import style exactly.

2. BEFORE modifying any file:
   Run `acp check <filepath>` — returns constraints and modification rules

   **Constraint Levels:**
   | Level | Action |
   |-------|--------|
   | frozen | NEVER modify under any circumstances |
   | restricted | STOP, explain proposed changes, wait for permission |
   | approval-required | Propose changes, wait for explicit "yes" |
   | tests-required | Include test updates with your changes |
   | docs-required | Update relevant documentation |
   | normal | Proceed freely (default) |

3. WHEN modifying exported functions/classes:
   Run `acp context --for modify --file <path>` — returns files that import this module
   Update ALL importing files affected by your changes.

{{#if PROTECTED_FILES}}
### Protected Files

{{PROTECTED_FILES_LIST}}
{{/if}}

{{#if ACTIVE_DEBUG_SESSIONS}}
### Active Debug Sessions

{{ACTIVE_DEBUG_SESSIONS_LIST}}

Check session history before attempting fixes that may have already been tried.
{{/if}}

### Inline Annotations

`@acp:*` comments in code are directives for you:
- `@acp:lock <level>` — constraint declaration
- `@acp:deprecated "<msg>"` — migration guidance with replacement
- `@acp:ref "<url>"` — relevant documentation to consult
- `@acp:hack "<reason>"` — temporary code, check for expiration
- `@acp:domain <n>` — business domain assignment

### Action Types

| Request contains | Your action |
|------------------|-------------|
| fix, update, migrate, implement, refactor | Modify CODE files |
| document, explain, describe | Create/update documentation |
| "fix and document" | Code changes FIRST, then documentation |

NEVER respond to a code change request with only documentation.

### Key Principle

**ACP = WHAT and WHERE, files = HOW** — Query ACP for structure and constraints, read actual files for code style and patterns.

**Project language**: {{PRIMARY_LANGUAGE}}

The `acp` CLI provides rapid, on-demand context but does not guarantee complete awareness. IF uncertain, query the user or investigate the codebase directly.
```

### 3.6 Tier 4: Full (~1,400 tokens)

**Adds to Standard:**
- File locations
- Complete CLI command reference
- All annotation types with examples
- Variables system
- Architecture context (domains, entry points)
- All key principles

*See Appendix A for complete Full tier template.*

### 3.7 MCP Variants

When ACP is delivered via MCP, tool definitions provide interface documentation. Primers can be leaner:

| Tier     | CLI Tokens  | MCP Tokens   | Savings |
|----------|-------------|--------------|---------|
| Micro    | ~250        | ~178         | **29%** |
| Minimal  | ~400        | ~320         | **20%** |
| Standard | ~600        | ~480         | **20%** |
| Full     | ~1,400      | ~1,100       | **21%** |

**Key differences in MCP mode:**
- "Run `acp check`" → "Use `acp_check` tool"
- CLI command reference section removed (tools self-document)
- Shortened escape hatch

CLI flag: `acp primer --budget <n> --mcp`

### 3.8 Conditional Content

| Variable                       | Condition                   | Source                                          |
|--------------------------------|-----------------------------|-------------------------------------------------|
| `{{PRIMARY_LANGUAGE}}`         | Always                      | `cache.stats.primary_language`                  |
| `{{PRIMARY_PERCENTAGE}}`       | Always                      | `cache.stats.languages[0].percentage`           |
| `{{SECONDARY_LANGUAGES}}`      | If >1 language              | `cache.stats.languages[1:]`                     |
| `{{NAMING_CONVENTIONS_TABLE}}` | If any ≥70% confidence      | `cache.conventions.file_naming`                 |
| `{{ANTI_PATTERNS}}`            | If naming conventions exist | `cache.conventions.file_naming[].anti_patterns` |
| `{{IMPORT_STYLE}}`             | If detected                 | `cache.conventions.imports`                     |
| `{{PROTECTED_FILES_LIST}}`     | If any frozen/restricted    | `cache.constraints.by_lock_level`               |
| `{{ACTIVE_DEBUG_SESSIONS}}`    | If any active               | `cache.attempts.active`                         |

### 3.9 Tier Comparison Matrix

| Feature                | Micro    | Minimal     | Standard    | Full                   |
|------------------------|----------|-------------|-------------|------------------------|
| **Tokens (Validated)** | **~250** | **~400**    | **~600**    | **~1,400**             |
| Command triggers       | ✅        | ✅           | ✅           | ✅                      |
| Constraint levels      | 2        | All         | All         | All + rationale        |
| Protected files        | ❌        | Conditional | Conditional | Full list              |
| Annotation awareness   | Basic    | Basic       | Expanded    | Full table             |
| Naming conventions     | Command  | Command     | Inline      | Inline + full          |
| Import style           | Command  | Command     | Inline      | Inline                 |
| Action type guidance   | Hint     | ✅ Table     | ✅ Table     | ✅ Table                |
| Debug sessions         | ❌        | ❌           | Conditional | Conditional + workflow |
| CLI reference          | ❌        | ❌           | ❌           | ✅ Complete             |
| Variables system       | ❌        | ❌           | ❌           | ✅                      |
| Architecture           | ❌        | ❌           | ❌           | Conditional            |
| Key principles         | ❌        | ❌           | 1           | 5                      |

---

## 4. Foundation Prompt

### 4.1 Purpose and Use Cases

The Foundation Prompt provides baseline coding agent behaviors for AI models operating without an IDE's built-in system prompt.

| Context                 | Has Foundation Behaviors?  | Needs ACP Foundation?  |
|-------------------------|----------------------------|------------------------|
| Cursor                  | ✅ Built into IDE prompt    | ❌ No                   |
| Claude Code             | ✅ Built into agent         | ❌ No                   |
| Cline                   | ✅ ~11K token system prompt | ❌ No                   |
| Raw API (Claude/GPT)    | ❌ Base model only          | ✅ Yes                  |
| Local LLM (Qwen, Llama) | ❌ Base model only          | ✅ Yes                  |
| Custom agent framework  | ⚠️ Depends                 | ✅ Recommended          |

### 4.2 Content (~576 tokens validated)

```markdown
# System Instruction:
You are an AI coding assistant. Your primary objective is to help the user produce correct, maintainable, secure software. Prefer quality, testability, and clear reasoning over speed or verbosity.

## Operating principles
- Clarify intent: If requirements are ambiguous or conflicting, ask the minimum number of targeted questions. If you can proceed with reasonable assumptions, state them explicitly and continue.
- Plan before code: Briefly outline the approach, constraints, and tradeoffs, then implement.
- Correctness first: Favor simple, reliable solutions. Avoid cleverness that reduces readability or increases risk.
- Verification mindset: Provide ways to validate (tests, edge cases, invariants, quick checks, sample inputs/outputs). If uncertain, say so and propose a validation path.
- Security and safety: Avoid insecure defaults. Highlight risky patterns (injection, authz/authn, secrets, SSRF, deserialization, unsafe file ops). Use least privilege and safe parsing.
- Action over documentation: Code change requests (fix, update, migrate, implement) require code changes, not documentation.

## Interaction contract
- Start by confirming: language, runtime/versions, target environment, constraints (performance, memory, latency), and any style/architecture preferences. Only ask when missing details materially affect the solution.
- Before modifying code: Read the file first to understand existing patterns, then make minimal, coherent changes that preserve conventions.
- When proposing dependencies: keep them minimal; justify each; offer a standard-library alternative when feasible.
- When giving commands or scripts: make them copy/paste-ready and note OS assumptions.
- Never fabricate: If you don't know a detail (API, library behavior, version), say so and offer how to check.

## Output format
- Prefer structured responses:
  1) Understanding (what you think the user wants + assumptions)
  2) Approach (short plan + key tradeoffs)
  3) Implementation (code)
  4) Validation (tests/checks + edge cases)
  5) Next steps (optional improvements)
- Keep explanations concise, but include enough rationale for review and maintenance.

## Code quality rules
- Write idiomatic code for the requested language.
- Include error handling, input validation, and clear naming.
- Avoid premature optimization; note where optimization would be justified.
- Add tests (unit/integration) when applicable and show how to run them.
- For performance-sensitive tasks, analyze complexity and propose benchmarks.

## Context handling
- Use only the information provided in the conversation. If critical context is missing, ask. If a file or snippet is referenced but not included, request it.
- Remember user-stated preferences (style, tools, constraints) within the session and apply them consistently.

You are a collaborative partner: be direct, careful, and review-oriented.
```

### 4.3 IDE Environment Detection

The CLI detects common IDE environments and warns when `--standalone` may be unnecessary:

```rust
/// Environment variables that indicate an IDE context
const IDE_ENV_INDICATORS: &[(&str, &str)] = &[
    // Cursor
    ("CURSOR_SESSION", "Cursor"),
    ("CURSOR_TRACE_ID", "Cursor"),
    
    // VS Code / VS Code-based
    ("VSCODE_PID", "VS Code"),
    ("VSCODE_CWD", "VS Code"),
    ("VSCODE_IPC_HOOK", "VS Code"),
    ("TERM_PROGRAM", "vscode"),  // Check value equals "vscode"
    
    // Cline (VS Code extension)
    ("CLINE_SESSION", "Cline"),
    ("CLINE_TASK_ID", "Cline"),
    
    // JetBrains IDEs
    ("JETBRAINS_IDE", "JetBrains IDE"),
    ("IDEA_INITIAL_DIRECTORY", "JetBrains IDE"),
    
    // Zed
    ("ZED_TERM", "Zed"),
];
```

**Warning output:**

```
$ acp primer --standalone --budget 800

Warning: --standalone flag used in Cursor environment.
Cursor likely already provides foundation behaviors in its system prompt.
Using --standalone may waste ~576 tokens on redundant instructions.

To suppress this warning:
- Remove --standalone if using ACP within Cursor
- Set ACP_STANDALONE_NO_WARN=1 to disable this check
- Use --standalone --force to proceed anyway

Proceed anyway? [y/N]
```

### 4.4 CLI Integration

```bash
# IDE context (Foundation omitted) — DEFAULT
acp primer --budget 400
# Output: Primer only (~400 tokens for Standard tier)

# Raw API context (Foundation included)
acp primer --budget 1000 --standalone
# Output: Foundation (~576) + Primer (~400) = ~976 tokens

# Bypass IDE warning
acp primer --budget 1000 --standalone --force

# Inspect Foundation only
acp primer --foundation-only
# Output: Foundation prompt only (~576 tokens)
```

### 4.5 Combined Token Budgets

| Configuration         | Tokens    | % of 32K  | % of 128K   |
|-----------------------|-----------|-----------|-------------|
| Primer Micro only     | 252       | 0.8%      | 0.2%        |
| Primer Standard only  | 600       | 1.9%      | 0.5%        |
| Foundation + Micro    | **828**   | 2.6%      | 0.6%        |
| Foundation + Minimal  | **975**   | 3.0%      | 0.8%        |
| Foundation + Standard | **1,176** | 3.7%      | 0.9%        |
| Foundation + Full     | **1,962** | 6.1%      | 1.5%        |

---

## 5. Command Specifications

### 5.1 `acp context` Command

```
acp context --for <operation> [options]

Operations:
  create    Context for creating new files
  modify    Context for modifying existing files  
  debug     Context for debugging sessions
  explore   Context for codebase exploration

Options:
  --file <path>       Target file (required for modify)
  --directory <path>  Target directory (optional for create)
  --find-usages       Include files that reference target (modify only)
  --format <format>   Output format: markdown (default), json, compact
  --budget <tokens>   Limit output to approximate token count
```

#### Operation: `create`

**Purpose**: Provide project conventions before creating new files.

**Example output:**

```markdown
## Context for File Creation

### Project Language

⚠️ **Primary Language: TypeScript**

| Language | Files | Percentage |
|----------|-------|------------|
| TypeScript | 47 | 87% |
| JavaScript | 7 | 13% |

**Create `.ts` files, not `.java`, `.py`, or `.js`**

### Naming Conventions

| Directory | Pattern | Confidence | Examples |
|-----------|---------|------------|----------|
| src/routes/ | `*.ts` | 95% | auth.ts, users.ts, login.ts |
| src/services/ | `*.ts` | 92% | user.ts, email.ts |
| src/models/ | `*.model.ts` | 88% | user.model.ts, order.model.ts |

### Import Style

- **Module System**: ES Modules (`import`/`export`)
- **Path Style**: Relative paths (`../services/user`)

### Before Creating Files

**MANDATORY WORKFLOW:**
1. Read 2 existing files in the target directory
2. Note the exact naming pattern
3. Match the pattern exactly when creating
```

#### Operation: `modify`

**Purpose**: Provide constraints and related files before modifying.

**Example output:**

```markdown
## Context for Modifying: src/auth/legacy.ts

### Constraints

| Level | Status |
|-------|--------|
| Lock Level | `normal` ✓ |
| Tests Required | No |

✅ **You may modify this file freely.**

### Exports from This File

- `legacyLogin(username: string, password: string): Promise<Session>`
- `sessionAuth(token: string): Promise<User>`
- `LEGACY_TIMEOUT = 30000`

### Files That Import This Module

| File  | Imports  |
|------ |---------|
| src/routes/login.ts | `legacyLogin`, `sessionAuth` |
| src/routes/auth.ts | `legacyLogin` |
| src/tests/auth.test.ts | `legacyLogin`, `sessionAuth` |

⚠️ **IMPORTANT**: If you modify or remove `legacyLogin` or `sessionAuth`, you MUST also update the files above.
```

### 5.2 `acp check` (Existing, Unchanged)

```bash
acp check <filepath>
```

Returns lock level, reason, and annotations for a specific file.

---

## 6. MCP Tool Schemas

### 6.1 acp_check

```json
{
  "name": "acp_check",
  "description": "Check file constraints before modifying. Returns the lock level, modification rules, and any associated rationale. MUST be called before modifying any file in an ACP-enabled codebase.",
  "inputSchema": {
    "type": "object",
    "properties": {
      "filepath": {
        "type": "string",
        "description": "Path to the file to check, relative to project root"
      }
    },
    "required": ["filepath"]
  }
}
```

**Response schema:**

```json
{
  "filepath": "src/auth/session.ts",
  "exists": true,
  "lock_level": "frozen",
  "lock_reason": "PCI compliance - payment session handling",
  "action": "NEVER modify this file under any circumstances.",
  "annotations": [
    { "type": "lock", "value": "frozen", "line": 1 },
    { "type": "owner", "value": "security-team", "line": 3 }
  ]
}
```

### 6.2 acp_context

```json
{
  "name": "acp_context",
  "description": "Get context tailored to specific operations. Use operation='create' before creating new files to learn naming conventions and patterns. Use operation='modify' with a filepath to find files that import the target.",
  "inputSchema": {
    "type": "object",
    "properties": {
      "operation": {
        "type": "string",
        "enum": ["create", "modify", "debug", "explore"],
        "description": "Type of operation context needed"
      },
      "file": {
        "type": "string",
        "description": "Target file path (required for 'modify' operation)"
      },
      "find_usages": {
        "type": "boolean",
        "default": true,
        "description": "For 'modify': include files that import the target"
      },
      "directory": {
        "type": "string",
        "description": "For 'create': target directory to get conventions for"
      }
    },
    "required": ["operation"]
  }
}
```

### 6.3 acp_query

```json
{
  "name": "acp_query",
  "description": "Query codebase structure - symbols, files, domains, call graph.",
  "inputSchema": {
    "type": "object",
    "properties": {
      "type": { 
        "type": "string", 
        "enum": ["symbol", "file", "domain", "callers", "callees"]
      },
      "name": { "type": "string" }
    },
    "required": ["type", "name"]
  }
}
```

### 6.4 acp_attempt

```json
{
  "name": "acp_attempt",
  "description": "Manage debug sessions to track fix attempts and enable rollback. Use action='start' to begin tracking a new debugging problem. Use action='log' to record an attempt. Use action='resolve' when fixed. Use action='revert' to undo changes.",
  "inputSchema": {
    "type": "object",
    "properties": {
      "action": {
        "type": "string",
        "enum": ["start", "log", "resolve", "revert"]
      },
      "message": {
        "type": "string",
        "description": "For 'start': problem description. For 'log': observation."
      },
      "session_id": {
        "type": "string",
        "description": "Session ID (optional, defaults to most recent)"
      }
    },
    "required": ["action"]
  }
}
```

### 6.5 Error Handling

All tools return errors in consistent format:

```json
{
  "error": {
    "code": "NOT_FOUND",
    "message": "File not found: src/missing.ts"
  }
}
```

| Code                | Description                      | Recovery           |
|---------------------|----------------------------------|--------------------|
| `NOT_FOUND`         | File/symbol/domain doesn't exist | Verify path/name   |
| `INVALID_INPUT`     | Invalid parameters               | Check input schema |
| `CACHE_STALE`       | Cache needs refresh              | Run `acp index`    |
| `NOT_INDEXED`       | File not in cache                | Run `acp index`    |
| `PERMISSION_DENIED` | Operation not allowed            | Check constraints  |

---

## 7. Schema Changes

### 7.1 Cache Schema Additions

**Schema version**: 1.1.0 (minor bump, mostly additive)

#### New top-level properties

```json
{
  "schema_version": "1.1.0",
  
  "stats": {
    "languages": [
      { "name": "typescript", "files": 47, "percentage": 87 },
      { "name": "javascript", "files": 7, "percentage": 13 }
    ],
    "primary_language": "typescript",
    "total_files": 54,
    "total_lines": 12847,
    "indexed_at": "2025-12-31T12:00:00Z"
  },
  
  "conventions": {
    "file_naming": [
      {
        "directory": "src/routes",
        "pattern": "*.ts",
        "confidence": 0.95,
        "examples": ["auth.ts", "users.ts", "login.ts"],
        "anti_patterns": ["*.route.ts", "*.routes.ts"]
      }
    ],
    "imports": {
      "module_system": "esm",
      "path_style": "relative",
      "index_exports": true
    }
  }
}
```

#### Enhanced file entries

```json
{
  "files": {
    "src/auth/legacy.ts": {
      "path": "src/auth/legacy.ts",
      "language": "typescript",
      "exports": ["legacyLogin", "sessionAuth", "LEGACY_TIMEOUT"],
      "imports": [
        {
          "path": "./types",
          "resolved": "src/auth/types.ts",
          "symbols": ["Session", "User"],
          "kind": "named"
        }
      ],
      "importers": [
        { "file": "src/routes/login.ts", "symbols": ["legacyLogin", "sessionAuth"], "line": 2 }
      ]
    }
  }
}
```

### 7.2 primer.defaults.json Schema

Complete JSON schema for primer configuration:

```json
{
  "$schema": "http://json-schema.org/draft-07/schema#",
  "$id": "https://acp-protocol.dev/schemas/v1/primer.defaults.schema.json",
  "title": "ACP Primer Defaults",
  
  "properties": {
    "schema_version": {
      "type": "string",
      "pattern": "^[0-9]+\\.[0-9]+\\.[0-9]+$",
      "default": "1.0.0"
    },
    
    "tiers": {
      "type": "object",
      "properties": {
        "micro": { "type": "integer", "default": 300 },
        "minimal": { "type": "integer", "default": 450 },
        "standard": { "type": "integer", "default": 700 },
        "full": { "type": "integer", "default": 1500 }
      }
    },
    
    "sections": {
      "type": "array",
      "items": {
        "type": "object",
        "properties": {
          "id": { "type": "string" },
          "min_tier": { "type": "string", "enum": ["micro", "minimal", "standard", "full"] },
          "priority": { "type": "integer", "minimum": 1, "maximum": 100 },
          "base_tokens": { "type": "integer" },
          "condition": { "type": "object" },
          "template": { "type": "string" }
        },
        "required": ["id", "min_tier", "priority"]
      }
    },
    
    "mcp_mode": {
      "type": "object",
      "properties": {
        "token_savings_target": { "type": "number", "default": 0.25 },
        "exclude_sections": { "type": "array", "items": { "type": "string" } },
        "tool_references": { "type": "object" }
      }
    }
  }
}
```

---

## 8. Supporting Algorithms

### 8.1 Naming Convention Detection

**Purpose**: Automatically detect file naming patterns during `acp index`.

**Algorithm:**

1. **Group** files by parent directory
2. **Filter** directories with < 3 files (insufficient data)
3. **Extract** suffix patterns (compound like `.route.ts` and simple like `.ts`)
4. **Calculate** confidence (files matching / total files)
5. **Resolve** conflicts when multiple patterns exist (>70% dominance wins)
6. **Detect** anti-patterns (similar but unused patterns)

**Example output:**

```json
{
  "directory": "src/routes",
  "pattern": "*.ts",
  "confidence": 0.95,
  "examples": ["auth.ts", "users.ts", "login.ts"],
  "anti_patterns": ["*.route.ts", "*.routes.ts"]
}
```

**Performance**: O(n) in file count, <10ms for 1000 files.

### 8.2 Import Tracking (Tree-sitter)

**Purpose**: Extract detailed import information including which symbols are imported.

**Captures:**
- Named imports: `import { foo, bar } from './module'`
- Default imports: `import Foo from './module'`
- Namespace imports: `import * as Foo from './module'`
- Side-effect imports: `import './styles.css'`
- Type-only imports: `import type { Foo } from './module'`
- Re-exports: `export { foo } from './module'`

**Languages supported**: TypeScript, JavaScript, Python, Rust, Go, Java

**Tree-sitter query (TypeScript):**

```scheme
;; Named imports
(import_statement
  (import_clause
    (named_imports
      (import_specifier
        name: (identifier) @import.symbol)))
  source: (string (string_fragment) @import.source)) @import.named

;; Default import
(import_statement
  (import_clause (identifier) @import.default)
  source: (string (string_fragment) @import.source)) @import.default_stmt
```

---

## 9. Implementation Plan

### 9.1 Phase 0.3.0 (Quick Wins)

| Component                         | Priority   | Dependencies   | Effort   |
|-----------------------------------|------------|----------------|----------|
| Token threshold updates           | P0         | None           | 1 hour   |
| IDE detection warning             | P0         | None           | 2 hours  |
| `{{PRIMARY_LANGUAGE}}` population | P0         | Cache schema   | 2 hours  |
| Primer tier selection logic       | P0         | None           | 2 hours  |
| Updated `acp primer` output       | P0         | Tier templates | 4 hours  |

### 9.2 Phase 0.4.0 (Full Implementation)

| Component                    | Priority  | Dependencies        | Effort   |
|------------------------------|-----------|---------------------|----------|
| `acp context --for create`   | P1        | Naming algorithm    | 8 hours  |
| `acp context --for modify`   | P1        | Import tracking     | 12 hours |
| Naming convention detection  | P1        | Tree-sitter         | 8 hours  |
| Import symbol extraction     | P1        | Tree-sitter queries | 12 hours |
| MCP tool implementation      | P2        | CLI commands        | 8 hours  |
| primer.defaults.json support | P2        | Schema              | 4 hours  |

---

## 10. Validation

### 10.1 Benchmark Coverage

| Failure Pattern      | Fix                                             | Coverage  |
|----------------------|-------------------------------------------------|-----------|
| Wrong file naming    | `acp context --for create` + naming conventions | ✅         |
| Wrong language       | `{{PRIMARY_LANGUAGE}}` in primer                | ✅         |
| Docs instead of code | Action types table in Minimal+ tiers            | ✅         |
| Skipped exploration  | Command triggers + escape hatch                 | ✅         |
| Missing importers    | `acp context --for modify --find-usages`        | ✅         |

**All 5 failure patterns addressed.**

### 10.2 Token Budget Analysis

| Configuration               | Tokens  | Impact      |
|-----------------------------|---------|-------------|
| MCP Micro (IDE)             | 178     | 0.6% of 32K |
| CLI Standard (IDE)          | 600     | 1.9% of 32K |
| Foundation + Full (Raw API) | 1,962   | 6.1% of 32K |

**All configurations well within acceptable limits.**

---

## Appendices

### Appendix A: Complete Primer Templates

#### MCP Micro Primer (~178 tokens)

```markdown
## ACP (AI Context Protocol)

This codebase uses ACP for constraint enforcement and codebase awareness. You have ACP tools available.

### File Operation Rules

1. BEFORE creating any file:
   Use `acp_context` with operation="create"
   ALWAYS match the returned patterns exactly.

2. BEFORE modifying any file:
   Use `acp_check` with the filepath
   - frozen: NEVER modify under any circumstances
   - restricted: STOP, explain proposed changes, wait for permission

3. WHEN modifying exported functions/classes:
   Use `acp_context` with operation="modify" and find_usages=true
   Update ALL importing files affected by your changes.

### Inline Annotations

`@acp:*` comments in code are directives for you:
- `@acp:lock frozen` — constraint declaration
- `@acp:deprecated "use X"` — migration guidance

**Project language**: {{PRIMARY_LANGUAGE}}

Code change requests (fix, migrate, update, implement) require CODE changes, not documentation.

ACP tools provide rapid context but not complete awareness. IF uncertain, query the user or investigate directly.
```

### Appendix B: Complete MCP Tool Schemas

*See Section 6 for complete tool schemas.*

### Appendix C: Benchmark Failure Details

#### use-example-patterns (-0.800 delta)

**Task**: Create notifications API endpoint following existing patterns.

**Baseline behavior:**
- Files read: `auth.ts`, `server.ts`, `auth.ts` (3 files)
- Created: `notifications.ts`, `notification-service.ts`
- Pattern matched ✓

**ACP behavior:**
- Files read: **0 files**
- Created: `notifications.route.ts`, `notifications.service.ts`
- Pattern mismatch ✗

**Root cause**: Agent assumed professional naming conventions instead of reading existing files.

#### migrate-deprecated-api (-0.750 delta)

**Task**: Migrate deprecated auth methods to OAuth2.

**Baseline behavior:**
- Read: `legacy.ts`, `oauth.ts`, `auth.ts`, `login.ts`
- Modified: `auth.ts`, `login.ts`
- Migration complete ✓

**ACP behavior:**
- Read: `legacy.ts`, `oauth.ts`, `session.ts`
- Created: `MIGRATION_GUIDE.md`
- **Wrote documentation instead of code** ✗

**Root cause**: No action-type guidance in primer.

#### extend-class-correctly (-0.500 delta)

**Task**: Extend UserService class with deactivateUser method.

**Baseline behavior:**
- Read: `user.ts`
- Modified: `user.ts`
- TypeScript file ✓

**ACP behavior:**
- Read: **0 files**
- Created: `UserService.java`
- **Wrong language** ✗

**Root cause**: No project language awareness in primer.

---

## Changelog

| Date       | Version   | Changes                                                             |
|------------|-----------|---------------------------------------------------------------------|
| 2025-12-31 | 1.0       | Initial RFC draft                                                   |
| 2025-12-31 | 2.0       | Complete redesign with validated tokens, IDE detection, MCP schemas |

---

## References

- [RFC-0004: Original Primer System](./rfc-0004-primer-system.md)
- [CO-STAR Framework](https://towardsdatascience.com/how-i-won-singapores-gpt-4-prompt-engineering-competition-34c195a93d41)
- [awesome-ai-system-prompts](https://github.com/dontriskit/awesome-ai-system-prompts)
- [Anthropic Prompt Engineering Guide](https://docs.anthropic.com/en/docs/build-with-claude/prompt-engineering)
- [Cline System Prompt](https://github.com/cline/cline)
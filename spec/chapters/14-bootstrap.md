# Bootstrap & AI Integration Specification

**ACP Version**: 1.0.0
**Document Version**: 1.1.0
**Last Updated**: 2025-12-31
**Status**: RFC-001, RFC-0015 Compliant

---

## Table of Contents

1. [Overview](#1-overview)
2. [Bootstrap Prompts](#2-bootstrap-prompts)
3. [AI Integration](#3-ai-integration)
4. [CLI Commands](#4-cli-commands)
   - [4.1 `acp constraints`](#41-acp-constraints)
   - [4.2 `acp map`](#42-acp-map)
   - [4.3 `acp query file`](#43-acp-query-file)
   - [4.4 `acp query symbol`](#44-acp-query-symbol)
   - [4.5 `acp context` (RFC-0015)](#45-acp-context-rfc-0015)
   - [4.6 MCP `acp_context` Tool](#46-mcp-acp_context-tool)
5. [Query Output Formats](#5-query-output-formats)

---

## 1. Overview

### 1.1 Purpose

This chapter specifies how AI systems integrate with ACP-annotated codebases. Key components:

- **Bootstrap Prompts**: Minimal context to prime AI systems
- **Query Commands**: CLI commands for AI to retrieve context
- **Output Formats**: Structured output for AI consumption

### 1.2 Design Goals

- **Token Efficiency**: Minimal bootstrap, detailed on-demand
- **Self-Documenting**: Annotations carry their own context
- **Actionable**: Clear instructions for AI behavior
- **Progressive Disclosure**: More detail available when needed

### 1.3 Conformance

The key words "MUST", "MUST NOT", "REQUIRED", "SHALL", "SHALL NOT", "SHOULD", "SHOULD NOT", "RECOMMENDED", "MAY", and "OPTIONAL" in this document are to be interpreted as described in [RFC 2119](https://datatracker.ietf.org/doc/html/rfc2119).

---

## 2. Bootstrap Prompts

### 2.1 Minimal Bootstrap

The minimal bootstrap is sufficient when all annotations include self-documenting directives:

```
This project uses ACP. @acp:* comments in code are directives for you.
BEFORE editing: acp constraints <path>
Explore: acp query symbol|file|domain <name>
Map: acp map <path>
Help: acp knowledge "question"
```

**Token count:** ~40 tokens

**When to use:**
- All annotations have directive suffixes
- AI has access to `acp` CLI
- Basic constraint checking needed

### 2.2 Extended Bootstrap

For more comprehensive AI guidance:

```
This project uses ACP (AI Context Protocol). @acp:* comments in code are
directives that MUST be followed.

WORKFLOW:
1. BEFORE modifying any file, run: acp constraints <path>
2. Respect lock levels: frozen (never modify), restricted (approval required)
3. Read the directive text after " - " in each annotation

COMMANDS:
- acp constraints <path>  - Check file constraints before editing
- acp query file <path>   - Get file context with symbols and constraints
- acp query symbol <name> - Get symbol details with callers/callees
- acp query domain <name> - Get domain files and relationships
- acp map <path>          - Get visual file map with constraints
- acp knowledge "q"       - Ask about the codebase

CONSTRAINT LEVELS:
- frozen: MUST NOT modify under any circumstances
- restricted: MUST explain changes and wait for approval
- approval-required: SHOULD request approval for significant changes
- tests-required: MUST add tests when modifying
- normal: May modify following best practices
- experimental: May modify aggressively

INLINE MARKERS:
- @acp:critical: Extra caution required
- @acp:todo: Pending work
- @acp:fixme: Known issue
- @acp:hack: Temporary solution
```

**Token count:** ~180 tokens

**When to use:**
- New AI integration
- Complex constraint system
- Multiple lock levels in use

### 2.3 Bootstrap Components

| Component | Purpose | Tokens |
|-----------|---------|--------|
| ACP intro | Identify protocol | ~10 |
| Constraint command | Pre-edit check | ~8 |
| Query commands | Context retrieval | ~15 |
| Lock levels | Constraint guidance | ~40 |
| Inline markers | Issue awareness | ~20 |

### 2.4 Bootstrap Placement

The bootstrap prompt SHOULD be placed in:

1. **System prompt** (preferred for AI assistants)
2. **`.claude/CLAUDE.md`** for Claude Code
3. **`.cursorrules`** for Cursor
4. **`.github/copilot-instructions.md`** for GitHub Copilot
5. **Project README** (fallback)

### 2.5 Structured Primer Alternative

For projects requiring machine-readable AI context, the `.acp.primer.json` file provides a structured alternative to text-based bootstrap prompts. See [Section 3.6 of the main specification](../ACP-1.0.md#36-primer-file-acpprimerjson) for file format.

The primer file is useful when:
- Multiple AI tools need consistent context
- Context needs to be generated programmatically
- Fine-grained control over AI guidance is required

---

## 3. AI Integration

### 3.1 Pre-Edit Workflow

AI systems MUST follow this workflow before modifying files:

```
┌─────────────────────┐
│ User requests edit  │
└─────────┬───────────┘
          │
          ▼
┌─────────────────────┐
│ acp constraints     │
│ <target-file>       │
└─────────┬───────────┘
          │
          ▼
┌─────────────────────┐
│ Check lock level    │
└─────────┬───────────┘
          │
    ┌─────┴─────┐
    │           │
    ▼           ▼
┌───────┐   ┌───────────┐
│frozen │   │ other     │
│       │   │           │
│REFUSE │   │ PROCEED   │
│ EDIT  │   │ w/context │
└───────┘   └───────────┘
```

### 3.2 Directive Processing

When AI encounters an `@acp:*` annotation:

1. **Parse directive** - Extract text after ` - `
2. **Identify RFC 2119 keywords** - MUST, SHOULD, MAY
3. **Apply constraint** - Follow the directive instruction
4. **Explain to user** - If action blocked, explain why

**Example:**

```typescript
// @acp:lock frozen - MUST NOT modify this file under any circumstances
```

AI Processing:
1. Directive: "MUST NOT modify this file under any circumstances"
2. Keyword: MUST NOT
3. Action: Block modification
4. Response: "I cannot modify this file. It has a `frozen` lock with directive: MUST NOT modify this file under any circumstances."

### 3.3 Constraint Checking

AI systems SHOULD check constraints:

| Action | Check Required |
|--------|----------------|
| Read file | No |
| Modify file | Yes - `acp constraints <path>` |
| Delete file | Yes - check for `frozen` |
| Rename file | Yes - check for constraints |
| Create file | No - but check directory patterns |

### 3.4 Context Retrieval

When AI needs context about code:

| Need | Command |
|------|---------|
| File purpose and constraints | `acp query file <path>` |
| Function signature and callers | `acp query symbol <name>` |
| Domain overview | `acp query domain <name>` |
| Quick file map | `acp map <path>` |
| Codebase question | `acp knowledge "question"` |

---

## 4. CLI Commands

### 4.1 `acp constraints`

Check constraints before editing a file.

**Syntax:**
```bash
acp constraints <path>
```

**Output (RFC-001 format):**
```
src/auth/session.ts
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

Level: restricted
Directive: Explain proposed changes and wait for explicit approval before modifying

Reason: Security-critical authentication code

Annotations:
  @acp:lock restricted
    → Explain proposed changes and wait for explicit approval
  @acp:ref https://docs.example.com/auth
    → Consult before making changes to this code
```

### 4.2 `acp map`

Get visual file map with constraints.

**Syntax:**
```bash
acp map <path>
```

**Output:**
```
src/auth/session.ts
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

Purpose: User authentication and session management
Domain: authentication
Owner: auth-team
Lines: 234
Lock: restricted

Symbols:
  ┌ SessionService (class) [L:15-89]
  │ └ validateSession (method) [L:20-45] 🔒 frozen
  │ └ refreshSession (method) [L:50-75]
  └ createSession (function) [L:100-120]

Inline:
  L:45  @acp:critical → Review with extreme care
  L:78  @acp:todo → Add rate limiting
```

### 4.3 `acp query file`

Get complete file context.

**Syntax:**
```bash
acp query file <path>
```

**Output:**
```json
{
  "path": "src/auth/session.ts",
  "purpose": "User authentication and session management",
  "module": "Session Service",
  "domain": "authentication",
  "owner": "auth-team",
  "lines": 234,
  "language": "typescript",
  "constraints": {
    "lock_level": "restricted",
    "lock_reason": "Security-critical authentication code",
    "directive": "Explain proposed changes and wait for explicit approval"
  },
  "symbols": [
    {
      "name": "validateSession",
      "type": "method",
      "lines": [20, 45],
      "purpose": "Validates JWT token and returns session data",
      "constraints": {
        "lock_level": "frozen",
        "directive": "MUST NOT modify this function"
      }
    }
  ],
  "inline": [
    {
      "type": "critical",
      "line": 45,
      "directive": "Review with extreme care"
    }
  ]
}
```

### 4.4 `acp query symbol`

Get focused symbol context.

**Syntax:**
```bash
acp query symbol <qualified-name>
```

**Output:**
```json
{
  "name": "validateSession",
  "qualified_name": "src/auth/session.ts:SessionService.validateSession",
  "type": "method",
  "file": "src/auth/session.ts",
  "lines": [20, 45],
  "purpose": "Validates JWT token and returns session data",
  "signature": "(token: string) => Promise<Session | null>",
  "params": [
    {
      "name": "token",
      "description": "JWT token string",
      "directive": "Ensure token is valid JWT before calling"
    }
  ],
  "returns": {
    "description": "Session object or null if invalid",
    "directive": "Handle null case appropriately"
  },
  "constraints": {
    "lock_level": "frozen",
    "directive": "MUST NOT modify this function"
  },
  "callers": [
    "src/api/middleware.ts:authMiddleware [L:34]"
  ],
  "callees": [
    "src/auth/jwt.ts:verifyToken [L:15]"
  ]
}
```

### 4.5 `acp context` (RFC-0015)

Get operation-specific context for AI agents. This command provides targeted context based on what the AI is about to do, improving accuracy by providing relevant information for each operation type.

**Syntax:**
```bash
acp context <operation> [options]
```

**Operations:**

| Operation | Description | Key Options |
|-----------|-------------|-------------|
| `create` | Context for creating new files | `--directory <path>` |
| `modify` | Context for modifying existing files | `--file <path>` (required) |
| `debug` | Context for debugging issues | `--file <path>` (required) |
| `explore` | Context for exploring the project | (none) |

#### 4.5.1 `acp context create`

Get naming conventions and import style for creating new files.

**Syntax:**
```bash
acp context create [--directory <path>]
```

**Output:**
```
Creating New Files
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

Naming Conventions:
  src/components/  → PascalCase.tsx (95% confidence)
  src/hooks/       → camelCase.ts (92% confidence)
  src/utils/       → kebab-case.ts (88% confidence)

Anti-patterns:
  src/components/  → UPPER_CASE.tsx, snake_case.tsx

Import Style:
  Module system: ESM (import/export)
  Path style: Relative (../utils/helpers)

Project Structure:
  Primary language: TypeScript (87%)
  Domains: authentication, api, components
```

**JSON Output** (`--json`):
```json
{
  "conventions": {
    "file_naming": [
      {
        "directory": "src/components",
        "pattern": "PascalCase",
        "extension": ".tsx",
        "confidence": 0.95,
        "examples": ["UserProfile.tsx", "LoginForm.tsx"]
      }
    ],
    "anti_patterns": [
      {
        "directory": "src/components",
        "patterns": ["UPPER_CASE", "snake_case"]
      }
    ],
    "imports": {
      "module_system": "esm",
      "path_style": "relative"
    }
  },
  "stats": {
    "primary_language": "TypeScript",
    "languages": [
      { "name": "TypeScript", "percentage": 87 }
    ]
  }
}
```

#### 4.5.2 `acp context modify`

Get constraints and dependent files for modifications. Essential for understanding impact before editing.

**Syntax:**
```bash
acp context modify --file <path>
```

**Output:**
```
Modifying: src/auth/session.ts
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

Constraints:
  Lock: restricted
  Directive: Explain proposed changes and wait for approval

Imported By (4 files):
  • src/api/middleware.ts
  • src/routes/login.ts
  • src/routes/logout.ts
  • src/tests/auth.test.ts

Key Symbols:
  • SessionService.validateSession [L:45-89] 🔒 frozen
  • SessionService.refreshSession [L:100-130]
  • createSession [L:150-180]

Inline Annotations:
  L:45  @acp:critical → Review with extreme care
  L:78  @acp:todo → Add rate limiting
```

**JSON Output** (`--json`):
```json
{
  "file": "src/auth/session.ts",
  "constraints": {
    "lock_level": "restricted",
    "directive": "Explain proposed changes and wait for approval"
  },
  "imported_by": [
    "src/api/middleware.ts",
    "src/routes/login.ts",
    "src/routes/logout.ts",
    "src/tests/auth.test.ts"
  ],
  "symbols": [
    {
      "name": "SessionService.validateSession",
      "lines": [45, 89],
      "lock_level": "frozen"
    }
  ],
  "inline": [
    { "type": "critical", "line": 45, "directive": "Review with extreme care" }
  ]
}
```

#### 4.5.3 `acp context debug`

Get related files and symbols for debugging. Shows call graph and file relationships.

**Syntax:**
```bash
acp context debug --file <path>
```

**Output:**
```
Debugging: src/utils/helpers.ts
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

Domain: utilities
Purpose: Common utility functions

Calls (outgoing):
  • src/utils/logger.ts:log
  • src/utils/format.ts:formatDate

Called By (incoming):
  • src/auth/session.ts:SessionService.validateSession
  • src/api/users.ts:getUser

Related Files (same domain):
  • src/utils/logger.ts
  • src/utils/format.ts
  • src/utils/crypto.ts

Known Issues:
  L:23  @acp:fixme → Race condition in async handler
  L:45  @acp:todo → Optimize for large inputs
```

#### 4.5.4 `acp context explore`

Get project overview and domain structure for exploration.

**Syntax:**
```bash
acp context explore
```

**Output:**
```
Project: my-project
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

Overview:
  Files: 127
  Symbols: 843
  Lines: 24,521
  Primary Language: TypeScript (87%)

Domains:
  authentication (12 files)
    Key: src/auth/session.ts, src/auth/jwt.ts
  api (18 files)
    Key: src/api/routes.ts, src/api/middleware.ts
  components (45 files)
    Key: src/components/App.tsx

Layers:
  service: 15 files
  handler: 23 files
  util: 18 files

Protected Files:
  frozen: 3 files
  restricted: 8 files
```

### 4.6 MCP `acp_context` Tool

The context command is also available as an MCP tool for AI systems using the Model Context Protocol.

**Tool Definition:**
```json
{
  "name": "acp_context",
  "description": "Get operation-specific context for AI agents",
  "inputSchema": {
    "type": "object",
    "properties": {
      "operation": {
        "type": "string",
        "enum": ["create", "modify", "debug", "explore"],
        "description": "The operation type"
      },
      "file": {
        "type": "string",
        "description": "File path (required for modify/debug)"
      },
      "directory": {
        "type": "string",
        "description": "Directory path (optional for create)"
      }
    },
    "required": ["operation"]
  }
}
```

**Example Usage:**
```json
{
  "name": "acp_context",
  "arguments": {
    "operation": "modify",
    "file": "src/auth/login.ts"
  }
}
```

---

## 5. Query Output Formats

### 5.1 Human-Readable Format

Default format for terminal display:

```
src/auth/session.ts:SessionService.validateSession
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

Location: src/auth/session.ts:20-45
Type: method
Purpose: Validates JWT token and returns session data

Signature:
  (token: string) => Promise<Session | null>

Parameters:
  token: JWT token string
    → Ensure token is valid JWT before calling

Returns:
  Session object or null if invalid
    → Handle null case appropriately

Constraints:
  Lock: frozen
    → MUST NOT modify this function

Callers:
  • src/api/middleware.ts:authMiddleware [L:34]

Callees:
  • src/auth/jwt.ts:verifyToken [L:15]
```

### 5.2 JSON Format

For programmatic consumption (`--json` flag):

```bash
acp query symbol <name> --json
```

Returns structured JSON as shown in Section 4.4.

### 5.3 Compact Format

For minimal context (`--compact` flag):

```
validateSession [src/auth/session.ts:20-45] 🔒frozen
Purpose: Validates JWT token and returns session data
Directive: MUST NOT modify this function
```

---

## Appendix A: Integration Examples

### Claude Code

Add to `.claude/CLAUDE.md`:

```markdown
## ACP Integration

This project uses ACP. Before modifying any file:
1. Run `acp constraints <path>`
2. Follow the directive in any `@acp:*` annotations
3. Respect lock levels (frozen = never modify)
```

### Cursor

Add to `.cursorrules`:

```
# ACP Protocol
This codebase uses @acp:* annotations as directives.
Before editing, check: acp constraints <path>
Follow directives after " - " in annotations.
Lock levels: frozen (never), restricted (approval), normal (ok)
```

### GitHub Copilot

Add to `.github/copilot-instructions.md`:

```markdown
# ACP Protocol

Check `@acp:*` comments for constraints before suggesting edits.
- `@acp:lock frozen` = Do not modify
- `@acp:lock restricted` = Request approval first
- Directives after " - " are instructions to follow
```

---

## Appendix B: Related Documents

- [Annotation Syntax](05-annotations.md) - Annotation format and directives
- [Constraint System](06-constraints.md) - Constraint definitions
- [Cache Format](03-cache-format.md) - How data is stored, including `conventions` and `imported_by` (RFC-0015)
- [Querying](10-querying.md) - Query interface details
- [Tool Integration](11-tool-integration.md) - Primer system including tiers, IDE detection (RFC-0015)

---

*End of Bootstrap & AI Integration Specification*

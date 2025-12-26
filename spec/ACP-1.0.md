# AI Context Protocol (ACP) Specification

**Version**: 1.2.0
**Date**: 2025-12-21
**Status**: RFC-001 Compliant

---

## Table of Contents

1. [Overview](#1-overview)
2. [Design Principles](#2-design-principles)
3. [File Formats](#3-file-formats)
   - 3.1 [Cache File](#31-cache-file-acpcachejson)
   - 3.2 [Variables File](#32-variables-file-acpvarsjson)
   - 3.3 [Config File](#33-config-file-acpconfigjson)
4. [Annotation Syntax](#4-annotation-syntax)
   - 4.1 [EBNF Grammar](#41-ebnf-grammar)
   - 4.2 [Examples](#42-examples)
   - 4.3 [Reserved Namespaces](#43-reserved-namespaces)
   - 4.4 [Annotation Scoping](#44-annotation-scoping)
   - 4.5 [Extension Namespaces](#45-extension-namespaces)
   - 4.6 [Parsing Algorithm](#46-parsing-algorithm)
5. [Constraint System](#5-constraint-system)
   - 5.1 [Overview](#51-overview)
   - 5.2 [Lock Levels](#52-lock-levels)
   - 5.3 [Style Constraints](#53-style-constraints)
   - 5.4 [Behavior Constraints](#54-behavior-constraints)
   - 5.5 [Quality Requirements](#55-quality-requirements)
   - 5.6 [Constraint Violation Tracking](#56-constraint-violation-tracking-optional)
6. [Variable System](#6-variable-system)
   - 6.1 [Overview](#61-overview)
   - 6.2 [Syntax](#62-syntax)
   - 6.3 [Variable Types](#63-variable-types)
   - 6.4 [Modifiers](#64-modifiers)
   - 6.5 [Error Handling](#65-error-handling)
   - 6.6 [Variable Scoping](#66-variable-scoping)
7. [Inheritance & Cascade](#7-inheritance--cascade)
   - 7.1 [Precedence Levels](#71-precedence-levels)
   - 7.2 [Merging Rules](#72-merging-rules)
   - 7.3 [Examples](#73-examples)
8. [File Discovery](#8-file-discovery)
   - 8.1 [Discovery Algorithm](#81-discovery-algorithm)
   - 8.2 [Exclusion Patterns](#82-exclusion-patterns)
   - 8.3 [Cache Building Details](#83-cache-building-details)
   - 8.4 [Language Detection](#84-language-detection)
   - 8.5 [Implementation Limits](#85-implementation-limits)
9. [Query Interface](#9-query-interface)
   - 9.1 [jq Queries](#91-jq-queries)
   - 9.2 [Command Line Interface](#92-command-line-interface)
   - 9.3 [MCP Server Interface](#93-mcp-server-interface)
10. [Conformance Levels](#10-conformance-levels)
11. [Error Handling](#11-error-handling)
12. [Versioning](#12-versioning)
13. [Bootstrap & AI Integration](#13-bootstrap--ai-integration) *(RFC-001)*

**Appendices:**
- [Appendix A: Complete Annotation Reference](#appendix-a-complete-annotation-reference)
- [Appendix B: JSON Schema Reference](#appendix-b-json-schema-reference)
- [Appendix C: Language Support](#appendix-c-language-support)

---

## 1. Overview

The AI Context Protocol (ACP) provides a standardized way to embed machine-readable documentation and constraints directly in source code. It enables AI systems to understand code structure, respect developer intentions, and navigate codebases efficiently.

### 1.1 Purpose

ACP addresses three core needs:

1. **Context Discovery**: Help AI find relevant code quickly
2. **Intent Communication**: Let developers specify how code should be modified
3. **Token Efficiency**: Minimize context sent to AI systems

### 1.2 Key Features

- **Annotations**: Structured metadata in source code comments
- **Constraints**: Advisory rules for AI behavior
- **Variables**: Token-efficient references to code elements
- **Cache**: Pre-computed codebase index
- **Query Interface**: Flexible ways to access metadata

### 1.3 Non-Goals

ACP is explicitly NOT:
- A runtime enforcement mechanism
- A type system or static analyzer
- A replacement for existing documentation
- A security or access control system

---

## 2. Design Principles

### 2.1 Advisory, Not Enforced

Constraints are advisory to AI systems. ACP does not provide runtime enforcement or access control. This design enables:
- Flexibility for AI decision-making
- No runtime overhead
- Simple implementation
- Trust-based collaboration

### 2.2 Token Efficiency

AI systems have context limits. ACP optimizes for minimal token usage through:
- Pre-computed cache files
- Variable references instead of inlining
- Summarized metadata

### 2.3 Language Agnostic

ACP works across programming languages via:
- Comment-based annotations (no language changes needed)
- JSON cache format (universal)
- Extensible language support

### 2.4 Incremental Adoption

Teams can adopt ACP gradually:
- Works without annotations (basic cache still useful)
- Annotations add value incrementally
- No all-or-nothing migration

### 2.5 Standard-Based

ACP follows established conventions:
- RFC 2119 keywords (MUST, SHOULD, MAY)
- Semantic versioning
- EBNF grammar
- JSON Schema

---

## 3. File Formats

<!-- CHANGED: v1 - Completed field specifications, added staleness detection, converted to snake_case per GAP-C3, GAP-C7, GAP-C8 -->

ACP uses six JSON files:

| File                      | Purpose                     | Required   |
|---------------------------|-----------------------------|------------|
| `.acp.cache.json`         | Pre-computed codebase index | Yes        |
| `.acp.vars.json`          | Variable definitions        | Optional   |
| `.acp.config.json`        | Configuration               | Optional   |
| `.acp/acp.attempts.json`  | Debugging attempt tracking  | Optional   |
| `.acp/acp.sync.json`      | Tool synchronization        | Optional   |
| `.acp.primer.json`        | AI context primer           | Optional   |

All JSON files MUST use `snake_case` for field names.

### 3.1 Cache File (`.acp.cache.json`)

The cache file contains a pre-computed index of the codebase.

#### 3.1.1 Top-Level Structure

```json
{
  "version": "1.0.0",
  "generated_at": "2024-12-17T15:30:00Z",
  "git_commit": "abc123def456",
  "project": {
    "name": "My Project",
    "root": "/path/to/project",
    "description": "Project description"
  },
  "stats": {
    "files": 42,
    "symbols": 387,
    "lines": 12450
  },
  "source_files": {
    "src/auth/session.ts": "2024-12-17T15:29:00Z",
    "src/utils/helpers.ts": "2024-12-17T14:20:00Z"
  },
  "files": { /* FileEntry objects */ },
  "symbols": { /* SymbolEntry objects */ },
  "graph": { /* CallGraph */ },
  "domains": { /* DomainEntry objects */ },
  "constraints": { /* ConstraintIndex */ }
}
```

#### 3.1.2 Top-Level Fields

| Field          | Type   | Required  |  Default | Description                             |
|----------------|--------|-----------|----------|-----------------------------------------|
| `version`      | string | ✓ MUST    | -        | SemVer version of ACP spec              |
| `generated_at` | string | ✓ MUST    | -        | ISO 8601 timestamp                      |
| `git_commit`   | string | ✗ MAY     | null     | Git commit SHA if in repo               |
| `project`      | object | ✓ MUST    | -        | Project metadata                        |
| `stats`        | object | ⚠ SHOULD  | {}       | Aggregate statistics                    |
| `source_files` | object | ✓ MUST    | -        | Map of file paths to modification times |
| `files`        | object | ✓ MUST    | -        | Map of file paths to FileEntry          |
| `symbols`      | object | ✓ MUST    | -        | Map of qualified names to SymbolEntry   |
| `graph`        | object | ⚠ SHOULD  | {}       | Call graph structure                    |
| `domains`      | object | ✗ MAY     | {}       | Domain classifications                  |
| `constraints`  | object | ⚠ SHOULD  | {}       | Constraint index for quick lookup       |

#### 3.1.3 FileEntry Specification

| Field       | Type     | Required   | Default  | Description                                        |
|-------------|----------|------------|----------|----------------------------------------------------|
| `path`      | string   | ✓ MUST     | -        | Relative path from project root                    |
| `module`    | string   | ⚠ SHOULD   | null     | Human-readable module name                         |
| `summary`   | string   | ⚠ SHOULD   | null     | Brief file description                             |
| `purpose`   | string   | ⚠ SHOULD   | null     | File purpose from @acp:purpose (RFC-001)           |
| `owner`     | string   | ✗ MAY      | null     | Team ownership from @acp:owner (RFC-001)           |
| `lines`     | integer  | ✓ MUST     | -        | Line count                                         |
| `language`  | string   | ✓ MUST     | -        | Programming language                               |
| `domains`   | string[] | ✗ MAY      | []       | Domain classifications                             |
| `layer`     | string   | ✗ MAY      | null     | Architectural layer                                |
| `stability` | string   | ✗ MAY      | null     | Stability level (stable, experimental, deprecated) |
| `exports`   | string[] | ⚠ SHOULD   | []       | Exported symbols (qualified names)                 |
| `imports`   | string[] | ⚠ SHOULD   | []       | Imported modules                                   |
| `inline`    | array    | ✗ MAY      | []       | Inline annotations (RFC-001)                       |

**Example:**
```json
{
  "path": "src/auth/session.ts",
  "module": "Session Management",
  "summary": "Handles user session lifecycle and validation",
  "purpose": "User session lifecycle and JWT validation",
  "owner": "security-team",
  "lines": 234,
  "language": "typescript",
  "domains": ["authentication"],
  "layer": "service",
  "stability": "stable",
  "exports": ["src/auth/session.ts:SessionService"],
  "imports": ["jsonwebtoken", "src/db/users"],
  "inline": [
    {
      "type": "critical",
      "line": 45,
      "directive": "Token validation - security boundary"
    },
    {
      "type": "todo",
      "line": 78,
      "directive": "Add rate limiting"
    }
  ]
}
```

#### 3.1.4 SymbolEntry Specification

<!-- CHANGED: v1 - Specified symbol qualification format per GAP-I17, Decision Q7 -->
<!-- CHANGED: v1.1 - Added RFC-001 fields: purpose, params, returns, throws, constraints -->

| Field            | Type       | Required   | Default   | Description                               |
|------------------|------------|------------|-----------|-------------------------------------------|
| `name`           | string     | ✓ MUST     | -         | Simple symbol name                        |
| `qualified_name` | string     | ✓ MUST     | -         | Format: `file_path:class.symbol`          |
| `type`           | string     | ✓ MUST     | -         | Symbol type (fn, class, const, etc.)      |
| `file`           | string     | ✓ MUST     | -         | Containing file path                      |
| `lines`          | [int, int] | ✓ MUST     | -         | [start_line, end_line]                    |
| `signature`      | string     | ⚠ SHOULD   | null      | Function signature if applicable          |
| `summary`        | string     | ⚠ SHOULD   | null      | Brief description                         |
| `purpose`        | string     | ⚠ SHOULD   | null      | Symbol purpose from @acp:fn/etc (RFC-001) |
| `params`         | array      | ✗ MAY      | []        | Parameter descriptions (RFC-001)          |
| `returns`        | object     | ✗ MAY      | null      | Return value description (RFC-001)        |
| `throws`         | array      | ✗ MAY      | []        | Exception descriptions (RFC-001)          |
| `constraints`    | object     | ✗ MAY      | null      | Symbol-level constraints (RFC-001)        |
| `async`          | boolean    | ✗ MAY      | false     | Whether async                             |
| `exported`       | boolean    | ✓ MUST     | -         | Whether exported                          |
| `visibility`     | string     | ✗ MAY      | "public"  | public/private/protected                  |
| `calls`          | string[]   | ✗ MAY      | []        | Symbols this calls (qualified names)      |
| `called_by`      | string[]   | ✗ MAY      | []        | Symbols calling this (qualified names)    |

**Symbol Qualification Format:**
- Format: `{relative_path}:{qualified_symbol}`
- Examples:
  - `src/auth/session.ts:SessionService.validateSession`
  - `src/utils/helpers.ts:formatDate`
  - `lib/core.py:CoreEngine.process`

**Example:**
```json
{
  "name": "validateSession",
  "qualified_name": "src/auth/session.ts:SessionService.validateSession",
  "type": "method",
  "file": "src/auth/session.ts",
  "lines": [45, 89],
  "signature": "(token: string) => Promise<Session | null>",
  "summary": "Validates JWT token and returns session",
  "purpose": "Validates JWT token and checks session store",
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
  "throws": [
    {
      "exception": "TokenExpiredError",
      "description": "When token is expired",
      "directive": "Catch and redirect to login"
    }
  ],
  "constraints": {
    "lock_level": "frozen",
    "directive": "MUST NOT modify validation logic"
  },
  "async": true,
  "exported": true,
  "visibility": "public",
  "calls": [
    "src/auth/jwt.ts:verifyToken",
    "src/db/sessions.ts:findSession"
  ],
  "called_by": [
    "src/api/middleware.ts:authMiddleware"
  ]
}
```

#### 3.1.5 Staleness Detection

<!-- ADDED: v1 - New subsection per GAP-C7, Decision Q3 -->

The cache includes metadata to detect when it becomes stale:

**Git-Aware Detection** (when `.git` directory exists):
- Compare cache `git_commit` field with current HEAD
- If different, cache is stale
- Recommendation: Rebuild cache

**Timestamp-Based Fallback**:
- Compare cache `generated_at` with source file modification times
- If any source file newer than cache, mark stale
- Recommendation: Warn and suggest rebuild

**Always Available**:
- `--force` flag to rebuild regardless of staleness

**Implementation Requirements:**
- Level 2+ implementations MUST implement staleness detection
- Level 2+ implementations SHOULD use git-aware detection when available
- Level 2+ implementations MUST fall back to timestamp-based when git unavailable

#### 3.1.6 CallGraph Structure

```json
{
  "forward": {
    "src/auth/session.ts:SessionService.validateSession": [
      "src/auth/jwt.ts:verifyToken",
      "src/db/sessions.ts:findSession"
    ]
  },
  "reverse": {
    "src/auth/jwt.ts:verifyToken": [
      "src/auth/session.ts:SessionService.validateSession"
    ]
  }
}
```

#### 3.1.7 DomainEntry Structure

```json
{
  "name": "authentication",
  "description": "User authentication and session management",
  "files": [
    "src/auth/session.ts",
    "src/auth/token.ts"
  ],
  "symbols": [
    "src/auth/session.ts:SessionService.validateSession",
    "src/auth/token.ts:generateToken"
  ]
}
```

#### 3.1.8 ConstraintIndex Structure

<!-- CHANGED: v1.1 - Added RFC-001 fields: directive, auto_generated -->

```json
{
  "by_file": {
    "src/auth/session.ts": {
      "lock_level": "restricted",
      "lock_reason": "Security critical",
      "directive": "Explain proposed changes and wait for approval",
      "auto_generated": false,
      "style": "google-typescript"
    },
    "src/config/production.ts": {
      "lock_level": "frozen",
      "lock_reason": "Production configuration",
      "directive": "MUST NOT modify this file under any circumstances",
      "auto_generated": false
    }
  },
  "by_lock_level": {
    "frozen": ["src/config/production.ts"],
    "restricted": ["src/auth/session.ts"]
  }
}
```

**Constraint Fields (RFC-001):**

| Field | Type | Required | Default | Description |
|-------|------|----------|---------|-------------|
| `lock_level` | string | ⚠ SHOULD | null | Lock level constraint |
| `lock_reason` | string | ⚠ SHOULD | null | Structured reason for lock |
| `directive` | string | ⚠ SHOULD | null | Self-documenting directive for AI (RFC-001) |
| `auto_generated` | boolean | ✗ MAY | false | True if directive was auto-generated (RFC-001) |
| `style` | string | ✗ MAY | null | Style guide constraint |

### 3.2 Variables File (`.acp.vars.json`)

<!-- CHANGED: v1 - Converted to snake_case per GAP-C8 -->

The variables file defines reusable references to code elements.

**Format:**
```json
{
  "version": "1.0.0",
  "variables": {
    "SYM_VALIDATE": {
      "type": "symbol",
      "value": "src/auth/session.ts:SessionService.validateSession"
    },
    "FILE_SESSION": {
      "type": "file",
      "value": "src/auth/session.ts"
    },
    "DOMAIN_AUTH": {
      "type": "domain",
      "value": "authentication"
    }
  }
}
```

**Field Specifications:**

| Field | Type | Required | Default | Description |
|-------|------|----------|---------|-------------|
| `version` | string | ✓ MUST | - | SemVer version of ACP spec |
| `variables` | object | ✓ MUST | - | Map of variable names to definitions |

**Variable Definition:**

| Field | Type | Required | Default | Description |
|-------|------|----------|---------|-------------|
| `type` | string | ✓ MUST | - | Variable type (symbol, file, domain) |
| `value` | string | ✓ MUST | - | Reference value (qualified name, path, etc.) |
| `description` | string | ✗ MAY | null | Human-readable description |

### 3.3 Config File (`.acp.config.json`)

<!-- CHANGED: v1 - Added error_handling configuration per GAP-C1, Decision Q1, converted to snake_case -->

The config file controls ACP behavior.

**Format:**
```json
{
  "version": "1.0.0",
  "include": ["src/**/*.ts", "lib/**/*.js"],
  "exclude": ["**/*.test.ts", "node_modules/**"],
  "error_handling": {
    "strictness": "permissive",
    "max_errors": 100,
    "auto_correct": false
  },
  "constraints": {
    "defaults": {
      "lock": "normal"
    },
    "track_violations": false,
    "audit_file": ".acp.violations.log"
  },
  "domains": {
    "authentication": {
      "patterns": ["src/auth/**", "lib/security/**"]
    },
    "database": {
      "patterns": ["src/db/**", "src/models/**"]
    }
  },
  "call_graph": {
    "include_stdlib": false,
    "max_depth": null,
    "exclude_patterns": ["**/test/**"]
  },
  "limits": {
    "max_file_size_mb": 10,
    "max_files": 100000,
    "max_annotations_per_file": 1000,
    "max_cache_size_mb": 100
  }
}
```

**Field Specifications:**

| Field            | Type     | Required  | Default   | Description                    |
|------------------|----------|-----------|-----------|--------------------------------|
| `version`        | string   | ✓ MUST    | -         | SemVer version of ACP spec     |
| `include`        | string[] | ⚠ SHOULD  | ["**/*"]  | Glob patterns to include       |
| `exclude`        | string[] | ⚠ SHOULD  | []        | Glob patterns to exclude       |
| `error_handling` | object   | ✗ MAY     | {}        | Error handling configuration   |
| `constraints`    | object   | ✗ MAY     | {}        | Constraint configuration       |
| `domains`        | object   | ✗ MAY     | {}        | Domain pattern definitions     |
| `call_graph`     | object   | ✗ MAY     | {}        | Call graph configuration       |
| `limits`         | object   | ✗ MAY     | {}        | Implementation limit overrides |

### 3.4 Attempts File (`.acp/acp.attempts.json`)

The attempts file tracks debugging sessions and troubleshooting history. It provides persistent storage for debugging context across sessions.

See [Chapter 13: Debug Sessions](chapters/13-debug-sessions.md) for full specification.

**Key fields:**

| Field         | Type     | Required  | Description                        |
|---------------|----------|-----------|-------------------------------------|
| `version`     | string   | ✓ MUST    | SemVer version of ACP spec         |
| `updated_at`  | string   | ✓ MUST    | ISO 8601 timestamp of last update  |
| `attempts`    | object   | ✓ MUST    | Active debugging attempts           |
| `checkpoints` | object   | ✓ MUST    | Saved restore points               |
| `history`     | array    | ✓ MUST    | Completed attempt history          |

### 3.5 Sync File (`.acp/acp.sync.json`)

The sync file configures tool synchronization and state sharing between ACP-aware tools.

See [Chapter 11: Tool Integration](chapters/11-tool-integration.md) for full specification.

**Key fields:**

| Field     | Type    | Required  | Description                         |
|-----------|---------|-----------|--------------------------------------|
| `version` | string  | ✓ MUST    | SemVer version of sync config       |
| `enabled` | boolean | ⚠ SHOULD  | Whether sync is active              |
| `tools`   | object  | ✗ MAY     | Tool-specific configurations        |
| `primer`  | object  | ✗ MAY     | Primer auto-generation settings     |

### 3.6 Primer File (`.acp.primer.json`)

The primer file provides structured AI context and bootstrapping information. It offers a machine-readable alternative to text-based bootstrap prompts.

See [Chapter 14: Bootstrap & AI Integration](chapters/14-bootstrap.md) for usage patterns.

**Key fields:**

| Field      | Type   | Required  | Description                          |
|------------|--------|-----------|---------------------------------------|
| `version`  | string | ✓ MUST    | SemVer version of primer schema      |
| `metadata` | object | ⚠ SHOULD  | Primer metadata and versioning       |
| `protocol` | object | ⚠ SHOULD  | ACP protocol awareness sections      |
| `project`  | object | ✗ MAY     | Project-specific context             |

---

## 4. Annotation Syntax

Annotations embed structured metadata in source code comments.

### 4.1 EBNF Grammar

```ebnf
annotation     = "@acp:" namespace [ ":" sub-namespace ] [ " " value ]
                 " - " directive
namespace      = identifier
sub-namespace  = identifier
identifier     = letter { letter | digit | "-" }
value          = quoted-string | unquoted-string
quoted-string  = '"' { any-char-except-quote } '"'
unquoted-string= { any-char-except-separator }
directive      = { any-char-except-newline }
```

**RFC-001 Directive Requirement:**

All `@acp:*` annotations MUST include a directive suffix that:
- Follows the tag and any parameters
- Is separated by ` - ` (space-dash-space)
- Contains actionable instructions for the AI agent
- Is written in imperative mood

### 4.2 Examples

**JavaScript/TypeScript:**
```javascript
/**
 * @acp:module "Session Management" - Core session handling for authentication
 * @acp:purpose User session lifecycle and JWT validation - Review before modifying auth flow
 * @acp:domain authentication - Part of authentication domain
 * @acp:lock restricted - Explain proposed changes and wait for approval
 * @acp:owner security-team - Contact for questions or significant changes
 */
```

**Python:**
```python
"""
@acp:module "Database Layer" - Core database abstraction layer
@acp:purpose Provides database abstraction - Ensure queries are optimized
@acp:domain data - Part of data access domain
@acp:layer repository - Repository pattern implementation
"""
```

**Rust:**
```rust
//! @acp:module "Core Engine" - Main processing engine
//! @acp:purpose Main processing engine - High performance required
//! @acp:domain processing - Part of processing domain
```

### 4.3 Reserved Namespaces

The following namespaces are reserved:

**File-Level Annotations:**

| Namespace          | Purpose                           | Directive Requirement               |
|--------------------|-----------------------------------|-------------------------------------|
| `@acp:module`      | Module/file metadata              | SHOULD include purpose description  |
| `@acp:purpose`     | File/module purpose (RFC-001)     | MUST describe file purpose          |
| `@acp:summary`     | Brief description                 | SHOULD include context              |
| `@acp:domain`      | Domain classification             | SHOULD identify domain              |
| `@acp:layer`       | Architectural layer               | SHOULD describe layer role          |
| `@acp:stability`   | Stability indicator               | SHOULD explain implications         |
| `@acp:lock`        | Mutation constraints              | MUST include behavioral instruction |
| `@acp:lock-reason` | Justification for lock            | SHOULD explain why locked           |
| `@acp:style`       | Style guide reference             | SHOULD specify guide to follow      |
| `@acp:style-rules` | Custom style rules                | SHOULD list rules                   |
| `@acp:behavior`    | AI behavior guidance              | SHOULD describe approach            |
| `@acp:quality`     | Quality requirements              | SHOULD list requirements            |
| `@acp:owner`       | Team ownership (RFC-001)          | SHOULD include contact info         |
| `@acp:ref`         | Reference documentation (RFC-001) | SHOULD describe when to consult     |

**Symbol-Level Annotations:**

| Namespace         | Purpose                            | Directive Requirement          |
|-------------------|------------------------------------|--------------------------------|
| `@acp:fn`         | Function description (RFC-001)     | MUST describe function purpose |
| `@acp:class`      | Class description (RFC-001)        | MUST describe class purpose    |
| `@acp:method`     | Method description (RFC-001)       | MUST describe method behavior  |
| `@acp:param`      | Parameter description (RFC-001)    | MUST describe parameter        |
| `@acp:returns`    | Return value description (RFC-001) | MUST describe return value     |
| `@acp:throws`     | Exception description (RFC-001)    | MUST describe exception        |
| `@acp:example`    | Usage example (RFC-001)            | SHOULD include example code    |
| `@acp:test`       | Testing requirements               | SHOULD describe test needs     |
| `@acp:deprecated` | Deprecation notice                 | MUST include replacement info  |
| `@acp:debug`      | Debug session tracking             | SHOULD include session context |

**Inline Annotations:**

| Namespace           | Purpose                        | Directive Requirement         |
|---------------------|--------------------------------|-------------------------------|
| `@acp:hack`         | Temporary solution marker      | MUST explain workaround       |
| `@acp:hack-ticket`  | Issue ticket reference         | SHOULD include ticket ID      |
| `@acp:hack-expires` | Hack expiration date           | MUST include date             |
| `@acp:critical`     | Critical code marker (RFC-001) | MUST explain criticality      |
| `@acp:todo`         | Pending work item (RFC-001)    | MUST describe work needed     |
| `@acp:fixme`        | Known issue marker (RFC-001)   | MUST describe issue           |
| `@acp:perf`         | Performance note (RFC-001)     | SHOULD describe consideration |

### 4.4 Annotation Scoping

**File-level annotations** apply to the entire file:
- Placed before first code element or in file header
- Inherited by all symbols unless overridden

**Symbol-level annotations** apply to a specific symbol:
- Placed immediately before symbol definition
- Override file-level annotations

### 4.5 Extension Namespaces

<!-- ADDED: v1 - New subsection per GAP-I15, Decision Q4 -->

Custom extensions MUST use the following format:

**Format**: `@acp:x-{vendor}:{feature}`

**Examples:**
- `@acp:x-github:copilot-context`
- `@acp:x-cursor:rules-ref`
- `@acp:x-mycompany:internal-audit`

**Rules:**
1. Extensions MUST start with `x-` prefix
2. Vendor name MUST follow `x-`
3. Feature name follows vendor with colon separator
4. Vendor and feature MUST use lowercase with hyphens
5. Extensions MAY NOT override reserved namespaces (Section 4.3)

**Reserved Patterns:**
- `@acp:x-acp-*` reserved for future official extensions
- Do not use `@acp:x-acp-` prefix for custom extensions

**Preservation:**
- Extensions MUST be preserved in cache under `extensions` field
- Extensions MUST NOT be interpreted by core (pass-through only)
- Tools MAY provide extension-specific handling

**Future Conflicts:**
- If a future ACP version reserves a namespace matching your extension, you MUST migrate to a different name
- No automatic conflict resolution is provided
- Choose vendor-specific names carefully

### 4.6 Parsing Algorithm

<!-- ADDED: v1 - New subsection per GAP-I9 -->

This section specifies how to extract annotations from source files.

#### 4.6.1 Step 1: Identify Documentation Comments

**By Language:**

| Language              | Doc Comment Syntax                                             |
|-----------------------|----------------------------------------------------------------|
| JavaScript/TypeScript | `/** ... */` (JSDoc style)                                     |
| Python                | `"""..."""` or `'''...'''` (docstrings) or `#` at module level |
| Rust                  | `//!` (module-level) or `///` (item-level)                     |
| Go                    | `//` comments immediately preceding declarations               |
| Java/C#               | `/** ... */` (Javadoc style)                                   |
| Ruby                  | `=begin...=end` or `#` comments                                |
| PHP                   | `/** ... */` (PHPDoc style)                                    |

#### 4.6.2 Step 2: Extract Annotation Lines

For each documentation comment:
1. Remove comment delimiters
2. Extract lines containing `@acp:`
3. Parse each line according to EBNF grammar (Section 4.1)

#### 4.6.3 Step 3: Handle Multi-line Annotations

Consecutive lines with same namespace are treated as single annotation:

```
@acp:summary This is a long
summary that spans multiple
lines and should be combined
```

Results in: `@acp:summary "This is a long summary that spans multiple lines and should be combined"`

#### 4.6.4 Step 4: Associate with Code Elements

- **Module-level**: Annotations before first code element or in file header
- **Symbol-level**: Annotations immediately preceding symbol definition
- **Scope**: Annotations apply to immediately following element only

#### 4.6.5 Step 5: Error Handling

- **Malformed annotation**: Handle per Section 11 (strictness mode)
- **Unknown namespace**: Warn (permissive) or error (strict)
- **In string literal**: Ignore (use language-aware parser to detect)

**For detailed per-language parsing notes**, see external documentation: `docs/LANGUAGE-SUPPORT.md`

---

## 5. Constraint System

<!-- CHANGED: v1 - Clarified advisory semantics per GAP-I12, added defaults per GAP-I16, Decision Q6 -->

Constraints define rules for how AI systems should interact with code.

### 5.1 Overview

**Advisory Nature:**
Constraints are **advisory** to AI systems, meaning ACP cannot enforce them through access control or runtime checks. There is no mechanism to prevent an AI from violating constraints.

However, AI systems that claim ACP conformance (Section 10) **MUST** respect constraint semantics as specified in this section. The MUST/SHOULD language in constraint behavior tables applies to **conformant AI systems**.

**Rationale:** Advisory model enables:
- Flexibility for AI decision-making
- No runtime overhead
- Simple implementation
- Trust-based collaboration

AI systems MAY log constraint violations if tracking is enabled (Section 5.6).

### 5.2 Lock Levels

<!-- CHANGED: v1 - Added default lock level per GAP-I16, Decision Q6 -->

Lock levels control how freely AI can modify code.

**Default Lock Level:**
Files and symbols without explicit `@acp:lock` annotation default to `normal` level.

This means:
- Absence of lock annotation permits unrestricted modification
- To restrict access, explicit annotation is required
- Default is permissive by design

**Lock Levels (most to least restrictive):**

| Level               | AI Behavior                                       | Use Case                             |
|---------------------|---------------------------------------------------|--------------------------------------|
| `frozen`            | MUST NOT modify under any circumstances           | Production config, security critical |
| `restricted`        | MUST get explicit user approval before any change | Authentication, payment processing   |
| `approval-required` | SHOULD get user approval for significant changes  | Core business logic                  |
| `tests-required`    | MUST add/update tests with changes                | Complex logic, regression-prone code |
| `docs-required`     | MUST update documentation with changes            | Public APIs, user-facing features    |
| `normal`            | No special restrictions (default)                 | Most code                            |
| `experimental`      | Encourage aggressive changes                      | Prototypes, proofs-of-concept        |

**Annotation:**
```javascript
/**
 * @acp:lock frozen
 * @acp:lock-reason "Production database credentials"
 */
```

### 5.3 Style Constraints

Style constraints guide code formatting and conventions.

**Annotation:**
```javascript
/**
 * @acp:style google-typescript
 * @acp:style-rules max-line-length=100, no-any
 */
```

**Behavior:**
- AI SHOULD follow specified style guide
- AI SHOULD apply custom rules
- Style is advisory (not enforced)

### 5.4 Behavior Constraints

Behavior constraints guide AI decision-making approach.

**Annotation:**
```javascript
/**
 * @acp:behavior conservative
 */
```

**Values:**
- `conservative`: Prefer safe, minimal changes
- `balanced`: Balance safety and functionality (default)
- `aggressive`: Prioritize functionality, accept more risk

### 5.5 Quality Requirements

Quality requirements specify additional checks needed.

**Annotation:**
```javascript
/**
 * @acp:quality security-review, performance-test
 */
```

**Common Requirements:**
- `security-review`: Requires security audit
- `performance-test`: Requires performance validation
- `manual-test`: Requires manual testing
- `regression-test`: Requires regression testing

### 5.6 Constraint Violation Tracking (Optional)

<!-- ADDED: v1 - New subsection per GAP-I12, Decision Q8 -->

Implementations MAY provide optional constraint violation logging.

**Configuration** (in `.acp.config.json`):
```json
{
  "constraints": {
    "track_violations": true,
    "audit_file": ".acp.violations.log"
  }
}
```

**Log Format** (if enabled):
```json
{
  "timestamp": "2024-12-17T15:30:00Z",
  "file": "src/auth/session.ts",
  "constraint": "lock:frozen",
  "action": "modify",
  "user_override": false,
  "context": "AI attempted modification of frozen file"
}
```

Implementations MAY provide `--audit` flag to enable tracking for a session.

**Note**: Violation tracking is OPTIONAL. Implementations claiming conformance are not required to provide this feature.

---

## 6. Variable System

Variables provide token-efficient references to code elements.

### 6.1 Overview

Variables replace verbose inline references with short tokens:

**Without variables** (high token cost):
```
Please review the validateSession function in src/auth/session.ts
(lines 45-89) which validates JWT tokens and returns session data...
```

**With variables** (low token cost):
```
Please review $SYM_VALIDATE...
```

### 6.2 Syntax

**Reference syntax**: `$VARIABLE_NAME[.modifier]`

**Examples:**
- `$SYM_VALIDATE` - Base reference
- `$SYM_VALIDATE.full` - Full JSON expansion
- `$SYM_VALIDATE.ref` - File reference only
- `$SYM_VALIDATE.signature` - Signature only

### 6.3 Variable Types

| Type     | Value Format          | Example Definition                                     |
|----------|-----------------------|--------------------------------------------------------|
| `symbol` | Qualified symbol name | `"src/auth/session.ts:SessionService.validateSession"` |
| `file`   | File path             | `"src/auth/session.ts"`                                |
| `domain` | Domain name           | `"authentication"`                                     |

### 6.4 Modifiers

<!-- CHANGED: v1 - Clarified modifier semantics with detailed table per GAP-I13 -->

Modifiers control what information is included in variable expansion.

| Modifier     | Returns        | Example                                                                              | Use Case          |
|--------------|----------------|--------------------------------------------------------------------------------------|-------------------|
| (none)       | Summary        | `$SYM_VALIDATE` → "validateSession (src/auth/session.ts:45-89) validates JWT tokens" | Quick reference   |
| `.full`      | Complete JSON  | `$SYM_VALIDATE.full` → entire SymbolEntry object                                     | Detailed analysis |
| `.ref`       | File reference | `$SYM_VALIDATE.ref` → "src/auth/session.ts:45-89"                                    | Source location   |
| `.signature` | Signature only | `$SYM_VALIDATE.signature` → "(token: string) => Promise<Session \| null>"            | Type checking     |

**Summary Format** (no modifier):
- Symbol: `{name} ({file}:{lines}) {summary}`
- File: `{path} ({lines} lines) {summary}`
- Domain: `{name}: {description}`

**Full Format** (`.full`):
- Returns complete JSON object from cache
- Includes all fields

**Reference Format** (`.ref`):
- Format: `{file}:{start_line}-{end_line}`
- Example: `src/auth/session.ts:45-89`

**Signature Format** (`.signature`):
- Only for symbols with signatures (functions, methods)
- Returns signature string
- If not applicable: Use base expansion, emit warning

### 6.5 Error Handling

<!-- ADDED: v1 - New subsection per GAP-C6, Decision Q5 -->

#### 6.5.1 Undefined Variables

When a variable reference cannot be resolved:

**Behavior:**
- Variable remains as literal string: `$UNDEFINED_VAR`
- Implementation MUST emit warning to stderr/log
- Processing continues (no abort)

**Example:**
- Input: `"Check $SYM_DOES_NOT_EXIST for details"`
- Output: `"Check $SYM_DOES_NOT_EXIST for details"` + warning

**Rationale:**
- User sees problem clearly (literal preserved)
- Non-breaking (downstream processing continues)
- Easy to detect (text search for `$`)

**Escape Sequence:**
To include literal `$VAR` in output (not as variable):
Use double dollar: `$$VAR` → `$VAR` (no expansion, no warning)

#### 6.5.2 Circular References

When variable expansion creates a cycle:

**Detection:**
- Track expansion depth
- Max depth: 10 levels
- If exceeded, cycle detected

**Behavior:**
- Stop expansion at max depth
- Return: `[CIRCULAR: $VAR_NAME]`
- Emit warning with cycle path

**Example:**
- `$VAR_A` → references `$VAR_B`
- `$VAR_B` → references `$VAR_A`
- Result: `[CIRCULAR: $VAR_A -> $VAR_B -> $VAR_A]`

#### 6.5.3 Invalid Modifiers

When modifier doesn't apply to variable type:

**Example:** `$FILE_README.signature` (files don't have signatures)

**Behavior:**
- Ignore invalid modifier
- Return base expansion (without modifier)
- Emit warning

#### 6.5.4 Strict Mode

In strict mode (Section 11):
- Undefined variables: Error, abort
- Circular references: Error, abort
- Invalid modifiers: Error, abort

### 6.6 Variable Scoping

<!-- ADDED: v1 - New subsection per GAP-C6 -->

**Scope:** Variables are **global** within a project.

**Rules:**
- All variables in `.acp.vars.json` are project-global
- Variables MAY NOT be overridden or shadowed
- No file-scoped or module-scoped variables

**Rationale:** Simplicity and predictability. File-scoped variables would require complex resolution rules and could create confusion.

**Future:** If scoping becomes necessary, it will be added in a future version with clear precedence rules.

---

## 7. Inheritance & Cascade

Constraints can be defined at multiple levels and cascade to more specific scopes.

### 7.1 Precedence Levels

From most to least specific:

1. **Symbol-level annotations** - Apply to specific function/class
2. **File-level annotations** - Apply to entire file
3. **Directory-level config** (`.acp.dir.json`) - Apply to directory tree
4. **Project-level config** (`.acp.config.json`) - Apply to entire project

More specific levels override less specific ones.

### 7.2 Merging Rules

<!-- CHANGED: v1 - Clarified merge rules with detailed examples per GAP-I11 -->

When constraints exist at multiple precedence levels, they merge according to these rules:

#### 7.2.1 Lock Levels

**Rule:** Most restrictive wins

**Example:**
- Project default: `normal`
- Directory: `approval-required`
- File: `restricted`
- **Result:** `restricted` (most restrictive)

**Precedence** (most to least restrictive):
`frozen` > `restricted` > `approval-required` > `tests-required` > `docs-required` > `normal` > `experimental`

#### 7.2.2 Style Guides

**Rule:** Most specific **guide** wins, but rules from all levels **accumulate**

**Example:**
- Project: `google-typescript`
- Directory: custom rule `max-line-length=100`
- File: custom rule `indent=2`
- **Result:**
  - Base guide: `google-typescript` (from project)
  - Additional rules: `max-line-length=100`, `indent=2` (accumulated)

**Rationale:** File-level custom rules augment project-level guide, not replace.

#### 7.2.3 Behavior Constraints

**Rule:** Most specific wins completely (no merging)

**Example:**
- Project: `approach=conservative`
- File: `approach=aggressive`
- **Result:** `aggressive` (file-level overrides)

#### 7.2.4 Quality Requirements

**Rule:** Requirements accumulate (all levels apply)

**Example:**
- Project: `tests-required`
- File: `security-review`
- **Result:** Both requirements apply

#### 7.2.5 Equal Specificity

If two constraints at the **same precedence level** conflict:

**Rule:** Last defined wins

**Example** (both in same file):
```javascript
/**
 * @acp:lock frozen
 */

// ... later in same file ...

/**
 * @acp:lock normal
 */
```
**Result:** `normal` (last wins)

**Recommendation:** Avoid this. Use linter to detect conflicts.

### 7.3 Examples

<!-- ADDED: v1 - Complex inheritance examples per GAP-I11 -->

#### 7.3.1 Example 1: Multi-Level Inheritance

**Setup:**
- `.acp.config.json`: `{"constraints": {"defaults": {"lock": "normal"}}}`
- `.acp.dir.json` in `src/auth/`: `{"lock": "approval-required"}`
- `src/auth/session.ts` file annotation: `@acp:lock restricted`
- `validateSession` symbol annotation: `@acp:lock frozen`

**Resolution:**

| Element                                | Effective Lock Level   | Reason                            |
|----------------------------------------|------------------------|-----------------------------------|
| `src/utils/helper.ts`                  | `normal`               | Project default                   |
| `src/auth/token.ts`                    | `approval-required`    | Directory override                |
| `src/auth/session.ts` (file)           | `restricted`           | File annotation                   |
| `src/auth/session.ts::validateSession` | `frozen`               | Symbol annotation (most specific) |

#### 7.3.2 Example 2: Style Guide Accumulation

**Setup:**
- Project: `@acp:style google-typescript`
- Directory `src/api/`: `@acp:style-rules max-params=4, async-required`
- File `src/api/users.ts`: `@acp:style-rules no-any`

**Resolution for `src/api/users.ts`:**
- Base guide: `google-typescript`
- Additional rules: `max-params=4`, `async-required`, `no-any`
- All rules apply together

#### 7.3.3 Example 3: Conflict Resolution

**Setup** (same file):
```typescript
/**
 * @acp:lock restricted
 * @acp:behavior conservative
 */

/**
 * @acp:lock normal
 */
function dangerousOperation() { }
```

**Resolution:**
- File level: `lock=restricted`, `behavior=conservative`
- Symbol level: `lock=normal`
- For `dangerousOperation`: `lock=normal` (symbol override), `behavior=conservative` (inherited from file)

---

## 8. File Discovery

### 8.1 Discovery Algorithm

1. Start from project root (containing `.acp.config.json` or first `.acp.cache.json`)
2. Recursively scan directories
3. For each file:
   - Check if matches `include` patterns (default: all files)
   - Check if matches `exclude` patterns
   - If included and not excluded, process file
4. Parse annotations from each file
5. Build cache structure

### 8.2 Exclusion Patterns

**Default exclusions:**
```json
{
  "exclude": [
    "node_modules/**",
    ".git/**",
    "dist/**",
    "build/**",
    "coverage/**",
    "**/*.test.*",
    "**/*.spec.*"
  ]
}
```

**Custom exclusions** can be added in `.acp.config.json`.

### 8.3 Cache Building Details

<!-- ADDED: v1 - New subsection per GAP-I10 -->

This section specifies algorithms for cache generation.

#### 8.3.1 Domain Detection

**Algorithm:**
1. Check for `@acp:domain` annotation (Priority 1)
2. Check directory patterns in config (Priority 2)
3. Analyze imports to infer domain (Priority 3)
4. Leave unclassified if inconclusive

**Directory Pattern Example** (in `.acp.config.json`):
```json
{
  "domains": {
    "authentication": {
      "patterns": ["src/auth/**", "lib/security/**"]
    },
    "database": {
      "patterns": ["src/db/**", "src/models/**"]
    }
  }
}
```

**Import Analysis:**
- If file imports primarily from one domain, classify in that domain
- Threshold: >60% of imports from single domain

#### 8.3.2 Layer Detection

**Algorithm:**
1. Check for `@acp:layer` annotation (Priority 1)
2. Check directory naming conventions (Priority 2)
3. Analyze dependencies to infer layer (Priority 3)
4. Default to null if inconclusive

**Directory Naming Conventions:**

| Pattern                            | Layer      |
|------------------------------------|------------|
| `**/handlers/**`, `**/routes/**`   | handler    |
| `**/services/**`, `**/business/**` | service    |
| `**/repositories/**`, `**/data/**` | repository |
| `**/models/**`, `**/entities/**`   | model      |
| `**/utils/**`, `**/helpers/**`     | utility    |

#### 8.3.3 Call Graph Construction

**Algorithm:**
1. Use static analysis to identify function calls
2. Exclude standard library calls (configurable)
3. Build forward map: caller → [callees]
4. Build reverse map: callee → [callers]
5. Handle indirect calls conservatively (include if detectable)

**Limitations:**
- Dynamic calls may not be detected
- Reflection/metaprogramming not tracked
- Cross-language calls require explicit annotation

**Configuration** (in `.acp.config.json`):
```json
{
  "call_graph": {
    "include_stdlib": false,
    "max_depth": null,
    "exclude_patterns": ["**/test/**"]
  }
}
```

### 8.4 Language Detection

<!-- ADDED: v1 - New subsection per GAP-I18 -->

Files are classified by extension:

| Extension(s)                  | Language   |
|-------------------------------|------------|
| `.ts`, `.tsx`                 | typescript |
| `.js`, `.jsx`, `.mjs`         | javascript |
| `.py`, `.pyw`                 | python     |
| `.rs`                         | rust       |
| `.go`                         | go         |
| `.java`                       | java       |
| `.cs`                         | c-sharp    |
| `.rb`                         | ruby       |
| `.php`                        | php        |
| `.cpp`, `.cc`, `.cxx`, `.hpp` | cpp        |
| `.c`, `.h`                    | c          |
| `.swift`                      | swift      |
| `.kt`, `.kts`                 | kotlin     |

**Ambiguous Extensions:**

| Extension   | Check For                             | If Found    | Else             |
|-------------|---------------------------------------|-------------|------------------|
| `.h`        | `#include <iostream>` or C++ keywords | cpp         | c                |
| `.m`        | `@interface`, `@implementation`       | objective-c | (error: unknown) |

**Unknown Extensions:**
- Emit warning
- Skip file in permissive mode
- Error in strict mode

### 8.5 Implementation Limits

<!-- ADDED: v1 - New subsection per GAP-I14 -->

Implementations SHOULD respect these limits:

| Limit                        | Default  | Rationale                          |
|------------------------------|----------|------------------------------------|
| Max source file size         | 10 MB    | Prevent parser hang, memory issues |
| Max files in project         | 100,000  | Performance, memory                |
| Max annotations per file     | 1,000    | Performance                        |
| Max symbols per file         | 10,000   | Performance, cache size            |
| Max cache file size          | 100 MB   | Memory, network transfer           |
| Max variable expansion depth | 10       | Circular reference protection      |
| Max inheritance depth        | 4        | Complexity management              |

**Configuration:** Limits SHOULD be configurable in `.acp.config.json`:
```json
{
  "limits": {
    "max_file_size_mb": 10,
    "max_files": 100000,
    "max_annotations_per_file": 1000,
    "max_cache_size_mb": 100
  }
}
```

**Behavior When Exceeded:**
- Permissive: Warn, skip offending item, continue
- Strict: Error, abort

**Large Projects:**
For projects exceeding limits, consider:
- Exclude generated files
- Separate into multiple ACP projects
- Increase limits (with caution)

---

## 9. Query Interface

ACP supports three query interfaces.

### 9.1 jq Queries

<!-- CHANGED: v1 - Converted examples to snake_case per GAP-C8 -->

The cache is standard JSON, queryable with `jq`:

**Find all frozen files:**
```bash
jq '.constraints.by_lock_level.frozen[]' .acp.cache.json
```

**Get file summary:**
```bash
jq '.files["src/auth/session.ts"]' .acp.cache.json
```

**Find authentication domain files:**
```bash
jq '.domains.authentication.files[]' .acp.cache.json
```

### 9.2 Command Line Interface

Implementations SHOULD provide a CLI:

```bash
# Query by domain
acp query --domain authentication

# Query by lock level
acp query --lock frozen

# Find symbol
acp query --symbol validateSession

# Rebuild cache
acp index --force
```

### 9.3 MCP Server Interface

Implementations MAY provide MCP server integration:

**Resources:**
- `acp://cache` - Full cache
- `acp://file/{path}` - Specific file metadata
- `acp://symbol/{qualified_name}` - Specific symbol metadata

**Tools:**
- `acp_query` - Execute queries
- `acp_expand_variable` - Expand variable references

---

<!-- ADDED: v1 - New section per GAP-C2 and human decision D2 -->
## 10. Conformance Levels

### 10.1 Overview

This section defines conformance levels for ACP implementations. An implementation MAY claim conformance to one or more levels. Each level builds upon the previous, adding additional capabilities.

Conformance levels enable:
- Clear capability declarations
- Interoperability expectations
- Progressive implementation paths
- Ecosystem development

### 10.2 Level Definitions

#### Level 1: Reader

A **Level 1 (Reader)** conformant implementation MUST:

- Parse `.acp.cache.json` files conforming to Section 3.1
- Support basic queries via `jq` or programmatic JSON access
- Read and interpret constraint annotations (Section 5)
- Expand variables in read-only mode (Section 6)
- Support all reserved annotation namespaces (Section 4.3)

A Level 1 implementation MAY:
- Generate warnings for undefined variables
- Provide helper utilities for common queries

A Level 1 implementation MUST NOT:
- Generate or modify cache files
- Generate or modify variable files
- Execute write operations

**Use Cases:** AI assistants that consume existing ACP metadata but don't modify codebases.

#### Level 2: Standard

A **Level 2 (Standard)** conformant implementation MUST:

- Meet all Level 1 requirements
- Generate `.acp.cache.json` files from source code (Section 8)
- Generate `.acp.vars.json` files (Section 3.2)
- Parse `.acp.config.json` files (Section 3.3)
- Implement file discovery algorithm (Section 8.1)
- Apply constraint inheritance and cascade rules (Section 7)
- Detect cache staleness (Section 3.1.5)
- Handle errors according to configured strictness (Section 11)
- Use snake_case for all JSON field names (Section 3)

A Level 2 implementation MAY:
- Generate `.acp/acp.attempts.json` files (Section 3.4)
- Parse `.acp/acp.sync.json` files (Section 3.5)
- Generate `.acp.primer.json` files (Section 3.6)

A Level 2 implementation SHOULD:
- Detect staleness using git when available (Section 3.1.5)
- Support configurable strictness modes (Section 11)
- Provide CLI interface for queries (Section 9.2)
- Log constraint violations if enabled (Section 5.6)

**Use Cases:** Development tools, linters, IDE plugins that generate and maintain ACP metadata.

#### Level 3: Full

A **Level 3 (Full)** conformant implementation MUST:

- Meet all Level 2 requirements
- Implement MCP server interface (Section 9.3)
- Support debug session tracking (@acp:debug namespace)
- Support hack tracking (@acp:hack namespace)
- Implement watch mode for automatic cache rebuilding
- Support all query interfaces (jq, CLI, MCP)
- Provide conformance test suite results

A Level 3 implementation SHOULD:
- Support distributed/remote cache access
- Provide rich error diagnostics
- Support IDE integrations
- Provide performance optimizations for large projects

**Use Cases:** Production AI development environments, full-featured IDE integrations, team collaboration platforms.

### 10.3 Feature Matrix

| Feature                 | Level 1  | Level 2   | Level 3  |
|-------------------------|----------|-----------|----------|
| Parse cache.json        | ✓ MUST   | ✓ MUST    | ✓ MUST   |
| Read constraints        | ✓ MUST   | ✓ MUST    | ✓ MUST   |
| Expand variables (read) | ✓ MUST   | ✓ MUST    | ✓ MUST   |
| Basic queries (jq)      | ✓ MUST   | ✓ MUST    | ✓ MUST   |
| Generate cache          | ✗        | ✓ MUST    | ✓ MUST   |
| Generate variables      | ✗        | ✓ MUST    | ✓ MUST   |
| Parse config            | ✗        | ✓ MUST    | ✓ MUST   |
| File discovery          | ✗        | ✓ MUST    | ✓ MUST   |
| Staleness detection     | ✗        | ✓ MUST    | ✓ MUST   |
| Constraint inheritance  | ✗        | ✓ MUST    | ✓ MUST   |
| Error handling          | ✗        | ✓ MUST    | ✓ MUST   |
| CLI queries             | ✗        | ⚠ SHOULD  | ✓ MUST   |
| MCP interface           | ✗        | ✗         | ✓ MUST   |
| Debug sessions          | ✗        | ✗         | ✓ MUST   |
| Watch mode              | ✗        | ✗         | ✓ MUST   |
| Conformance tests       | ✗        | ✗         | ✓ MUST   |

### 10.4 Conformance Claims

An implementation claiming conformance MUST:

1. **Specify the level:** "Implements ACP 1.0 Level N"
2. **Provide version:** Include ACP specification version (e.g., "1.0.0")
3. **Document deviations:** List any MUST requirements not implemented
4. **Provide evidence:** Link to test results or documentation

**Claim Format:**
```
Implements ACP 1.0.0 Level 2 (Standard)
Conformance Documentation: [URL]
Test Results: [URL]
Known Limitations: [list any MUST requirements not met]
```

### 10.5 Partial Conformance

An implementation MAY claim partial conformance by specifying:
- Base level: The highest level fully supported
- Additional features: Specific features from higher levels

**Example:**
```
Implements ACP 1.0.0 Level 1 (Reader)
Additional Features:
  - MCP interface (from Level 3)
  - Staleness detection (from Level 2)
```

### 10.6 Non-Conformance

An implementation that uses ACP concepts but doesn't meet a conformance level MUST NOT claim conformance. Instead, it MAY state:

```
Compatible with ACP 1.0.0 concepts
Does not claim formal conformance
```

### 10.7 Conformance Testing

Conformance testing resources:
- **Test Suite:** github.com/acp-spec/conformance-tests (when available)
- **Test Categories:** Parsing, Generation, Queries, Constraints, Variables
- **Pass Criteria:** 100% of MUST requirements for claimed level

Implementations SHOULD publish test results when claiming conformance.

---

<!-- ADDED: v1 - New section per GAP-C1 and human decision D1 -->
## 11. Error Handling

### 11.1 Overview

This section defines how ACP implementations MUST handle error conditions. Proper error handling ensures:
- Predictable behavior across implementations
- Clear diagnostics for users
- Graceful degradation when possible
- Consistent error reporting

### 11.2 Strictness Modes

Implementations MUST support configurable strictness via `.acp.config.json`:

```json
{
  "version": "1.0.0",
  "error_handling": {
    "strictness": "permissive"  // or "strict"
  }
}
```

#### Permissive Mode (Default)

In permissive mode, implementations:
- MUST emit warnings for recoverable errors
- MUST continue processing when possible
- SHOULD collect all errors before failing
- MUST provide partial results when safe

Use permissive mode for:
- Development environments
- Large codebases with mixed quality
- Gradual ACP adoption

#### Strict Mode

In strict mode, implementations:
- MUST fail fast on first error
- MUST NOT produce partial or potentially incorrect output
- MUST provide detailed error context
- MUST exit with non-zero status

Use strict mode for:
- CI/CD pipelines
- Production builds
- Conformance testing
- High-reliability requirements

### 11.3 Error Categories

#### Syntax Errors

**Definition:** Malformed input that cannot be parsed.

**Examples:**
- Invalid annotation syntax
- Malformed JSON in config/cache/vars files
- EBNF grammar violations

**Handling:**
- Permissive: Warn, skip malformed annotation, continue
- Strict: Error, abort immediately

**Error Format:**
```json
{
  "category": "syntax",
  "severity": "error",
  "code": "E001",
  "message": "Invalid annotation syntax",
  "location": {
    "file": "src/auth/session.ts",
    "line": 45,
    "column": 12
  },
  "snippet": "@acp:lock frozen extra-text",
  "suggestion": "Remove trailing text or use lock-reason"
}
```

#### Semantic Errors

**Definition:** Valid syntax but invalid semantics.

**Examples:**
- Unknown annotation namespace
- Invalid constraint value
- Conflicting annotations
- Circular variable references

**Handling:**
- Permissive: Warn, use default/skip, continue
- Strict: Error, abort

**Error Format:**
```json
{
  "category": "semantic",
  "severity": "error",
  "code": "E101",
  "message": "Unknown annotation namespace",
  "location": {
    "file": "src/utils/helper.ts",
    "line": 23
  },
  "snippet": "@acp:custom-feature value",
  "suggestion": "Use @acp:x-vendor:custom-feature for extensions"
}
```

#### Runtime Errors

**Definition:** Errors during cache generation or queries.

**Examples:**
- File not found
- Permission denied
- Out of memory
- Network errors (MCP)

**Handling:**
- Permissive: Warn, skip file/operation, continue
- Strict: Error, abort

**Error Format:**
```json
{
  "category": "runtime",
  "severity": "error",
  "code": "E201",
  "message": "File not found",
  "location": {
    "file": "src/deleted.ts"
  },
  "context": "Referenced in cache but file no longer exists",
  "suggestion": "Rebuild cache with: acp index --force"
}
```

#### Resource Errors

**Definition:** Resource limits exceeded.

**Examples:**
- File too large
- Too many files
- Cache too large
- Max depth exceeded

**Handling:**
- Permissive: Warn, skip large files, continue
- Strict: Error, abort

**Error Format:**
```json
{
  "category": "resource",
  "severity": "warning",
  "code": "W301",
  "message": "File exceeds size limit",
  "location": {
    "file": "src/generated/huge.ts",
    "size": "15MB"
  },
  "limit": "10MB",
  "suggestion": "Add to exclude pattern or increase limit in config"
}
```

### 11.4 Error Response Format

All errors MUST be reported in structured format:

```typescript
interface AcpError {
  category: "syntax" | "semantic" | "runtime" | "resource";
  severity: "error" | "warning" | "info";
  code: string;  // E### for errors, W### for warnings
  message: string;
  location?: {
    file?: string;
    line?: number;
    column?: number;
    size?: string;  // for resource errors
  };
  snippet?: string;  // Code snippet showing error
  context?: string;  // Additional context
  suggestion?: string;  // How to fix
}
```

### 11.5 Error Codes

| Code      | Category   | Description                 |
|-----------|------------|-----------------------------|
| E001-E099 | Syntax     | Parsing errors              |
| E001      | Syntax     | Invalid annotation syntax   |
| E002      | Syntax     | Malformed JSON              |
| E003      | Syntax     | Invalid EBNF grammar        |
| E101-E199 | Semantic   | Semantic errors             |
| E101      | Semantic   | Unknown namespace           |
| E102      | Semantic   | Invalid constraint value    |
| E103      | Semantic   | Conflicting annotations     |
| E104      | Semantic   | Circular variable reference |
| E105      | Semantic   | Undefined variable          |
| E201-E299 | Runtime    | Runtime errors              |
| E201      | Runtime    | File not found              |
| E202      | Runtime    | Permission denied           |
| E203      | Runtime    | Disk full                   |
| E204      | Runtime    | Out of memory               |
| E301-E399 | Resource   | Resource limit errors       |
| E301      | Resource   | File too large              |
| E302      | Resource   | Too many files              |
| E303      | Resource   | Cache too large             |
| E304      | Resource   | Max depth exceeded          |
| W001-W999 | All        | Warnings (non-fatal)        |

### 11.6 Operation-Specific Error Handling

#### Cache Generation Errors

**Malformed Annotation:**
- Permissive: Warn, skip annotation, include file without that annotation
- Strict: Error, abort indexing

**Parse Error:**
- Permissive: Warn, skip file, continue with others
- Strict: Error, abort indexing

**Missing Required Field:**
- Permissive: Use default if available, warn; else skip
- Strict: Error, abort

**File Discovery Error:**
- Permissive: Warn, skip inaccessible file
- Strict: Error, abort

#### Variable Expansion Errors

**Undefined Variable** (as per human decision D5):
- Permissive: Warn, leave as literal "$VAR_NAME"
- Strict: Error, abort

**Circular Reference:**
- Permissive: Detect (max depth 10), warn, return [CIRCULAR: $VAR]
- Strict: Error, abort

**Invalid Modifier:**
- Permissive: Warn, ignore modifier, return base expansion
- Strict: Error, abort

#### Query Errors

**Invalid Query Syntax:**
- Both modes: Error, return error response

**Result Too Large:**
- Permissive: Warn, truncate with notice
- Strict: Error, abort

#### Constraint Violation Errors

**Note:** Constraints are advisory. Violations are not errors in the traditional sense, but MAY be logged if violation tracking is enabled (Section 5.6).

### 11.7 Error Reporting

#### Standard Error Output

Implementations SHOULD write errors to stderr in human-readable format:

```
[ERROR E001] Invalid annotation syntax (src/auth/session.ts:45:12)
  @acp:lock frozen extra-text
              ^^^^^^^^^^^^^^
  Suggestion: Remove trailing text or use lock-reason
```

#### Structured Output

Implementations MUST support `--json` flag for structured error output:

```bash
acp index --json 2> errors.json
```

Output format:
```json
{
  "errors": [
    { /* AcpError object */ }
  ],
  "warnings": [
    { /* AcpError object */ }
  ],
  "summary": {
    "total_errors": 3,
    "total_warnings": 7,
    "files_processed": 127,
    "files_failed": 3
  }
}
```

### 11.8 Exit Codes

Implementations MUST use these exit codes:

| Code  | Meaning                         |
|-------|---------------------------------|
| 0     | Success (no errors)             |
| 1     | General error                   |
| 2     | Syntax error                    |
| 3     | Semantic error                  |
| 4     | Runtime error                   |
| 5     | Resource error                  |
| 10    | Configuration error             |
| 64    | Usage error (invalid arguments) |

In permissive mode:
- Exit 0 if warnings only
- Exit with error code if any errors

In strict mode:
- Exit with first error code encountered

### 11.9 Error Recovery

Implementations MAY provide error recovery mechanisms:

**Auto-correction:**
- Fix common syntax errors automatically
- Log corrections made
- Require explicit flag: `--auto-correct`

**Partial Results:**
- In permissive mode, provide best-effort results
- Mark uncertain/incomplete data clearly
- Include error summary with results

**Error Reports:**
- Generate detailed error reports: `errors.txt`
- Include all context needed for diagnosis
- Provide reproduction steps when possible

---

<!-- CHANGED: v1 - Renumbered from Section 10 to Section 12 -->
## 12. Versioning

### 12.1 Specification Versioning

ACP uses semantic versioning (SemVer):

- **Major version** (X.0.0): Breaking changes
- **Minor version** (1.X.0): Backward-compatible additions
- **Patch version** (1.0.X): Backward-compatible fixes

### 12.2 File Format Versioning

All ACP files include a `version` field:

```json
{
  "version": "1.0.0",
  ...
}
```

This indicates the ACP specification version used to generate the file.

### 12.3 Compatibility

Implementations SHOULD:
- Support their declared spec version
- Accept older minor versions (backward compatible)
- Warn on newer minor versions (may miss features)
- Error on different major versions (incompatible)

---

## 13. Bootstrap & AI Integration

<!-- ADDED: v1.1 - RFC-001 Bootstrap & AI Integration -->

This section provides a summary of AI integration. For complete details, see [Chapter 14: Bootstrap & AI Integration](chapters/14-bootstrap.md).

### 13.1 Minimal Bootstrap Prompt

The minimal bootstrap is sufficient when all annotations include self-documenting directives:

```
This project uses ACP. @acp:* comments in code are directives for you.
BEFORE editing: acp constraints <path>
Explore: acp query symbol|file|domain <name>
Map: acp map <path>
Help: acp knowledge "question"
```

**Token count:** ~40 tokens

### 13.2 Pre-Edit Workflow

AI systems MUST follow this workflow before modifying files:

1. Run `acp constraints <target-file>` before proposing changes
2. Check the `lock_level` in the response
3. If `frozen`: REFUSE to modify
4. If `restricted`: Explain proposed changes and wait for approval
5. Otherwise: Proceed with context from constraints

### 13.3 Directive Processing

When AI encounters an `@acp:*` annotation:

1. **Parse directive** - Extract text after ` - `
2. **Identify RFC 2119 keywords** - MUST, SHOULD, MAY
3. **Apply constraint** - Follow the directive instruction
4. **Explain to user** - If action blocked, explain why

### 13.4 CLI Commands Reference

| Command                   | Purpose                                       |
|---------------------------|-----------------------------------------------|
| `acp constraints <path>`  | Check file constraints before editing         |
| `acp query file <path>`   | Get file context with symbols and constraints |
| `acp query symbol <name>` | Get symbol details with callers/callees       |
| `acp query domain <name>` | Get domain files and relationships            |
| `acp map <path>`          | Get visual file map with constraints          |
| `acp knowledge "q"`       | Ask about the codebase                        |

---

## Appendix A: Complete Annotation Reference

<!-- ADDED: v1 - Complete annotation reference per GAP-C5 -->

This appendix provides a complete reference for all ACP annotations.

### Reserved Namespaces

All annotations use the `@acp:` prefix followed by a namespace.

#### @acp:module

**Scope:** File/module level
**Purpose:** File metadata

| Annotation       | Parameters   | Example                                 | Description                                            |
|------------------|--------------|-----------------------------------------|--------------------------------------------------------|
| `@acp:module`    | `<name>`     | `@acp:module "Auth Service"`            | Human-readable module name                             |
| `@acp:summary`   | `<text>`     | `@acp:summary "Handles authentication"` | Brief module description                               |
| `@acp:domain`    | `<name>`     | `@acp:domain authentication`            | Domain classification                                  |
| `@acp:layer`     | `<name>`     | `@acp:layer service`                    | Architectural layer                                    |
| `@acp:stability` | `<level>`    | `@acp:stability stable`                 | Stability indicator (stable, experimental, deprecated) |

#### @acp:symbol

**Scope:** Symbol level (function, class, etc.)
**Purpose:** Symbol metadata

| Annotation        | Parameters   | Example                                       | Description              |
|-------------------|--------------|-----------------------------------------------|--------------------------|
| `@acp:summary`    | `<text>`     | `@acp:summary "Validates user session"`       | Brief symbol description |
| `@acp:deprecated` | `<message>`  | `@acp:deprecated "Use validateToken instead"` | Deprecation notice       |

#### @acp:lock

**Scope:** File or symbol level
**Purpose:** Mutation constraints

| Annotation         | Parameters   | Example                                | Description                                                                                             |
|--------------------|--------------|----------------------------------------|---------------------------------------------------------------------------------------------------------|
| `@acp:lock`        | `<level>`    | `@acp:lock frozen`                     | Lock level (frozen, restricted, approval-required, tests-required, docs-required, normal, experimental) |
| `@acp:lock-reason` | `<text>`     | `@acp:lock-reason "Security critical"` | Justification for lock level                                                                            |

**Default:** Files and symbols without explicit `@acp:lock` default to `normal`.

#### @acp:style (RFC-0002)

**Scope:** File or symbol level
**Purpose:** Style/format constraints

| Annotation           | Parameters   | Example                                        | Description                          |
|----------------------|--------------|------------------------------------------------|--------------------------------------|
| `@acp:style`         | `<guide>`    | `@acp:style google-typescript`                 | Style guide reference                |
| `@acp:style-rules`   | `<rules>`    | `@acp:style-rules max-line-length=100, no-any` | Custom style rules (comma-separated) |
| `@acp:style-extends` | `<guide>`    | `@acp:style-extends google-typescript`         | Parent style guide (RFC-0002)        |

**Style Guide Resolution (RFC-0002):** Style guides can be:
1. **Built-in guides** - Standard guides like `google-typescript`, `airbnb`, `pep8`
2. **Custom guides** - Defined in `documentation.styleGuides` in `.acp.config.json`
3. **Source-linked guides** - Guides with associated documentation sources

See [Chapter 4: Configuration Format](#4-configuration-format) for custom style guide configuration and [Chapter 5: Annotations](#5-annotations) for the built-in style guide registry.

**Example with style inheritance:**
```typescript
/**
 * @acp:style company-standard
 * @acp:style-extends google-typescript
 * @acp:style-rules max-line-length=120
 */
```

#### @acp:behavior

**Scope:** File or symbol level
**Purpose:** AI behavior guidance

| Annotation      | Parameters   | Example                      | Description                                   |
|-----------------|--------------|------------------------------|-----------------------------------------------|
| `@acp:behavior` | `<approach>` | `@acp:behavior conservative` | Approach (conservative, balanced, aggressive) |

#### @acp:quality

**Scope:** File or symbol level
**Purpose:** Quality requirements

| Annotation     | Parameters       | Example                                          | Description                            |
|----------------|------------------|--------------------------------------------------|----------------------------------------|
| `@acp:quality` | `<requirements>` | `@acp:quality security-review, performance-test` | Quality requirements (comma-separated) |

Common values: `security-review`, `performance-test`, `manual-test`, `regression-test`

#### @acp:test

**Scope:** Symbol level
**Purpose:** Testing requirements

| Annotation  | Parameters   | Example              | Description         |
|-------------|--------------|----------------------|---------------------|
| `@acp:test` | `<coverage>` | `@acp:test required` | Testing requirement |

#### @acp:debug

**Scope:** Symbol level
**Purpose:** Debug session tracking

| Annotation   | Parameters   | Example                         | Description              |
|--------------|--------------|---------------------------------|--------------------------|
| `@acp:debug` | `<session>`  | `@acp:debug session-2024-12-17` | Debug session identifier |

**Note:** Debug feature detailed specification deferred to v1.1

#### @acp:hack

**Scope:** Inline
**Purpose:** Temporary solution marker

| Annotation          | Parameters  | Example                                                  | Description               |
|---------------------|-------------|----------------------------------------------------------|---------------------------|
| `@acp:hack`         | none        | `@acp:hack - Timezone workaround for server clock drift` | Temporary solution marker |
| `@acp:hack-ticket`  | `<id>`      | `@acp:hack-ticket JIRA-1234`                             | Related issue ticket      |
| `@acp:hack-expires` | `<date>`    | `@acp:hack-expires 2025-06-01`                           | Expiration date           |

### RFC-001 Annotations

<!-- ADDED: v1.1 - RFC-001 Self-Documenting Annotations -->

The following annotations were added in v1.1 per RFC-001.

#### @acp:purpose (RFC-001)

**Scope:** File level
**Purpose:** File/module purpose description
**Directive:** MUST describe file purpose

| Annotation     | Parameters      | Example                                                                                  |
|----------------|-----------------|------------------------------------------------------------------------------------------|
| `@acp:purpose` | `<description>` | `@acp:purpose Session management and JWT validation - Review before modifying auth flow` |

#### @acp:owner (RFC-001)

**Scope:** File level
**Purpose:** Team ownership
**Directive:** SHOULD include contact info

| Annotation   | Parameters   | Example                                                                   |
|--------------|--------------|---------------------------------------------------------------------------|
| `@acp:owner` | `<team>`     | `@acp:owner security-team - Contact for questions or significant changes` |

#### @acp:ref (RFC-001, RFC-0002)

**Scope:** File or symbol level
**Purpose:** Reference documentation
**Directive:** SHOULD describe when to consult

| Annotation         | Parameters     | Example                                                                   | Description                               |
|--------------------|----------------|---------------------------------------------------------------------------|-------------------------------------------|
| `@acp:ref`         | `<url\|source>` | `@acp:ref https://docs.example.com/auth - Consult before making changes` | Documentation reference (URL or source ID)|
| `@acp:ref`         | `<source>`     | `@acp:ref react:hooks - Consult for hooks patterns`                      | Reference using approved source ID        |
| `@acp:ref-version` | `<version>`    | `@acp:ref-version 18.2`                                                  | Documentation version (RFC-0002)          |
| `@acp:ref-section` | `<section>`    | `@acp:ref-section hooks/custom-hooks`                                    | Section within documentation (RFC-0002)   |
| `@acp:ref-fetch`   | `[true\|false]` | `@acp:ref-fetch true`                                                    | Whether AI should fetch reference (RFC-0002)|

**Source ID Resolution (RFC-0002):** When using a source ID (e.g., `react:hooks`), the tool resolves against `documentation.approvedSources` in `.acp.config.json`. See [Chapter 4: Configuration Format](#4-configuration-format) for approved source configuration.

**Example with approved source:**
```typescript
/**
 * @acp:ref react:hooks - Follow React hooks patterns
 * @acp:ref-version 18.2
 * @acp:ref-section hooks/rules-of-hooks
 * @acp:ref-fetch true
 */
```

#### @acp:fn (RFC-001)

**Scope:** Symbol level (function)
**Purpose:** Function description
**Directive:** MUST describe function purpose

| Annotation  | Parameters  | Example                                                                  |
|-------------|-------------|--------------------------------------------------------------------------|
| `@acp:fn`   | `<name>`    | `@acp:fn validateSession - Validates JWT token and checks session store` |

#### @acp:class (RFC-001)

**Scope:** Symbol level (class)
**Purpose:** Class description
**Directive:** MUST describe class purpose

| Annotation   | Parameters   | Example                                                        |
|--------------|--------------|----------------------------------------------------------------|
| `@acp:class` | `<name>`     | `@acp:class SessionStore - In-memory session storage with TTL` |

#### @acp:method (RFC-001)

**Scope:** Symbol level (method)
**Purpose:** Method description
**Directive:** MUST describe method behavior

| Annotation    | Parameters  | Example                                                              |
|---------------|-------------|----------------------------------------------------------------------|
| `@acp:method` | `<name>`    | `@acp:method get - Retrieves session by ID, returns null if expired` |

#### @acp:param (RFC-001)

**Scope:** Symbol level
**Purpose:** Parameter description
**Directive:** MUST describe parameter

| Annotation   | Parameters   | Example                                        |
|--------------|--------------|------------------------------------------------|
| `@acp:param` | `<name>`     | `@acp:param token - The expired but valid JWT` |

#### @acp:returns (RFC-001)

**Scope:** Symbol level
**Purpose:** Return value description
**Directive:** MUST describe return value

| Annotation     | Parameters  | Example                                                 |
|----------------|-------------|---------------------------------------------------------|
| `@acp:returns` | none        | `@acp:returns New JWT string or null if refresh denied` |

#### @acp:throws (RFC-001)

**Scope:** Symbol level
**Purpose:** Exception description
**Directive:** MUST describe exception

| Annotation    | Parameters  | Example                                                  |
|---------------|-------------|----------------------------------------------------------|
| `@acp:throws` | `<type>`    | `@acp:throws TokenExpiredError - When token has expired` |

#### @acp:example (RFC-001)

**Scope:** Symbol level
**Purpose:** Usage example
**Directive:** SHOULD include example code

| Annotation     | Parameters  | Example                                               |
|----------------|-------------|-------------------------------------------------------|
| `@acp:example` | none        | `@acp:example const session = validateSession(token)` |

#### @acp:critical (RFC-001)

**Scope:** Inline
**Purpose:** Critical code marker
**Directive:** MUST explain criticality

| Annotation      | Parameters  | Example                                                 |
|-----------------|-------------|---------------------------------------------------------|
| `@acp:critical` | none        | `@acp:critical - Token expiry check, security boundary` |

#### @acp:todo (RFC-001)

**Scope:** Inline
**Purpose:** Pending work item
**Directive:** MUST describe work needed

| Annotation  | Parameters  | Example                                     |
|-------------|-------------|---------------------------------------------|
| `@acp:todo` | none        | `@acp:todo - Add rate limiting per session` |

#### @acp:fixme (RFC-001)

**Scope:** Inline
**Purpose:** Known issue marker
**Directive:** MUST describe issue

| Annotation   | Parameters   | Example                                            |
|--------------|--------------|----------------------------------------------------|
| `@acp:fixme` | none         | `@acp:fixme - Race condition in concurrent access` |

#### @acp:perf (RFC-001)

**Scope:** Inline
**Purpose:** Performance note
**Directive:** SHOULD describe consideration

| Annotation  | Parameters  | Example                                                                  |
|-------------|-------------|--------------------------------------------------------------------------|
| `@acp:perf` | none        | `@acp:perf - O(n²) complexity, consider optimization for large datasets` |

### RFC-0003 Annotations

<!-- ADDED: v0.5 - RFC-0003 Annotation Provenance Tracking -->

The following annotations were added per RFC-0003 for tracking annotation provenance.

#### @acp:source (RFC-0003)

**Scope:** Following annotation block
**Purpose:** Annotation origin marker
**Directive:** Identifies how an annotation was created

| Annotation               | Parameters   | Example                          | Description                              |
|--------------------------|--------------|----------------------------------|------------------------------------------|
| `@acp:source`            | `<origin>`   | `@acp:source heuristic`          | Origin of preceding annotation(s)        |
| `@acp:source-confidence` | `<0.0-1.0>`  | `@acp:source-confidence 0.85`    | Confidence score for auto-generated      |
| `@acp:source-reviewed`   | `<boolean>`  | `@acp:source-reviewed true`      | Whether human has reviewed               |
| `@acp:source-id`         | `<id>`       | `@acp:source-id gen-20251222-01` | Generation batch identifier              |

**Source Origin Values:**

| Origin     | Description                                    | Usage                              |
|------------|------------------------------------------------|------------------------------------|
| `explicit` | Human-written (default if no `@acp:source`)    | Manual annotation                  |
| `converted`| Converted from JSDoc/docstring/etc.            | `acp annotate --convert`           |
| `heuristic`| Generated by naming/path/visibility heuristics | `acp annotate`                     |
| `refined`  | AI-improved from previous auto-generation      | `acp review --refine`              |
| `inferred` | Inferred from code analysis (future)           | Reserved for future use            |

**Example - Single annotation with provenance:**
```javascript
/**
 * @acp:summary "Validates user authentication tokens"
 * @acp:source heuristic
 * @acp:source-confidence 0.85
 */
function validateToken(token) { ... }
```

**Example - Multiple annotations, single provenance block:**
```javascript
/**
 * @acp:summary "Payment processing service"
 * @acp:domain billing
 * @acp:lock restricted
 * @acp:source heuristic
 * @acp:source-confidence 0.72
 */
```

**Example - Refined annotation:**
```javascript
/**
 * @acp:summary "Securely validates JWT tokens with RS256 signature verification"
 * @acp:source refined
 * @acp:source-reviewed true
 */
```

**Grepability:** Provenance markers are designed to be easily searchable:
```bash
# Find all heuristic-generated annotations
grep -r "@acp:source heuristic" src/

# Find low-confidence annotations
grep -r "@acp:source-confidence 0\.[0-6]" src/

# Find reviewed annotations
grep -r "@acp:source-reviewed true" src/
```

See [Chapter 5: Annotations](#5-annotations) for detailed provenance tracking documentation.

### Extension Annotations

Custom annotations use: `@acp:x-{vendor}:{feature}`

**Examples:**
- `@acp:x-github:copilot-context "Provide detailed context"`
- `@acp:x-cursor:rules-ref "project-rules.md"`
- `@acp:x-mycompany:audit-required "SOX compliance"`

See Section 4.5 for extension rules.

### Deprecated Annotations

(None in v1.0)

### Future Reserved

The following namespaces are reserved for future use:
- `@acp:security` - Security annotations
- `@acp:access` - Access control annotations

**Note:** `@acp:perf` was reserved in v1.0 and implemented in v1.1 (RFC-001).

---

## Appendix B: JSON Schema Reference

<!-- ADDED: v1 - JSON Schema definitions per GAP-C4 -->

This appendix provides JSON Schema definitions for validating ACP files.

### Schema Locations

Official schemas are hosted at:
- Cache: `https://acp-protocol.dev/schemas/v1/cache.schema.json`
- Variables: `https://acp-protocol.dev/schemas/v1/vars.schema.json`
- Config: `https://acp-protocol.dev/schemas/v1/config.schema.json`

### B.1 Cache Schema (`cache.schema.json`)

```json
{
  "$schema": "http://json-schema.org/draft-07/schema#",
  "$id": "https://acp-protocol.dev/schemas/v1/cache.schema.json",
  "title": "ACP Cache Format",
  "description": "Schema for .acp.cache.json files",
  "type": "object",
  "required": ["version", "generated_at", "project", "files", "symbols"],
  "properties": {
    "version": {
      "type": "string",
      "pattern": "^\\d+\\.\\d+\\.\\d+$",
      "description": "SemVer version of ACP spec"
    },
    "generated_at": {
      "type": "string",
      "format": "date-time",
      "description": "ISO 8601 timestamp of generation"
    },
    "git_commit": {
      "type": "string",
      "pattern": "^[0-9a-f]{40}$",
      "description": "Git commit SHA if in repo"
    },
    "project": {
      "type": "object",
      "required": ["name", "root"],
      "properties": {
        "name": {"type": "string"},
        "root": {"type": "string"},
        "description": {"type": "string"}
      }
    },
    "stats": {
      "type": "object",
      "properties": {
        "files": {"type": "integer", "minimum": 0},
        "symbols": {"type": "integer", "minimum": 0},
        "lines": {"type": "integer", "minimum": 0}
      }
    },
    "source_files": {
      "type": "object",
      "patternProperties": {
        "^.*$": {
          "type": "string",
          "format": "date-time"
        }
      }
    },
    "files": {
      "type": "object",
      "patternProperties": {
        "^.*$": {"$ref": "#/definitions/FileEntry"}
      }
    },
    "symbols": {
      "type": "object",
      "patternProperties": {
        "^.*$": {"$ref": "#/definitions/SymbolEntry"}
      }
    },
    "graph": {"$ref": "#/definitions/CallGraph"},
    "domains": {
      "type": "object",
      "patternProperties": {
        "^.*$": {"$ref": "#/definitions/DomainEntry"}
      }
    },
    "constraints": {"$ref": "#/definitions/ConstraintIndex"}
  },
  "definitions": {
    "FileEntry": {
      "type": "object",
      "required": ["path", "lines", "language"],
      "properties": {
        "path": {"type": "string"},
        "module": {"type": ["string", "null"]},
        "summary": {"type": ["string", "null"]},
        "purpose": {"type": ["string", "null"], "description": "RFC-001: File purpose from @acp:purpose"},
        "owner": {"type": ["string", "null"], "description": "RFC-001: Team ownership from @acp:owner"},
        "lines": {"type": "integer", "minimum": 0},
        "language": {"type": "string"},
        "domains": {"type": "array", "items": {"type": "string"}},
        "layer": {"type": ["string", "null"]},
        "stability": {
          "type": ["string", "null"],
          "enum": ["stable", "experimental", "deprecated", null]
        },
        "exports": {"type": "array", "items": {"type": "string"}},
        "imports": {"type": "array", "items": {"type": "string"}},
        "inline": {
          "type": "array",
          "items": {"$ref": "#/definitions/InlineAnnotation"},
          "description": "RFC-001: Inline annotations"
        }
      }
    },
    "InlineAnnotation": {
      "type": "object",
      "required": ["type", "line", "directive"],
      "properties": {
        "type": {"type": "string", "enum": ["critical", "todo", "fixme", "perf", "hack"]},
        "value": {"type": "string"},
        "line": {"type": "integer", "minimum": 1},
        "directive": {"type": "string", "description": "RFC-001: Self-documenting directive"},
        "ticket": {"type": "string"},
        "expires": {"type": "string", "format": "date"},
        "auto_generated": {"type": "boolean", "default": false}
      }
    },
    "SymbolEntry": {
      "type": "object",
      "required": ["name", "qualified_name", "type", "file", "lines", "exported"],
      "properties": {
        "name": {"type": "string"},
        "qualified_name": {"type": "string"},
        "type": {"type": "string"},
        "file": {"type": "string"},
        "lines": {
          "type": "array",
          "items": {"type": "integer"},
          "minItems": 2,
          "maxItems": 2
        },
        "signature": {"type": ["string", "null"]},
        "summary": {"type": ["string", "null"]},
        "purpose": {"type": ["string", "null"], "description": "RFC-001: Symbol purpose from @acp:fn/etc"},
        "params": {
          "type": "array",
          "items": {"$ref": "#/definitions/ParamEntry"},
          "description": "RFC-001: Parameter descriptions"
        },
        "returns": {"$ref": "#/definitions/ReturnsEntry", "description": "RFC-001: Return value description"},
        "throws": {
          "type": "array",
          "items": {"$ref": "#/definitions/ThrowsEntry"},
          "description": "RFC-001: Exception descriptions"
        },
        "constraints": {"$ref": "#/definitions/SymbolConstraints", "description": "RFC-001: Symbol-level constraints"},
        "async": {"type": "boolean"},
        "exported": {"type": "boolean"},
        "visibility": {"type": "string", "enum": ["public", "private", "protected"]},
        "calls": {"type": "array", "items": {"type": "string"}},
        "called_by": {"type": "array", "items": {"type": "string"}}
      }
    },
    "ParamEntry": {
      "type": "object",
      "required": ["name"],
      "properties": {
        "name": {"type": "string"},
        "description": {"type": "string"},
        "directive": {"type": "string", "description": "RFC-001: Directive for parameter usage"}
      }
    },
    "ReturnsEntry": {
      "type": "object",
      "properties": {
        "description": {"type": "string"},
        "directive": {"type": "string", "description": "RFC-001: Directive for handling return value"}
      }
    },
    "ThrowsEntry": {
      "type": "object",
      "required": ["exception"],
      "properties": {
        "exception": {"type": "string"},
        "description": {"type": "string"},
        "directive": {"type": "string", "description": "RFC-001: How to handle the exception"}
      }
    },
    "SymbolConstraints": {
      "type": "object",
      "properties": {
        "lock_level": {
          "type": "string",
          "enum": ["frozen", "restricted", "approval-required", "tests-required", "docs-required", "review-required", "normal", "experimental"]
        },
        "lock_reason": {"type": "string"},
        "directive": {"type": "string", "description": "RFC-001: Self-documenting directive"},
        "auto_generated": {"type": "boolean", "default": false}
      }
    },
    "CallGraph": {
      "type": "object",
      "properties": {
        "forward": {
          "type": "object",
          "patternProperties": {
            "^.*$": {"type": "array", "items": {"type": "string"}}
          }
        },
        "reverse": {
          "type": "object",
          "patternProperties": {
            "^.*$": {"type": "array", "items": {"type": "string"}}
          }
        }
      }
    },
    "DomainEntry": {
      "type": "object",
      "required": ["name"],
      "properties": {
        "name": {"type": "string"},
        "description": {"type": "string"},
        "files": {"type": "array", "items": {"type": "string"}},
        "symbols": {"type": "array", "items": {"type": "string"}}
      }
    },
    "ConstraintIndex": {
      "type": "object",
      "properties": {
        "by_file": {
          "type": "object",
          "patternProperties": {
            "^.*$": {
              "type": "object",
              "properties": {
                "lock_level": {"type": "string"},
                "lock_reason": {"type": "string"},
                "directive": {"type": "string", "description": "RFC-001: Self-documenting directive"},
                "auto_generated": {"type": "boolean", "default": false, "description": "RFC-001: True if auto-generated"},
                "style": {"type": "string"}
              }
            }
          }
        },
        "by_lock_level": {
          "type": "object",
          "patternProperties": {
            "^.*$": {"type": "array", "items": {"type": "string"}}
          }
        }
      }
    }
  }
}
```

### B.2 Variables Schema (`vars.schema.json`)

```json
{
  "$schema": "http://json-schema.org/draft-07/schema#",
  "$id": "https://acp-protocol.dev/schemas/v1/vars.schema.json",
  "title": "ACP Variables Format",
  "description": "Schema for .acp.vars.json files",
  "type": "object",
  "required": ["version", "variables"],
  "properties": {
    "version": {
      "type": "string",
      "pattern": "^\\d+\\.\\d+\\.\\d+$"
    },
    "variables": {
      "type": "object",
      "patternProperties": {
        "^[A-Z_]+$": {
          "type": "object",
          "required": ["type", "value"],
          "properties": {
            "type": {
              "type": "string",
              "enum": ["symbol", "file", "domain"]
            },
            "value": {"type": "string"},
            "description": {"type": "string"}
          }
        }
      }
    }
  }
}
```

### B.3 Config Schema (`config.schema.json`)

```json
{
  "$schema": "http://json-schema.org/draft-07/schema#",
  "$id": "https://acp-protocol.dev/schemas/v1/config.schema.json",
  "title": "ACP Config Format",
  "description": "Schema for .acp.config.json files",
  "type": "object",
  "required": ["version"],
  "properties": {
    "version": {
      "type": "string",
      "pattern": "^\\d+\\.\\d+\\.\\d+$"
    },
    "include": {
      "type": "array",
      "items": {"type": "string"}
    },
    "exclude": {
      "type": "array",
      "items": {"type": "string"}
    },
    "error_handling": {
      "type": "object",
      "properties": {
        "strictness": {
          "type": "string",
          "enum": ["permissive", "strict"]
        },
        "max_errors": {"type": "integer", "minimum": 1},
        "auto_correct": {"type": "boolean"}
      }
    },
    "constraints": {
      "type": "object",
      "properties": {
        "defaults": {
          "type": "object",
          "properties": {
            "lock": {"type": "string"}
          }
        },
        "track_violations": {"type": "boolean"},
        "audit_file": {"type": "string"}
      }
    },
    "domains": {
      "type": "object",
      "patternProperties": {
        "^.*$": {
          "type": "object",
          "properties": {
            "patterns": {
              "type": "array",
              "items": {"type": "string"}
            }
          }
        }
      }
    },
    "call_graph": {
      "type": "object",
      "properties": {
        "include_stdlib": {"type": "boolean"},
        "max_depth": {"type": ["integer", "null"]},
        "exclude_patterns": {
          "type": "array",
          "items": {"type": "string"}
        }
      }
    },
    "limits": {
      "type": "object",
      "properties": {
        "max_file_size_mb": {"type": "integer", "minimum": 1},
        "max_files": {"type": "integer", "minimum": 1},
        "max_annotations_per_file": {"type": "integer", "minimum": 1},
        "max_cache_size_mb": {"type": "integer", "minimum": 1}
      }
    }
  }
}
```

### Usage

To validate a cache file:
```bash
jsonschema -i .acp.cache.json https://acp-protocol.dev/schemas/v1/cache.schema.json
```

To validate programmatically (Node.js with Ajv):
```javascript
const Ajv = require('ajv');
const ajv = new Ajv();

const schema = require('./cache.schema.json');
const data = require('./.acp.cache.json');

const valid = ajv.validate(schema, data);
if (!valid) console.log(ajv.errors);
```

---

## Appendix C: Language Support

<!-- CHANGED: v1 - Updated to external reference per Decision Q9 -->

Detailed language-specific parsing notes are maintained in external documentation for easier community contribution and updates.

**Documentation:** `docs/LANGUAGE-SUPPORT.md`
**Repository:** https://github.com/acp-protocol/acp-spec

This document covers:
- Per-language annotation extraction details
- Comment syntax variations
- Language-specific edge cases
- Parser recommendations
- Community-contributed language support

Implementations SHOULD consult this documentation when adding language support.

**Core languages with official support** (as of v1.0):
- TypeScript/JavaScript
- Python
- Rust
- Go
- Java
- C#

**Community-contributed languages:**
See documentation for current list and contribution guidelines.

---

## Changelog

### 1.2.0 (2025-12-21) - Complete File Format Documentation

#### Added
- **Section 3.4**: Attempts File (`.acp/acp.attempts.json`) with key field documentation
- **Section 3.5**: Sync File (`.acp/acp.sync.json`) with key field documentation
- **Section 3.6**: Primer File (`.acp.primer.json`) with key field documentation
- Cross-references from Chapter 13 to Section 3.4 for attempts file
- Cross-references from Chapter 14 to Section 3.6 for primer file

#### Changed
- Updated Section 3 file count from "three" to "six" JSON files
- Expanded file format table with all schema-backed files
- Added Level 2 implementation notes for new file support

### 1.1.0 (2025-12-21) - RFC-001 Compliance

#### Added
- **Directive suffix requirement** for all annotations (` - <directive>`) per RFC-001
- **New file-level annotations**: `@acp:purpose`, `@acp:owner`, `@acp:ref`
- **New symbol-level annotations**: `@acp:fn`, `@acp:class`, `@acp:method`, `@acp:param`, `@acp:returns`, `@acp:throws`, `@acp:example`
- **New inline annotations**: `@acp:critical`, `@acp:todo`, `@acp:fixme`, `@acp:perf`
- **FileEntry fields**: `purpose`, `owner`, `inline` array
- **SymbolEntry fields**: `purpose`, `params`, `returns`, `throws`, `constraints`
- **ConstraintIndex fields**: `directive`, `auto_generated`
- **Chapter 14**: Bootstrap & AI Integration (see `spec/chapters/14-bootstrap.md`)

#### Changed
- EBNF grammar to include directive suffix
- All annotation examples updated with directive suffixes
- Reserved namespaces table reorganized by annotation level (file/symbol/inline)
- JSON Schema updated for all new fields

### 1.0.0-revised (2025-12-17)

#### Added
- Section 10: Conformance Levels (3-tier structure: Reader/Standard/Full)
- Section 11: Error Handling (4 categories, strictness modes, error codes)
- Section 4.5: Extension Namespaces (@acp:x-vendor:feature pattern)
- Section 4.6: Parsing Algorithm details
- Section 5.6: Constraint Violation Tracking (optional)
- Section 6.5: Variable Error Handling (undefined, circular, invalid modifiers)
- Section 6.6: Variable Scoping (global scope specified)
- Section 7.3: Complex inheritance examples
- Section 8.3: Cache Building Details (domain/layer detection, call graph algorithms)
- Section 8.4: Language Detection (extension mapping table)
- Section 8.5: Implementation Limits (configurable limits)
- Section 3.1.5: Staleness Detection (git-aware + timestamp fallback)
- Appendix A: Complete Annotation Reference (full table of all annotations)
- Appendix B: JSON Schema Reference (complete schemas for all file formats)

#### Changed
- **All JSON field names to snake_case** (e.g., `generatedAt` → `generated_at`, `qualifiedName` → `qualified_name`)
- Section 3.1: Completed FileEntry and SymbolEntry field specifications with required/optional/default markers
- Section 3.1: Specified symbol qualification format (`file_path:class.symbol`)
- Section 3.2-3.3: Completed variables and config file specifications
- Section 5.1: Clarified advisory semantics (MUST applies to conformant AI systems)
- Section 5.2: Added default lock level (`normal`)
- Section 6.4: Clarified modifier semantics with detailed behavior table
- Section 7.2: Clarified merge rules (most restrictive, accumulation, override)
- Section 12: Renumbered from Section 10 (due to new sections 10 & 11)
- Appendix C: Updated to external reference model

#### Removed
- None (additive changes only)

#### Fixed
- GAP-C1: Error handling now fully specified
- GAP-C2: Conformance levels defined with feature matrix
- GAP-C3: File formats complete with all field specifications
- GAP-C4: JSON schemas provided
- GAP-C5: Annotation reference complete
- GAP-C6: Variable behavior specified (undefined, circular, scoping)
- GAP-C7: Staleness detection algorithm added
- GAP-C8: JSON naming consistent (snake_case throughout)
- GAP-I9: Parsing algorithm specified
- GAP-I10: Cache building algorithms specified
- GAP-I11: Merge rules clarified with examples
- GAP-I12: Advisory semantics clear
- GAP-I13: Modifier semantics detailed
- GAP-I14: Implementation limits documented
- GAP-I15: Extension mechanism specified
- GAP-I16: Default values consolidated
- GAP-I17: Symbol format specified
- GAP-I18: Language mapping provided
- GAP-I19: Test suite referenced in conformance section
- All critical and important gaps addressed (19/19)

#### Deferred to v1.1
- Debug/Hack feature detailed specifications (GAP-M20)
- Comprehensive complex examples (GAP-M21)

### 1.0.0-draft (2025-12-15)

Initial draft specification release.

---

**End of Specification**

---

*This specification is maintained by the ACP working group.*
*For issues, contributions, or questions: https://github.com/acp-protocol/acp-spec*


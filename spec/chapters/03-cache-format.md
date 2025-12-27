# Cache File Format Specification

**ACP Version**: 1.0.0
**Document Version**: 1.3.0
**Last Updated**: 2025-12-26
**Status**: RFC-001, RFC-0002, RFC-0003, RFC-0008 Compliant

---

## Table of Contents

1. [Overview](#1-overview)
2. [File Format](#2-file-format)
3. [Root Structure](#3-root-structure) - Updated with `documentation` index (RFC-0002), `provenance` statistics (RFC-0003)
4. [File Entries](#4-file-entries) - Updated with `purpose`, `owner`, `inline`, `refs`, `style`, `annotations` (RFC-0003)
5. [Symbol Entries](#5-symbol-entries) - Updated with `purpose`, `params`, `returns`, `throws`, `constraints`, `annotations` (RFC-0003), `type_info` (RFC-0008)
6. [Call Graph](#6-call-graph)
7. [Domain Index](#7-domain-index)
8. [Constraint Index](#8-constraint-index) - Updated with `directive`, `auto_generated`
9. [Documentation Index (RFC-0002)](#9-documentation-index-rfc-0002)
10. [Provenance Index (RFC-0003)](#10-provenance-index-rfc-0003)
11. [Generation](#11-generation)
12. [Validation](#12-validation)

---

## 1. Overview

### 1.1 Purpose

The cache file (`.acp.cache.json`) is the indexed representation of a codebase. It provides AI systems with structured access to:

- File and module metadata with purpose descriptions
- Symbol definitions with directives and relationships
- Call graphs and dependencies
- Domain classifications
- Constraint and guardrail information with directives
- Inline annotations for tracking items (todo, fixme, critical)

The cache enables token-efficient AI interactions by pre-computing codebase structure rather than analyzing files on every request.

### 1.2 Design Principles

- **Query Optimized**: Structure enables efficient queries via `jq` or programmatic access
- **Self-Contained**: All information needed for AI context in one file
- **Deterministic**: Same codebase always produces identical cache (for diffing)
- **Incremental**: Supports partial updates without full regeneration
- **Token Efficient**: Minimizes context sent to AI systems

### 1.3 File Location

The cache file:
- MUST be named `.acp.cache.json`
- SHOULD be located in the project root
- MAY be placed in a configured location via `.acp.config.json`
- SHOULD be added to `.gitignore` (generated artifact)

### 1.4 Conformance

The key words "MUST", "MUST NOT", "REQUIRED", "SHALL", "SHALL NOT", "SHOULD", "SHOULD NOT", "RECOMMENDED", "MAY", and "OPTIONAL" in this document are to be interpreted as described in [RFC 2119](https://datatracker.ietf.org/doc/html/rfc2119).

---

## 2. File Format

### 2.1 Encoding

- MUST be valid JSON (RFC 8259)
- MUST use UTF-8 encoding
- SHOULD use 2-space indentation for readability
- MUST NOT include comments (standard JSON)
- MUST use snake_case for all field names

### 2.2 Size Considerations

| Codebase Size | Typical Cache Size |
|---------------|-------------------|
| Small (<100 files) | <100 KB |
| Medium (100-1000 files) | 100 KB - 1 MB |
| Large (1000-10000 files) | 1 MB - 10 MB |
| Very Large (>10000 files) | Consider splitting |

### 2.3 Schema

The cache file MUST conform to the JSON Schema at:
`https://acp-spec.org/schemas/v1/cache.schema.json`

---

## 3. Root Structure

### 3.1 Top-Level Fields

```json
{
  "version": "1.0.0",
  "generated_at": "2025-12-21T15:30:00Z",
  "git_commit": "abc123def456",
  "project": { },
  "stats": { },
  "source_files": { },
  "files": { },
  "symbols": { },
  "annotations": { },
  "graph": { },
  "domains": { },
  "constraints": { },
  "documentation": { },
  "provenance": { }
}
```

**Notes:**
- The `annotations` section is new in RFC-001 and stores all annotations with their directives.
- The `documentation` section is new in RFC-0002 and stores the project-wide documentation index.
- The `provenance` section is new in RFC-0003 and stores annotation provenance statistics.

### 3.2 Field Definitions

#### `version` (required)

ACP specification version this cache conforms to.

- Type: `string`
- Format: Semantic version (e.g., `"1.0.0"`)
- MUST match major version of ACP spec

#### `generated_at` (required)

Timestamp when cache was generated.

- Type: `string`
- Format: ISO 8601 datetime with timezone
- Example: `"2024-12-17T15:30:00Z"`
- Used for staleness detection

#### `git_commit` (optional)

Git commit SHA if project is in a git repository.

- Type: `string` or `null`
- Format: 40-character hex SHA
- Example: `"abc123def456789..."`
- Used for git-aware staleness detection

#### `project` (required)

Project metadata.

```json
{
  "project": {
    "name": "my-project",
    "root": "/absolute/path/to/project",
    "description": "Optional project description"
  }
}
```

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | string | Yes | Project name |
| `root` | string | Yes | Absolute path to project root |
| `description` | string | No | Project description |

#### `stats` (required)

Aggregate statistics.

```json
{
  "stats": {
    "files": 127,
    "symbols": 843,
    "lines": 24521
  }
}
```

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `files` | integer | Yes | Total indexed files |
| `symbols` | integer | Yes | Total indexed symbols |
| `lines` | integer | Yes | Total lines of code |

#### `source_files` (required)

Map of file paths to modification times for staleness detection.

```json
{
  "source_files": {
    "src/auth/session.ts": "2024-12-17T15:29:00Z",
    "src/utils/helpers.ts": "2024-12-17T14:20:00Z"
  }
}
```

- Keys: Relative file paths
- Values: ISO 8601 timestamps (last modification time)

---

## 4. File Entries

### 4.1 Structure

The `files` object maps relative file paths to file entry objects.

```json
{
  "files": {
    "src/auth/session.ts": {
      "path": "src/auth/session.ts",
      "module": "Session Management",
      "summary": "Handles user session lifecycle and validation",
      "lines": 234,
      "language": "typescript",
      "domains": ["authentication"],
      "layer": "service",
      "stability": "stable",
      "exports": ["src/auth/session.ts:SessionService"],
      "imports": ["jsonwebtoken", "src/db/users"]
    }
  }
}
```

### 4.2 Field Definitions

| Field | Type | Required | Default | Description |
|-------|------|----------|---------|-------------|
| `path` | string | ✓ MUST | - | Relative path from project root |
| `purpose` | string | ⚠ SHOULD | null | File purpose (from `@acp:purpose`) - RFC-001 |
| `module` | string | ⚠ SHOULD | null | Human-readable module name (from `@acp:module`) |
| `summary` | string | ✗ MAY | null | Brief file description (legacy, use `purpose`) |
| `owner` | string | ✗ MAY | null | Team ownership (from `@acp:owner`) - RFC-001 |
| `lines` | integer | ✓ MUST | - | Line count |
| `language` | string | ✓ MUST | - | Programming language |
| `domains` | array[string] | ✗ MAY | [] | Domain classifications (from `@acp:domain`) |
| `layer` | string | ✗ MAY | null | Architectural layer (from `@acp:layer`) |
| `stability` | string | ✗ MAY | null | Stability level: `stable`, `experimental`, `deprecated` |
| `exports` | array[string] | ⚠ SHOULD | [] | Exported symbols (qualified names) |
| `imports` | array[string] | ⚠ SHOULD | [] | Imported modules |
| `inline` | array[object] | ✗ MAY | [] | Inline annotations in file - RFC-001 |
| `refs` | array[object] | ✗ MAY | [] | Documentation references - RFC-0002 |
| `style` | object | ✗ MAY | null | Style guide configuration - RFC-0002 |
| `annotations` | object | ✗ MAY | {} | Annotation provenance tracking - RFC-0003 |

#### `refs` Array (RFC-0002)

The `refs` array stores documentation references from `@acp:ref` and related annotations:

```json
{
  "refs": [
    {
      "url": "https://react.dev/reference/react/hooks",
      "sourceId": "react",
      "version": "18.2",
      "section": "hooks/rules-of-hooks",
      "fetch": true,
      "scope": "file",
      "line": 5
    },
    {
      "url": "https://docs.company.com/api/auth",
      "sourceId": "company-api",
      "fetch": false,
      "scope": "symbol",
      "symbolName": "validateSession",
      "line": 45
    }
  ]
}
```

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `url` | string | Yes | Documentation URL |
| `sourceId` | string | No | Approved source ID from config (if applicable) |
| `version` | string | No | Documentation version (from `@acp:ref-version`) |
| `section` | string | No | Section within documentation (from `@acp:ref-section`) |
| `fetch` | boolean | No | Whether AI should fetch (from `@acp:ref-fetch`), default: false |
| `scope` | string | No | Reference scope: `file` or `symbol`, default: `file` |
| `symbolName` | string | No | Symbol name if scope is `symbol` |
| `line` | integer | No | Line number where reference appears |

#### `style` Object (RFC-0002)

The `style` object stores style guide configuration from `@acp:style` and related annotations:

```json
{
  "style": {
    "name": "company-react",
    "extends": "airbnb-react",
    "source": "company-api",
    "url": "https://docs.company.com/style",
    "rules": ["prefer-function-components", "max-line-length=120"],
    "scope": "file",
    "line": 3
  }
}
```

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | string | No | Style guide name or ID |
| `extends` | string | No | Parent style guide (from `@acp:style-extends`) |
| `source` | string | No | Documentation source ID for this style |
| `url` | string | No | Direct URL to style guide documentation |
| `rules` | array[string] | No | Specific style rules applied |
| `scope` | string | No | Style scope: `file` or `symbol`, default: `file` |
| `symbolName` | string | No | Symbol name if scope is `symbol` |
| `line` | integer | No | Line number where style is declared |

#### `inline` Array (RFC-001)

The `inline` array stores inline annotations (`@acp:critical`, `@acp:todo`, `@acp:fixme`, `@acp:perf`):

```json
{
  "inline": [
    {
      "type": "critical",
      "line": 45,
      "directive": "Review with extreme care; errors here have severe consequences"
    },
    {
      "type": "todo",
      "value": "Add rate limiting",
      "line": 78,
      "directive": "This work is pending; consider completing before related changes",
      "ticket": "JIRA-123"
    },
    {
      "type": "fixme",
      "value": "Race condition",
      "line": 102,
      "directive": "Known issue that needs resolution; avoid relying on current behavior"
    }
  ]
}
```

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `type` | string | Yes | Annotation type: `critical`, `todo`, `fixme`, `perf`, `hack` |
| `value` | string | No | Annotation value (e.g., task description) |
| `line` | integer | Yes | Line number |
| `directive` | string | Yes | Self-documenting directive text |
| `ticket` | string | No | Related issue/ticket reference |
| `expires` | string | No | Expiration date (ISO 8601) for hacks |
| `auto_generated` | boolean | No | True if directive was auto-generated |

#### `annotations` Object (RFC-0003)

The `annotations` object stores provenance tracking information for annotations in the file:

```json
{
  "annotations": {
    "summary": {
      "value": "Session management utilities",
      "source": "heuristic",
      "confidence": 0.82,
      "needsReview": false,
      "reviewed": true,
      "reviewedAt": "2025-01-15T10:30:00Z",
      "generatedAt": "2025-01-10T14:22:00Z",
      "generationId": "gen-20250110-001"
    },
    "domain": {
      "value": "authentication",
      "source": "heuristic",
      "confidence": 0.75,
      "needsReview": true,
      "reviewed": false,
      "generatedAt": "2025-01-10T14:22:00Z",
      "generationId": "gen-20250110-001"
    }
  }
}
```

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `value` | string | Yes | The annotation value |
| `source` | string | No | Origin: `explicit`, `converted`, `heuristic`, `refined`, `inferred` |
| `confidence` | number | No | Confidence score (0.0-1.0) |
| `needsReview` | boolean | No | Whether annotation is flagged for review |
| `reviewed` | boolean | No | Whether annotation has been reviewed |
| `reviewedAt` | string | No | ISO 8601 timestamp of review |
| `generatedAt` | string | No | ISO 8601 timestamp of generation |
| `generationId` | string | No | Batch identifier for generation run |

### 4.3 Language Detection

Languages MUST be identified using standard identifiers:

| Language | Identifier | Extensions |
|----------|------------|------------|
| TypeScript | `typescript` | `.ts`, `.tsx`, `.mts`, `.cts` |
| JavaScript | `javascript` | `.js`, `.jsx`, `.mjs`, `.cjs` |
| Python | `python` | `.py`, `.pyi`, `.pyw` |
| Rust | `rust` | `.rs` |
| Go | `go` | `.go` |
| Java | `java` | `.java` |
| C# | `c-sharp` | `.cs` |
| C++ | `cpp` | `.cpp`, `.cc`, `.cxx`, `.hpp` |
| C | `c` | `.c`, `.h` |
| Ruby | `ruby` | `.rb` |
| PHP | `php` | `.php` |
| Swift | `swift` | `.swift` |
| Kotlin | `kotlin` | `.kt`, `.kts` |

### 4.4 Examples

#### Minimal File Entry

```json
{
  "path": "src/utils/helpers.ts",
  "lines": 45,
  "language": "typescript",
  "exports": [],
  "imports": []
}
```

#### Complete File Entry

```json
{
  "path": "src/auth/session.ts",
  "module": "Session Management",
  "summary": "Handles user session lifecycle and validation",
  "lines": 234,
  "language": "typescript",
  "domains": ["authentication", "security"],
  "layer": "service",
  "stability": "stable",
  "exports": [
    "src/auth/session.ts:SessionService",
    "src/auth/session.ts:SessionService.validateSession"
  ],
  "imports": [
    "jsonwebtoken",
    "redis",
    "src/db/users",
    "src/utils/crypto"
  ]
}
```

---

## 5. Symbol Entries

### 5.1 Structure

The `symbols` object maps qualified symbol names to symbol entry objects.

```json
{
  "symbols": {
    "src/auth/session.ts:SessionService.validateSession": {
      "name": "validateSession",
      "qualified_name": "src/auth/session.ts:SessionService.validateSession",
      "type": "method",
      "file": "src/auth/session.ts",
      "lines": [45, 89],
      "signature": "(token: string) => Promise<Session | null>",
      "summary": "Validates JWT token and returns session",
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
  }
}
```

### 5.2 Field Definitions

| Field | Type | Required | Default | Description |
|-------|------|----------|---------|-------------|
| `name` | string | ✓ MUST | - | Simple symbol name |
| `qualified_name` | string | ✓ MUST | - | Format: `file_path:class.symbol` |
| `type` | string | ✓ MUST | - | Symbol type (see below) |
| `file` | string | ✓ MUST | - | Containing file path |
| `lines` | [int, int] | ✓ MUST | - | [start_line, end_line] |
| `signature` | string | ⚠ SHOULD | null | Function signature if applicable |
| `purpose` | string | ⚠ SHOULD | null | Symbol purpose (from `@acp:fn`, `@acp:class`, etc.) - RFC-001 |
| `summary` | string | ✗ MAY | null | Brief description (legacy, use `purpose`) |
| `params` | array[object] | ✗ MAY | [] | Parameter descriptions - RFC-001 |
| `returns` | object | ✗ MAY | null | Return value description - RFC-001 |
| `throws` | array[object] | ✗ MAY | [] | Exception descriptions - RFC-001 |
| `async` | boolean | ✗ MAY | false | Whether async |
| `exported` | boolean | ✓ MUST | - | Whether exported |
| `visibility` | string | ✗ MAY | "public" | `public`, `private`, `protected` |
| `calls` | array[string] | ✗ MAY | [] | Symbols this calls (qualified names) |
| `called_by` | array[string] | ✗ MAY | [] | Symbols calling this (qualified names) |
| `constraints` | object | ✗ MAY | null | Symbol-level constraints with directives - RFC-001 |
| `annotations` | object | ✗ MAY | {} | Annotation provenance tracking - RFC-0003 |
| `type_info` | object | ✗ MAY | null | Type annotation information - RFC-0008 |

#### Symbol Documentation Fields (RFC-001)

```json
{
  "params": [
    {
      "name": "token",
      "description": "JWT token string",
      "directive": "Ensure token is a valid JWT string before calling"
    }
  ],
  "returns": {
    "description": "Session object or null if invalid",
    "directive": "Handle null case appropriately in calling code"
  },
  "throws": [
    {
      "exception": "AuthError",
      "description": "When token is malformed",
      "directive": "Handle AuthError appropriately when calling"
    }
  ],
  "constraints": {
    "lock_level": "frozen",
    "lock_reason": "Core authentication logic",
    "directive": "MUST NOT modify this function under any circumstances"
  }
}
```

#### Type Information Fields (RFC-0008)

The `type_info` object stores type annotation information extracted from `@acp:param {Type}`, `@acp:returns {Type}`, and `@acp:template` annotations:

```json
{
  "type_info": {
    "params": [
      {
        "name": "token",
        "type": "string",
        "typeSource": "acp",
        "optional": false,
        "directive": "JWT token string to validate"
      },
      {
        "name": "options",
        "type": "ValidateOptions",
        "typeSource": "acp",
        "optional": true,
        "default": "{}",
        "directive": "Optional validation options"
      }
    ],
    "returns": {
      "type": "Promise<Session | null>",
      "typeSource": "acp",
      "directive": "Session object or null if invalid"
    },
    "typeParams": [
      {
        "name": "T",
        "constraint": "BaseSession",
        "directive": "Session type extending BaseSession"
      }
    ]
  }
}
```

**type_info Field Definitions:**

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `params` | array[ParamInfo] | No | Parameter type information |
| `returns` | ReturnInfo | No | Return type information |
| `typeParams` | array[TypeParam] | No | Generic type parameters |

**ParamInfo Structure:**

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | string | Yes | Parameter name |
| `type` | string | No | Type expression (e.g., `"string"`, `"Promise<User>"`) |
| `typeSource` | string | No | Origin: `"acp"`, `"inferred"`, or `"native"` |
| `optional` | boolean | No | Whether parameter is optional (from `[name]` syntax) |
| `default` | string | No | Default value (from `[name=default]` syntax) |
| `directive` | string | No | Directive text for this parameter |

**ReturnInfo Structure:**

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `type` | string | No | Return type expression |
| `typeSource` | string | No | Origin: `"acp"`, `"inferred"`, or `"native"` |
| `directive` | string | No | Directive text for return value |

**TypeParam Structure:**

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | string | Yes | Type parameter name (e.g., `"T"`) |
| `constraint` | string | No | Constraint type (from `extends` clause) |
| `directive` | string | No | Directive text for type parameter |

**typeSource Values:**

| Value | Description | Example Source |
|-------|-------------|----------------|
| `acp` | Type from ACP annotation `{Type}` | `@acp:param {string} name` |
| `inferred` | Type inferred from source code | Function signature parsing |
| `native` | Type bridged from native docs | JSDoc `@param {string}` |

### 5.3 Symbol Types

| Type | Description | Languages |
|------|-------------|-----------|
| `function` | Standalone function | All |
| `method` | Class/object method | All |
| `class` | Class definition | TS, JS, Python, Java, etc. |
| `interface` | Interface definition | TS, Java, Go |
| `type` | Type alias | TS |
| `enum` | Enumeration | TS, Java, Rust |
| `struct` | Struct definition | Rust, Go, C |
| `trait` | Trait definition | Rust |
| `const` | Constant | All |

### 5.4 Qualified Names

Qualified names MUST follow this format:

```
{relative_path}:{qualified_symbol}
```

**Examples:**
- `src/auth/session.ts:SessionService.validateSession` - Class method
- `src/utils/helpers.ts:formatDate` - Standalone function
- `lib/core.py:CoreEngine.process` - Python class method
- `src/auth/session.ts:SessionService` - Class itself

### 5.5 Examples

#### Function Symbol

```json
{
  "name": "formatDate",
  "qualified_name": "src/utils/helpers.ts:formatDate",
  "type": "function",
  "file": "src/utils/helpers.ts",
  "lines": [12, 15],
  "signature": "(date: Date) => string",
  "summary": "Formats date to ISO 8601 string",
  "exported": true,
  "visibility": "public"
}
```

#### Method Symbol with Call Graph

```json
{
  "name": "validateSession",
  "qualified_name": "src/auth/session.ts:SessionService.validateSession",
  "type": "method",
  "file": "src/auth/session.ts",
  "lines": [45, 89],
  "signature": "(token: string) => Promise<Session | null>",
  "summary": "Validates JWT token and returns session",
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

---

## 6. Call Graph

### 6.1 Structure

The `graph` object contains call relationships in both directions.

```json
{
  "graph": {
    "forward": {
      "src/auth/session.ts:SessionService.validateSession": [
        "src/auth/jwt.ts:verifyToken",
        "src/db/sessions.ts:findSession"
      ],
      "src/api/middleware.ts:authMiddleware": [
        "src/auth/session.ts:SessionService.validateSession"
      ]
    },
    "reverse": {
      "src/auth/jwt.ts:verifyToken": [
        "src/auth/session.ts:SessionService.validateSession"
      ],
      "src/auth/session.ts:SessionService.validateSession": [
        "src/api/middleware.ts:authMiddleware"
      ]
    }
  }
}
```

### 6.2 Field Definitions

| Field | Type | Description |
|-------|------|-------------|
| `forward` | object | Map of symbol → symbols it calls |
| `reverse` | object | Map of symbol → symbols that call it |

### 6.3 Graph Properties

- Both `forward` and `reverse` MUST be present
- They MUST be consistent (inverse of each other)
- External calls (to libraries) MAY be included with qualified names
- Recursive calls MUST be included (symbol appears in own list)

### 6.4 Construction Algorithm

See [File Discovery Specification](discovery.md) Section 8.3.3 for call graph construction details.

---

## 7. Domain Index

### 7.1 Structure

The `domains` object groups files and symbols by domain.

```json
{
  "domains": {
    "authentication": {
      "name": "authentication",
      "description": "User authentication and session management",
      "files": [
        "src/auth/session.ts",
        "src/auth/jwt.ts",
        "src/auth/middleware.ts"
      ],
      "symbols": [
        "src/auth/session.ts:SessionService.validateSession",
        "src/auth/jwt.ts:generateToken",
        "src/auth/middleware.ts:authMiddleware"
      ]
    }
  }
}
```

### 7.2 Field Definitions

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | string | Yes | Domain identifier |
| `description` | string | No | Human description |
| `files` | array[string] | Yes | Files in this domain |
| `symbols` | array[string] | Yes | Symbols in this domain (qualified names) |

### 7.3 Domain Detection

Domains are detected via:
1. `@acp:domain` annotations (Priority 1)
2. Directory patterns in config (Priority 2)
3. Import analysis (Priority 3)

See [File Discovery Specification](discovery.md) Section 8.3.1 for domain detection algorithm.

---

## 8. Constraint Index

### 8.1 Structure

The `constraints` object indexes all constraints for efficient lookup.

```json
{
  "constraints": {
    "by_file": {
      "src/auth/session.ts": {
        "lock_level": "restricted",
        "lock_reason": "Security critical",
        "style": "google-typescript"
      }
    },
    "by_lock_level": {
      "frozen": ["src/config/production.ts"],
      "restricted": ["src/auth/session.ts"]
    }
  }
}
```

### 8.2 Field Definitions

| Field | Type | Description |
|-------|------|-------------|
| `by_file` | object | Map of file path → constraints |
| `by_lock_level` | object | Map of lock level → file paths |

### 8.3 Example Constraint Entry (RFC-001 Compliant)

```json
{
  "by_file": {
    "src/auth/session.ts": {
      "lock_level": "restricted",
      "lock_reason": "Security-critical code",
      "directive": "Explain proposed changes and wait for explicit approval before modifying",
      "style": "google-typescript",
      "behavior": "conservative",
      "quality": ["security-review", "tests-required"],
      "auto_generated": false
    }
  }
}
```

### 8.4 Constraint Field Definitions (RFC-001)

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `lock_level` | string | No | Lock level: `frozen`, `restricted`, `approval-required`, etc. |
| `lock_reason` | string | No | Structured reason for lock (grepable) |
| `directive` | string | Yes* | Self-documenting directive for AI (*required for constraints) |
| `style` | string | No | Style guide reference |
| `behavior` | string | No | AI behavior guidance |
| `quality` | array[string] | No | Quality requirements |
| `auto_generated` | boolean | No | True if directive was auto-generated from defaults |

**Note:** The `directive` field is REQUIRED for all constraints per RFC-001. The `lock_reason` provides a structured, grepable value while `directive` provides human/AI-readable instructions.

See [Constraint System Specification](constraints.md) for detailed constraint definitions.

---

## 9. Documentation Index (RFC-0002)

### 9.1 Structure

The `documentation` object provides a project-wide index of documentation references and styles.

```json
{
  "documentation": {
    "sources": {
      "react": {
        "url": "https://react.dev/reference",
        "version": "18.2",
        "fileCount": 15,
        "files": ["src/components/UserProfile.tsx", "src/hooks/useAuth.ts"]
      },
      "company-api": {
        "url": "https://docs.company.com/api",
        "version": null,
        "fileCount": 8,
        "files": ["src/api/auth.ts", "src/api/users.ts"]
      }
    },
    "styles": {
      "google-typescript": {
        "fileCount": 42,
        "files": ["src/utils/helpers.ts"],
        "source": null
      },
      "company-react": {
        "fileCount": 15,
        "files": ["src/components/UserProfile.tsx"],
        "source": "company-api"
      }
    },
    "unresolvedRefs": [
      {
        "file": "src/legacy/old.ts",
        "ref": "deprecated-api",
        "reason": "Source ID not found in approved sources"
      }
    ]
  }
}
```

### 9.2 Field Definitions

#### `sources`

Map of source ID to usage information.

| Field | Type | Description |
|-------|------|-------------|
| `url` | string | Documentation URL |
| `version` | string | Documentation version (if specified) |
| `fileCount` | integer | Number of files referencing this source |
| `files` | array[string] | List of files referencing this source |

#### `styles`

Map of style guide name to usage information.

| Field | Type | Description |
|-------|------|-------------|
| `fileCount` | integer | Number of files using this style |
| `files` | array[string] | List of files using this style |
| `source` | string | Associated documentation source ID (if any) |

#### `unresolvedRefs`

Array of references that could not be resolved.

| Field | Type | Description |
|-------|------|-------------|
| `file` | string | File containing the unresolved reference |
| `ref` | string | The unresolved reference value |
| `reason` | string | Why the reference could not be resolved |

### 9.3 Use Cases

The documentation index enables:
- Quick lookup of which files reference specific documentation
- Tracking of unresolved references for validation
- Aggregation of style guide usage across the project
- Discovery of documentation dependencies

---

## 10. Provenance Index (RFC-0003)

### 10.1 Structure

The `provenance` object provides project-wide statistics about annotation provenance.

```json
{
  "provenance": {
    "summary": {
      "total": 150,
      "bySource": {
        "explicit": 80,
        "converted": 20,
        "heuristic": 45,
        "refined": 5,
        "inferred": 0
      },
      "needsReview": 12,
      "reviewed": 58,
      "averageConfidence": {
        "converted": 0.92,
        "heuristic": 0.76
      }
    },
    "lowConfidence": [
      {
        "target": "src/utils/helpers.ts",
        "annotation": "domain",
        "confidence": 0.45,
        "value": "utility"
      }
    ],
    "lastGeneration": {
      "id": "gen-20251222-001",
      "timestamp": "2025-12-22T10:30:00Z",
      "annotationsGenerated": 25,
      "filesAffected": 8
    }
  }
}
```

### 10.2 Field Definitions

#### `summary`

Aggregate provenance statistics.

| Field | Type | Description |
|-------|------|-------------|
| `total` | integer | Total annotations tracked |
| `bySource` | object | Count by source type (explicit, converted, heuristic, refined, inferred) |
| `needsReview` | integer | Annotations flagged for review |
| `reviewed` | integer | Annotations that have been reviewed |
| `averageConfidence` | object | Average confidence by source type |

#### `lowConfidence`

Array of annotations with confidence below threshold.

| Field | Type | Description |
|-------|------|-------------|
| `target` | string | File path or symbol qualified name |
| `annotation` | string | Annotation type (e.g., summary, domain) |
| `confidence` | number | Confidence score (0.0-1.0) |
| `value` | string | The annotation value |

#### `lastGeneration`

Information about the most recent annotation generation run.

| Field | Type | Description |
|-------|------|-------------|
| `id` | string | Generation batch ID |
| `timestamp` | string | ISO 8601 timestamp |
| `annotationsGenerated` | integer | Number of annotations generated |
| `filesAffected` | integer | Number of files modified |

### 10.3 Use Cases

The provenance index enables:
- Tracking annotation generation history
- Identifying low-confidence annotations for review
- Monitoring review progress across the project
- Auditing annotation origins for quality assurance
- Bulk operations on annotations by source type

---

## 11. Generation

### 11.1 Generation Algorithm

```
FUNCTION generateCache(projectRoot, config):
    cache = initializeCache(projectRoot, config)
    files = discoverFiles(projectRoot, config)

    FOR each file IN files:
        fileEntry = indexFile(file)
        cache.files[file.path] = fileEntry

        symbols = extractSymbols(file)
        FOR each symbol IN symbols:
            cache.symbols[symbol.qualified_name] = symbol
            updateGraph(cache.graph, symbol)

    cache.domains = buildDomainIndex(cache.files)
    cache.constraints = buildConstraintIndex(cache.files, cache.symbols)
    cache.stats = calculateStats(cache)

    RETURN cache
```

### 11.2 Staleness Detection

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

### 11.3 Incremental Updates

For efficiency, generators SHOULD support incremental updates:

1. Compare file modification times with previous cache
2. Only re-index changed files
3. Update affected graph entries
4. Recompute affected indexes

### 11.4 Determinism

Cache generation MUST be deterministic:
- Same input MUST produce identical output
- Object keys MUST be sorted alphabetically
- Arrays SHOULD maintain consistent ordering (alphabetical or by line number)

---

## 12. Validation

### 12.1 Schema Validation

Cache files MUST validate against the JSON Schema:

```bash
ajv validate -s https://acp-spec.org/schemas/v1/cache.schema.json -d .acp.cache.json
```

### 12.2 Integrity Checks

Implementations SHOULD verify:

- [ ] All file paths in `files` exist
- [ ] All symbol files reference existing file entries
- [ ] Graph is consistent (forward/reverse match)
- [ ] Domain files reference existing file entries
- [ ] Constraint files reference existing file entries
- [ ] All qualified names follow format specification

### 12.3 Validation Errors

Common validation errors:

| Error | Cause | Fix |
|-------|-------|-----|
| Invalid JSON | Syntax error in JSON | Check for trailing commas, unclosed brackets |
| Schema violation | Field type mismatch | Check field types against schema |
| Broken reference | Symbol references non-existent file | Rebuild cache |
| Inconsistent graph | Forward/reverse don't match | Rebuild cache |

---

## Appendix A: Complete Example

```json
{
  "version": "1.0.0",
  "generated_at": "2024-12-17T15:30:00Z",
  "git_commit": "abc123def456",
  "project": {
    "name": "auth-service",
    "root": "/home/user/projects/auth-service",
    "description": "Authentication microservice"
  },
  "stats": {
    "files": 3,
    "symbols": 2,
    "lines": 270
  },
  "source_files": {
    "src/auth/session.ts": "2024-12-17T15:29:00Z",
    "src/auth/jwt.ts": "2024-12-17T15:20:00Z"
  },
  "files": {
    "src/auth/session.ts": {
      "path": "src/auth/session.ts",
      "module": "Session Service",
      "summary": "User session management",
      "lines": 245,
      "language": "typescript",
      "domains": ["authentication"],
      "layer": "service",
      "stability": "stable",
      "exports": [
        "src/auth/session.ts:SessionService",
        "src/auth/session.ts:SessionService.validateSession"
      ],
      "imports": ["jsonwebtoken", "src/auth/jwt"]
    }
  },
  "symbols": {
    "src/auth/session.ts:SessionService.validateSession": {
      "name": "validateSession",
      "qualified_name": "src/auth/session.ts:SessionService.validateSession",
      "type": "method",
      "file": "src/auth/session.ts",
      "lines": [45, 89],
      "signature": "(token: string) => Promise<Session | null>",
      "summary": "Validates JWT and returns session",
      "exported": true,
      "async": true,
      "calls": ["src/auth/jwt.ts:verifyToken"],
      "called_by": []
    }
  },
  "graph": {
    "forward": {
      "src/auth/session.ts:SessionService.validateSession": [
        "src/auth/jwt.ts:verifyToken"
      ]
    },
    "reverse": {
      "src/auth/jwt.ts:verifyToken": [
        "src/auth/session.ts:SessionService.validateSession"
      ]
    }
  },
  "domains": {
    "authentication": {
      "name": "authentication",
      "description": "User authentication and session management",
      "files": ["src/auth/session.ts"],
      "symbols": ["src/auth/session.ts:SessionService.validateSession"]
    }
  },
  "constraints": {
    "by_file": {
      "src/auth/session.ts": {
        "lock_level": "restricted",
        "lock_reason": "Security critical",
        "style": "google-typescript"
      }
    },
    "by_lock_level": {
      "restricted": ["src/auth/session.ts"]
    }
  }
}
```

---

## Appendix B: jq Query Examples

```bash
# Get all files
jq '.files | keys' .acp.cache.json

# Get symbol by name
jq '.symbols["src/auth/session.ts:SessionService.validateSession"]' .acp.cache.json

# Get all restricted files
jq '.constraints.by_lock_level.restricted' .acp.cache.json

# Get callers of a function
jq '.graph.reverse["src/auth/session.ts:SessionService.validateSession"]' .acp.cache.json

# Get files in a domain
jq '.domains["authentication"].files' .acp.cache.json

# Check if file can be modified
jq '.constraints.by_file["src/auth/session.ts"].lock_level' .acp.cache.json

# Get all TypeScript files
jq '.files | to_entries | map(select(.value.language == "typescript")) | .[].key' .acp.cache.json

# Count symbols by type
jq '.symbols | to_entries | group_by(.value.type) | map({type: .[0].value.type, count: length})' .acp.cache.json
```

---

## Appendix C: Related Documents

- [Annotation Syntax](annotations.md) - How annotations are written
- [Constraint System](constraints.md) - Constraint definitions
- [Variable System](vars.md) - Variable file format
- [Configuration](config.md) - Configuration options
- [File Discovery](discovery.md) - How cache is built
- [Inheritance & Cascade](inheritance.md) - Constraint inheritance rules

---

*End of Cache File Format Specification*

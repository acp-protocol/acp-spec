# Cache File Format Specification

**ACP Version**: 1.0.0-revised
**Document Version**: 1.0.0
**Last Updated**: 2024-12-17
**Status**: Revised Draft

---

## Table of Contents

1. [Overview](#1-overview)
2. [File Format](#2-file-format)
3. [Root Structure](#3-root-structure)
4. [File Entries](#4-file-entries)
5. [Symbol Entries](#5-symbol-entries)
6. [Call Graph](#6-call-graph)
7. [Domain Index](#7-domain-index)
8. [Constraint Index](#8-constraint-index)
9. [Generation](#9-generation)
10. [Validation](#10-validation)

---

## 1. Overview

### 1.1 Purpose

The cache file (`.acp.cache.json`) is the indexed representation of a codebase. It provides AI systems with structured access to:

- File and module metadata
- Symbol definitions and relationships
- Call graphs and dependencies
- Domain classifications
- Constraint and guardrail information

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
  "generated_at": "2024-12-17T15:30:00Z",
  "git_commit": "abc123def456",
  "project": { },
  "stats": { },
  "source_files": { },
  "files": { },
  "symbols": { },
  "graph": { },
  "domains": { },
  "constraints": { }
}
```

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
| `module` | string | ⚠ SHOULD | null | Human-readable module name (from `@acp:module`) |
| `summary` | string | ⚠ SHOULD | null | Brief file description (from `@acp:summary`) |
| `lines` | integer | ✓ MUST | - | Line count |
| `language` | string | ✓ MUST | - | Programming language |
| `domains` | array[string] | ✗ MAY | [] | Domain classifications (from `@acp:domain`) |
| `layer` | string | ✗ MAY | null | Architectural layer (from `@acp:layer`) |
| `stability` | string | ✗ MAY | null | Stability level: `stable`, `experimental`, `deprecated` |
| `exports` | array[string] | ⚠ SHOULD | [] | Exported symbols (qualified names) |
| `imports` | array[string] | ⚠ SHOULD | [] | Imported modules |

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
| `summary` | string | ⚠ SHOULD | null | Brief description (from `@acp:summary`) |
| `async` | boolean | ✗ MAY | false | Whether async |
| `exported` | boolean | ✓ MUST | - | Whether exported |
| `visibility` | string | ✗ MAY | "public" | `public`, `private`, `protected` |
| `calls` | array[string] | ✗ MAY | [] | Symbols this calls (qualified names) |
| `called_by` | array[string] | ✗ MAY | [] | Symbols calling this (qualified names) |

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

### 8.3 Example Constraint Entry

```json
{
  "by_file": {
    "src/auth/session.ts": {
      "lock_level": "restricted",
      "lock_reason": "Security-critical code",
      "style": "google-typescript",
      "behavior": "conservative",
      "quality": ["security-review", "tests-required"]
    }
  }
}
```

See [Constraint System Specification](constraints.md) for detailed constraint definitions.

---

## 9. Generation

### 9.1 Generation Algorithm

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

### 9.2 Staleness Detection

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

### 9.3 Incremental Updates

For efficiency, generators SHOULD support incremental updates:

1. Compare file modification times with previous cache
2. Only re-index changed files
3. Update affected graph entries
4. Recompute affected indexes

### 9.4 Determinism

Cache generation MUST be deterministic:
- Same input MUST produce identical output
- Object keys MUST be sorted alphabetically
- Arrays SHOULD maintain consistent ordering (alphabetical or by line number)

---

## 10. Validation

### 10.1 Schema Validation

Cache files MUST validate against the JSON Schema:

```bash
ajv validate -s https://acp-spec.org/schemas/v1/cache.schema.json -d .acp.cache.json
```

### 10.2 Integrity Checks

Implementations SHOULD verify:

- [ ] All file paths in `files` exist
- [ ] All symbol files reference existing file entries
- [ ] Graph is consistent (forward/reverse match)
- [ ] Domain files reference existing file entries
- [ ] Constraint files reference existing file entries
- [ ] All qualified names follow format specification

### 10.3 Validation Errors

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

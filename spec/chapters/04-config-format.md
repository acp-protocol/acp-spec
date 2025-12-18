# Config File Specification

**ACP Version**: 1.0.0-revised
**Document Version**: 1.0.0
**Last Updated**: 2024-12-17
**Status**: Revised Draft

---

## Table of Contents

1. [Overview](#1-overview)
2. [File Format](#2-file-format)
3. [Configuration Fields](#3-configuration-fields)
4. [Error Handling Configuration](#4-error-handling-configuration)
5. [Constraint Configuration](#5-constraint-configuration)
6. [Implementation Limits](#6-implementation-limits)
7. [Examples](#7-examples)

---

## 1. Overview

### 1.1 Purpose

The config file (`.acp.config.json`) controls ACP behavior at the project level. It allows developers to:

- Configure file inclusion/exclusion patterns
- Set error handling strictness
- Define default constraints
- Configure domain patterns
- Set implementation limits
- Control call graph generation

### 1.2 File Location

The config file:
- MUST be named `.acp.config.json`
- SHOULD be located in the project root
- MAY be committed to version control
- Is OPTIONAL (ACP works without configuration)

### 1.3 Design Principles

- **Optional**: ACP works with sensible defaults
- **Incremental**: Configure only what you need
- **Explicit**: Clear, documented options
- **Validated**: Configuration errors are detected early

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

### 2.2 Schema

The config file MUST conform to the JSON Schema at:
`https://acp-spec.org/schemas/v1/config.schema.json`

### 2.3 Top-Level Structure

```json
{
  "version": "1.0.0",
  "include": ["src/**/*.ts", "lib/**/*.js"],
  "exclude": ["**/*.test.ts", "node_modules/**"],
  "error_handling": { },
  "constraints": { },
  "domains": { },
  "call_graph": { },
  "limits": { }
}
```

---

## 3. Configuration Fields

### 3.1 Version (required)

ACP specification version this config conforms to.

```json
{
  "version": "1.0.0"
}
```

- Type: `string`
- Format: Semantic version
- MUST match major version of ACP spec

### 3.2 Include Patterns (optional)

Glob patterns to include in indexing.

```json
{
  "include": ["src/**/*.ts", "lib/**/*.js"]
}
```

- Type: `array[string]`
- Default: `["**/*"]` (all files)
- Patterns use glob syntax

**Examples:**
- `src/**/*.ts` - All TypeScript files in src/
- `lib/**/*.{ts,js}` - TypeScript or JavaScript in lib/
- `*.config.js` - Config files in root

### 3.3 Exclude Patterns (optional)

Glob patterns to exclude from indexing.

```json
{
  "exclude": [
    "**/*.test.ts",
    "**/*.spec.ts",
    "node_modules/**",
    "dist/**",
    "build/**",
    ".git/**",
    "coverage/**"
  ]
}
```

- Type: `array[string]`
- Default: Common build/test directories
- Patterns use glob syntax
- Takes precedence over include patterns

**Default exclusions:**
- `node_modules/**`
- `.git/**`
- `dist/**`
- `build/**`
- `coverage/**`
- `**/*.test.*`
- `**/*.spec.*`

---

## 4. Error Handling Configuration

Controls how ACP handles errors during indexing and operations.

**From main specification Section 11 (Lines 1443-1770):**

### 4.1 Structure

```json
{
  "error_handling": {
    "strictness": "permissive",
    "max_errors": 100,
    "auto_correct": false
  }
}
```

### 4.2 Strictness Mode

Controls error handling behavior.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `strictness` | string | `"permissive"` | `"permissive"` or `"strict"` |

**Permissive Mode** (default):
- Emit warnings for recoverable errors
- Continue processing when possible
- Provide partial results when safe
- Collect all errors before failing

**Strict Mode**:
- Fail fast on first error
- Do NOT produce partial output
- Provide detailed error context
- Exit with non-zero status

**Example:**
```json
{
  "error_handling": {
    "strictness": "strict"
  }
}
```

### 4.3 Max Errors

Maximum number of errors before aborting (permissive mode only).

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `max_errors` | integer | 100 | Stop after N errors |

**Example:**
```json
{
  "error_handling": {
    "strictness": "permissive",
    "max_errors": 50
  }
}
```

### 4.4 Auto-correct

Whether to automatically fix common errors.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `auto_correct` | boolean | false | Auto-fix syntax errors |

**Example:**
```json
{
  "error_handling": {
    "auto_correct": true
  }
}
```

---

## 5. Constraint Configuration

Configure default constraints and violation tracking.

**From main specification Section 3.3 (Lines 396-453) and Section 5.6 (Lines 729-759):**

### 5.1 Structure

```json
{
  "constraints": {
    "defaults": {
      "lock": "normal"
    },
    "track_violations": false,
    "audit_file": ".acp.violations.log"
  }
}
```

### 5.2 Default Constraints

Set project-wide default constraint values.

```json
{
  "constraints": {
    "defaults": {
      "lock": "normal",
      "style": "prettier",
      "behavior": "balanced"
    }
  }
}
```

| Field | Type | Description |
|-------|------|-------------|
| `lock` | string | Default lock level |
| `style` | string | Default style guide |
| `behavior` | string | Default AI behavior |

### 5.3 Violation Tracking

Enable tracking of constraint violations.

```json
{
  "constraints": {
    "track_violations": true,
    "audit_file": ".acp.violations.log"
  }
}
```

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `track_violations` | boolean | false | Log violations |
| `audit_file` | string | `".acp.violations.log"` | Log file path |

See [Constraint System Specification](constraints.md) for constraint details.

---

## 6. Domain Configuration

Define domain patterns for automatic classification.

**From main specification Section 8.3.1 (Lines 1090-1113):**

### 6.1 Structure

```json
{
  "domains": {
    "authentication": {
      "patterns": ["src/auth/**", "lib/security/**"]
    },
    "database": {
      "patterns": ["src/db/**", "src/models/**"]
    },
    "api": {
      "patterns": ["src/api/**", "src/routes/**"]
    }
  }
}
```

### 6.2 Domain Entry

Each domain has:

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `patterns` | array[string] | Yes | Glob patterns for this domain |

**Example:**
```json
{
  "domains": {
    "billing": {
      "patterns": [
        "src/billing/**",
        "src/payments/**",
        "src/subscriptions/**"
      ]
    }
  }
}
```

---

## 7. Call Graph Configuration

Configure call graph generation.

**From main specification Section 8.3.3 (Lines 1136-1159):**

### 7.1 Structure

```json
{
  "call_graph": {
    "include_stdlib": false,
    "max_depth": null,
    "exclude_patterns": ["**/test/**"]
  }
}
```

### 7.2 Fields

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `include_stdlib` | boolean | false | Include standard library calls |
| `max_depth` | integer\|null | null | Maximum call depth (null = unlimited) |
| `exclude_patterns` | array[string] | [] | Patterns to exclude from graph |

**Example:**
```json
{
  "call_graph": {
    "include_stdlib": false,
    "max_depth": 5,
    "exclude_patterns": ["**/test/**", "**/mocks/**"]
  }
}
```

---

## 8. Implementation Limits

Configure implementation limits.

**From main specification Section 8.5 (Lines 1196-1232):**

### 8.1 Structure

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

### 8.2 Limit Fields

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `max_file_size_mb` | integer | 10 | Max source file size in MB |
| `max_files` | integer | 100000 | Max files in project |
| `max_annotations_per_file` | integer | 1000 | Max annotations per file |
| `max_cache_size_mb` | integer | 100 | Max cache file size in MB |

**Behavior When Exceeded:**
- **Permissive mode**: Warn, skip offending item, continue
- **Strict mode**: Error, abort

**For large projects:**
- Exclude generated files
- Separate into multiple ACP projects
- Increase limits with caution

---

## 9. Examples

### 9.1 Minimal Configuration

```json
{
  "version": "1.0.0"
}
```

### 9.2 TypeScript Project

```json
{
  "version": "1.0.0",
  "include": ["src/**/*.ts", "src/**/*.tsx"],
  "exclude": [
    "**/*.test.ts",
    "**/*.spec.ts",
    "node_modules/**",
    "dist/**"
  ],
  "error_handling": {
    "strictness": "permissive",
    "max_errors": 100
  },
  "constraints": {
    "defaults": {
      "lock": "normal",
      "style": "google-typescript"
    }
  },
  "domains": {
    "api": {
      "patterns": ["src/api/**", "src/routes/**"]
    },
    "database": {
      "patterns": ["src/db/**", "src/models/**"]
    }
  }
}
```

### 9.3 Strict Mode for CI/CD

```json
{
  "version": "1.0.0",
  "error_handling": {
    "strictness": "strict",
    "auto_correct": false
  },
  "constraints": {
    "track_violations": true,
    "audit_file": ".acp.violations.log"
  }
}
```

### 9.4 Large Monorepo

```json
{
  "version": "1.0.0",
  "include": ["packages/*/src/**/*.ts"],
  "exclude": [
    "**/*.test.ts",
    "node_modules/**",
    "**/dist/**",
    "**/build/**"
  ],
  "limits": {
    "max_files": 500000,
    "max_cache_size_mb": 500
  },
  "call_graph": {
    "max_depth": 3,
    "exclude_patterns": ["**/test/**", "**/__mocks__/**"]
  }
}
```

---

## Appendix A: Complete Example

```json
{
  "version": "1.0.0",
  "include": ["src/**/*.ts", "lib/**/*.ts"],
  "exclude": [
    "**/*.test.ts",
    "**/*.spec.ts",
    "node_modules/**",
    "dist/**",
    "build/**",
    "coverage/**"
  ],
  "error_handling": {
    "strictness": "permissive",
    "max_errors": 100,
    "auto_correct": false
  },
  "constraints": {
    "defaults": {
      "lock": "normal",
      "style": "google-typescript",
      "behavior": "balanced"
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
    },
    "api": {
      "patterns": ["src/api/**", "src/routes/**"]
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

---

## Appendix B: Related Documents

- [Annotation Syntax](annotations.md) - How annotations work
- [Cache Format](cache.md) - Cache file structure
- [Constraint System](constraints.md) - Constraint configuration
- [File Discovery](discovery.md) - How inclusion/exclusion works
- [Inheritance & Cascade](inheritance.md) - Constraint precedence

---

*End of Config File Specification*

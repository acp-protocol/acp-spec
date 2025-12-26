# Config File Specification

**ACP Version**: 1.0.0-revised
**Document Version**: 1.1.0
**Last Updated**: 2025-12-22
**Status**: RFC-0002, RFC-0003 Compliant

---

## Table of Contents

1. [Overview](#1-overview)
2. [File Format](#2-file-format)
3. [Configuration Fields](#3-configuration-fields)
4. [Error Handling Configuration](#4-error-handling-configuration)
5. [Constraint Configuration](#5-constraint-configuration)
6. [Domain Configuration](#6-domain-configuration)
7. [Call Graph Configuration](#7-call-graph-configuration)
8. [Implementation Limits](#8-implementation-limits)
9. [Documentation Configuration (RFC-0002)](#9-documentation-configuration-rfc-0002)
10. [Annotate Configuration (RFC-0003)](#10-annotate-configuration-rfc-0003)
11. [Examples](#11-examples)

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

## 9. Documentation Configuration (RFC-0002)

Configure documentation sources and style guides for `@acp:ref` and `@acp:style` annotations.

### 9.1 Structure

```json
{
  "documentation": {
    "approvedSources": [],
    "styleGuides": {},
    "defaults": {
      "fetchRefs": false,
      "style": null
    },
    "validation": {
      "requireApprovedSources": false,
      "warnUnknownStyle": true
    }
  }
}
```

### 9.2 Approved Sources

Define trusted documentation sources that can be referenced with source IDs in `@acp:ref` annotations.

```json
{
  "documentation": {
    "approvedSources": [
      {
        "id": "react",
        "url": "https://react.dev/reference",
        "version": "18.2",
        "description": "React documentation",
        "sections": {
          "hooks": "react/hooks",
          "components": "react/components"
        },
        "fetchable": true,
        "lastVerified": "2024-12-15T00:00:00Z"
      },
      {
        "id": "internal-api",
        "url": "https://docs.internal.company.com/api",
        "description": "Internal API documentation",
        "fetchable": false
      }
    ]
  }
}
```

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `id` | string | Yes | Unique identifier (lowercase, alphanumeric with hyphens) |
| `url` | string | Yes | Base URL for documentation |
| `version` | string | No | Documentation version |
| `description` | string | No | Human-readable description |
| `sections` | object | No | Named section shortcuts (paths relative to URL) |
| `fetchable` | boolean | No | Whether AI should attempt to fetch (default: true) |
| `lastVerified` | string | No | ISO 8601 timestamp of last verification |

**Usage in annotations:**
```typescript
// @acp:ref react:hooks - Follow React hooks patterns
// @acp:ref-section hooks/rules-of-hooks - Specifically this section
```

### 9.3 Custom Style Guides

Define custom style guides that extend or complement built-in guides.

```json
{
  "documentation": {
    "styleGuides": {
      "company-react": {
        "extends": "airbnb-react",
        "source": "internal-api",
        "description": "Company React conventions",
        "languages": ["typescript", "javascript"],
        "rules": [
          "prefer-function-components",
          "use-custom-hooks",
          "max-component-lines=200"
        ],
        "filePatterns": ["src/components/**/*.tsx"]
      },
      "api-style": {
        "extends": "google-typescript",
        "description": "API layer coding style",
        "rules": [
          "async-required",
          "error-handling-required"
        ],
        "filePatterns": ["src/api/**/*.ts"]
      }
    }
  }
}
```

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `extends` | string | No | Base style guide to extend |
| `source` | string | No | Approved source ID for documentation |
| `url` | string | No | Direct URL to style guide documentation |
| `description` | string | No | Human-readable description |
| `languages` | array[string] | No | Languages this guide applies to |
| `rules` | array[string] | No | Style rules (key or key=value format) |
| `filePatterns` | array[string] | No | Glob patterns for auto-applying this guide |

**Usage in annotations:**
```typescript
// @acp:style company-react - Follow company React conventions
// @acp:style-extends airbnb-react - Explicitly extend Airbnb
```

### 9.4 Default Settings

Configure project-wide defaults for documentation handling.

```json
{
  "documentation": {
    "defaults": {
      "fetchRefs": false,
      "style": "google-typescript"
    }
  }
}
```

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `fetchRefs` | boolean | false | Default value for `@acp:ref-fetch` |
| `style` | string | null | Default style guide for all files |

### 9.5 Validation Settings

Configure how references and styles are validated.

```json
{
  "documentation": {
    "validation": {
      "requireApprovedSources": false,
      "warnUnknownStyle": true
    }
  }
}
```

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `requireApprovedSources` | boolean | false | Only allow refs from approvedSources list |
| `warnUnknownStyle` | boolean | true | Warn when unknown style guide is referenced |

When `requireApprovedSources` is `true`:
- Direct URLs in `@acp:ref` are rejected
- Only source IDs from `approvedSources` are accepted
- Useful for restricting documentation to vetted sources

---

## 10. Annotate Configuration (RFC-0003)

Configure annotation generation and provenance tracking for `acp annotate` command.

### 10.1 Structure

```json
{
  "annotate": {
    "provenance": {
      "enabled": true,
      "includeConfidence": true,
      "reviewThreshold": 0.8,
      "minConfidence": 0.5
    },
    "defaults": {
      "markNeedsReview": false,
      "overwriteExisting": false
    }
  }
}
```

### 10.2 Provenance Settings

Configure how provenance information is tracked for auto-generated annotations.

```json
{
  "annotate": {
    "provenance": {
      "enabled": true,
      "includeConfidence": true,
      "reviewThreshold": 0.8,
      "minConfidence": 0.5
    }
  }
}
```

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `enabled` | boolean | true | Enable provenance tracking for generated annotations |
| `includeConfidence` | boolean | true | Include confidence scores in generated annotations |
| `reviewThreshold` | number | 0.8 | Confidence threshold below which annotations are flagged for review |
| `minConfidence` | number | 0.5 | Minimum confidence required to emit an annotation |

**Threshold Behavior:**
- Annotations with confidence ≥ `reviewThreshold` are not flagged for review
- Annotations with confidence < `reviewThreshold` are flagged with `@acp:source-reviewed false`
- Annotations with confidence < `minConfidence` are not emitted at all

**Example thresholds:**
```json
{
  "annotate": {
    "provenance": {
      "reviewThreshold": 0.9,
      "minConfidence": 0.6
    }
  }
}
```

### 10.3 Default Settings

Configure default behavior for annotation generation.

```json
{
  "annotate": {
    "defaults": {
      "markNeedsReview": false,
      "overwriteExisting": false
    }
  }
}
```

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `markNeedsReview` | boolean | false | Mark all generated annotations as needing review |
| `overwriteExisting` | boolean | false | Overwrite existing annotations when generating |

**markNeedsReview:**
- When `true`, all generated annotations include `@acp:source-reviewed false`
- Useful for projects that require human verification of all auto-generated annotations
- Overrides confidence-based review flagging

**overwriteExisting:**
- When `false` (default), existing annotations are preserved
- When `true`, existing annotations are replaced with newly generated ones
- Use with caution to avoid losing manual annotations

### 10.4 Complete Example

```json
{
  "annotate": {
    "provenance": {
      "enabled": true,
      "includeConfidence": true,
      "reviewThreshold": 0.85,
      "minConfidence": 0.5
    },
    "defaults": {
      "markNeedsReview": false,
      "overwriteExisting": false
    }
  }
}
```

### 10.5 Disabling Provenance

To generate annotations without provenance tracking:

```json
{
  "annotate": {
    "provenance": {
      "enabled": false
    }
  }
}
```

Or use the `--no-provenance` CLI flag:

```bash
acp annotate --no-provenance
```

---

## 11. Examples

### 11.1 Minimal Configuration

```json
{
  "version": "1.0.0"
}
```

### 11.2 TypeScript Project

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

### 11.3 Strict Mode for CI/CD

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

### 11.4 Large Monorepo

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

### 11.5 With Documentation Configuration (RFC-0002)

```json
{
  "version": "1.0.0",
  "include": ["src/**/*.ts"],
  "exclude": ["**/*.test.ts", "node_modules/**"],
  "documentation": {
    "approvedSources": [
      {
        "id": "react",
        "url": "https://react.dev/reference",
        "version": "18.2",
        "sections": {
          "hooks": "react/hooks"
        }
      },
      {
        "id": "company-api",
        "url": "https://docs.company.com/api",
        "fetchable": false
      }
    ],
    "styleGuides": {
      "company-react": {
        "extends": "airbnb-react",
        "rules": ["prefer-function-components", "max-component-lines=200"],
        "filePatterns": ["src/components/**/*.tsx"]
      }
    },
    "defaults": {
      "fetchRefs": false,
      "style": "google-typescript"
    },
    "validation": {
      "requireApprovedSources": false,
      "warnUnknownStyle": true
    }
  }
}
```

### 11.6 With Annotate Configuration (RFC-0003)

```json
{
  "version": "1.0.0",
  "include": ["src/**/*.ts"],
  "exclude": ["**/*.test.ts", "node_modules/**"],
  "annotate": {
    "provenance": {
      "enabled": true,
      "includeConfidence": true,
      "reviewThreshold": 0.85,
      "minConfidence": 0.5
    },
    "defaults": {
      "markNeedsReview": false,
      "overwriteExisting": false
    }
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
  },
  "annotate": {
    "provenance": {
      "enabled": true,
      "includeConfidence": true,
      "reviewThreshold": 0.8,
      "minConfidence": 0.5
    },
    "defaults": {
      "markNeedsReview": false,
      "overwriteExisting": false
    }
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

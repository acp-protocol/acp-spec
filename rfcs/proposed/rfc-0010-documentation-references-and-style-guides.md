# RFC-0010: Documentation References and Style Guides

- **RFC ID**: 0010
- **Title**: Documentation References and Style Guides
- **Author**: David (ACP Protocol)
- **Status**: Draft
- **Created**: 2025-12-20
- **Updated**: 2025-12-20
- **Discussion**: [Pending GitHub Discussion]

---

## Summary

This RFC formalizes and extends the `@acp:ref` and `@acp:style` annotation system to provide a comprehensive mechanism for linking code to authoritative external documentation sources and enforcing style conventions. It introduces project-level configuration for approved documentation sources, schema support for storing references in the cache, and clear AI behavioral guidelines for consulting and applying documentation.

## Motivation

### Problem Statement

AI coding assistants frequently need guidance on which documentation to follow, especially for rapidly evolving frameworks and libraries. Currently:

1. **No formal `@acp:ref` specification** — The annotation exists in code but is not documented in the main specification (ACP-1.0.md Appendix A)
2. **No project-level approved sources** — Cannot define trusted documentation URLs at the project level in `.acp.config.json`
3. **No schema validation** — Cache schema doesn't store refs, config schema doesn't validate source URLs
4. **No enforcement/linking** — No formal connection between `@acp:ref` (documentation source) and `@acp:style` (style guide name)
5. **Version drift** — No mechanism to pin documentation to specific versions (e.g., Tailwind v4 vs v3)

### Real-World Use Case

A developer using Tailwind CSS v4 wants to ensure AI assistants:

1. Know to follow Tailwind v4 conventions (not v3)
2. Can access the official v4 documentation when uncertain
3. Apply consistent formatting across all files using Tailwind

**Current workaround** (unreliable):
```javascript
/**
 * Must follow Tailwind v4 format
 * Docs: https://tailwindcss.com/docs/v4
 * @acp:style tailwindcss-v4
 */
```

**Desired solution** (this RFC):
```javascript
/**
 * @acp:ref "https://tailwindcss.com/docs/v4"
 * @acp:ref-version "4.0"
 * @acp:style tailwindcss-v4
 */
```

With project-level configuration:
```json
{
  "documentation": {
    "approvedSources": [
      {
        "id": "tailwindcss-v4",
        "url": "https://tailwindcss.com/docs/v4",
        "version": "4.0",
        "description": "Tailwind CSS v4 official documentation"
      }
    ],
    "styleGuides": {
      "tailwindcss-v4": {
        "source": "tailwindcss-v4",
        "rules": ["utility-first", "no-custom-css-when-utility-exists"]
      }
    }
  }
}
```

### Goals

1. **Formalize `@acp:ref`** — Add to reserved namespaces with complete specification
2. **Enhance `@acp:style`** — Link to documentation sources and support custom guides
3. **Project-level configuration** — Define approved sources and custom style guides in config
4. **Schema support** — Store refs and style info in cache for querying
5. **AI behavior specification** — Clear guidelines for when/how AI should use references
6. **Version pinning** — Support version-specific documentation references

### Non-Goals

1. **Automatic documentation fetching** — Implementation-specific (deferred to tools)
2. **Documentation caching/mirroring** — Out of scope
3. **Style linting/enforcement** — ACP provides advisory constraints, not enforcement
4. **Documentation format parsing** — Refs are URLs; content parsing is tool-specific

---

## Detailed Design

### Overview

This RFC introduces a three-layer documentation reference system:

```
┌─────────────────────────────────────────────────────────────┐
│                    .acp.config.json                         │
│  ┌───────────────────────────────────────────────────────┐  │
│  │ documentation.approvedSources[]                       │  │
│  │   - Project-wide trusted documentation URLs           │  │
│  │   - Version pinning                                   │  │
│  │   - Named identifiers for reuse                       │  │
│  └───────────────────────────────────────────────────────┘  │
│  ┌───────────────────────────────────────────────────────┐  │
│  │ documentation.styleGuides{}                           │  │
│  │   - Custom style guide definitions                    │  │
│  │   - Links to approved sources                         │  │
│  │   - Custom rules per guide                            │  │
│  └───────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────┐
│                   Source Code Annotations                   │
│  ┌───────────────────────────────────────────────────────┐  │
│  │ @acp:ref <url|id>                                     │  │
│  │   - File or symbol-level documentation reference      │  │
│  │   - Can use approved source ID or direct URL          │  │
│  └───────────────────────────────────────────────────────┘  │
│  ┌───────────────────────────────────────────────────────┐  │
│  │ @acp:style <guide-name>                               │  │
│  │   - Style guide to follow                             │  │
│  │   - Built-in or custom (from config)                  │  │
│  └───────────────────────────────────────────────────────┘  │
│  ┌───────────────────────────────────────────────────────┐  │
│  │ @acp:style-rules <rule1>, <rule2>                     │  │
│  │   - Additional or override rules                      │  │
│  └───────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────┐
│                     .acp.cache.json                         │
│  ┌───────────────────────────────────────────────────────┐  │
│  │ files[path].refs[]                                    │  │
│  │   - Resolved documentation references per file        │  │
│  └───────────────────────────────────────────────────────┘  │
│  ┌───────────────────────────────────────────────────────┐  │
│  │ files[path].style                                     │  │
│  │   - Resolved style guide with rules                   │  │
│  └───────────────────────────────────────────────────────┘  │
│  ┌───────────────────────────────────────────────────────┐  │
│  │ documentation{}                                       │  │
│  │   - Index of all refs and styles across codebase      │  │
│  └───────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────┘
```

---

### 1. Annotation Specifications

#### 1.1 `@acp:ref` — Documentation Reference

**Scope:** File or symbol level  
**Purpose:** Link code to authoritative documentation

| Annotation         | Parameters   | Example                                      | Description                             |
|--------------------|--------------|----------------------------------------------|-----------------------------------------|
| `@acp:ref`         | `<url\|id>`  | `@acp:ref "https://tailwindcss.com/docs/v4"` | Documentation URL or approved source ID |
| `@acp:ref`         | `<id>`       | `@acp:ref tailwindcss-v4`                    | Reference by approved source ID         |
| `@acp:ref-version` | `<version>`  | `@acp:ref-version "4.0"`                     | Pin to specific version                 |
| `@acp:ref-section` | `<path>`     | `@acp:ref-section "utility-classes/spacing"` | Reference specific section              |
| `@acp:ref-fetch`   | `<boolean>`  | `@acp:ref-fetch true`                        | Hint that AI should fetch content       |

**Multiple refs allowed:**
```javascript
/**
 * @acp:ref "https://react.dev/reference/react"
 * @acp:ref tailwindcss-v4
 * @acp:ref-fetch true
 */
```

**Grammar (EBNF):**
```ebnf
ref_annotation     = "@acp:ref" , whitespace , ref_value ;
ref_value          = quoted_url | source_id ;
quoted_url         = '"' , url , '"' ;
source_id          = identifier ;
url                = scheme , "://" , host , [ path ] ;
scheme             = "http" | "https" ;

ref_version        = "@acp:ref-version" , whitespace , quoted_string ;
ref_section        = "@acp:ref-section" , whitespace , quoted_string ;
ref_fetch          = "@acp:ref-fetch" , whitespace , boolean_value ;
boolean_value      = "true" | "false" ;
```

#### 1.2 `@acp:style` — Style Guide Reference

**Scope:** File or symbol level  
**Purpose:** Specify style conventions to follow

| Annotation           | Parameters     | Example                                        | Description                        |
|----------------------|----------------|------------------------------------------------|------------------------------------|
| `@acp:style`         | `<guide-name>` | `@acp:style google-typescript`                 | Built-in or custom style guide     |
| `@acp:style-rules`   | `<rules>`      | `@acp:style-rules no-any, max-line-length=100` | Additional rules (comma-separated) |
| `@acp:style-extends` | `<guide-name>` | `@acp:style-extends prettier`                  | Extend another style guide         |

**Grammar (EBNF):**
```ebnf
style_annotation   = "@acp:style" , whitespace , style_name ;
style_name         = identifier | quoted_string ;

style_rules        = "@acp:style-rules" , whitespace , rule_list ;
rule_list          = rule , { "," , rule } ;
rule               = rule_name , [ "=" , rule_value ] ;
rule_name          = identifier ;
rule_value         = identifier | number | quoted_string ;

style_extends      = "@acp:style-extends" , whitespace , style_name ;
```

#### 1.3 Built-in Style Guides

The following style guide names are reserved and recognized by default:

| Guide Name          | Language   | Source                        |
|---------------------|------------|-------------------------------|
| `google-typescript` | TypeScript | Google TypeScript Style Guide |
| `google-javascript` | JavaScript | Google JavaScript Style Guide |
| `google-python`     | Python     | Google Python Style Guide     |
| `google-java`       | Java       | Google Java Style Guide       |
| `google-cpp`        | C++        | Google C++ Style Guide        |
| `google-go`         | Go         | Effective Go                  |
| `airbnb-javascript` | JavaScript | Airbnb JavaScript Style Guide |
| `airbnb-react`      | React      | Airbnb React/JSX Style Guide  |
| `pep8`              | Python     | PEP 8                         |
| `black`             | Python     | Black code formatter defaults |
| `prettier`          | Multi      | Prettier defaults             |
| `rustfmt`           | Rust       | rustfmt defaults              |
| `standardjs`        | JavaScript | JavaScript Standard Style     |
| `tailwindcss-v3`    | CSS        | Tailwind CSS v3 conventions   |
| `tailwindcss-v4`    | CSS        | Tailwind CSS v4 conventions   |

Custom guides can extend or override these via config.

---

### 2. Configuration Schema

#### 2.1 Config Schema Additions (`config.schema.json`)

```json
{
  "documentation": {
    "type": "object",
    "description": "Documentation and style guide configuration",
    "properties": {
      "approvedSources": {
        "type": "array",
        "description": "Trusted documentation sources for this project",
        "items": {
          "$ref": "#/$defs/approved_source"
        }
      },
      "styleGuides": {
        "type": "object",
        "description": "Custom style guide definitions",
        "additionalProperties": {
          "$ref": "#/$defs/style_guide_definition"
        }
      },
      "defaults": {
        "type": "object",
        "description": "Default documentation settings",
        "properties": {
          "fetchRefs": {
            "type": "boolean",
            "default": false,
            "description": "Default value for @acp:ref-fetch"
          },
          "style": {
            "type": "string",
            "description": "Default style guide for all files"
          }
        }
      },
      "validation": {
        "type": "object",
        "description": "Reference validation settings",
        "properties": {
          "requireApprovedSources": {
            "type": "boolean",
            "default": false,
            "description": "Only allow refs from approvedSources list"
          },
          "warnUnknownStyle": {
            "type": "boolean",
            "default": true,
            "description": "Warn when unknown style guide is referenced"
          }
        }
      }
    }
  }
}
```

#### 2.2 Approved Source Definition

```json
{
  "$defs": {
    "approved_source": {
      "type": "object",
      "required": ["id", "url"],
      "properties": {
        "id": {
          "type": "string",
          "pattern": "^[a-z][a-z0-9-]*$",
          "description": "Unique identifier for this source (used in @acp:ref)"
        },
        "url": {
          "type": "string",
          "format": "uri",
          "description": "Base URL for documentation"
        },
        "version": {
          "type": "string",
          "description": "Version of documentation (semver or custom)"
        },
        "description": {
          "type": "string",
          "description": "Human-readable description"
        },
        "sections": {
          "type": "object",
          "description": "Named section shortcuts",
          "additionalProperties": {
            "type": "string",
            "description": "Path relative to base URL"
          }
        },
        "fetchable": {
          "type": "boolean",
          "default": true,
          "description": "Whether AI tools should attempt to fetch this source"
        },
        "lastVerified": {
          "type": "string",
          "format": "date-time",
          "description": "When this source was last verified accessible"
        }
      }
    }
  }
}
```

#### 2.3 Style Guide Definition

```json
{
  "$defs": {
    "style_guide_definition": {
      "type": "object",
      "properties": {
        "extends": {
          "type": "string",
          "description": "Base style guide to extend"
        },
        "source": {
          "type": "string",
          "description": "Approved source ID for documentation"
        },
        "url": {
          "type": "string",
          "format": "uri",
          "description": "Direct URL to style guide documentation"
        },
        "description": {
          "type": "string",
          "description": "Human-readable description"
        },
        "languages": {
          "type": "array",
          "items": { "type": "string" },
          "description": "Languages this guide applies to"
        },
        "rules": {
          "type": "array",
          "items": { "type": "string" },
          "description": "Style rules (key or key=value format)"
        },
        "filePatterns": {
          "type": "array",
          "items": { "type": "string" },
          "description": "Glob patterns for auto-applying this guide"
        }
      }
    }
  }
}
```

---

### 3. Cache Schema Additions

#### 3.1 File Entry Additions (`cache.schema.json`)

```json
{
  "$defs": {
    "file_entry": {
      "properties": {
        "refs": {
          "type": "array",
          "items": {
            "$ref": "#/$defs/ref_entry"
          },
          "default": [],
          "description": "Documentation references for this file"
        },
        "style": {
          "$ref": "#/$defs/style_entry",
          "description": "Style guide configuration for this file"
        }
      }
    },
    "ref_entry": {
      "type": "object",
      "required": ["url"],
      "properties": {
        "url": {
          "type": "string",
          "format": "uri",
          "description": "Resolved documentation URL"
        },
        "sourceId": {
          "type": ["string", "null"],
          "description": "Approved source ID if applicable"
        },
        "version": {
          "type": ["string", "null"],
          "description": "Documentation version"
        },
        "section": {
          "type": ["string", "null"],
          "description": "Specific section path"
        },
        "fetch": {
          "type": "boolean",
          "default": false,
          "description": "Whether AI should fetch this reference"
        },
        "scope": {
          "type": "string",
          "enum": ["file", "symbol"],
          "default": "file",
          "description": "Scope of this reference"
        },
        "symbolName": {
          "type": ["string", "null"],
          "description": "Symbol name if scope is 'symbol'"
        }
      }
    },
    "style_entry": {
      "type": "object",
      "properties": {
        "guide": {
          "type": "string",
          "description": "Style guide name"
        },
        "extends": {
          "type": ["string", "null"],
          "description": "Extended guide (if any)"
        },
        "rules": {
          "type": "array",
          "items": { "type": "string" },
          "default": [],
          "description": "Applied style rules"
        },
        "source": {
          "type": ["string", "null"],
          "description": "Documentation source for this style"
        },
        "sourceUrl": {
          "type": ["string", "null"],
          "format": "uri",
          "description": "Resolved URL to style documentation"
        }
      }
    }
  }
}
```

#### 3.2 Top-Level Documentation Index

```json
{
  "documentation": {
    "type": "object",
    "description": "Project-wide documentation index",
    "properties": {
      "sources": {
        "type": "object",
        "description": "Map of source ID to usage info",
        "additionalProperties": {
          "type": "object",
          "properties": {
            "url": { "type": "string" },
            "version": { "type": ["string", "null"] },
            "fileCount": { "type": "integer" },
            "files": {
              "type": "array",
              "items": { "type": "string" },
              "description": "Files referencing this source"
            }
          }
        }
      },
      "styles": {
        "type": "object",
        "description": "Map of style guide to usage info",
        "additionalProperties": {
          "type": "object",
          "properties": {
            "fileCount": { "type": "integer" },
            "files": {
              "type": "array",
              "items": { "type": "string" }
            },
            "source": { "type": ["string", "null"] }
          }
        }
      },
      "unresolvedRefs": {
        "type": "array",
        "items": {
          "type": "object",
          "properties": {
            "file": { "type": "string" },
            "ref": { "type": "string" },
            "reason": { "type": "string" }
          }
        },
        "description": "References that could not be resolved"
      }
    }
  }
}
```

---

### 4. AI Behavior Specification

#### 4.1 When to Consult References

AI tools SHOULD consult documentation references when:

| Scenario                               | Action                               |
|----------------------------------------|--------------------------------------|
| `@acp:ref-fetch true` is set           | SHOULD fetch and read documentation  |
| Making changes to file with `@acp:ref` | MAY fetch for context                |
| User asks about conventions            | SHOULD cite referenced documentation |
| Uncertainty about framework usage      | SHOULD check referenced docs         |
| `@acp:style` with linked source        | SHOULD follow style guide            |

AI tools SHOULD NOT:

- Fetch documentation on every request (performance)
- Ignore `@acp:ref-fetch false` directives
- Assume documentation content without fetching
- Override style rules without user request

#### 4.2 Style Application Behavior

| Style Setting                                           | AI Behavior                               |
|---------------------------------------------------------|-------------------------------------------|
| `@acp:style google-typescript`                          | Follow Google TS conventions for new code |
| `@acp:style-rules max-line-length=100`                  | Apply specific rule                       |
| `@acp:style tailwindcss-v4` + `@acp:ref tailwindcss-v4` | Follow style AND consult docs             |
| No style specified                                      | Follow surrounding code patterns          |
| Conflicting styles (file vs symbol)                     | Symbol-level takes precedence             |

#### 4.3 Reference Resolution Order

When resolving `@acp:ref <id>`:

1. Check `documentation.approvedSources` for matching ID
2. If found, use source URL + version
3. If `@acp:ref-section` specified, append section path
4. If not found and `validation.requireApprovedSources: true`, error
5. If not found and validation disabled, treat as literal URL

#### 4.4 Example AI Prompt Integration

```markdown
## File Context

**File:** `src/components/Button.tsx`
**Style:** `tailwindcss-v4` (from @acp:style)
**References:**
- https://tailwindcss.com/docs/v4 (fetch: true)
- https://react.dev/reference/react (fetch: false)

**Style Rules:**
- utility-first
- no-custom-css-when-utility-exists
- prefer-component-classes

**AI Instructions:**
When modifying this file, follow Tailwind CSS v4 conventions. The official
documentation is available at https://tailwindcss.com/docs/v4 and should be
consulted when uncertain about utility class usage.
```

---

### 5. Complete Examples

#### 5.1 Config Example

```json
{
  "version": "1.0.0",
  "documentation": {
    "approvedSources": [
      {
        "id": "tailwindcss-v4",
        "url": "https://tailwindcss.com/docs/v4",
        "version": "4.0",
        "description": "Tailwind CSS v4 official documentation",
        "sections": {
          "utilities": "utility-classes",
          "components": "component-patterns",
          "config": "configuration"
        },
        "fetchable": true
      },
      {
        "id": "react-docs",
        "url": "https://react.dev/reference/react",
        "version": "18.x",
        "description": "React official documentation",
        "fetchable": true
      },
      {
        "id": "internal-api",
        "url": "https://wiki.internal.company.com/api",
        "description": "Internal API documentation",
        "fetchable": false
      }
    ],
    "styleGuides": {
      "company-react": {
        "extends": "airbnb-react",
        "source": "internal-api",
        "description": "Company React conventions",
        "languages": ["typescript", "javascript"],
        "rules": [
          "prefer-function-components",
          "use-strict-types",
          "max-component-lines=200"
        ],
        "filePatterns": ["src/components/**/*.tsx"]
      },
      "tailwindcss-v4": {
        "extends": "prettier",
        "source": "tailwindcss-v4",
        "description": "Tailwind CSS v4 with Prettier",
        "languages": ["css", "typescript", "javascript"],
        "rules": [
          "utility-first",
          "no-custom-css-when-utility-exists",
          "prefer-component-classes"
        ]
      }
    },
    "defaults": {
      "fetchRefs": false,
      "style": "prettier"
    },
    "validation": {
      "requireApprovedSources": false,
      "warnUnknownStyle": true
    }
  }
}
```

#### 5.2 Source Code Example

```typescript
/**
 * @acp:module "UI Components"
 * @acp:summary "Shared UI component library"
 * @acp:domain ui
 * 
 * @acp:ref tailwindcss-v4
 * @acp:ref react-docs
 * @acp:ref-fetch true
 * 
 * @acp:style tailwindcss-v4
 * @acp:style-rules prefer-composition
 */

import React from 'react';

/**
 * @acp:summary "Primary action button component"
 * @acp:ref-section "components/button"
 * @acp:style-rules no-inline-styles
 */
export function Button({ children, variant = 'primary' }: ButtonProps) {
  // AI knows to use Tailwind v4 utility classes here
  const baseClasses = 'px-4 py-2 rounded-lg font-medium transition-colors';
  
  const variantClasses = {
    primary: 'bg-blue-600 text-white hover:bg-blue-700',
    secondary: 'bg-gray-200 text-gray-900 hover:bg-gray-300',
  };

  return (
    <button className={`${baseClasses} ${variantClasses[variant]}`}>
      {children}
    </button>
  );
}
```

#### 5.3 Cache Output Example

```json
{
  "files": {
    "src/components/Button.tsx": {
      "path": "src/components/Button.tsx",
      "module": "UI Components",
      "summary": "Shared UI component library",
      "refs": [
        {
          "url": "https://tailwindcss.com/docs/v4",
          "sourceId": "tailwindcss-v4",
          "version": "4.0",
          "fetch": true,
          "scope": "file"
        },
        {
          "url": "https://react.dev/reference/react",
          "sourceId": "react-docs",
          "version": "18.x",
          "fetch": true,
          "scope": "file"
        },
        {
          "url": "https://tailwindcss.com/docs/v4/components/button",
          "sourceId": "tailwindcss-v4",
          "section": "components/button",
          "fetch": true,
          "scope": "symbol",
          "symbolName": "Button"
        }
      ],
      "style": {
        "guide": "tailwindcss-v4",
        "extends": "prettier",
        "rules": [
          "utility-first",
          "no-custom-css-when-utility-exists",
          "prefer-component-classes",
          "prefer-composition",
          "no-inline-styles"
        ],
        "source": "tailwindcss-v4",
        "sourceUrl": "https://tailwindcss.com/docs/v4"
      }
    }
  },
  "documentation": {
    "sources": {
      "tailwindcss-v4": {
        "url": "https://tailwindcss.com/docs/v4",
        "version": "4.0",
        "fileCount": 47,
        "files": ["src/components/Button.tsx", "..."]
      },
      "react-docs": {
        "url": "https://react.dev/reference/react",
        "version": "18.x",
        "fileCount": 23,
        "files": ["src/components/Button.tsx", "..."]
      }
    },
    "styles": {
      "tailwindcss-v4": {
        "fileCount": 47,
        "files": ["src/components/Button.tsx", "..."],
        "source": "tailwindcss-v4"
      },
      "company-react": {
        "fileCount": 12,
        "files": ["..."],
        "source": "internal-api"
      }
    },
    "unresolvedRefs": []
  }
}
```

#### 5.4 CLI Query Examples

```bash
# List all documentation sources used in project
acp query --documentation sources

# Find files using a specific style guide
acp query --style tailwindcss-v4

# Get documentation refs for a specific file
acp query --file src/components/Button.tsx --refs

# Check for unresolved references
acp query --documentation unresolved

# List all style guides and their usage
acp query --documentation styles
```

---

### 6. Error Handling

| Error Condition                        | Permissive Mode            | Strict Mode   |
|----------------------------------------|----------------------------|---------------|
| Unknown source ID in `@acp:ref`        | Warn, treat as literal URL | Error, abort  |
| Unknown style guide                    | Warn, ignore               | Error, abort  |
| Invalid URL format                     | Warn, skip ref             | Error, abort  |
| Circular style extends                 | Warn, use base only        | Error, abort  |
| `requireApprovedSources` + unknown ref | Warn, skip                 | Error, abort  |
| Conflicting style rules                | Warn, last wins            | Error, abort  |

**Error Format:**
```json
{
  "category": "semantic",
  "severity": "warning",
  "code": "W110",
  "message": "Unknown documentation source ID",
  "location": {
    "file": "src/utils/helper.ts",
    "line": 5
  },
  "snippet": "@acp:ref unknown-source",
  "suggestion": "Add 'unknown-source' to documentation.approvedSources in config, or use a full URL"
}
```

---

## Drawbacks

1. **Complexity increase** — Adds another configuration layer that users must understand
2. **Maintenance burden** — Approved sources and style guides need to be kept current
3. **URL validity** — External documentation URLs can change or become unavailable
4. **Fetch overhead** — AI tools fetching documentation adds latency
5. **Version sync** — Documentation versions may drift from actual library versions used

## Alternatives

### Alternative A: Inline-Only References

Keep `@acp:ref` as simple URLs without config-level management.

**Rejected because:**
- No version pinning
- No project-wide consistency
- Cannot define approved sources
- Cannot link refs to style guides

### Alternative B: External Style Config Files

Use separate `.acp.styles.json` file for style definitions.

**Rejected because:**
- Additional file to manage
- Fragmented configuration
- Already have config.json for project settings

### Alternative C: Rely on Tool-Specific Configs

Let each AI tool (Cursor, Copilot, etc.) manage their own style configs.

**Rejected because:**
- No portability between tools
- Inconsistent behavior across team
- Violates ACP's tool-agnostic principle

### Alternative D: Do Nothing

Leave `@acp:ref` and `@acp:style` partially specified.

**Impact:**
- Users cannot reliably use documentation references
- No schema validation for refs
- AI behavior remains undefined
- Key design requirement unmet

---

## Compatibility

### Backward Compatibility

- **Existing `@acp:style` annotations:** Fully compatible, continue working
- **Existing `@acp:ref` in code:** Will be formalized, no breaking changes
- **Existing configs:** No breaking changes, new `documentation` section is optional

### Forward Compatibility

- **Future style guides:** Can be added to built-in list in minor versions
- **Future ref options:** Can extend annotation syntax in minor versions
- **Schema versioning:** New fields have defaults, old schemas remain valid

### Migration Path

**For existing projects:**

1. No immediate action required
2. Optionally add `documentation.approvedSources` for used docs
3. Optionally add `documentation.styleGuides` for custom styles
4. Existing `@acp:style` and `@acp:ref` annotations continue working

---

## Implementation

### Specification Changes

| Document                          | Changes                                                                                         |
|-----------------------------------|-------------------------------------------------------------------------------------------------|
| `ACP-1.0.md` Appendix A           | Add `@acp:ref`, `@acp:ref-version`, `@acp:ref-section`, `@acp:ref-fetch` to reserved namespaces |
| `ACP-1.0.md` Appendix A           | Update `@acp:style` with source linkage                                                         |
| `spec/chapters/05-annotations.md` | Add Section 5.x for documentation references                                                    |
| `spec/chapters/06-constraints.md` | Update style constraints section                                                                |

### Schema Changes

| Schema               | Changes                                                                                    |
|----------------------|--------------------------------------------------------------------------------------------|
| `config.schema.json` | Add `documentation` object with `approvedSources`, `styleGuides`, `defaults`, `validation` |
| `cache.schema.json`  | Add `refs` and `style` to `file_entry`, add top-level `documentation`                      |
| `vars.schema.json`   | No changes                                                                                 |

### CLI Changes

| Command        | Changes                                          |
|----------------|--------------------------------------------------|
| `acp index`    | Parse and resolve `@acp:ref` annotations         |
| `acp query`    | Add `--refs`, `--style`, `--documentation` flags |
| `acp validate` | Validate ref URLs and style guide names          |
| `acp init`     | Add documentation config prompts                 |

### MCP Server Changes

| Tool/Resource            | Changes                                    |
|--------------------------|--------------------------------------------|
| `acp_query`              | Add `refs` and `style` query types         |
| `acp_constraints`        | Include style info in constraint responses |
| New: `acp_documentation` | Query project documentation sources        |

### Documentation Changes

| Document                       | Changes                       |
|--------------------------------|-------------------------------|
| `docs/annotation-reference.md` | Add ref and style sections    |
| `docs/getting-started.md`      | Add documentation setup guide |
| `primers/*.md`                 | Add ref and style handling    |
| `cli/README.md`                | Update annotation table       |

### Grammar Changes

Add to `spec/grammar/annotations.ebnf`:

```ebnf
ref_annotation     = "@acp:ref" , whitespace , ref_value ;
ref_value          = quoted_url | source_id ;
ref_version        = "@acp:ref-version" , whitespace , quoted_string ;
ref_section        = "@acp:ref-section" , whitespace , quoted_string ;
ref_fetch          = "@acp:ref-fetch" , whitespace , boolean_value ;

style_extends      = "@acp:style-extends" , whitespace , style_name ;
```

---

## Ecosystem Impact

| Tool              | Impact  | Required Changes               |
|-------------------|---------|--------------------------------|
| VS Code Extension | Medium  | Parse refs, show hover info    |
| Language Server   | Medium  | Provide ref completions        |
| Cursor            | Low     | Read style from cache          |
| Claude Code       | Low     | Read refs and style from cache |
| Third-party tools | Low     | Cache format documented        |

---

## Rollout Plan

1. **Phase 1** (v1.0.1): Add to specification, update schemas
2. **Phase 2** (v1.0.2): Implement in CLI (`acp index` parsing)
3. **Phase 3** (v1.1.0): Add CLI query support, MCP tools
4. **Phase 4** (v1.2.0): IDE integration, documentation fetching helpers

---

## Open Questions

1. **Should we provide a documentation fetching library?**
    - Pro: Consistent behavior across tools
    - Con: Scope creep, tool-specific needs vary

2. **How should conflicting style rules be merged?**
    - Current proposal: Last wins (symbol > file > config default)
    - Alternative: Explicit merge strategies

3. **Should we support non-HTTP sources (e.g., file://, man://)?**
    - Current proposal: HTTPS/HTTP only
    - Alternative: Extensible scheme support

4. **Rate limiting for documentation fetches?**
    - Should spec define rate limit recommendations?
    - Leave to implementations?

---

## Resolved Questions

1. **Q**: Should `@acp:ref` support multiple URLs per annotation?
   **A**: No, use multiple `@acp:ref` annotations. Cleaner parsing.

2. **Q**: Should style guides be versioned separately from ACP?
   **A**: Yes, built-in guides reference external sources that version independently.

3. **Q**: Should we validate URL accessibility at index time?
   **A**: Optional (`lastVerified` field), not required. Network conditions vary.

---

## References

- Related discussion: Initial design requirement for documentation linking
- Prior art: TypeScript `@see` JSDoc tag
- Prior art: Rust `#[doc]` attributes with URLs
- Prior art: Python PEP 257 documentation conventions
- External: [Tailwind CSS Documentation](https://tailwindcss.com/docs)

---

## Appendix

### A. Built-in Style Guide URLs

| Guide Name          | Documentation URL                                            |
|---------------------|--------------------------------------------------------------|
| `google-typescript` | https://google.github.io/styleguide/tsguide.html             |
| `google-javascript` | https://google.github.io/styleguide/jsguide.html             |
| `google-python`     | https://google.github.io/styleguide/pyguide.html             |
| `google-java`       | https://google.github.io/styleguide/javaguide.html           |
| `google-cpp`        | https://google.github.io/styleguide/cppguide.html            |
| `google-go`         | https://go.dev/doc/effective_go                              |
| `airbnb-javascript` | https://github.com/airbnb/javascript                         |
| `airbnb-react`      | https://github.com/airbnb/javascript/tree/master/react       |
| `pep8`              | https://peps.python.org/pep-0008/                            |
| `black`             | https://black.readthedocs.io/en/stable/the_black_code_style/ |
| `prettier`          | https://prettier.io/docs/en/options.html                     |
| `rustfmt`           | https://rust-lang.github.io/rustfmt/                         |
| `standardjs`        | https://standardjs.com/rules.html                            |
| `tailwindcss-v3`    | https://v2.tailwindcss.com/docs                              |
| `tailwindcss-v4`    | https://tailwindcss.com/docs                                 |

### B. Security Considerations

1. **URL validation** — Only HTTPS/HTTP schemes allowed by default
2. **Approved sources** — `requireApprovedSources` option prevents arbitrary URLs
3. **Fetch control** — `@acp:ref-fetch false` prevents automatic fetching
4. **Internal URLs** — Can mark sources as `fetchable: false` for internal wikis
5. **No code execution** — Refs are informational only, never executed

### C. Performance Considerations

1. **Index time** — Ref parsing adds minimal overhead
2. **Cache size** — Refs stored compactly (URLs as strings)
3. **Query time** — Documentation index enables O(1) source lookups
4. **Fetch latency** — Fetching is AI tool responsibility, not ACP
5. **Caching** — AI tools should cache fetched documentation

---

## Changelog

| Date       | Change        |
|------------|---------------|
| 2025-12-20 | Initial draft |

---

<!--
## RFC Process Checklist (for maintainers)

- [ ] RFC number assigned
- [ ] Added to proposed/
- [ ] Discussion link added
- [ ] Initial review complete
- [ ] Community feedback period (2+ weeks)
- [ ] FCP announced
- [ ] FCP complete (10 days)
- [ ] Decision made
- [ ] Moved to accepted/ or rejected/
-->

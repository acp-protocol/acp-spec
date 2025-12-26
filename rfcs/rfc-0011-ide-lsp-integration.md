# RFC-0011: IDE and LSP Integration

- **RFC ID**: 0011
- **Title**: IDE and LSP Integration
- **Author**: David (ACP Protocol)
- **Status**: Draft
- **Created**: 2025-12-23
- **Updated**: 2025-12-23
- **Discussion**: [Pending GitHub Discussion]
- **Parent**: RFC-0007 (ACP Complete Documentation Solution)
- **Depends On**: RFC-0006 (Documentation System Bridging), RFC-0008 (Type Annotations), RFC-0009 (Extended Annotations)
- **Related**: RFC-0001

---

## Summary

This RFC defines a Language Server Protocol (LSP) specification for ACP, enabling IDE integration with features like hover documentation, auto-completion, diagnostics, and go-to-definition for ACP annotations. It also specifies a VS Code extension as the reference implementation.

**Core feature**: Real-time ACP annotation support in any LSP-compatible editor.

This enables developers to write, navigate, and validate ACP annotations with the same tooling experience as native language features.

---

## Motivation

### Problem Statement

Currently, ACP annotations are plain text in comments with no IDE support:

1. No auto-completion for annotation namespaces or values
2. No hover documentation explaining directives
3. No validation or diagnostics for malformed annotations
4. No go-to-definition for cross-references
5. No inline rendering of related documentation

### Goals

1. Full LSP protocol specification for ACP
2. Reference implementation as `acp-lsp` server
3. VS Code extension as primary IDE integration
4. Support for hover, completions, diagnostics, go-to-definition
5. Inline documentation preview

### Non-Goals

1. Replacing native language servers (TypeScript, Python, etc.)
2. Supporting non-LSP editors (will use LSP as universal standard)
3. Semantic code analysis (focus is on annotations)

---

## Detailed Design

### 1. LSP Server Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                      ACP LSP Server                         │
├─────────────────────────────────────────────────────────────┤
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────────────┐  │
│  │   Parser    │  │    Cache    │  │   Document Manager  │  │
│  │ (Annotation │  │  (Read-only │  │  (Open file state)  │  │
│  │  Parsing)   │  │   access)   │  │                     │  │
│  └──────┬──────┘  └──────┬──────┘  └──────────┬──────────┘  │
│         │                │                    │             │
│         └────────────────┼────────────────────┘             │
│                          │                                  │
│                    ┌─────▼─────┐                            │
│                    │  Handler  │                            │
│                    │  Router   │                            │
│                    └─────┬─────┘                            │
│                          │                                  │
├──────────────────────────┼──────────────────────────────────┤
│         LSP Protocol (JSON-RPC over stdio/socket)           │
└──────────────────────────┼──────────────────────────────────┘
                           │
                    ┌──────▼──────┐
                    │   Editor    │
                    │ (VS Code,   │
                    │  Neovim,    │
                    │  etc.)      │
                    └─────────────┘
```

### 2. LSP Capabilities

The ACP LSP server implements the following LSP capabilities:

| Capability      | Method                            | Description                    |
|-----------------|-----------------------------------|--------------------------------|
| Hover           | `textDocument/hover`              | Show annotation docs on hover  |
| Completion      | `textDocument/completion`         | Auto-complete annotations      |
| Diagnostics     | `textDocument/publishDiagnostics` | Validate annotation syntax     |
| Definition      | `textDocument/definition`         | Navigate to referenced symbols |
| References      | `textDocument/references`         | Find all uses of an annotation |
| Code Actions    | `textDocument/codeAction`         | Quick fixes and refactoring    |
| Semantic Tokens | `textDocument/semanticTokens`     | Syntax highlighting            |
| Code Lens       | `textDocument/codeLens`           | Inline metrics and links       |

### 3. Hover Documentation

```typescript
// Request: textDocument/hover
{
  "textDocument": { "uri": "file:///src/auth.ts" },
  "position": { "line": 10, "character": 15 }
}

// Response when hovering over @acp:lock
{
  "contents": {
    "kind": "markdown",
    "value": "## @acp:lock\n\n**Level:** `restricted`\n\n**Directive:** Authentication logic - do not modify without security review\n\n**Provenance:** Explicit (human-written)\n\n---\n\n### Lock Levels\n- `frozen`: Never modify\n- `restricted`: Requires approval\n- `guarded`: Modify with caution\n\n[View in cache](command:acp.showInCache)"
  },
  "range": {
    "start": { "line": 10, "character": 3 },
    "end": { "line": 10, "character": 68 }
  }
}
```

### 4. Auto-Completion

```typescript
// Request: textDocument/completion
{
  "textDocument": { "uri": "file:///src/auth.ts" },
  "position": { "line": 10, "character": 8 }  // After typing "@acp:"
}

// Response
{
  "items": [
    {
      "label": "@acp:lock",
      "kind": 14,  // Keyword
      "detail": "Constraint annotation",
      "documentation": "Marks code that should not be modified",
      "insertText": "@acp:lock ${1|frozen,restricted,guarded|} - ${2:directive}",
      "insertTextFormat": 2  // Snippet
    },
    {
      "label": "@acp:param",
      "kind": 14,
      "detail": "Parameter annotation",
      "documentation": "Documents a function parameter",
      "insertText": "@acp:param {${1:type}} ${2:name} - ${3:directive}",
      "insertTextFormat": 2
    },
    {
      "label": "@acp:critical",
      "kind": 14,
      "detail": "Constraint annotation",
      "documentation": "Marks critical code sections"
    },
    // ... more completions
  ]
}
```

**Context-aware completions:**

```typescript
// After "@acp:lock " - complete with lock levels
{
  "items": [
    { "label": "frozen", "detail": "Never modify" },
    { "label": "restricted", "detail": "Requires approval" },
    { "label": "guarded", "detail": "Modify with caution" }
  ]
}

// After "@acp:param {" - complete with types
{
  "items": [
    { "label": "string", "kind": 1 },
    { "label": "number", "kind": 1 },
    { "label": "boolean", "kind": 1 },
    { "label": "Array<", "kind": 1 },
    // ... project types from cache
  ]
}
```

### 5. Diagnostics

```typescript
// Published diagnostics
{
  "uri": "file:///src/auth.ts",
  "diagnostics": [
    {
      "range": {
        "start": { "line": 10, "character": 3 },
        "end": { "line": 10, "character": 20 }
      },
      "severity": 1,  // Error
      "code": "acp/invalid-lock-level",
      "source": "acp",
      "message": "Invalid lock level 'critical'. Did you mean 'restricted'?",
      "relatedInformation": [
        {
          "location": {
            "uri": "file:///docs/lock-levels.md",
            "range": { "start": { "line": 0 }, "end": { "line": 0 } }
          },
          "message": "Valid lock levels: frozen, restricted, guarded"
        }
      ]
    },
    {
      "range": {
        "start": { "line": 15, "character": 3 },
        "end": { "line": 15, "character": 45 }
      },
      "severity": 2,  // Warning
      "code": "acp/missing-directive",
      "source": "acp",
      "message": "Annotation missing directive (the '- explanation' part)"
    },
    {
      "range": {
        "start": { "line": 20, "character": 3 },
        "end": { "line": 20, "character": 50 }
      },
      "severity": 3,  // Information
      "code": "acp/type-mismatch",
      "source": "acp",
      "message": "ACP type {string} doesn't match inferred type 'number'",
      "tags": [1]  // Unnecessary (dimmed)
    }
  ]
}
```

**Diagnostic codes:**

| Code                        | Severity  | Description                   |
|-----------------------------|-----------|-------------------------------|
| `acp/invalid-namespace`     | Error     | Unknown annotation namespace  |
| `acp/invalid-lock-level`    | Error     | Invalid lock level value      |
| `acp/missing-directive`     | Warning   | Annotation without directive  |
| `acp/type-mismatch`         | Info      | ACP type doesn't match source |
| `acp/deprecated-annotation` | Warning   | Deprecated annotation type    |
| `acp/duplicate-annotation`  | Warning   | Same annotation on symbol     |

### 6. Go-to-Definition

```typescript
// Request: textDocument/definition
// When clicking on "@acp:see UserService"
{
  "textDocument": { "uri": "file:///src/auth.ts" },
  "position": { "line": 10, "character": 25 }
}

// Response
{
  "uri": "file:///src/users.ts",
  "range": {
    "start": { "line": 5, "character": 0 },
    "end": { "line": 45, "character": 1 }
  }
}
```

### 7. Code Actions

```typescript
// Request: textDocument/codeAction
{
  "textDocument": { "uri": "file:///src/auth.ts" },
  "range": { "start": { "line": 10 }, "end": { "line": 10 } },
  "context": {
    "diagnostics": [
      { "code": "acp/missing-directive" }
    ]
  }
}

// Response
{
  "actions": [
    {
      "title": "Add directive to annotation",
      "kind": "quickfix",
      "edit": {
        "changes": {
          "file:///src/auth.ts": [
            {
              "range": { "start": { "line": 10, "character": 25 } },
              "newText": " - TODO: Add directive"
            }
          ]
        }
      }
    },
    {
      "title": "Generate annotation from JSDoc",
      "kind": "refactor.extract",
      "command": {
        "title": "Generate ACP Annotation",
        "command": "acp.generateFromDoc",
        "arguments": ["file:///src/auth.ts", 10]
      }
    }
  ]
}
```

### 8. Semantic Tokens

Provide syntax highlighting for ACP annotations:

```typescript
// Token types for ACP
const tokenTypes = [
  'namespace',    // @acp:
  'keyword',      // lock, critical, param
  'type',         // {string}, {number}
  'parameter',    // parameter names
  'operator',     // -, |, &
  'string',       // directive text
  'comment',      // provenance markers
];

const tokenModifiers = [
  'declaration',  // First occurrence
  'deprecated',   // @acp:deprecated
  'readonly',     // @acp:lock frozen
];
```

### 9. VS Code Extension

Reference implementation as VS Code extension:

**Features:**
- Syntax highlighting for ACP annotations
- IntelliSense for annotation types and values
- Hover documentation with formatted markdown
- Diagnostic underlines with quick fixes
- Go-to-definition for cross-references
- Outline view showing annotations
- Commands for common operations

**Extension structure:**

```
acp-vscode/
├── package.json           # Extension manifest
├── src/
│   ├── extension.ts       # Extension entry point
│   ├── client.ts          # LSP client
│   ├── commands.ts        # Extension commands
│   └── providers/
│       ├── hover.ts       # Fallback hover provider
│       ├── completion.ts  # Fallback completion
│       └── decoration.ts  # Custom decorations
├── syntaxes/
│   └── acp.tmLanguage.json  # TextMate grammar
├── language-configuration.json
└── README.md
```

**package.json (excerpt):**

```json
{
  "name": "acp-vscode",
  "displayName": "ACP - AI Context Protocol",
  "description": "Language support for ACP annotations",
  "version": "1.0.0",
  "engines": { "vscode": "^1.80.0" },
  "categories": ["Programming Languages", "Linters"],
  "activationEvents": [
    "onLanguage:typescript",
    "onLanguage:javascript",
    "onLanguage:python",
    "onLanguage:rust",
    "onLanguage:go"
  ],
  "main": "./out/extension.js",
  "contributes": {
    "configuration": {
      "title": "ACP",
      "properties": {
        "acp.enable": {
          "type": "boolean",
          "default": true,
          "description": "Enable ACP language features"
        },
        "acp.lspPath": {
          "type": "string",
          "default": "",
          "description": "Path to ACP LSP server (auto-detected if empty)"
        },
        "acp.diagnostics.enable": {
          "type": "boolean",
          "default": true,
          "description": "Enable ACP diagnostics"
        },
        "acp.diagnostics.typeCheck": {
          "type": "boolean",
          "default": false,
          "description": "Check ACP types against source types"
        }
      }
    },
    "commands": [
      {
        "command": "acp.showCache",
        "title": "ACP: Show Cache"
      },
      {
        "command": "acp.generateAnnotation",
        "title": "ACP: Generate Annotation from Comment"
      },
      {
        "command": "acp.validateFile",
        "title": "ACP: Validate Current File"
      }
    ],
    "grammars": [
      {
        "scopeName": "comment.block.acp",
        "path": "./syntaxes/acp.tmLanguage.json",
        "injectTo": ["source.ts", "source.js", "source.python", "source.rust", "source.go"]
      }
    ]
  }
}
```

### 10. LSP Server Command

```bash
# Start LSP server
$ acp lsp

# With options
$ acp lsp \
  --stdio                    # Use stdio (default)
  --socket 8080              # Use TCP socket
  --cache ./acp.cache.json   # Cache file location
  --log-level debug          # Logging verbosity
  --validate                 # Enable type validation
```

### 11. Configuration

```json
{
  "lsp": {
    "enable": true,
    "port": null,
    "diagnostics": {
      "enable": true,
      "severity": {
        "invalidNamespace": "error",
        "missingDirective": "warning",
        "typeMismatch": "information"
      }
    },
    "completion": {
      "includeTypes": true,
      "includeProjectSymbols": true,
      "snippets": true
    },
    "hover": {
      "showProvenance": true,
      "showConstraints": true,
      "linkToCache": true
    }
  }
}
```

---

## Examples

### Complete IDE Experience

1. **User types `@acp:`**
   - Completion popup shows all annotation types
   - Each completion has documentation preview

2. **User selects `@acp:param`**
   - Snippet inserted: `@acp:param {|} name - directive`
   - Cursor positioned in type placeholder

3. **User types `{str`**
   - Completions: `string`, `string[]`, project types starting with "str"

4. **User completes annotation**
   - Diagnostics run on save
   - Warning if directive is empty
   - Info if type doesn't match source

5. **User hovers over annotation**
   - Formatted markdown documentation
   - Lock level explanation
   - Links to related annotations

---

## Drawbacks

1. **Maintenance burden**: Separate LSP server and VS Code extension
   - *Mitigation*: Share core logic with CLI; community can maintain extensions

2. **Language server complexity**: Full LSP implementation is substantial
   - *Mitigation*: Start with core capabilities; expand over time

3. **Performance concerns**: Real-time parsing in large files
   - *Mitigation*: Incremental parsing; debounced diagnostics

4. **Editor fragmentation**: Each editor needs client setup
   - *Mitigation*: LSP is universal; clients are thin wrappers

---

## Implementation

### Phase 1: Core LSP Server (2 weeks)

1. LSP server scaffold with stdio transport
2. Document synchronization
3. Basic hover support
4. Annotation parsing in real-time

### Phase 2: Completions and Diagnostics (2 weeks)

1. Context-aware completions
2. Snippet support
3. Diagnostic generation
4. Quick fix code actions

### Phase 3: Advanced Features (1 week)

1. Go-to-definition
2. Find references
3. Semantic tokens
4. Code lens

### Phase 4: VS Code Extension (1 week)

1. Extension packaging
2. Configuration UI
3. Custom commands
4. Marketplace preparation

**Total Effort**: ~6 weeks

---

## Changelog

| Date       | Change                             |
|------------|------------------------------------|
| 2025-12-23 | Split from RFC-0007; initial draft |

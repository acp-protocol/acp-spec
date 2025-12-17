# Variable System Specification

**ACP Version**: 1.0.0-revised
**Document Version**: 1.0.0
**Last Updated**: 2024-12-17
**Status**: Revised Draft

---

## Table of Contents

1. [Overview](#1-overview)
2. [Variable File Format](#2-variable-file-format)
3. [Variable Syntax](#3-variable-syntax)
4. [Variable Types](#4-variable-types)
5. [Variable Expansion](#5-variable-expansion)
6. [Error Handling](#6-error-handling)
7. [Variable Scoping](#7-variable-scoping)
8. [Examples](#8-examples)

---

## 1. Overview

### 1.1 Purpose

Variables provide token-efficient references to code elements. Instead of including full context in every prompt, users and AI can reference compact variable names that expand to complete information.

**Without variables:**
```
Check the validateSession function in src/auth/session.ts lines 45-89
which validates JWT tokens and returns Session objects...
```

**With variables:**
```
Check $SYM_VALIDATE for the auth logic.
```

This reduces token usage by 50-90% while maintaining full context.

### 1.2 Benefits

| Benefit | Description |
|---------|-------------|
| **Token Efficiency** | Reduce prompt size by 50-90% |
| **Consistency** | Same reference always expands to same value |
| **Abstraction** | Hide implementation details |
| **Updateability** | Change underlying code without updating prompts |

### 1.3 Design Principles

- Variables are **read-only** references (not assignable)
- Variables are **statically defined** in `.acp.vars.json`
- Variables **always expand** to the same value
- Unknown variables are **preserved as literals** (not silently removed)
- Variables have **global scope** within a project

### 1.4 Conformance

The key words "MUST", "MUST NOT", "REQUIRED", "SHALL", "SHALL NOT", "SHOULD", "SHOULD NOT", "RECOMMENDED", "MAY", and "OPTIONAL" in this document are to be interpreted as described in [RFC 2119](https://datatracker.ietf.org/doc/html/rfc2119).

---

## 2. Variable File Format

### 2.1 File Location

The variables file:
- MUST be named `.acp.vars.json`
- SHOULD be located in the project root
- MAY be placed in a configured location via `.acp.config.json`
- SHOULD be added to `.gitignore` (generated artifact)

### 2.2 File Structure

```json
{
  "version": "1.0.0",
  "variables": {
    "SYM_VALIDATE": {
      "type": "symbol",
      "value": "src/auth/session.ts:SessionService.validateSession",
      "description": "Validates JWT tokens and returns session"
    },
    "FILE_SESSION": {
      "type": "file",
      "value": "src/auth/session.ts",
      "description": "Session management service"
    },
    "DOMAIN_AUTH": {
      "type": "domain",
      "value": "authentication",
      "description": "Authentication domain"
    }
  }
}
```

### 2.3 Root Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `version` | string | Yes | ACP specification version |
| `variables` | object | Yes | Variable definitions |

### 2.4 Variable Entry Structure

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `type` | string | Yes | Variable type: `symbol`, `file`, `domain` |
| `value` | string | Yes | Reference value (qualified name, path, etc.) |
| `description` | string | No | Human-readable description |

---

## 3. Variable Syntax

### 3.1 Reference Syntax

Variables are referenced using the format: `$VARIABLE_NAME[.modifier]`

**Examples:**
- `$SYM_VALIDATE` - Base reference
- `$SYM_VALIDATE.full` - Full JSON expansion
- `$SYM_VALIDATE.ref` - File reference only
- `$SYM_VALIDATE.signature` - Signature only

### 3.2 Naming Convention

Variables follow this format:

```
$<CATEGORY>_<IDENTIFIER>
```

- MUST start with `$`
- Category prefix MUST be uppercase
- Identifier MUST be uppercase
- Words separated by underscores
- Only letters, digits, underscores after `$`

**Valid names:**
```
$SYM_VALIDATE_SESSION     ✓ Valid
$FILE_AUTH_SERVICE        ✓ Valid
$DOMAIN_AUTHENTICATION    ✓ Valid
```

**Invalid names:**
```
$SYM_validateSession      ✗ Invalid (not uppercase)
$sym_validate_session     ✗ Invalid (prefix not uppercase)
$VALIDATE_SESSION         ✗ Invalid (no category prefix)
$SYM-VALIDATE-SESSION     ✗ Invalid (hyphens not allowed)
```

### 3.3 Category Prefixes

| Prefix | Category | Description |
|--------|----------|-------------|
| `SYM_` | Symbol | Functions, classes, variables |
| `FILE_` | File | File/module references |
| `DOM_` | Domain | Domain groupings |

---

## 4. Variable Types

### 4.1 Symbol Variables (`SYM_`)

Reference code symbols (functions, classes, etc.).

**Value Format:** Qualified symbol name
```
"src/auth/session.ts:SessionService.validateSession"
```

**Expansion (default):**
```
validateSession (src/auth/session.ts:45-89) - Validates JWT tokens
```

**Example:**
```json
{
  "SYM_VALIDATE": {
    "type": "symbol",
    "value": "src/auth/session.ts:SessionService.validateSession",
    "description": "Validates JWT tokens and returns session"
  }
}
```

### 4.2 File Variables (`FILE_`)

Reference files/modules.

**Value Format:** File path
```
"src/auth/session.ts"
```

**Expansion (default):**
```
src/auth/session.ts (Session Service) - User session management, 245 lines
```

**Example:**
```json
{
  "FILE_SESSION": {
    "type": "file",
    "value": "src/auth/session.ts",
    "description": "Session management service"
  }
}
```

### 4.3 Domain Variables (`DOM_`)

Reference logical domains.

**Value Format:** Domain name
```
"authentication"
```

**Expansion (default):**
```
authentication domain (3 files, 15 symbols) - User authentication and session management
```

**Example:**
```json
{
  "DOMAIN_AUTH": {
    "type": "domain",
    "value": "authentication",
    "description": "User authentication and session management"
  }
}
```

---

## 5. Variable Expansion

### 5.1 Expansion Modifiers

Modifiers control what information is included in variable expansion.

| Modifier | Returns | Example | Use Case |
|----------|---------|---------|----------|
| (none) | Summary | `$SYM_VALIDATE` → "validateSession (src/auth/session.ts:45-89) validates JWT tokens" | Quick reference |
| `.full` | Complete JSON | `$SYM_VALIDATE.full` → entire SymbolEntry object | Detailed analysis |
| `.ref` | File reference | `$SYM_VALIDATE.ref` → "src/auth/session.ts:45-89" | Source location |
| `.signature` | Signature only | `$SYM_VALIDATE.signature` → "(token: string) => Promise<Session \| null>" | Type checking |

### 5.2 Summary Format (no modifier)

**Symbol:**
```
{name} ({file}:{lines}) {summary}
```

**File:**
```
{path} ({lines} lines) {summary}
```

**Domain:**
```
{name}: {description}
```

### 5.3 Full Format (`.full`)

Returns complete JSON object from cache:
- Symbol: Complete SymbolEntry with all fields
- File: Complete FileEntry with all fields
- Domain: Complete DomainEntry with all fields

### 5.4 Reference Format (`.ref`)

**For symbols:**
```
{file}:{start_line}-{end_line}
```

**Example:** `src/auth/session.ts:45-89`

### 5.5 Signature Format (`.signature`)

Only for symbols with signatures (functions, methods).

**Example:** `(token: string) => Promise<Session | null>`

**Behavior:**
- If not applicable: Use base expansion, emit warning

---

## 6. Error Handling

### 6.1 Undefined Variables

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
- Use double dollar: `$$VAR` → `$VAR` (no expansion, no warning)

### 6.2 Circular References

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

### 6.3 Invalid Modifiers

When modifier doesn't apply to variable type:

**Example:** `$FILE_README.signature` (files don't have signatures)

**Behavior:**
- Ignore invalid modifier
- Return base expansion (without modifier)
- Emit warning

### 6.4 Strict Mode

In strict mode (configured in `.acp.config.json`):
- Undefined variables: Error, abort
- Circular references: Error, abort
- Invalid modifiers: Error, abort

**Permissive mode** (default):
- All errors become warnings
- Processing continues

---

## 7. Variable Scoping

**Scope:** Variables are **global** within a project.

**Rules:**
- All variables in `.acp.vars.json` are project-global
- Variables MAY NOT be overridden or shadowed
- No file-scoped or module-scoped variables

**Rationale:** Simplicity and predictability. File-scoped variables would require complex resolution rules and could create confusion.

**Future:** If scoping becomes necessary, it will be added in a future version with clear precedence rules.

---

## 8. Examples

### 8.1 Complete Variables File

```json
{
  "version": "1.0.0",
  "variables": {
    "SYM_VALIDATE": {
      "type": "symbol",
      "value": "src/auth/session.ts:SessionService.validateSession",
      "description": "Validates JWT tokens and returns session"
    },
    "SYM_CREATE_SESSION": {
      "type": "symbol",
      "value": "src/auth/session.ts:SessionService.createSession",
      "description": "Creates a new user session"
    },
    "FILE_SESSION": {
      "type": "file",
      "value": "src/auth/session.ts",
      "description": "Session management service"
    },
    "FILE_MIDDLEWARE": {
      "type": "file",
      "value": "src/auth/middleware.ts",
      "description": "Authentication middleware"
    },
    "DOMAIN_AUTH": {
      "type": "domain",
      "value": "authentication",
      "description": "User authentication and session management"
    }
  }
}
```

### 8.2 Expansion Examples

**Input:**
```
Debug $SYM_VALIDATE - users getting 401 errors
```

**Expanded (default):**
```
Debug validateSession (src/auth/session.ts:45-89) - Validates JWT tokens and returns session - users getting 401 errors
```

**Input:**
```
Check $SYM_VALIDATE.full for the auth bug
```

**Expanded (.full modifier):**
```json
Check {
  "name": "validateSession",
  "qualified_name": "src/auth/session.ts:SessionService.validateSession",
  "type": "method",
  "file": "src/auth/session.ts",
  "lines": [45, 89],
  "signature": "(token: string) => Promise<Session | null>",
  "summary": "Validates JWT tokens and returns session",
  "exported": true,
  "async": true
} for the auth bug
```

**Input:**
```
The bug is in $SYM_VALIDATE.ref
```

**Expanded (.ref modifier):**
```
The bug is in src/auth/session.ts:45-89
```

### 8.3 Usage in Prompts

```
User: "I'm getting 401 errors. Check $SYM_VALIDATE and $FILE_MIDDLEWARE for issues."

AI receives expanded:
"I'm getting 401 errors. Check validateSession (src/auth/session.ts:45-89) -
Validates JWT tokens and returns session and src/auth/middleware.ts
(Authentication Middleware) - Express middleware for request authentication, 50 lines for issues."
```

### 8.4 jq Queries

```bash
# Get all symbol variables
jq '.variables | to_entries | map(select(.value.type == "symbol"))' .acp.vars.json

# Get variable by name
jq '.variables["SYM_VALIDATE"]' .acp.vars.json

# Get all variables referencing a file
jq '.variables | to_entries | map(select(.value.value | contains("session.ts")))' .acp.vars.json

# List all variable names
jq '.variables | keys' .acp.vars.json
```

---

## Appendix A: Quick Reference

| Category | Prefix | Example |
|----------|--------|---------|
| Symbol | `SYM_` | `$SYM_VALIDATE_SESSION` |
| File | `FILE_` | `$FILE_AUTH_SERVICE` |
| Domain | `DOM_` | `$DOMAIN_AUTHENTICATION` |

| Modifier | Result |
|----------|--------|
| (none) | Summary |
| `.full` | Complete details (JSON) |
| `.ref` | File reference |
| `.signature` | Type signature |

**Error Handling:**
- Undefined variable → preserved as literal + warning
- Circular reference → `[CIRCULAR: ...]` + warning
- Invalid modifier → base expansion + warning
- Escape literal `$`: use `$$VAR` → `$VAR`

---

## Appendix B: Related Documents

- [Cache Format](cache.md) - Source data for variables
- [Annotation Syntax](annotations.md) - How annotations work
- [Configuration](config.md) - Variable configuration options
- [Constraints](constraints.md) - Using variables in constraints

---

*End of Variable System Specification*

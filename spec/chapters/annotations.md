# Annotation Syntax Specification

**ACP Version**: 1.0.0-revised
**Document Version**: 1.0.0
**Last Updated**: 2024-12-17
**Status**: Revised Draft

---

## Table of Contents

1. [Overview](#1-overview)
2. [Syntax Definition](#2-syntax-definition)
3. [Comment Formats](#3-comment-formats)
4. [Annotation Namespaces](#4-annotation-namespaces)
5. [Core Annotations](#5-core-annotations)
6. [Annotation Parsing](#6-annotation-parsing)
7. [Error Handling](#7-error-handling)
8. [Examples](#8-examples)

---

## 1. Overview

### 1.1 Purpose

Annotations are structured comments that provide machine-readable metadata about code elements. They enable AI systems to understand codebase structure, respect developer constraints, and maintain consistency.

The AI Context Protocol (ACP) uses annotations to embed metadata directly in source code comments, enabling:
- Context discovery for AI systems
- Intent communication from developers
- Constraint enforcement guidance
- Token-efficient code references

### 1.2 Design Principles

- **Human Readable**: Annotations should be understandable by developers
- **Machine Parseable**: Unambiguous syntax for reliable extraction
- **Language Agnostic**: Same annotation format across all programming languages
- **Non-Intrusive**: Annotations live in comments, not affecting code execution
- **Incrementally Adoptable**: Useful with minimal annotations, powerful with full coverage
- **Advisory, Not Enforced**: Annotations guide AI behavior but don't enforce runtime restrictions

### 1.3 Conformance

The key words "MUST", "MUST NOT", "REQUIRED", "SHALL", "SHALL NOT", "SHOULD", "SHOULD NOT", "RECOMMENDED", "MAY", and "OPTIONAL" in this document are to be interpreted as described in [RFC 2119](https://datatracker.ietf.org/doc/html/rfc2119).

---

## 2. Syntax Definition

### 2.1 Formal Grammar (EBNF)

```ebnf
(* ACP Annotation Grammar *)

annotation     = "@acp:" , namespace , [ ":" , sub_namespace ] , [ whitespace , value ] ;

namespace      = identifier ;

sub_namespace  = identifier ;

identifier     = letter , { letter | digit | "-" } ;

value          = quoted_string | unquoted_string ;

quoted_string  = '"' , { any_char - '"' | '\"' } , '"' ;

unquoted_string= { any_char - newline } ;

(* Character classes *)
letter         = "a" | "b" | ... | "z" | "A" | "B" | ... | "Z" ;
digit          = "0" | "1" | ... | "9" ;
whitespace     = " " | "\t" ;
any_char       = ? any Unicode character ? ;
```

### 2.2 Syntax Components

#### 2.2.1 Annotation Prefix

All ACP annotations MUST begin with the literal string `@acp:`.

```
@acp:                    ✓ Valid prefix
@ACP:                    ✗ Invalid (case-sensitive)
@ acp:                   ✗ Invalid (no space allowed)
@acp :                   ✗ Invalid (no space allowed)
```

#### 2.2.2 Namespace

The namespace identifies the category of the annotation.

- MUST start with a letter
- MAY contain letters, digits, and hyphens
- MUST be lowercase
- SHOULD be a single word when possible

```
@acp:lock                ✓ Valid
@acp:lock-reason         ✓ Valid (with hyphen)
@acp:123invalid          ✗ Invalid (starts with digit)
@acp:LOCK                ✗ Invalid (uppercase)
```

#### 2.2.3 Sub-namespace (Optional)

An optional secondary identifier following a colon.

```
@acp:lock restricted     ✓ Namespace only
@acp:lock:level          ✓ With sub-namespace (future use)
```

#### 2.2.4 Value

The value following the annotation name.

**Unquoted values**: Single words or simple values
```
@acp:lock restricted
@acp:domain authentication
```

**Quoted values**: Values containing spaces or special characters
```
@acp:summary "Handles user authentication"
@acp:lock-reason "Legacy code, do not modify"
```

### 2.3 Whitespace Rules

- One or more spaces or tabs MAY appear between annotation name and value
- Whitespace within quoted strings is preserved
- Trailing whitespace is ignored
- Newlines terminate the annotation (unless in multi-line format)

### 2.4 Multi-Line Annotations

Annotations MAY span multiple lines using continuation:

```javascript
/**
 * @acp:summary This is a long
 * summary that spans multiple
 * lines and should be combined
 */
```

The continuation is recognized when subsequent comment lines continue the annotation without a new `@acp:` prefix.

---

## 3. Comment Formats

### 3.1 Supported Comment Styles

ACP annotations MUST appear within documentation comments. The format varies by language:

#### 3.1.1 JavaScript/TypeScript/Java/C#/C++

**Block comments (preferred)**:
```javascript
/**
 * @acp:module "Authentication Service"
 * @acp:domain authentication
 * @acp:stability stable
 */
export class AuthService { }
```

**Single-line documentation comments**:
```javascript
/// @acp:lock frozen
function criticalFunction() { }
```

#### 3.1.2 Python

**Docstrings (preferred)**:
```python
"""
@acp:module "Authentication Service"
@acp:domain authentication
@acp:stability stable
"""
class AuthService:
    pass
```

**Hash comments**:
```python
# @acp:lock frozen
def critical_function():
    pass
```

#### 3.1.3 Rust

**Module documentation**:
```rust
//! @acp:module "Authentication Service"
//! @acp:domain authentication

/// @acp:lock frozen
fn critical_function() { }
```

#### 3.1.4 Go

**Standard comments**:
```go
// @acp:module "Authentication Service"
// @acp:domain authentication

// @acp:lock frozen
func CriticalFunction() { }
```

#### 3.1.5 Ruby

```ruby
# @acp:module "Authentication Service"
# @acp:domain authentication

# @acp:lock frozen
def critical_function
end
```

### 3.2 Placement Rules

#### 3.2.1 File-Level Annotations

File-level annotations MUST appear at the top of the file, before any code:

```javascript
/**
 * @acp:module "User Authentication"
 * @acp:domain authentication
 * @acp:stability stable
 */

import { something } from 'somewhere';
```

#### 3.2.2 Symbol-Level Annotations

Symbol-level annotations MUST appear immediately before the symbol they annotate:

```javascript
/**
 * @acp:summary "Validates user session token"
 * @acp:lock restricted
 */
function validateSession(token) { }
```

#### 3.2.3 Inline Annotations

Inline annotations appear on the same line as code (limited use):

```javascript
const API_KEY = "xxx"; // @acp:sensitive
```

### 3.3 Scope Determination

| Placement | Scope | Example |
|-----------|-------|---------|
| File header (before imports) | Entire file | `@acp:module` |
| Before class/function | That symbol | `@acp:lock` |
| Before method | That method | `@acp:summary` |
| End of line | That line only | `@acp:sensitive` |

**Inheritance:** File-level annotations are inherited by all symbols in the file unless overridden by symbol-level annotations.

---

## 4. Annotation Namespaces

### 4.1 Reserved Namespaces

The following namespaces are reserved by ACP and MUST NOT be used for custom annotations:

| Namespace | Purpose | Scope | Document |
|-----------|---------|-------|----------|
| `module` | File/module metadata | File | This document |
| `summary` | Brief description | File or symbol | This document |
| `domain` | Domain classification | File or symbol | This document |
| `layer` | Architectural layer | File or symbol | This document |
| `stability` | Stability indicator | File or symbol | This document |
| `lock` | Mutation constraints | File or symbol | [constraints.md](constraints.md) |
| `lock-reason` | Justification for lock | File or symbol | [constraints.md](constraints.md) |
| `style` | Style guide reference | File or symbol | [constraints.md](constraints.md) |
| `style-rules` | Custom style rules | File or symbol | [constraints.md](constraints.md) |
| `behavior` | AI behavior guidance | File or symbol | [constraints.md](constraints.md) |
| `quality` | Quality requirements | File or symbol | [constraints.md](constraints.md) |
| `test` | Testing requirements | Symbol | [constraints.md](constraints.md) |
| `deprecated` | Deprecation notice | Symbol | This document |
| `debug` | Debug session tracking | Symbol | [debug-sessions.md](debug-sessions.md) |
| `hack` | Temporary solution marker | Symbol | [debug-sessions.md](debug-sessions.md) |

### 4.2 Extension Namespaces

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
5. Extensions MAY NOT override reserved namespaces (Section 4.1)

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

### 4.3 Future Namespaces

The following prefixes are reserved for future use:
- `perf` - Performance annotations
- `security` - Security annotations
- `access` - Access control annotations

---

## 5. Core Annotations

### 5.1 Module-Level Annotations

#### `@acp:module`

Human-readable name for the file/module.

**Syntax**: `@acp:module <name>`

**Example**:
```javascript
/**
 * @acp:module "User Authentication Service"
 */
```

**Behavior**:
- Value SHOULD be a quoted string
- Used in cache for human-readable display
- Does not affect code behavior

---

#### `@acp:summary`

Brief description of the file or symbol.

**Syntax**: `@acp:summary <text>`

**Example**:
```javascript
/**
 * @acp:summary "Handles JWT validation and session management"
 */
```

**Behavior**:
- Value SHOULD be under 200 characters
- Included in symbol index for AI context
- SHOULD complete the sentence "This [module/function]..."
- Can be used at both file and symbol level

---

#### `@acp:domain`

Logical domain classification.

**Syntax**: `@acp:domain <domain-name>`

**Example**:
```javascript
/**
 * @acp:domain authentication
 * @acp:domain security
 */
```

**Behavior**:
- Multiple domains MAY be specified
- Domain names SHOULD be lowercase, hyphenated
- Used for grouping and filtering in cache
- Common domains: `authentication`, `billing`, `user-management`, `api`, `database`

---

#### `@acp:layer`

Architectural layer classification.

**Syntax**: `@acp:layer <layer-name>`

**Example**:
```javascript
/**
 * @acp:layer service
 */
```

**Behavior**:
- Standard layers: `handler`, `controller`, `service`, `repository`, `model`, `utility`, `config`
- Custom layers are permitted
- Used for architectural analysis

---

#### `@acp:stability`

Stability indicator for the module.

**Syntax**: `@acp:stability <level>`

**Levels**:
| Level | Meaning |
|-------|---------|
| `stable` | API is stable, breaking changes unlikely |
| `experimental` | API may change, use with caution |
| `deprecated` | Will be removed, migrate away |

**Example**:
```javascript
/**
 * @acp:stability experimental
 */
```

---

### 5.2 Symbol-Level Annotations

#### `@acp:deprecated`

Mark as deprecated with migration info.

**Syntax**: `@acp:deprecated [<message>]`

**Example**:
```javascript
/**
 * @acp:deprecated "Use validateSessionV2 instead"
 */
function validateSession(token) { }
```

**Behavior**:
- AI SHOULD warn when this symbol is used
- AI SHOULD suggest the replacement if provided
- Message explains why deprecated and what to use instead

---

## 6. Annotation Parsing

### 6.1 Parsing Algorithm

This section specifies how to extract annotations from source files.

#### Step 1: Identify Documentation Comments

**By Language:**

| Language | Doc Comment Syntax |
|----------|-------------------|
| JavaScript/TypeScript | `/** ... */` (JSDoc style) |
| Python | `"""..."""` or `'''...'''` (docstrings) or `#` at module level |
| Rust | `//!` (module-level) or `///` (item-level) |
| Go | `//` comments immediately preceding declarations |
| Java/C# | `/** ... */` (Javadoc style) |
| Ruby | `=begin...=end` or `#` comments |
| PHP | `/** ... */` (PHPDoc style) |

#### Step 2: Extract Annotation Lines

For each documentation comment:
1. Remove comment delimiters
2. Extract lines containing `@acp:`
3. Parse each line according to EBNF grammar (Section 2.1)

#### Step 3: Handle Multi-line Annotations

Consecutive lines with same namespace are treated as single annotation:

```
@acp:summary This is a long
summary that spans multiple
lines and should be combined
```

Results in: `@acp:summary "This is a long summary that spans multiple lines and should be combined"`

#### Step 4: Associate with Code Elements

- **Module-level**: Annotations before first code element or in file header
- **Symbol-level**: Annotations immediately preceding symbol definition
- **Scope**: Annotations apply to immediately following element only

#### Step 5: Error Handling

- **Malformed annotation**: Handle per strictness mode (see Section 7)
- **Unknown namespace**: Warn (permissive) or error (strict)
- **In string literal**: Ignore (use language-aware parser to detect)

### 6.2 Regex Patterns

**Main annotation pattern**:
```regex
@acp:([a-z][a-z0-9-]*)(?::([a-z][a-z0-9-]*))?\s*(.+)?$
```

**Quoted string pattern**:
```regex
"(?:[^"\\]|\\.)*"
```

### 6.3 Conflict Resolution

When multiple annotations of the same type appear:

| Scenario | Resolution |
|----------|------------|
| Same annotation, same scope | Last one wins |
| Same annotation, different scopes | More specific wins |
| Contradictory values | Warning, last one wins |

---

## 7. Error Handling

### 7.1 Parse Errors

| Error | Cause | Recovery |
|-------|-------|----------|
| `E001` Invalid annotation syntax | Annotation doesn't match EBNF | Skip annotation, warn |
| `E002` Malformed value | Unclosed quote or invalid format | Skip annotation, error |
| `E003` Invalid namespace | Namespace contains invalid characters | Skip annotation, warn |

**Error Behavior:**
- **Permissive mode**: Warn, skip malformed annotation, continue
- **Strict mode**: Error, abort immediately

### 7.2 Semantic Errors

| Error | Cause | Recovery |
|-------|-------|----------|
| `E101` Unknown namespace | Namespace not in reserved list | Accept (may be extension), info |
| `E102` Invalid value | Value doesn't match expected format | Skip annotation, warn |
| `E103` Orphan annotation | No code element follows annotation | Include with null scope, warn |

**Error Behavior:**
- **Permissive mode**: Warn, use default/skip, continue
- **Strict mode**: Error, abort

### 7.3 Error Reporting Format

```json
{
  "category": "syntax",
  "severity": "warning",
  "code": "E001",
  "message": "Invalid annotation syntax",
  "location": {
    "file": "src/auth/session.ts",
    "line": 45,
    "column": 3
  },
  "snippet": "@acp:lock frozen extra-text",
  "suggestion": "Remove trailing text or use lock-reason"
}
```

See the main specification Section 11 (Error Handling) for complete error handling details.

---

## 8. Examples

### 8.1 Complete File Example

```typescript
/**
 * @acp:module "Session Management Service"
 * @acp:domain authentication
 * @acp:layer service
 * @acp:stability stable
 */

import { Redis } from 'redis';
import { JWT } from './jwt';

/**
 * @acp:summary "Validates and manages user sessions"
 * @acp:lock restricted
 * @acp:lock-reason "Security-critical authentication code"
 */
export class SessionService {

  /**
   * @acp:summary "Validates a JWT token and returns the session"
   * @acp:lock frozen
   * @acp:lock-reason "Core authentication logic - do not modify"
   */
  async validateSession(token: string): Promise<Session | null> {
    // Implementation
  }

  /**
   * @acp:deprecated "Use validateSession instead"
   */
  async validate(token: string): Promise<Session | null> {
    return this.validateSession(token);
  }
}
```

### 8.2 Multi-Domain Example

```python
"""
@acp:module "Payment Processing"
@acp:domain billing
@acp:domain compliance
@acp:layer service
@acp:lock restricted
@acp:lock-reason "Financial transactions require security review"
@acp:quality security-review
"""

class PaymentService:
    """
    @acp:summary "Processes payments via Stripe"
    """

    def process_payment(self, amount, customer_id):
        """
        @acp:summary "Processes a single payment transaction"
        @acp:lock frozen
        @acp:lock-reason "Payment logic validated by compliance team"
        """
        pass
```

### 8.3 Extension Example

```javascript
/**
 * @acp:module "API Gateway"
 * @acp:domain api
 * @acp:x-github:copilot-context "This is the main API gateway handling all external requests"
 * @acp:x-mycompany:compliance-level high
 * @acp:x-mycompany:audit-required true
 */
```

---

## Appendix A: Quick Reference

| Annotation | Level | Required | Description |
|------------|-------|----------|-------------|
| `@acp:module` | File | No | Human-readable module name |
| `@acp:summary` | Any | Recommended | Brief description |
| `@acp:domain` | File | Recommended | Domain classification |
| `@acp:layer` | File | No | Architectural layer |
| `@acp:stability` | File | No | API stability level |
| `@acp:lock` | Any | No | Mutation constraint level |
| `@acp:lock-reason` | Any | No | Justification for lock |
| `@acp:deprecated` | Symbol | No | Deprecation marker |

For constraint annotations (`@acp:lock`, `@acp:style`, `@acp:behavior`, `@acp:quality`), see [Constraint System](constraints.md).

For debug/hack annotations (`@acp:debug`, `@acp:hack`), see [Debug Sessions](debug-sessions.md).

---

## Appendix B: Related Documents

- [Constraint System](constraints.md) - `@acp:lock`, `@acp:style`, `@acp:behavior`, `@acp:quality`
- [Variable System](vars.md) - Variable expansion and references
- [Debug Sessions](debug-sessions.md) - `@acp:hack`, `@acp:debug`
- [Cache Format](cache.md) - How annotations are stored
- [Config File](config.md) - Configuration options
- [Inheritance & Cascade](inheritance.md) - How annotations cascade
- [File Discovery](discovery.md) - How files are indexed

---

*End of Annotation Syntax Specification*

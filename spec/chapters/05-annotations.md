# Annotation Syntax Specification

**ACP Version**: 1.0.0
**Document Version**: 1.1.0
**Last Updated**: 2025-12-21
**Status**: RFC-001 Compliant

---

## Table of Contents

1. [Overview](#1-overview)
2. [Syntax Definition](#2-syntax-definition)
3. [Directive Suffix](#3-directive-suffix)
4. [Comment Formats](#4-comment-formats)
5. [Annotation Levels](#5-annotation-levels)
6. [Annotation Namespaces](#6-annotation-namespaces)
7. [Core Annotations](#7-core-annotations)
8. [Annotation Parsing](#8-annotation-parsing)
9. [Error Handling](#9-error-handling)
10. [Examples](#10-examples)

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

- **Self-Documenting**: Every annotation includes a directive explaining its intent to AI agents
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
(* ACP Annotation Grammar - RFC-001 Compliant *)

annotation     = "@acp:" , namespace , [ ":" , sub_namespace ] , [ whitespace , value ] ,
                 " - " , directive ;

namespace      = identifier ;

sub_namespace  = identifier ;

identifier     = letter , { letter | digit | "-" } ;

value          = quoted_string | unquoted_string ;

quoted_string  = '"' , { any_char - '"' | '\"' } , '"' ;

unquoted_string= { any_char - (" - ") - newline } ;

directive      = directive_text ;

directive_text = { any_char - newline } ;

(* Character classes *)
letter         = "a" | "b" | ... | "z" | "A" | "B" | ... | "Z" ;
digit          = "0" | "1" | ... | "9" ;
whitespace     = " " | "\t" ;
any_char       = ? any Unicode character ? ;
```

**Key Components:**
- `@acp:` - Required prefix
- `namespace` - Annotation type (e.g., `lock`, `summary`, `purpose`)
- `value` - Optional parameter for the annotation
- ` - ` - Required directive separator (space-dash-space)
- `directive` - Self-documenting instruction for AI agents

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
 * lines and should be combined - Include this context when analyzing the file
 */
```

The continuation is recognized when subsequent comment lines continue the annotation without a new `@acp:` prefix. The directive (after ` - `) applies to the complete combined value.

---

## 3. Directive Suffix

### 3.1 Overview

All `@acp:*` annotations MUST include a **directive suffix** that provides self-documenting instructions for AI agents. The directive makes each annotation meaningful without requiring external documentation lookup.

**Syntax:**
```
@acp:<tag> [value] - <directive>
```

**Example:**
```typescript
// @acp:lock frozen - MUST NOT modify this file under any circumstances
```

### 3.2 Directive Separator

The directive is separated from the annotation by ` - ` (space-dash-space):

```
@acp:lock frozen - MUST NOT modify this file
                 ↑
        Space-dash-space separator
```

**Rules:**
- The separator MUST be exactly one space, one dash, one space
- There MUST be at least one character after the separator
- The separator MUST NOT appear within the annotation value

### 3.3 Directive Requirements

Directives MUST:
- Be actionable instructions for AI agents
- Use clear, unambiguous language
- Be self-contained (no external lookup required)
- Use RFC 2119 keywords (MUST, SHOULD, MAY) for clarity

Directives SHOULD:
- Be under 200 characters for single-line annotations
- Use imperative mood ("Do X" rather than "X should be done")
- Reference the specific action the AI should take

Directives MUST NOT:
- Be empty or placeholder text
- Contradict the annotation semantics
- Require access to external documentation

### 3.4 Multi-Line Directives

For longer directives, use multi-line format:

```typescript
/**
 * @acp:lock restricted - Explain proposed changes and get explicit
 *   user approval before modifying. This file contains security-critical
 *   authentication logic that has been audited for vulnerabilities.
 */
```

Continuation lines:
- MUST be indented (2+ spaces recommended)
- MUST NOT start with `@acp:` (which would start a new annotation)
- Are concatenated with a single space

### 3.5 Recommended Directives

The following table provides standard directive text for common annotations:

#### File-Level Annotations

| Annotation | Recommended Directive |
|------------|----------------------|
| `@acp:purpose <desc>` | `Use this understanding when analyzing or modifying this file` |
| `@acp:module <name>` | `Reference this module name in summaries and documentation` |
| `@acp:domain <name>` | `Consider domain context when making changes` |
| `@acp:owner <team>` | `Consult with <team> before making significant changes` |
| `@acp:ref <url>` | `Consult <url> before making changes to this code` |

#### Constraint Annotations

| Annotation | Recommended Directive |
|------------|----------------------|
| `@acp:lock frozen` | `MUST NOT modify this file under any circumstances` |
| `@acp:lock restricted` | `Explain proposed changes and wait for explicit approval before modifying` |
| `@acp:lock approval-required` | `Request approval for significant changes to this code` |
| `@acp:lock tests-required` | `MUST add or update tests when modifying this code` |
| `@acp:lock docs-required` | `MUST update documentation when modifying this code` |
| `@acp:lock normal` | `May modify following standard best practices` |
| `@acp:lock experimental` | `May modify aggressively; changes are expected to be reversible` |

#### Symbol-Level Annotations

| Annotation | Recommended Directive |
|------------|----------------------|
| `@acp:fn <desc>` | `Use this understanding when calling or modifying this function` |
| `@acp:class <desc>` | `Consider this description when working with this class` |
| `@acp:param <name> <desc>` | `Ensure <name> parameter satisfies this description` |
| `@acp:returns <desc>` | `Expect this return value format and behavior` |
| `@acp:throws <exception>` | `Handle this exception appropriately when calling` |
| `@acp:deprecated <msg>` | `Do not use this symbol; migrate to the suggested replacement` |

#### Inline Annotations

| Annotation | Recommended Directive |
|------------|----------------------|
| `@acp:critical` | `Review with extreme care; errors here have severe consequences` |
| `@acp:todo <task>` | `This work is pending; consider completing before related changes` |
| `@acp:fixme <issue>` | `Known issue that needs resolution; avoid relying on current behavior` |
| `@acp:perf <note>` | `Performance-sensitive code; benchmark any modifications` |

### 3.6 Custom Directives

While recommended directives provide consistency, you MAY customize directives to provide more specific context:

```typescript
// @acp:lock frozen - MUST NOT modify. This encryption constant was audited
//   by security team on 2024-03-15 and any change invalidates PCI compliance.
const ENCRYPTION_KEY_BITS = 256;
```

Custom directives SHOULD still follow the requirements in Section 3.3.

### 3.7 Directive Validation

The `acp index` command validates directives:

| Error Code | Description | Behavior |
|------------|-------------|----------|
| `E201` | Missing directive suffix | Error (directive required) |
| `E202` | Empty directive | Error |
| `E203` | Directive too long (>500 chars) | Warning |
| `E204` | Directive missing RFC 2119 keyword | Info (suggestion only) |

---

## 4. Comment Formats

### 4.1 Supported Comment Styles

ACP annotations MUST appear within documentation comments. The format varies by language:

#### 4.1.1 JavaScript/TypeScript/Java/C#/C++

**Block comments (preferred)**:
```javascript
/**
 * @acp:module "Authentication Service" - Reference this module name in documentation
 * @acp:domain authentication - Consider domain context when making changes
 * @acp:stability stable - Avoid breaking changes to public API
 */
export class AuthService { }
```

**Single-line documentation comments**:
```javascript
/// @acp:lock frozen - MUST NOT modify this function under any circumstances
function criticalFunction() { }
```

#### 4.1.2 Python

**Docstrings (preferred)**:
```python
"""
@acp:module "Authentication Service" - Reference this module name in documentation
@acp:domain authentication - Consider domain context when making changes
@acp:stability stable - Avoid breaking changes to public API
"""
class AuthService:
    pass
```

**Hash comments**:
```python
# @acp:lock frozen - MUST NOT modify this function under any circumstances
def critical_function():
    pass
```

#### 4.1.3 Rust

**Module documentation**:
```rust
//! @acp:module "Authentication Service" - Reference this module name in documentation
//! @acp:domain authentication - Consider domain context when making changes

/// @acp:lock frozen - MUST NOT modify this function under any circumstances
fn critical_function() { }
```

#### 4.1.4 Go

**Standard comments**:
```go
// @acp:module "Authentication Service" - Reference this module name in documentation
// @acp:domain authentication - Consider domain context when making changes

// @acp:lock frozen - MUST NOT modify this function under any circumstances
func CriticalFunction() { }
```

#### 4.1.5 Ruby

```ruby
# @acp:module "Authentication Service" - Reference this module name in documentation
# @acp:domain authentication - Consider domain context when making changes

# @acp:lock frozen - MUST NOT modify this function under any circumstances
def critical_function
end
```

### 4.2 Placement Rules

#### 4.2.1 File-Level Annotations

File-level annotations MUST appear at the top of the file, before any code:

```javascript
/**
 * @acp:purpose "User authentication and session management" - Use this context
 *   when analyzing or modifying any code in this file
 * @acp:domain authentication - Consider domain context when making changes
 * @acp:stability stable - Avoid breaking changes to public API
 */

import { something } from 'somewhere';
```

#### 4.2.2 Symbol-Level Annotations

Symbol-level annotations MUST appear immediately before the symbol they annotate:

```javascript
/**
 * @acp:fn "Validates user session token" - Use this understanding when calling
 *   or modifying this function
 * @acp:lock restricted - Explain proposed changes and wait for explicit approval
 */
function validateSession(token) { }
```

#### 4.2.3 Inline Annotations

Inline annotations appear on the same line as code:

```javascript
const API_KEY = "xxx"; // @acp:critical - Review with extreme care; exposure has severe consequences
```

### 4.3 Scope Determination

| Placement | Scope | Example |
|-----------|-------|---------|
| File header (before imports) | Entire file | `@acp:module` |
| Before class/function | That symbol | `@acp:lock` |
| Before method | That method | `@acp:summary` |
| End of line | That line only | `@acp:sensitive` |

**Inheritance:** File-level annotations apply to all symbols in the file. Symbol-level annotations do NOT inherit to nested symbols (explicit is better than implicit).

---

## 5. Annotation Levels

ACP annotations are organized into three hierarchical levels:

### 5.1 File-Level Annotations

Apply to the entire file. Placed at the top before any code.

| Annotation | Purpose | Required Directive |
|------------|---------|-------------------|
| `@acp:purpose` | File/module purpose description | Yes |
| `@acp:module` | Human-readable module name | Yes |
| `@acp:domain` | Domain classification | Yes |
| `@acp:owner` | Team ownership | Yes |
| `@acp:layer` | Architectural layer | Yes |
| `@acp:stability` | API stability level | Yes |
| `@acp:ref` | Reference documentation | Yes |

### 5.2 Symbol-Level Annotations

Apply to a single function, class, method, or constant. Placed immediately before the symbol.

| Annotation | Purpose | Required Directive |
|------------|---------|-------------------|
| `@acp:fn` | Function description | Yes |
| `@acp:class` | Class description | Yes |
| `@acp:method` | Method description | Yes |
| `@acp:param` | Parameter description | Yes |
| `@acp:returns` | Return value description | Yes |
| `@acp:throws` | Exception description | Yes |
| `@acp:example` | Usage example | Yes |
| `@acp:deprecated` | Deprecation notice | Yes |
| `@acp:lock` | Mutation constraint | Yes |

### 5.3 Inline Annotations

Apply to a single line or code block. Placed at end of line or on preceding line.

| Annotation | Purpose | Required Directive |
|------------|---------|-------------------|
| `@acp:critical` | Critical code marker | Yes |
| `@acp:todo` | Pending work item | Yes |
| `@acp:fixme` | Known issue marker | Yes |
| `@acp:perf` | Performance note | Yes |
| `@acp:hack` | Temporary solution | Yes |

### 5.4 Level Precedence

When annotations at different levels could conflict:
- **Most specific wins**: Inline > Symbol > File
- Symbol-level annotations do NOT inherit to nested symbols
- File-level constraints are defaults, overridable at symbol level

---

## 6. Annotation Namespaces

### 6.1 Reserved Namespaces

The following namespaces are reserved by ACP and MUST NOT be used for custom annotations:

#### File-Level Namespaces

| Namespace | Purpose | Document |
|-----------|---------|----------|
| `purpose` | File/module purpose | This document |
| `module` | Human-readable module name | This document |
| `domain` | Domain classification | This document |
| `owner` | Team ownership | This document |
| `layer` | Architectural layer | This document |
| `stability` | API stability level | This document |
| `ref` | Reference documentation | This document |

#### Symbol-Level Namespaces

| Namespace | Purpose | Document |
|-----------|---------|----------|
| `fn` | Function description | This document |
| `class` | Class description | This document |
| `method` | Method description | This document |
| `param` | Parameter description | This document |
| `returns` | Return value description | This document |
| `throws` | Exception description | This document |
| `example` | Usage example | This document |
| `deprecated` | Deprecation notice | This document |

#### Constraint Namespaces

| Namespace | Purpose | Document |
|-----------|---------|----------|
| `lock` | Mutation constraints | [constraints.md](constraints.md) |
| `lock-reason` | Justification for lock (structured) | [constraints.md](constraints.md) |
| `style` | Style guide reference | [constraints.md](constraints.md) |
| `style-rules` | Custom style rules | [constraints.md](constraints.md) |
| `behavior` | AI behavior guidance | [constraints.md](constraints.md) |
| `quality` | Quality requirements | [constraints.md](constraints.md) |
| `test` | Testing requirements | [constraints.md](constraints.md) |

#### Inline/Tracking Namespaces

| Namespace | Purpose | Document |
|-----------|---------|----------|
| `critical` | Critical code marker | This document |
| `todo` | Pending work item | This document |
| `fixme` | Known issue marker | This document |
| `perf` | Performance note | This document |
| `hack` | Temporary solution | [debug-sessions.md](debug-sessions.md) |
| `hack-ticket` | Related issue ticket | [debug-sessions.md](debug-sessions.md) |
| `hack-expires` | Expiration date | [debug-sessions.md](debug-sessions.md) |
| `debug` | Debug session tracking | [debug-sessions.md](debug-sessions.md) |

### 6.2 Extension Namespaces

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
5. Extensions MAY NOT override reserved namespaces (Section 6.1)

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

### 6.3 Future Namespaces

The following prefixes are reserved for future use:
- `security` - Security annotations
- `access` - Access control annotations

---

## 7. Core Annotations

This section documents all core annotations with their syntax, directives, and behavior.

### 7.1 File-Level Annotations

#### `@acp:purpose`

**NEW in RFC-001.** Primary file/module purpose description.

**Syntax**: `@acp:purpose <description> - <directive>`

**Example**:
```typescript
/**
 * @acp:purpose "User authentication and session management" - Use this
 *   understanding when analyzing or modifying any code in this file
 */
```

**Behavior**:
- Primary description of the file's purpose
- Stored in cache `purpose` field
- Used by AI for context understanding

---

#### `@acp:module`

Human-readable name for the file/module.

**Syntax**: `@acp:module <name> - <directive>`

**Example**:
```typescript
/**
 * @acp:module "User Authentication Service" - Reference this module name
 *   in summaries and documentation
 */
```

**Behavior**:
- Value SHOULD be a quoted string
- Used in cache for human-readable display
- Does not affect code behavior

---

#### `@acp:domain`

Logical domain classification.

**Syntax**: `@acp:domain <domain-name> - <directive>`

**Example**:
```typescript
/**
 * @acp:domain authentication - Consider domain context when making changes
 * @acp:domain security - Consider domain context when making changes
 */
```

**Behavior**:
- Multiple domains MAY be specified
- Domain names SHOULD be lowercase, hyphenated
- Used for grouping and filtering in cache
- Common domains: `authentication`, `billing`, `user-management`, `api`, `database`

---

#### `@acp:owner`

**NEW in RFC-001.** Team ownership for the file.

**Syntax**: `@acp:owner <team> - <directive>`

**Example**:
```typescript
/**
 * @acp:owner auth-team - Consult with auth-team before making significant changes
 */
```

**Behavior**:
- Identifies responsible team
- Used for routing questions and approvals
- Stored in cache for querying

---

#### `@acp:layer`

Architectural layer classification.

**Syntax**: `@acp:layer <layer-name> - <directive>`

**Example**:
```typescript
/**
 * @acp:layer service - Follow service layer patterns when modifying
 */
```

**Behavior**:
- Standard layers: `handler`, `controller`, `service`, `repository`, `model`, `utility`, `config`
- Custom layers are permitted
- Used for architectural analysis

---

#### `@acp:stability`

Stability indicator for the module.

**Syntax**: `@acp:stability <level> - <directive>`

**Levels**:
| Level | Meaning |
|-------|---------|
| `stable` | API is stable, breaking changes unlikely |
| `experimental` | API may change, use with caution |
| `deprecated` | Will be removed, migrate away |

**Example**:
```typescript
/**
 * @acp:stability experimental - API may change; use with caution and
 *   expect breaking changes
 */
```

---

#### `@acp:ref`

**NEW in RFC-001.** Reference to external documentation.

**Syntax**: `@acp:ref <url> - <directive>`

**Example**:
```typescript
/**
 * @acp:ref https://docs.example.com/auth - Consult this documentation
 *   before making changes to this code
 */
```

**Behavior**:
- Links to external documentation
- AI SHOULD consult referenced docs for context
- Multiple refs allowed

---

### 7.2 Symbol-Level Annotations

#### `@acp:fn`

**NEW in RFC-001.** Function description.

**Syntax**: `@acp:fn <description> - <directive>`

**Example**:
```typescript
/**
 * @acp:fn "Validates JWT token and returns session data" - Use this
 *   understanding when calling or modifying this function
 */
function validateSession(token: string): Session { }
```

**Behavior**:
- Primary description for functions
- Stored in symbol's `purpose` field
- Used by AI for understanding function intent

---

#### `@acp:class`

**NEW in RFC-001.** Class description.

**Syntax**: `@acp:class <description> - <directive>`

**Example**:
```typescript
/**
 * @acp:class "Manages user session lifecycle" - Consider this description
 *   when working with this class
 */
class SessionManager { }
```

---

#### `@acp:method`

**NEW in RFC-001.** Method description.

**Syntax**: `@acp:method <description> - <directive>`

**Example**:
```typescript
/**
 * @acp:method "Refreshes session token" - Use this understanding when
 *   calling or modifying this method
 */
refreshToken(): Token { }
```

---

#### `@acp:param`

**NEW in RFC-001.** Parameter description.

**Syntax**: `@acp:param <name> <description> - <directive>`

**Example**:
```typescript
/**
 * @acp:param token "JWT token string" - Ensure token parameter is a valid
 *   JWT string before calling
 */
function validate(token: string) { }
```

**Behavior**:
- Documents parameter requirements
- Used for AI understanding of function contracts

---

#### `@acp:returns`

**NEW in RFC-001.** Return value description.

**Syntax**: `@acp:returns <description> - <directive>`

**Example**:
```typescript
/**
 * @acp:returns "Session object or null if invalid" - Expect this return
 *   value format and handle null case
 */
function getSession(): Session | null { }
```

---

#### `@acp:throws`

**NEW in RFC-001.** Exception description.

**Syntax**: `@acp:throws <exception> <description> - <directive>`

**Example**:
```typescript
/**
 * @acp:throws AuthError "When token is expired or invalid" - Handle
 *   AuthError appropriately when calling this function
 */
function validateToken(token: string) { }
```

---

#### `@acp:example`

**NEW in RFC-001.** Usage example.

**Syntax**: `@acp:example <code> - <directive>`

**Example**:
```typescript
/**
 * @acp:example "const session = await validateSession(token)" - Follow
 *   this usage pattern when calling
 */
```

---

#### `@acp:deprecated`

Mark as deprecated with migration info.

**Syntax**: `@acp:deprecated <message> - <directive>`

**Example**:
```typescript
/**
 * @acp:deprecated "Use validateSessionV2 instead" - Do not use this
 *   symbol; migrate to the suggested replacement
 */
function validateSession(token: string) { }
```

**Behavior**:
- AI MUST NOT use this symbol in new code
- AI SHOULD suggest the replacement if provided
- Message explains why deprecated and what to use instead

---

### 7.3 Inline Annotations

#### `@acp:critical`

**NEW in RFC-001.** Marks critical code section.

**Syntax**: `@acp:critical - <directive>`

**Example**:
```typescript
const ENCRYPTION_KEY = process.env.KEY; // @acp:critical - Review with extreme care; errors here have severe consequences
```

**Behavior**:
- Flags code requiring extra caution
- AI SHOULD request approval before modifying
- Used for security-sensitive or business-critical code

---

#### `@acp:todo`

**NEW in RFC-001.** Pending work item.

**Syntax**: `@acp:todo <task> - <directive>`

**Example**:
```typescript
// @acp:todo "Add rate limiting" - This work is pending; consider completing before related changes
function handleRequest() { }
```

**Behavior**:
- Marks incomplete or planned work
- AI MAY offer to complete the todo
- Stored in cache for tracking

---

#### `@acp:fixme`

**NEW in RFC-001.** Known issue marker.

**Syntax**: `@acp:fixme <issue> - <directive>`

**Example**:
```typescript
// @acp:fixme "Race condition in concurrent access" - Known issue that needs
//   resolution; avoid relying on current behavior
function updateState() { }
```

**Behavior**:
- Marks known bugs or issues
- AI SHOULD NOT rely on current behavior
- Priority for fixing before related changes

---

#### `@acp:perf`

**NEW in RFC-001.** Performance note.

**Syntax**: `@acp:perf <note> - <directive>`

**Example**:
```typescript
// @acp:perf "O(n²) complexity, optimize for large datasets" - Performance-sensitive
//   code; benchmark any modifications
function processItems(items: Item[]) { }
```

**Behavior**:
- Documents performance characteristics
- AI SHOULD consider performance impact of changes
- May require benchmarking after modifications

---

## 8. Annotation Parsing

### 8.1 Parsing Algorithm

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

### 8.2 Regex Patterns

**Main annotation pattern**:
```regex
@acp:([a-z][a-z0-9-]*)(?::([a-z][a-z0-9-]*))?\s*(.+)?$
```

**Quoted string pattern**:
```regex
"(?:[^"\\]|\\.)*"
```

### 8.3 Conflict Resolution

When multiple annotations of the same type appear:

| Scenario | Resolution |
|----------|------------|
| Same annotation, same scope | Last one wins |
| Same annotation, different scopes | More specific wins |
| Contradictory values | Warning, last one wins |

---

## 9. Error Handling

### 9.1 Parse Errors

| Error | Cause | Recovery |
|-------|-------|----------|
| `E001` Invalid annotation syntax | Annotation doesn't match EBNF | Skip annotation, warn |
| `E002` Malformed value | Unclosed quote or invalid format | Skip annotation, error |
| `E003` Invalid namespace | Namespace contains invalid characters | Skip annotation, warn |

**Error Behavior:**
- **Permissive mode**: Warn, skip malformed annotation, continue
- **Strict mode**: Error, abort immediately

### 9.2 Semantic Errors

| Error | Cause | Recovery |
|-------|-------|----------|
| `E101` Unknown namespace | Namespace not in reserved list | Accept (may be extension), info |
| `E102` Invalid value | Value doesn't match expected format | Skip annotation, warn |
| `E103` Orphan annotation | No code element follows annotation | Include with null scope, warn |

**Error Behavior:**
- **Permissive mode**: Warn, use default/skip, continue
- **Strict mode**: Error, abort

### 9.3 Directive Errors

| Error | Cause | Recovery |
|-------|-------|----------|
| `E201` Missing directive suffix | Annotation has no ` - ` separator | Error (directive required) |
| `E202` Empty directive | Directive text is empty after separator | Error |
| `E203` Directive too long | Directive exceeds 500 characters | Warning |
| `E204` Missing RFC 2119 keyword | Directive lacks MUST/SHOULD/MAY | Info (suggestion only) |

### 9.4 Error Reporting Format

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

## 10. Examples

### 10.1 Complete File Example with Directives

```typescript
/**
 * @acp:purpose "Session lifecycle management for authenticated users" - Use
 *   this understanding when analyzing or modifying any code in this file
 * @acp:module "Session Management Service" - Reference this module name in
 *   summaries and documentation
 * @acp:domain authentication - Consider domain context when making changes
 * @acp:layer service - Follow service layer patterns when modifying
 * @acp:stability stable - Avoid breaking changes to public API
 * @acp:owner auth-team - Consult with auth-team before significant changes
 */

import { Redis } from 'redis';
import { JWT } from './jwt';

/**
 * @acp:class "Validates and manages user sessions" - Consider this description
 *   when working with this class
 * @acp:lock restricted - Explain proposed changes and wait for explicit
 *   approval before modifying
 * @acp:lock-reason "Security-critical authentication code"
 */
export class SessionService {

  /**
   * @acp:fn "Validates a JWT token and returns the session" - Use this
   *   understanding when calling or modifying this function
   * @acp:param token "JWT token string from request header" - Ensure token
   *   is a valid JWT string before calling
   * @acp:returns "Session object or null if invalid" - Handle null case
   *   appropriately in calling code
   * @acp:throws AuthError "When token is malformed" - Handle AuthError
   *   appropriately when calling
   * @acp:lock frozen - MUST NOT modify this function under any circumstances
   * @acp:lock-reason "Core authentication logic - audited and verified"
   */
  async validateSession(token: string): Promise<Session | null> {
    // Implementation
  }

  /**
   * @acp:deprecated "Use validateSession instead" - Do not use this symbol;
   *   migrate to the suggested replacement
   */
  async validate(token: string): Promise<Session | null> {
    return this.validateSession(token);
  }
}
```

### 10.2 Multi-Domain Example

```python
"""
@acp:purpose "Payment transaction processing and reconciliation" - Use this
  understanding when analyzing or modifying any code in this file
@acp:module "Payment Processing" - Reference this module name in documentation
@acp:domain billing - Consider domain context when making changes
@acp:domain compliance - Consider domain context when making changes
@acp:layer service - Follow service layer patterns when modifying
@acp:lock restricted - Explain proposed changes and wait for approval
@acp:lock-reason "Financial transactions require security review"
@acp:quality security-review - Ensure security review before merge
"""

class PaymentService:
    """
    @acp:class "Processes payments via Stripe API" - Consider this description
      when working with this class
    """

    def process_payment(self, amount, customer_id):
        """
        @acp:fn "Processes a single payment transaction" - Use this understanding
          when calling or modifying this function
        @acp:lock frozen - MUST NOT modify this function under any circumstances
        @acp:lock-reason "Payment logic validated by compliance team"
        """
        pass
```

### 10.3 Inline Annotations Example

```typescript
/**
 * @acp:purpose "Cryptographic utilities" - Use this understanding when
 *   analyzing or modifying any code in this file
 */

// @acp:critical - Review with extreme care; errors here have severe consequences
const ENCRYPTION_KEY_BITS = 256;

// @acp:todo "Add key rotation support" - This work is pending; consider
//   completing before related changes
function encrypt(data: string): string {
  // @acp:perf "AES-256-GCM is CPU intensive" - Performance-sensitive code;
  //   benchmark any modifications
  return crypto.encrypt(data);
}

// @acp:fixme "Timing attack vulnerability" - Known issue that needs resolution;
//   avoid relying on current behavior
function compare(a: string, b: string): boolean {
  return a === b; // Should use constant-time comparison
}
```

### 10.4 Extension Example

```javascript
/**
 * @acp:purpose "API Gateway for external requests" - Use this understanding
 *   when analyzing or modifying any code in this file
 * @acp:module "API Gateway" - Reference this module name in documentation
 * @acp:domain api - Consider domain context when making changes
 * @acp:x-github:copilot-context "Main API gateway handling all external requests"
 * @acp:x-mycompany:compliance-level high
 * @acp:x-mycompany:audit-required true
 */
```

---

## Appendix A: Quick Reference

### File-Level Annotations

| Annotation | Directive Required | Description |
|------------|-------------------|-------------|
| `@acp:purpose` | Yes | File/module purpose |
| `@acp:module` | Yes | Human-readable module name |
| `@acp:domain` | Yes | Domain classification |
| `@acp:owner` | Yes | Team ownership |
| `@acp:layer` | Yes | Architectural layer |
| `@acp:stability` | Yes | API stability level |
| `@acp:ref` | Yes | Reference documentation |

### Symbol-Level Annotations

| Annotation | Directive Required | Description |
|------------|-------------------|-------------|
| `@acp:fn` | Yes | Function description |
| `@acp:class` | Yes | Class description |
| `@acp:method` | Yes | Method description |
| `@acp:param` | Yes | Parameter description |
| `@acp:returns` | Yes | Return value description |
| `@acp:throws` | Yes | Exception description |
| `@acp:example` | Yes | Usage example |
| `@acp:deprecated` | Yes | Deprecation marker |
| `@acp:lock` | Yes | Mutation constraint |

### Inline Annotations

| Annotation | Directive Required | Description |
|------------|-------------------|-------------|
| `@acp:critical` | Yes | Critical code marker |
| `@acp:todo` | Yes | Pending work item |
| `@acp:fixme` | Yes | Known issue marker |
| `@acp:perf` | Yes | Performance note |
| `@acp:hack` | Yes | Temporary solution |

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

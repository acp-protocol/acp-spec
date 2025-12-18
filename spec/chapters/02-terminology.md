# Terminology

**ACP Version**: 1.0.0
**Document Version**: 1.0.0
**Last Updated**: 2024-12-18
**Status**: Draft

---

## Table of Contents

1. [RFC 2119 Keywords](#1-rfc-2119-keywords)
2. [Core Concepts](#2-core-concepts)
3. [File Types](#3-file-types)
4. [Annotations](#4-annotations)
5. [Constraints](#5-constraints)
6. [Variables](#6-variables)
7. [Symbols](#7-symbols)
8. [Organization](#8-organization)
9. [Conformance](#9-conformance)
10. [Glossary](#10-glossary)

---

## 1. RFC 2119 Keywords

This specification uses requirement level keywords as defined in [RFC 2119](https://datatracker.ietf.org/doc/html/rfc2119). These keywords indicate the level of requirement for conformant implementations.

### 1.1 Absolute Requirements

| Keyword | Meaning |
|---------|---------|
| **MUST** | Absolute requirement. Implementations that do not satisfy this are non-conformant. |
| **MUST NOT** | Absolute prohibition. Implementations that violate this are non-conformant. |
| **REQUIRED** | Synonym for MUST. |
| **SHALL** | Synonym for MUST. |
| **SHALL NOT** | Synonym for MUST NOT. |

### 1.2 Recommendations

| Keyword | Meaning |
|---------|---------|
| **SHOULD** | Strong recommendation. Valid reasons may exist to ignore, but implications must be understood. |
| **SHOULD NOT** | Strong discouragement. Valid reasons may exist to allow, but implications must be understood. |
| **RECOMMENDED** | Synonym for SHOULD. |

### 1.3 Optional Features

| Keyword | Meaning |
|---------|---------|
| **MAY** | Truly optional. Implementations may include or omit this feature. |
| **OPTIONAL** | Synonym for MAY. |

### 1.4 Usage in This Specification

Throughout ACP documentation:

- Requirements for **implementations** (tools, CLIs, libraries) use these keywords to specify conformance requirements
- Requirements for **AI systems** use these keywords to specify expected behavior when claiming ACP conformance
- Requirements for **file formats** use these keywords to specify validation rules

**Example usage:**
```
Cache files MUST use UTF-8 encoding.
Implementations SHOULD support incremental updates.
Extensions MAY define custom annotation namespaces.
```

---

## 2. Core Concepts

### 2.1 AI Context Protocol (ACP)

A standardized protocol for embedding machine-readable documentation and constraints in source code. ACP enables AI systems to understand code structure, respect developer intentions, and navigate codebases efficiently.

### 2.2 Annotation

A structured comment in source code that provides machine-readable metadata. All ACP annotations use the `@acp:` prefix.

**Example:**
```javascript
/**
 * @acp:module "Authentication Service"
 * @acp:domain authentication
 * @acp:lock restricted
 */
```

### 2.3 Constraint

An advisory rule that guides how AI systems should interact with code. Constraints are not enforced at runtime but MUST be respected by conformant AI systems.

**Types:** Lock, Style, Behavior, Quality

### 2.4 Variable

A token-efficient reference to a code element. Variables use the `$PREFIX_NAME` format and expand to full context.

**Example:** `$SYM_VALIDATE_SESSION` expands to full symbol information.

### 2.5 Cache

A pre-computed JSON file (`.acp.cache.json`) containing indexed codebase structure. The cache enables efficient queries without parsing source files on every request.

### 2.6 Index

The process of scanning source files to extract annotations, symbols, and relationships, producing a cache file.

### 2.7 Query

A request for information from the cache. Queries can be made via CLI, jq, or programmatic API.

---

## 3. File Types

### 3.1 Cache File (`.acp.cache.json`)

The pre-computed codebase index containing:
- File metadata
- Symbol definitions
- Call graphs
- Domain classifications
- Constraint index

**Status:** Required for ACP functionality
**Generation:** Created by `acp index` command
**Location:** Project root (default)

### 3.2 Config File (`.acp.config.json`)

Project-level configuration controlling:
- File inclusion/exclusion patterns
- Error handling strictness
- Default constraints
- Domain patterns
- Implementation limits

**Status:** Optional (sensible defaults apply)
**Generation:** Created by `acp init` or manually
**Location:** Project root

### 3.3 Variables File (`.acp.vars.json`)

Variable definitions for token-efficient references:
- Symbol variables (`SYM_*`)
- File variables (`FILE_*`)
- Domain variables (`DOM_*`)

**Status:** Optional
**Generation:** Created by `acp vars` command
**Location:** Project root (default)

### 3.4 Directory Config (`.acp.dir.json`)

Directory-level configuration that applies to all files in a directory tree. Used for setting constraints on entire directories.

**Status:** Optional
**Generation:** Manual
**Location:** Any directory

---

## 4. Annotations

### 4.1 Annotation Prefix

The literal string `@acp:` that begins all ACP annotations. Case-sensitive; `@ACP:` is invalid.

### 4.2 Namespace

The category identifier following the prefix. Identifies the type of annotation.

**Examples:** `module`, `domain`, `lock`, `summary`

### 4.3 Sub-namespace

An optional secondary identifier following the namespace, separated by a colon.

**Example:** `@acp:lock-reason` (where `lock-reason` could be considered a sub-namespace)

### 4.4 Value

The content following the namespace. May be quoted (for strings with spaces) or unquoted (for simple values).

**Examples:**
- Quoted: `@acp:module "User Authentication"`
- Unquoted: `@acp:lock frozen`

### 4.5 Reserved Namespaces

Namespaces defined by the ACP specification:

| Namespace | Purpose |
|-----------|---------|
| `module` | File/module name |
| `summary` | Brief description |
| `domain` | Domain classification |
| `layer` | Architectural layer |
| `stability` | Stability indicator |
| `lock` | Modification constraint |
| `lock-reason` | Lock justification |
| `style` | Style guide |
| `style-rules` | Custom style rules |
| `behavior` | AI behavior guidance |
| `quality` | Quality requirements |
| `deprecated` | Deprecation notice |
| `ref` | External reference |
| `hack` | Temporary code marker |
| `debug` | Debug session marker |

### 4.6 Extension Namespace

Custom namespaces defined by vendors, using the `x-` prefix.

**Format:** `@acp:x-vendor-feature`
**Example:** `@acp:x-acme-internal "team-only"`

---

## 5. Constraints

### 5.1 Advisory Constraint

A rule that guides AI behavior without runtime enforcement. AI systems claiming ACP conformance MUST respect these constraints, but there is no technical mechanism to prevent violations.

### 5.2 Lock Level

A graduated scale of modification restrictions. Listed from most to least restrictive:

| Level | Description | AI Behavior |
|-------|-------------|-------------|
| `frozen` | Never modify | MUST NOT change under any circumstances |
| `restricted` | Approval required | MUST get explicit user approval first |
| `approval-required` | Ask before changing | SHOULD get approval for significant changes |
| `tests-required` | Include tests | MUST add/update tests with changes |
| `docs-required` | Include docs | MUST update documentation with changes |
| `review-required` | Flag for review | MUST mark changes for human review |
| `normal` | No restrictions | Default; no special requirements |
| `experimental` | Encourage changes | Extra caution advised; API may change |

### 5.3 Style Constraint

A constraint specifying coding conventions and formatting requirements.

**Example:** `@acp:style google-typescript`

### 5.4 Style Rules

Custom rules extending a base style guide.

**Example:** `@acp:style-rules max-line-length=100, no-any`

### 5.5 Behavior Constraint

Guidance for AI decision-making approach.

| Value | Meaning |
|-------|---------|
| `conservative` | Prefer safe, minimal changes |
| `balanced` | Balance safety and functionality (default) |
| `aggressive` | Prioritize functionality, accept more risk |

### 5.6 Quality Constraint

Requirements for additional checks or validations.

**Common values:**
- `security-review` — Requires security audit
- `performance-test` — Requires performance validation
- `manual-test` — Requires manual testing
- `regression-test` — Requires regression testing

---

## 6. Variables

### 6.1 Variable Reference

A token in the format `$PREFIX_NAME` that expands to code element information.

**Example:** `$SYM_VALIDATE_SESSION`

### 6.2 Variable Prefix

The category identifier at the start of a variable name.

| Prefix | Category | References |
|--------|----------|------------|
| `SYM_` | Symbol | Functions, classes, methods |
| `FILE_` | File | Files and modules |
| `DOM_` | Domain | Domain groupings |

### 6.3 Variable Expansion

The process of replacing a variable reference with its full content.

**Example:**
- Input: `$SYM_VALIDATE`
- Output: `validateSession (src/auth/session.ts:45-89) - Validates JWT tokens`

### 6.4 Expansion Modifier

A suffix that controls what information is included in expansion.

| Modifier | Returns |
|----------|---------|
| (none) | Summary format |
| `.full` | Complete JSON object |
| `.ref` | File reference only |
| `.signature` | Type signature only |

### 6.5 Variable Scoping

Variables have **global scope** within a project. All variables in `.acp.vars.json` are accessible throughout the project.

---

## 7. Symbols

### 7.1 Symbol

A named code element: function, class, method, constant, type, etc.

### 7.2 Qualified Name

A unique identifier for a symbol, combining file path and symbol path.

**Format:** `{relative_path}:{qualified_symbol}`

**Examples:**
- `src/auth/session.ts:SessionService.validateSession`
- `src/utils/helpers.ts:formatDate`
- `lib/core.py:CoreEngine.process`

### 7.3 Symbol Types

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

### 7.4 Symbol Visibility

Access level for a symbol.

| Visibility | Meaning |
|------------|---------|
| `public` | Accessible from anywhere |
| `private` | Accessible only within defining scope |
| `protected` | Accessible within class hierarchy |

### 7.5 Exported Symbol

A symbol that is exported from its module and available for import by other modules.

---

## 8. Organization

### 8.1 Domain

A logical grouping of related code based on business or functional area.

**Examples:** `authentication`, `billing`, `user-management`, `api`, `database`

**Usage:** Declared via `@acp:domain` annotation; indexed in cache for filtering and navigation.

### 8.2 Layer

An architectural classification indicating a symbol's role in the application structure.

**Standard layers:**
| Layer | Purpose |
|-------|---------|
| `handler` | Request/event handlers |
| `controller` | HTTP controllers |
| `service` | Business logic |
| `repository` | Data access |
| `model` | Data models |
| `utility` | Helper functions |
| `config` | Configuration |

### 8.3 Stability

An indicator of how likely a module's API is to change.

| Level | Meaning |
|-------|---------|
| `stable` | API is stable; breaking changes unlikely |
| `experimental` | API may change; use with caution |
| `deprecated` | Will be removed; migrate away |

### 8.4 Call Graph

A data structure representing call relationships between symbols.

**Components:**
- **Forward graph:** Maps symbol → symbols it calls
- **Reverse graph:** Maps symbol → symbols that call it

---

## 9. Conformance

### 9.1 Conformance Level

A tier of implementation capability. ACP defines three levels:

| Level | Name | Capabilities |
|-------|------|--------------|
| **Level 1** | Reader | Parse and query cache files |
| **Level 2** | Standard | Generate cache, variables, enforce constraints |
| **Level 3** | Full | MCP integration, debug sessions, watch mode |

### 9.2 Conformant Implementation

An implementation that meets all MUST requirements for its claimed conformance level.

### 9.3 Strictness Mode

A configuration option controlling error handling behavior.

| Mode | Behavior |
|------|----------|
| `permissive` | Warn on errors, continue processing (default) |
| `strict` | Fail fast on first error |

### 9.4 Partial Conformance

An implementation that meets a base conformance level plus specific features from higher levels.

---

## 10. Glossary

Alphabetical listing of all defined terms.

| Term | Definition |
|------|------------|
| **ACP** | AI Context Protocol; the standardized protocol defined by this specification |
| **Annotation** | A structured `@acp:` comment providing machine-readable metadata |
| **Behavior constraint** | Guidance for AI decision-making approach (conservative, balanced, aggressive) |
| **Cache** | Pre-computed JSON file containing indexed codebase structure |
| **Call graph** | Data structure mapping call relationships between symbols |
| **Conformance level** | Tier of implementation capability (Level 1, 2, or 3) |
| **Constraint** | Advisory rule guiding AI interaction with code |
| **Domain** | Logical grouping of code by business/functional area |
| **Expansion** | Process of replacing a variable reference with full content |
| **Extension namespace** | Custom `x-` prefixed namespace for vendor extensions |
| **Frozen** | Lock level prohibiting all modifications |
| **Index** | Process of scanning source files to produce a cache |
| **Layer** | Architectural classification (handler, service, repository, etc.) |
| **Lock level** | Graduated scale of modification restrictions |
| **Modifier** | Suffix controlling variable expansion format |
| **Namespace** | Category identifier in an annotation (module, lock, domain, etc.) |
| **Qualified name** | Unique symbol identifier combining file path and symbol path |
| **Query** | Request for information from the cache |
| **Quality constraint** | Requirement for additional checks (security-review, etc.) |
| **Reserved namespace** | Namespace defined by the ACP specification |
| **Restricted** | Lock level requiring explicit approval for changes |
| **Stability** | Indicator of API change likelihood (stable, experimental, deprecated) |
| **Strictness mode** | Error handling configuration (permissive or strict) |
| **Style constraint** | Coding convention and formatting requirements |
| **Symbol** | Named code element (function, class, method, etc.) |
| **Variable** | Token-efficient reference in `$PREFIX_NAME` format |
| **Variable prefix** | Category identifier (SYM_, FILE_, DOM_) |
| **Visibility** | Symbol access level (public, private, protected) |

---

## Appendix A: Notation Conventions

### A.1 Code Examples

Code examples in this specification use the following conventions:

```javascript
// JavaScript/TypeScript examples use this comment style
/**
 * Block comments for annotations
 */
```

```python
# Python examples use hash comments
"""
Or docstrings for annotations
"""
```

```json
{
  "json_examples": "use standard JSON format",
  "field_names": "use snake_case per ACP convention"
}
```

### A.2 Placeholders

| Notation | Meaning |
|----------|---------|
| `<value>` | Required placeholder to be replaced |
| `[value]` | Optional value |
| `...` | Content omitted for brevity |
| `{field}` | Dynamic field reference |

### A.3 Tables

Requirement level indicators in tables:

| Symbol | Meaning |
|--------|---------|
| ✓ MUST | Required field |
| ⚠ SHOULD | Recommended field |
| ✗ MAY | Optional field |

---

## Appendix B: Related Documents

- [Introduction](01-introduction.md) — Overview and getting started
- [Cache Format](03-cache-format.md) — Cache file specification
- [Config Format](04-config-format.md) — Configuration options
- [Annotations](05-annotations.md) — Annotation syntax
- [Constraints](06-constraints.md) — Constraint system
- [Variables](07-variables.md) — Variable system
- [Main Specification](../ACP-1.0.md) — Complete protocol specification

---

*End of Terminology*
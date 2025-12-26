# RFC-0006: Documentation System Bridging

- **RFC ID**: 0006
- **Title**: Documentation System Bridging
- **Author**: David (ACP Protocol)
- **Status**: Implemented
- **Created**: 2025-12-22
- **Updated**: 2025-12-24
- **Spec Implemented**: 2025-12-23
- **CLI Implemented**: 2025-12-24
- **Release**: 0.5.0
- **Discussion**: [Pending GitHub Discussion]
- **Supersedes**: None
- **Related**: RFC-0001 (Self-Documenting Annotations), RFC-0003 (Provenance Tracking), RFC-0010 (Cache-First Documentation Generator)

---

## Summary

This RFC introduces a documentation system bridging layer that enables ACP to:

1. **Extract and reuse** existing documentation from JSDoc, Python docstrings (Google/NumPy/Sphinx styles), TypeDoc, Javadoc, and other language-specific documentation systems
2. **Avoid semantic collisions** by establishing clear precedence rules when both native docs and ACP annotations exist
3. **Prevent comment bloat** by allowing developers to write documentation once and have ACP leverage it automatically
4. **Extend selectively** by adding ACP-specific directives only when native documentation is insufficient for AI guidance
5. **Support type hint extraction** for languages with inline type annotations (Python, TypeScript)

The core principle is: **ACP should leverage what exists, then layer on AI-specific guidance.**

---

## Motivation

### Problem Statement

Developers face significant friction when adopting ACP alongside existing documentation practices:

1. **Duplication Burden**: Writing both `@param {string} userId - User ID` (JSDoc) AND `@acp:param userId - Validate UUID format` (ACP) for the same parameter
2. **Comment Bloat**: Documentation blocks become 2-3x longer with redundant information
3. **Maintenance Divergence**: Native docs and ACP annotations drift apart over time
4. **Semantic Confusion**: Unclear which system is authoritative for different purposes
5. **Tooling Conflicts**: IDEs, type checkers, and doc generators each read different annotations

### Current State Analysis

| Language   | Primary Doc System  | Type System          | Doc Generator  | IDE Support      |
|------------|---------------------|----------------------|----------------|------------------|
| JavaScript | JSDoc               | JSDoc `@type`        | JSDoc, TypeDoc | Native           |
| TypeScript | JSDoc/TSDoc         | Native types         | TypeDoc        | Native           |
| Python     | Docstrings          | Type hints (PEP 484) | Sphinx, mkdocs | Pylance          |
| Rust       | Doc comments        | Native types         | rustdoc        | rust-analyzer    |
| Java       | Javadoc             | Native types         | Javadoc        | IntelliJ/Eclipse |
| Go         | Doc comments        | Native types         | godoc          | gopls            |

Each system has **overlapping semantic concepts** with ACP:

| Concept        | JSDoc         | Python Docstring   | ACP               | Collision?    |
|----------------|---------------|--------------------|-------------------|---------------|
| Parameter docs | `@param`      | `Args:` section    | `@acp:param`      | ✅ Yes         |
| Return value   | `@returns`    | `Returns:` section | `@acp:returns`    | ✅ Yes         |
| Exceptions     | `@throws`     | `Raises:` section  | `@acp:throws`     | ✅ Yes         |
| Deprecation    | `@deprecated` | `.. deprecated::`  | `@acp:deprecated` | ✅ Yes         |
| Examples       | `@example`    | `Example:` section | `@acp:example`    | ✅ Yes         |
| References     | `@see`        | `See Also:`        | `@acp:ref`        | Partial       |
| Function desc  | First line    | First paragraph    | `@acp:fn`         | ✅ Yes         |

### Goals

1. **Zero duplication for common cases**: If JSDoc says `@param userId - The user's unique ID`, ACP should use that description automatically
2. **Selective enhancement**: Add ACP directives only for AI-specific guidance not present in native docs
3. **Clear precedence**: When both exist, define unambiguous rules for which takes priority
4. **Provenance tracking**: Mark bridged annotations with `source: "converted"` per RFC-0003
5. **Opt-in bridging**: Projects can enable/disable bridging per documentation system
6. **Format detection**: Automatically detect Google vs NumPy vs Sphinx docstring styles
7. **Type extraction**: Extract type hints from code and include in ACP cache

### Non-Goals

1. **Replace native documentation**: ACP doesn't replace JSDoc/Sphinx/etc. for their primary purposes (IDE hints, type checking)

2. **Type checking**: ACP doesn't validate types (that's mypy/tsc's job)

3. **Replace native doc generator workflows**: Projects using Sphinx, TypeDoc, Rustdoc, etc. can continue using them for their existing documentation sites. RFC-0006 enables ACP to *leverage* their annotations, not replace their build processes. For projects wanting ACP-native documentation, see RFC-0010 (Cache-First Documentation Generator).

4. **Runtime validation**: ACP doesn't enforce types at runtime (that's Pydantic's job)

5. **Full AST analysis**: Deep semantic analysis beyond documentation extraction

---

## Relationship to RFC-0010

RFC-0006 and RFC-0010 form complementary halves of the documentation pipeline:

```
RFC-0006: Native Docs ──► Bridge ──► Cache
                                       │
RFC-0010:                              └──► Templates ──► Generated Docs
```

- **RFC-0006** populates the cache with documentation extracted from native systems
- **RFC-0010** renders the cache as human-readable documentation

Projects can use either, both, or neither:

| Scenario | RFC-0006 | RFC-0010 | Result |
|----------|----------|----------|--------|
| Greenfield ACP | ❌ | ✅ | Pure ACP annotations → `acp docs` |
| Existing JSDoc | ✅ | ❌ | Bridge to cache for AI, keep TypeDoc for humans |
| Full ACP stack | ✅ | ✅ | Bridge existing docs + generate from cache |
| AI-only | ❌ | ❌ | ACP annotations for AI, separate doc system |

---

## Detailed Design

### 1. Architecture Overview

```
┌─────────────────────────────────────────────────────────────────┐
│                      Source Code File                           │
├─────────────────────────────────────────────────────────────────┤
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────────────────┐  │
│  │ Type Hints  │  │  Native     │  │    ACP Annotations      │  │
│  │ (inline)    │  │  Doc Block  │  │    (@acp:*)             │  │
│  └──────┬──────┘  └──────┬──────┘  └───────────┬─────────────┘  │
└─────────┼────────────────┼─────────────────────┼────────────────┘
          │                │                     │
          ▼                ▼                     ▼
┌─────────────────────────────────────────────────────────────────┐
│                    ACP Parser Pipeline                          │
│  ┌──────────────┐  ┌──────────────┐  ┌───────────────────────┐  │
│  │ Type Hint    │  │ Doc Format   │  │  ACP Annotation       │  │
│  │ Extractor    │  │ Parser       │  │  Parser               │  │
│  │ (AST-based)  │  │ (multi-fmt)  │  │  (existing)           │  │
│  └──────┬───────┘  └──────┬───────┘  └───────────┬───────────┘  │
│         │                 │                      │              │
│         └────────┬────────┴──────────────────────┘              │
│                  ▼                                              │
│         ┌────────────────┐                                      │
│         │  Bridge Layer  │ ← Merge + Precedence Resolution      │
│         └────────┬───────┘                                      │
└──────────────────┼──────────────────────────────────────────────┘
                   ▼
          ┌────────────────┐
          │ .acp.cache.json│  (Unified output with provenance)
          └────────────────┘
                   │
                   ▼ (Optional: RFC-0010)
          ┌────────────────┐
          │   acp docs     │  (Generate HTML/Markdown)
          └────────────────┘
```

### 2. Configuration Schema

Add to `.acp.config.json`:

```json
{
  "bridge": {
    "enabled": true,
    "precedence": "acp-first",
    
    "jsdoc": {
      "enabled": true,
      "extractTypes": true,
      "convertTags": ["param", "returns", "throws", "deprecated", "example", "see"],
      "skipTags": ["type", "typedef", "template", "callback", "this"]
    },
    
    "python": {
      "enabled": true,
      "docstringStyle": "auto",
      "extractTypeHints": true,
      "convertSections": ["Args", "Parameters", "Returns", "Raises", "Example", "See Also", "Warns", "Yields"],
      "skipSections": ["Attributes", "Notes", "References"]
    },
    
    "rust": {
      "enabled": true,
      "convertSections": ["Arguments", "Returns", "Panics", "Errors", "Examples", "Safety"]
    },
    
    "java": {
      "enabled": true,
      "convertTags": ["param", "return", "throws", "deprecated", "see"]
    },
    
    "go": {
      "enabled": true,
      "extractFromComments": true
    }
  }
}
```

### 3. Precedence Rules

When both native docs and ACP annotations exist for the same element:

| Precedence Mode | Behavior |
|-----------------|----------|
| `acp-first` (default) | ACP annotation wins; native used as fallback |
| `native-first` | Native doc wins; ACP used only for directives |
| `merge` | Combine: native description + ACP directive |

**Example with `merge` precedence:**

```typescript
/**
 * @param userId The user's unique identifier
 */
// @acp:param userId - MUST validate UUID format before database lookup
function getUser(userId: string) { }
```

Cache result:
```json
{
  "params": [{
    "name": "userId",
    "description": "The user's unique identifier",
    "directive": "MUST validate UUID format before database lookup",
    "provenance": "merged"
  }]
}
```

### 4. Provenance Tracking

All bridged annotations include source tracking:

```json
{
  "symbols": {
    "src/auth.ts:validateUser": {
      "purpose": "Validates user credentials against the database",
      "provenance": {
        "source": "jsdoc",
        "original_tag": "@description",
        "confidence": "high"
      },
      "params": [{
        "name": "email",
        "type": "string",
        "description": "User's email address",
        "provenance": {
          "source": "jsdoc",
          "original_tag": "@param"
        }
      }]
    }
  }
}
```

### 5. Format Detection

Automatic detection of documentation styles:

**Python docstring detection:**
```python
def detect_docstring_style(docstring: str) -> str:
    # NumPy: Section headers with underlines
    if re.search(r'\n\s*Parameters\s*\n\s*-{3,}', docstring):
        return "numpy"
    
    # Sphinx: :param:, :returns:, :raises: tags
    if re.search(r':(param|returns?|raises?|type)\s+', docstring):
        return "sphinx"
    
    # Epytext: @param, @return, @raise
    if re.search(r'@(param|return|raise)\s+', docstring):
        return "epytext"
    
    # Google: Args:, Returns:, Raises: sections
    if re.search(r'\n\s*(Args|Returns|Raises|Yields|Examples?):', docstring):
        return "google"
    
    return "unknown"
```

### 6. Type Extraction

Extract types from native annotations and source code:

**From JSDoc:**
```typescript
/**
 * @param {string} userId - The user ID
 * @returns {Promise<User>} The user object
 */
```

**From Python type hints:**
```python
def get_user(user_id: str) -> User:
    """Get a user by ID."""
```

**From TypeScript:**
```typescript
function getUser(userId: string): Promise<User> { }
```

All extracted to unified cache format:
```json
{
  "params": [{
    "name": "userId",
    "type": "string",
    "typeSource": "native"
  }],
  "returns": {
    "type": "Promise<User>",
    "typeSource": "native"
  }
}
```

---

## Tag Mapping Reference

### JSDoc → ACP

| JSDoc Tag                | ACP Equivalent           | Notes                     |
|--------------------------|--------------------------|---------------------------|
| `@param {T} name - desc` | `@acp:param name - desc` | Type extracted separately |
| `@returns {T} desc`      | `@acp:returns - desc`    | Type extracted separately |
| `@throws {T} desc`       | `@acp:throws T - desc`   |                           |
| `@deprecated msg`        | `@acp:deprecated - msg`  |                           |
| `@example code`          | `@acp:example - code`    |                           |
| `@see ref`               | `@acp:ref ref`           | Auto-generate directive   |
| `@description text`      | `@acp:fn - text`         |                           |
| `@author name`           | `@acp:owner name`        |                           |

### Python Docstring → ACP

| Docstring Section                        | ACP Equivalent    | Styles               |
|------------------------------------------|-------------------|----------------------|
| First paragraph                          | `@acp:fn`         | All                  |
| `Args:` / `Parameters` / `:param:`       | `@acp:param`      | Google/NumPy/Sphinx  |
| `Returns:` / `:returns:`                 | `@acp:returns`    | All                  |
| `Raises:` / `:raises:`                   | `@acp:throws`     | All                  |
| `Yields:` / `:yields:`                   | `@acp:returns`    | All (for generators) |
| `Example:` / `Examples`                  | `@acp:example`    | Google/NumPy         |
| `See Also:` / `.. seealso::`             | `@acp:ref`        | NumPy/Sphinx         |
| `Warning:` / `.. warning::`              | `@acp:critical`   | All                  |
| `.. deprecated::`                        | `@acp:deprecated` | Sphinx               |

### Rust → ACP

| Rust Section    | ACP Equivalent                          |
|-----------------|-----------------------------------------|
| First paragraph | `@acp:fn`                               |
| `# Arguments`   | `@acp:param` (each bullet)              |
| `# Returns`     | `@acp:returns`                          |
| `# Errors`      | `@acp:throws`                           |
| `# Panics`      | `@acp:throws` (with exception: "panic") |
| `# Examples`    | `@acp:example`                          |
| `# Safety`      | `@acp:critical`                         |

---

## Examples

### JavaScript/TypeScript with JSDoc

**Source:**
```typescript
/**
 * Validates user credentials against the database.
 * @param {string} email - The user's email address
 * @param {string} password - The password to validate
 * @returns {Promise<AuthResult>} Authentication result with token
 * @throws {InvalidCredentialsError} When credentials don't match
 * @example
 * const result = await validateUser('user@example.com', 'password123');
 */
// @acp:lock restricted - Security critical authentication
// @acp:param password - MUST NOT log or expose in errors
async function validateUser(email: string, password: string): Promise<AuthResult> {
```

**Cache output:**
```json
{
  "symbols": {
    "src/auth.ts:validateUser": {
      "purpose": "Validates user credentials against the database.",
      "provenance": { "source": "jsdoc" },
      "params": [
        {
          "name": "email",
          "type": "string",
          "description": "The user's email address",
          "provenance": { "source": "jsdoc" }
        },
        {
          "name": "password",
          "type": "string",
          "description": "The password to validate",
          "directive": "MUST NOT log or expose in errors",
          "provenance": { "source": "merged" }
        }
      ],
      "returns": {
        "type": "Promise<AuthResult>",
        "description": "Authentication result with token"
      },
      "throws": [{
        "type": "InvalidCredentialsError",
        "condition": "When credentials don't match"
      }],
      "constraints": {
        "lock_level": "restricted",
        "directive": "Security critical authentication"
      }
    }
  }
}
```

### Python with Google-style Docstrings

**Source:**
```python
def process_payment(
    amount: Decimal,
    currency: str,
    customer_id: str
) -> PaymentResult:
    """Process a payment transaction.
    
    Validates the payment details and submits to the payment processor.
    
    Args:
        amount: The payment amount. Must be positive.
        currency: ISO 4217 currency code (e.g., 'USD', 'EUR').
        customer_id: The customer's unique identifier.
    
    Returns:
        PaymentResult containing transaction ID and status.
    
    Raises:
        ValidationError: If amount is negative or currency invalid.
        PaymentDeclinedError: If the payment processor declines.
    
    Example:
        result = process_payment(Decimal('99.99'), 'USD', 'cust_123')
    """
    # @acp:lock frozen - PCI-DSS certified implementation
    # @acp:param amount - MUST validate against fraud thresholds
```

---

## Drawbacks

1. **Parser complexity**: Supporting multiple doc formats increases maintenance burden
   - *Mitigation*: Leverage existing parsers (doctrine for JSDoc, docstring_parser for Python)

2. **Precedence confusion**: Users may not understand which annotation takes priority
   - *Mitigation*: Clear defaults, explicit `provenance` field, lint warnings

3. **Stale bridging**: Native docs change but ACP cache not regenerated
   - *Mitigation*: Include file hashes, CI integration, watch mode

---

## Implementation

### Phase 1: Core Bridging (2 weeks)
1. JSDoc parser integration
2. Python docstring parser (Google style)
3. Basic precedence resolution
4. Provenance tracking

### Phase 2: Extended Format Support (2 weeks)
1. NumPy and Sphinx docstring styles
2. Rust doc comments
3. Java Javadoc
4. Go doc comments

### Phase 3: Type Extraction (1 week)
1. TypeScript type extraction
2. Python type hint extraction
3. Unified type representation

**Total Effort**: ~5 weeks

---

## Changelog

| Date       | Change |
|------------|--------|
| 2025-12-22 | Initial draft |
| 2025-12-23 | Resolved open questions; status changed to Accepted |
| 2025-12-24 | Updated Non-Goal #3 to clarify relationship with RFC-0010; added RFC-0010 relationship section |

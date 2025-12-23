# RFC-0006: Documentation System Bridging

- **RFC ID**: 0006
- **Title**: Documentation System Bridging
- **Author**: David (ACP Protocol)
- **Status**: Accepted
- **Created**: 2025-12-22
- **Updated**: 2025-12-23
- **Discussion**: [Pending GitHub Discussion]
- **Supersedes**: None
- **Related**: RFC-0001 (Self-Documenting Annotations), RFC-0003 (Provenance Tracking)

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

1. **Replace native documentation**: ACP doesn't replace JSDoc/Sphinx/etc. for their primary purposes
2. **Type checking**: ACP doesn't validate types (that's mypy/tsc's job)
3. **Doc generation**: ACP doesn't generate HTML documentation (that's Sphinx/TypeDoc's job)
4. **Runtime validation**: ACP doesn't enforce types at runtime (that's Pydantic's job)
5. **Full AST analysis**: Deep semantic analysis beyond documentation extraction

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
```

### 2. Configuration Schema

Add to `.acp.config.yaml`:

```yaml
# Documentation bridging configuration
bridge:
  # Master enable/disable
  enabled: true
  
  # Precedence when both native docs and ACP exist
  # Options: "acp-first" | "native-first" | "merge"
  precedence: "acp-first"
  
  # JavaScript/TypeScript
  jsdoc:
    enabled: true
    extractTypes: true  # Extract @type, @param {Type}
    convertTags:
      - param      # @param → @acp:param
      - returns    # @returns → @acp:returns
      - throws     # @throws → @acp:throws
      - deprecated # @deprecated → @acp:deprecated
      - example    # @example → @acp:example
      - see        # @see → @acp:ref
    skipTags:
      - type       # Don't convert @type (type-only)
      - typedef    # Don't convert @typedef
      - template   # Don't convert @template
    
  # Python docstrings
  python:
    enabled: true
    # Auto-detect or specify: "google" | "numpy" | "sphinx" | "epytext" | "auto"
    docstringStyle: "auto"
    extractTypeHints: true  # Extract from function signatures
    convertSections:
      - Args        # → @acp:param (Google style)
      - Parameters  # → @acp:param (NumPy style)  
      - Returns     # → @acp:returns
      - Raises      # → @acp:throws
      - Example     # → @acp:example
      - See Also    # → @acp:ref
      - Warns       # → @acp:throws (warnings)
      - Yields      # → @acp:returns (generators)
    skipSections:
      - Attributes  # Class attributes (not per-call)
      - Notes       # Informational only
      - References  # Bibliography, not code refs
      
  # Rust doc comments
  rust:
    enabled: true
    convertSections:
      - Arguments   # → @acp:param
      - Returns     # → @acp:returns
      - Panics      # → @acp:throws
      - Errors      # → @acp:throws
      - Examples    # → @acp:example
      - Safety      # → @acp:critical
      
  # Java/Kotlin Javadoc
  javadoc:
    enabled: true
    convertTags:
      - param       # → @acp:param
      - return      # → @acp:returns
      - throws      # → @acp:throws
      - exception   # → @acp:throws
      - deprecated  # → @acp:deprecated
      - see         # → @acp:ref
      
  # Go doc comments
  go:
    enabled: true
    # Go uses unstructured prose, extract heuristically
    extractHeuristics: true
    
  # Provenance settings (per RFC-0003)
  provenance:
    markConverted: true           # Add source: "converted" to bridged annotations
    includeSourceFormat: true     # Add sourceFormat: "jsdoc" etc.
    preserveOriginalText: false   # Don't store original doc text (saves space)
```

### 3. Precedence Rules

When both native documentation and ACP annotations exist for the same concept:

#### 3.1 `acp-first` Mode (Default)

ACP annotations take full precedence. Native docs only fill gaps.

```typescript
/**
 * @param userId - The user's unique identifier
 * @acp:param userId - MUST validate UUID format before database query
 */
```

Result:
```json
{
  "params": [{
    "name": "userId",
    "description": "The user's unique identifier",  // From JSDoc
    "directive": "MUST validate UUID format before database query",  // From ACP
    "source": "merged",
    "sourceFormats": ["jsdoc", "acp"]
  }]
}
```

#### 3.2 `native-first` Mode

Native docs are authoritative. ACP only adds directive layer.

```typescript
/**
 * @param userId - The user's unique identifier
 * @acp:param userId - Validate UUID format
 */
```

Result:
```json
{
  "params": [{
    "name": "userId",
    "description": "The user's unique identifier",
    "directive": "Validate UUID format",
    "source": "merged"
  }]
}
```

#### 3.3 `merge` Mode

Intelligently combine both, with conflict resolution.

```typescript
/**
 * @param userId - The user's unique identifier
 * @acp:param userId - MUST validate UUID format; reject malformed IDs
 */
```

Result:
```json
{
  "params": [{
    "name": "userId",
    "description": "The user's unique identifier",
    "directive": "MUST validate UUID format; reject malformed IDs",
    "source": "merged"
  }]
}
```

#### 3.4 Conflict Resolution Matrix

| Scenario                    | `acp-first`      | `native-first`   | `merge`          |
|-----------------------------|------------------|------------------|------------------|
| Both have description       | Use native       | Use native       | Use native       |
| Both have directive         | Use ACP          | Use ACP          | Concatenate      |
| Only ACP exists             | Use ACP          | Use ACP          | Use ACP          |
| Only native exists          | Use native       | Use native       | Use native       |
| Type in native, none in ACP | Include type     | Include type     | Include type     |
| Conflicting types           | Warn, use native | Use native       | Warn, use native |

### 4. Language-Specific Parsing

#### 4.1 JSDoc/TSDoc Parsing

**Input:**
```typescript
/**
 * Validates and processes a user session.
 * 
 * @param {string} token - The JWT token to validate
 * @param {ValidateOptions} [options] - Optional validation settings
 * @returns {Promise<Session | null>} The session if valid, null otherwise
 * @throws {TokenExpiredError} When the token has expired
 * @throws {MalformedTokenError} When the token structure is invalid
 * @example
 * const session = await validateSession(token);
 * if (session) {
 *   console.log('Authenticated:', session.userId);
 * }
 * @see {@link Session} for session structure
 * @deprecated Use validateSessionV2 instead
 * 
 * @acp:lock restricted - Security-critical; changes require review
 * @acp:param token - Sanitize before logging; contains user data
 */
async function validateSession(
  token: string,
  options?: ValidateOptions
): Promise<Session | null> {
  // ...
}
```

**Extracted Cache Entry:**
```json
{
  "name": "validateSession",
  "qualified_name": "src/auth/session.ts:validateSession",
  "type": "function",
  "file": "src/auth/session.ts",
  "lines": [15, 45],
  "signature": "(token: string, options?: ValidateOptions) => Promise<Session | null>",
  "async": true,
  "exported": true,
  "summary": "Validates and processes a user session.",
  "purpose": "Validates and processes a user session.",
  "params": [
    {
      "name": "token",
      "type": "string",
      "description": "The JWT token to validate",
      "directive": "Sanitize before logging; contains user data",
      "source": "merged",
      "sourceFormats": ["jsdoc", "acp"]
    },
    {
      "name": "options",
      "type": "ValidateOptions",
      "optional": true,
      "description": "Optional validation settings",
      "source": "converted",
      "sourceFormat": "jsdoc"
    }
  ],
  "returns": {
    "type": "Promise<Session | null>",
    "description": "The session if valid, null otherwise",
    "source": "converted",
    "sourceFormat": "jsdoc"
  },
  "throws": [
    {
      "exception": "TokenExpiredError",
      "description": "When the token has expired",
      "source": "converted",
      "sourceFormat": "jsdoc"
    },
    {
      "exception": "MalformedTokenError", 
      "description": "When the token structure is invalid",
      "source": "converted",
      "sourceFormat": "jsdoc"
    }
  ],
  "examples": [
    {
      "code": "const session = await validateSession(token);\nif (session) {\n  console.log('Authenticated:', session.userId);\n}",
      "source": "converted",
      "sourceFormat": "jsdoc"
    }
  ],
  "refs": [
    {
      "target": "Session",
      "description": "for session structure",
      "source": "converted",
      "sourceFormat": "jsdoc"
    }
  ],
  "deprecated": {
    "message": "Use validateSessionV2 instead",
    "source": "converted",
    "sourceFormat": "jsdoc"
  },
  "constraints": {
    "lock_level": "restricted",
    "directive": "Security-critical; changes require review",
    "source": "explicit"
  }
}
```

#### 4.2 Python Docstring Parsing (Google Style)

**Input:**
```python
def validate_session(
    token: str,
    *,
    strict: bool = True,
    max_age: Optional[int] = None
) -> Optional[Session]:
    """Validate a JWT token and return the session.
    
    Performs cryptographic verification of the token signature,
    checks expiration, and retrieves the associated session.
    
    Args:
        token: The JWT token string to validate.
        strict: If True, reject expired tokens immediately.
            Defaults to True.
        max_age: Maximum age in seconds. If None, uses default.
    
    Returns:
        The Session object if valid, None if invalid or expired.
    
    Raises:
        TokenExpiredError: If token is expired and strict=True.
        MalformedTokenError: If token structure is invalid.
        
    Example:
        >>> session = validate_session(token)
        >>> if session:
        ...     print(f"User: {session.user_id}")
        
    See Also:
        Session: The session data structure.
        validate_session_v2: Updated validation function.
        
    @acp:lock restricted - Contains credential handling logic
    @acp:param token - MUST sanitize before any logging
    @acp:returns - Verify session.is_active before granting access
    """
```

**Extracted Cache Entry:**
```json
{
  "name": "validate_session",
  "qualified_name": "src/auth/session.py:validate_session",
  "type": "function",
  "file": "src/auth/session.py",
  "lines": [12, 58],
  "signature": "(token: str, *, strict: bool = True, max_age: Optional[int] = None) -> Optional[Session]",
  "summary": "Validate a JWT token and return the session.",
  "purpose": "Validate a JWT token and return the session.",
  "params": [
    {
      "name": "token",
      "type": "str",
      "description": "The JWT token string to validate.",
      "directive": "MUST sanitize before any logging",
      "source": "merged",
      "sourceFormats": ["docstring:google", "acp"]
    },
    {
      "name": "strict",
      "type": "bool",
      "default": "True",
      "description": "If True, reject expired tokens immediately. Defaults to True.",
      "source": "converted",
      "sourceFormat": "docstring:google"
    },
    {
      "name": "max_age",
      "type": "Optional[int]",
      "default": "None",
      "description": "Maximum age in seconds. If None, uses default.",
      "source": "converted",
      "sourceFormat": "docstring:google"
    }
  ],
  "returns": {
    "type": "Optional[Session]",
    "description": "The Session object if valid, None if invalid or expired.",
    "directive": "Verify session.is_active before granting access",
    "source": "merged",
    "sourceFormats": ["docstring:google", "acp"]
  },
  "throws": [
    {
      "exception": "TokenExpiredError",
      "description": "If token is expired and strict=True.",
      "source": "converted",
      "sourceFormat": "docstring:google"
    },
    {
      "exception": "MalformedTokenError",
      "description": "If token structure is invalid.",
      "source": "converted",
      "sourceFormat": "docstring:google"
    }
  ],
  "constraints": {
    "lock_level": "restricted",
    "directive": "Contains credential handling logic",
    "source": "explicit"
  }
}
```

#### 4.3 Python Docstring Parsing (NumPy Style)

**Input:**

```python
def calculate_statistics(
    data: np.ndarray,
    weights: Optional[np.ndarray] = None,
    axis: int = 0
) -> StatisticsResult:
    """
    Calculate weighted statistics for a dataset.
    
    Computes mean, variance, and standard deviation with optional
    weighting factors. Handles NaN values according to numpy conventions.
    
    Parameters
    ----------
    data : np.ndarray
        Input data array. Must be numeric dtype.
    weights : np.ndarray, optional
        Weight factors for each data point. If None, uniform weights
        are used. Must match data shape along specified axis.
    axis : int, default 0
        Axis along which to compute statistics.
        
    Returns
    -------
    StatisticsResult
        Named tuple containing mean, variance, and std fields.
        
    Raises
    ------
    ValueError
        If weights shape doesn't match data shape.
    TypeError
        If data is not numeric dtype.
        
    See Also
    --------
    numpy.average : Weighted average calculation.
    numpy.std : Standard deviation.
    
    Notes
    -----
    NaN values are propagated unless data is masked.
    
    Examples
    --------
    >>> data = np.array([1, 2, 3, 4, 5])
    >>> result = calculate_statistics(data)
    >>> result.mean
    3.0
    
    @acp:perf "O(n) complexity" - Profile for large datasets
    @acp:param data - Validate dtype before processing
    """
```

**Detection:** NumPy style is identified by:
- Section headers with underlines (`Parameters\n----------`)
- Parameter format: `name : type`
- Multi-line parameter descriptions with indentation

#### 4.4 Python Docstring Parsing (Sphinx/reST Style)

**Input:**

```python
def authenticate_user(username: str, password: str) -> AuthResult:
    """Authenticate a user with username and password.
    
    :param username: The user's login name
    :type username: str
    :param password: The user's password (will be hashed)
    :type password: str
    :returns: Authentication result with token if successful
    :rtype: AuthResult
    :raises AuthenticationError: If credentials are invalid
    :raises RateLimitError: If too many attempts
    
    .. deprecated:: 2.0
        Use :func:`authenticate_with_oauth` instead.
        
    .. seealso::
        :func:`authenticate_with_oauth`
        :class:`AuthResult`
    
    @acp:lock frozen - Security-audited; do not modify
    """
```

### 5. Explicit Bridge Control Annotations

For fine-grained control over bridging behavior:

#### 5.1 `@acp:bridge` — Control bridging for a file/symbol

```typescript
/**
 * @acp:bridge enabled - Parse JSDoc tags in this file as ACP context
 */

/**
 * @acp:bridge disabled - Do not convert JSDoc; only use explicit @acp: tags
 */

/**
 * @acp:bridge jsdoc - Specifically enable JSDoc bridging
 */
```

#### 5.2 `@acp:bridge-style` — Specify docstring format

```python
"""Module docstring.

@acp:bridge-style google - Parse docstrings as Google style
"""

# Or at function level:
def process():
    """
    @acp:bridge-style numpy - This function uses NumPy style
    
    Parameters
    ----------
    ...
    """
```

#### 5.3 `@acp:bridge-skip` — Skip specific tags

```typescript
/**
 * @param userId - Internal implementation detail
 * @acp:bridge-skip param:userId - Don't convert this param to ACP
 * @returns {User} The user
 */
```

#### 5.4 `@acp:bridge-only` — Convert only specified tags

```typescript
/**
 * @acp:bridge-only returns,throws - Only convert these JSDoc tags
 * @param internal - This won't be converted
 * @returns {User} This will be converted
 * @throws {Error} This will be converted
 */
```

### 6. Type Hint Extraction

#### 6.1 TypeScript/JavaScript

Types are extracted from:
1. Inline type annotations: `function foo(x: string): number`
2. JSDoc type annotations: `@param {string} x`
3. Variable declarations: `const x: string = ""`

Priority: Inline > JSDoc (when both exist)

#### 6.2 Python

Types are extracted from:
1. PEP 484 type hints: `def foo(x: str) -> int:`
2. PEP 526 variable annotations: `x: str = ""`
3. Docstring type annotations: `:type x: str` (Sphinx) or `x : str` (NumPy)
4. `typing` module constructs: `Optional[T]`, `List[T]`, `Union[A, B]`

Priority: Inline hints > Docstring types

**Extracted types are included in cache:**
```json
{
  "name": "process_user",
  "params": [
    {
      "name": "user_id",
      "type": "str",
      "typeSource": "type_hint"
    },
    {
      "name": "options",
      "type": "Optional[ProcessOptions]",
      "typeSource": "type_hint",
      "typeResolved": "ProcessOptions | None"
    }
  ],
  "returns": {
    "type": "User",
    "typeSource": "type_hint"
  }
}
```

#### 6.3 Type Complexity Handling

For complex types, store both raw and simplified versions:

```json
{
  "type": "Callable[[int, str], Awaitable[Result[User, Error]]]",
  "typeSimplified": "async (int, str) -> Result<User, Error>",
  "typeComponents": {
    "kind": "callable",
    "async": true,
    "params": ["int", "str"],
    "returns": {
      "kind": "generic",
      "base": "Result",
      "args": ["User", "Error"]
    }
  }
}
```

### 7. Comment Bloat Prevention

#### 7.1 Minimal Enhancement Pattern

**Before (bloated):**
```typescript
/**
 * Fetch a user by ID.
 * 
 * @param {string} userId - The unique identifier for the user
 * @returns {Promise<User | null>} The user object or null
 * @throws {DatabaseError} On connection failure
 * 
 * @acp:fn - Fetches user from database
 * @acp:param userId - The unique identifier for the user  // DUPLICATE!
 * @acp:returns - The user object or null  // DUPLICATE!
 * @acp:throws DatabaseError - On connection failure  // DUPLICATE!
 * @acp:lock normal - Safe to modify
 */
```

**After (with bridging):**
```typescript
/**
 * Fetch a user by ID.
 * 
 * @param {string} userId - The unique identifier for the user
 * @returns {Promise<User | null>} The user object or null
 * @throws {DatabaseError} On connection failure
 * 
 * @acp:lock normal - Safe to modify following project conventions
 * @acp:param userId - Validate UUID format before query
 */
```

Only the ACP annotations that ADD value (lock constraint, directive enhancement) are needed. The rest is bridged from JSDoc.

#### 7.2 Directive-Only Enhancement

When native docs are sufficient for description, add only directives:

```python
def process_payment(
    amount: Decimal,
    currency: str,
    account_id: str
) -> PaymentResult:
    """Process a payment transaction.
    
    Args:
        amount: The payment amount.
        currency: ISO 4217 currency code.
        account_id: The account to charge.
        
    Returns:
        The payment result with transaction ID.
        
    Raises:
        InsufficientFundsError: If account balance is too low.
        InvalidCurrencyError: If currency code is not supported.
    
    @acp:lock frozen - PCI-DSS compliant; security-audited
    @acp:param amount - MUST be positive; reject zero/negative
    @acp:param account_id - Validate format; log access for audit
    @acp:throws InsufficientFundsError - Do NOT expose balance details in error
    """
```

The descriptions come from the Args/Returns/Raises sections; ACP adds only the directives.

#### 7.3 IDE/Linter Compatibility

**Challenge:** Some linters (pylint, eslint-jsdoc) may reject unknown tags.

**Solutions:**

1. **Place ACP at end of docstring:**
```python
def foo():
    """Standard docstring content.
    
    Args:
        x: Description.
    
    ACP Directives:
        @acp:lock restricted - Explain changes
    """
```

2. **Use comment after docstring:**
```python
def foo():
    """Standard docstring."""
    # @acp:lock restricted - Explain changes
```

3. **Configure linter to ignore @acp:**
```yaml
# .pylintrc
[MESSAGES CONTROL]
disable=unknown-option-value

# Or in pyproject.toml
[tool.pylint.messages_control]
disable = ["unknown-option-value"]
```

```javascript
// .eslintrc.js
module.exports = {
  settings: {
    jsdoc: {
      definedTags: ["acp:lock", "acp:param", "acp:fn", /* ... */]
    }
  }
};
```

### 8. Cache Schema Additions

Extend `cache.schema.json` to track bridging provenance:

```json
{
  "$defs": {
    "param_entry": {
      "type": "object",
      "properties": {
        "name": { "type": "string" },
        "type": { "type": ["string", "null"] },
        "typeSource": { 
          "type": "string",
          "enum": ["type_hint", "jsdoc", "docstring", "inferred"],
          "description": "Where the type was extracted from"
        },
        "optional": { "type": "boolean", "default": false },
        "default": { "type": ["string", "null"] },
        "description": { "type": ["string", "null"] },
        "directive": { "type": ["string", "null"] },
        "source": {
          "type": "string",
          "enum": ["explicit", "converted", "merged", "heuristic"],
          "description": "Provenance of this entry"
        },
        "sourceFormat": {
          "type": ["string", "null"],
          "description": "Original format (jsdoc, docstring:google, etc.)"
        },
        "sourceFormats": {
          "type": "array",
          "items": { "type": "string" },
          "description": "All formats that contributed (for merged)"
        }
      }
    },
    
    "bridge_metadata": {
      "type": "object",
      "description": "File-level bridging metadata",
      "properties": {
        "enabled": { "type": "boolean" },
        "detectedStyle": {
          "type": ["string", "null"],
          "description": "Auto-detected docstring style"
        },
        "convertedCount": {
          "type": "integer",
          "description": "Number of annotations bridged from native docs"
        },
        "mergedCount": {
          "type": "integer",
          "description": "Number of annotations merged with ACP"
        },
        "explicitCount": {
          "type": "integer",
          "description": "Number of pure ACP annotations"
        }
      }
    }
  }
}
```

### 9. CLI Commands

#### 9.1 `acp bridge status`

Show bridging statistics:

```bash
$ acp bridge status

Bridge Status for project
=========================

Configuration:
  Precedence: acp-first
  JSDoc: enabled
  Python: enabled (style: auto-detect)
  Rust: enabled
  
Detection Results:
  src/auth/*.py: Google style (15 files)
  src/utils/*.py: NumPy style (3 files)
  src/legacy/*.py: Sphinx style (2 files)
  
Statistics:
  Total annotations: 847
    Explicit (ACP only): 234 (28%)
    Converted (from native): 412 (49%)
    Merged (ACP + native): 156 (18%)
    Heuristic (auto-gen): 45 (5%)
    
  By source format:
    JSDoc: 312
    Google docstring: 187
    NumPy docstring: 45
    Type hints: 523
```

#### 9.2 `acp bridge convert`

Preview or apply bridging:

```bash
# Preview what would be bridged
$ acp bridge convert --dry-run src/auth/session.py

# Convert and show cache diff
$ acp bridge convert --show-diff src/auth/

# Convert entire project
$ acp bridge convert .
```

#### 9.3 `acp bridge lint`

Check for bridging issues:

```bash
$ acp bridge lint

Warnings:
  src/auth/session.py:45: Duplicate documentation
    - @param userId in JSDoc AND @acp:param userId
    - Consider removing redundant ACP annotation
    
  src/utils/helpers.py:12: Style inconsistency
    - Detected NumPy style but file uses Google style
    - Consider standardizing or adding @acp:bridge-style
    
  src/legacy/old.py:89: Unknown docstring format
    - Could not detect format, using raw extraction
    - Add @acp:bridge-style to specify format

Errors:
  src/broken.ts:23: Type mismatch
    - JSDoc says @param {string} id
    - TypeScript says id: number
    - Using TypeScript type (inline takes precedence)
```

---

## Examples

### Example 1: Minimal ACP Addition to JSDoc

**Before (JSDoc only):**

```typescript
/**
 * Authenticate a user with email and password.
 * 
 * @param {string} email - The user's email address
 * @param {string} password - The user's password
 * @returns {Promise<AuthResult>} Authentication result with token
 * @throws {InvalidCredentialsError} If email/password don't match
 */
async function authenticate(email: string, password: string): Promise<AuthResult> {
  // ...
}
```

**After (adding ACP where it adds value):**

```typescript
/**
 * Authenticate a user with email and password.
 * 
 * @param {string} email - The user's email address
 * @param {string} password - The user's password
 * @returns {Promise<AuthResult>} Authentication result with token
 * @throws {InvalidCredentialsError} If email/password don't match
 * 
 * @acp:lock restricted - Security-critical; requires security team review
 * @acp:param password - NEVER log; hash immediately; clear from memory after use
 * @acp:throws InvalidCredentialsError - Use constant-time comparison; don't reveal which field failed
 */
async function authenticate(email: string, password: string): Promise<AuthResult> {
  // ...
}
```

The JSDoc provides the basic documentation; ACP adds AI-specific security directives.

### Example 2: Python with Type Hints + Docstring + ACP

**Input:**

```python
from typing import Optional, List
from dataclasses import dataclass

@dataclass
class QueryResult:
    """Result of a database query."""
    rows: List[dict]
    total: int
    page: int

def search_users(
    query: str,
    *,
    limit: int = 50,
    offset: int = 0,
    include_deleted: bool = False
) -> Optional[QueryResult]:
    """Search for users matching a query string.
    
    Performs a full-text search across user names and emails.
    Results are paginated and sorted by relevance.
    
    Args:
        query: Search query string. Supports wildcards (*).
        limit: Maximum results per page. Default 50, max 200.
        offset: Pagination offset.
        include_deleted: Include soft-deleted users. Default False.
        
    Returns:
        QueryResult with matching users, or None if no matches.
        
    Raises:
        QuerySyntaxError: If query contains invalid syntax.
        RateLimitError: If too many requests.
        
    Example:
        >>> results = search_users("john*", limit=10)
        >>> for row in results.rows:
        ...     print(row['email'])
    
    @acp:param query - Sanitize for SQL injection; validate length < 500
    @acp:param include_deleted - MUST check caller permissions before enabling
    @acp:returns - Results may contain PII; filter based on caller's access level
    """
```

**Extracted (all three sources merged):**

```json
{
  "name": "search_users",
  "type": "function",
  "signature": "(query: str, *, limit: int = 50, offset: int = 0, include_deleted: bool = False) -> Optional[QueryResult]",
  "summary": "Search for users matching a query string.",
  "params": [
    {
      "name": "query",
      "type": "str",
      "typeSource": "type_hint",
      "description": "Search query string. Supports wildcards (*).",
      "directive": "Sanitize for SQL injection; validate length < 500",
      "source": "merged",
      "sourceFormats": ["type_hint", "docstring:google", "acp"]
    },
    {
      "name": "limit",
      "type": "int",
      "typeSource": "type_hint",
      "default": "50",
      "description": "Maximum results per page. Default 50, max 200.",
      "source": "converted",
      "sourceFormat": "docstring:google"
    },
    {
      "name": "offset",
      "type": "int",
      "typeSource": "type_hint",
      "default": "0",
      "description": "Pagination offset.",
      "source": "converted",
      "sourceFormat": "docstring:google"
    },
    {
      "name": "include_deleted",
      "type": "bool",
      "typeSource": "type_hint",
      "default": "False",
      "description": "Include soft-deleted users. Default False.",
      "directive": "MUST check caller permissions before enabling",
      "source": "merged",
      "sourceFormats": ["type_hint", "docstring:google", "acp"]
    }
  ],
  "returns": {
    "type": "Optional[QueryResult]",
    "typeSource": "type_hint",
    "description": "QueryResult with matching users, or None if no matches.",
    "directive": "Results may contain PII; filter based on caller's access level",
    "source": "merged",
    "sourceFormats": ["type_hint", "docstring:google", "acp"]
  },
  "throws": [
    {
      "exception": "QuerySyntaxError",
      "description": "If query contains invalid syntax.",
      "source": "converted",
      "sourceFormat": "docstring:google"
    },
    {
      "exception": "RateLimitError",
      "description": "If too many requests.",
      "source": "converted",
      "sourceFormat": "docstring:google"
    }
  ]
}
```

### Example 3: Rust Doc Comments

**Input:**

```rust
/// Validate and parse a configuration file.
///
/// Reads the configuration from the specified path, validates all fields,
/// and returns a strongly-typed configuration object.
///
/// # Arguments
///
/// * `path` - Path to the configuration file (TOML format)
/// * `strict` - If true, reject unknown fields
///
/// # Returns
///
/// The parsed configuration, or an error describing what went wrong.
///
/// # Errors
///
/// * [`ConfigError::NotFound`] - File doesn't exist
/// * [`ConfigError::ParseError`] - Invalid TOML syntax
/// * [`ConfigError::ValidationError`] - Field validation failed
///
/// # Panics
///
/// Panics if the path contains invalid UTF-8.
///
/// # Examples
///
/// ```
/// let config = parse_config("config.toml", true)?;
/// println!("Loaded: {}", config.name);
/// ```
///
/// # Safety
///
/// This function reads from the filesystem. Ensure the path is trusted.
///
/// @acp:lock restricted - Configuration affects all system behavior
/// @acp:param path - Validate path is within allowed directories
pub fn parse_config(path: &Path, strict: bool) -> Result<Config, ConfigError> {
    // ...
}
```

**Extracted:**

```json
{
  "name": "parse_config",
  "type": "function",
  "file": "src/config.rs",
  "signature": "fn parse_config(path: &Path, strict: bool) -> Result<Config, ConfigError>",
  "summary": "Validate and parse a configuration file.",
  "params": [
    {
      "name": "path",
      "type": "&Path",
      "description": "Path to the configuration file (TOML format)",
      "directive": "Validate path is within allowed directories",
      "source": "merged"
    },
    {
      "name": "strict",
      "type": "bool",
      "description": "If true, reject unknown fields",
      "source": "converted",
      "sourceFormat": "rustdoc"
    }
  ],
  "returns": {
    "type": "Result<Config, ConfigError>",
    "description": "The parsed configuration, or an error describing what went wrong.",
    "source": "converted",
    "sourceFormat": "rustdoc"
  },
  "throws": [
    {
      "exception": "ConfigError::NotFound",
      "description": "File doesn't exist",
      "source": "converted"
    },
    {
      "exception": "ConfigError::ParseError",
      "description": "Invalid TOML syntax",
      "source": "converted"
    },
    {
      "exception": "ConfigError::ValidationError",
      "description": "Field validation failed",
      "source": "converted"
    }
  ],
  "panics": [
    {
      "description": "Panics if the path contains invalid UTF-8.",
      "source": "converted"
    }
  ],
  "safety": {
    "description": "This function reads from the filesystem. Ensure the path is trusted.",
    "source": "converted"
  },
  "constraints": {
    "lock_level": "restricted",
    "directive": "Configuration affects all system behavior",
    "source": "explicit"
  }
}
```

---

## Drawbacks

### 1. Parser Complexity

Supporting multiple documentation formats adds significant parser complexity:
- JSDoc/TSDoc variations
- Four Python docstring styles
- Rust doc comment conventions
- Javadoc syntax
- Go's unstructured prose

**Mitigation:** Implement parsers incrementally; start with JSDoc and Google docstrings.

### 2. Format Detection Ambiguity

Auto-detecting docstring format isn't always reliable:
- Mixed styles in same project
- Malformed documentation
- Edge cases in format detection

**Mitigation:** Allow explicit `@acp:bridge-style` override; default to conservative extraction.

### 3. Merge Conflicts

When both native and ACP documentation exist, deciding what to keep is complex:
- Semantic meaning may differ
- Users may expect different behavior
- Debugging merged output is harder

**Mitigation:** Clear precedence rules; provenance tracking; `acp bridge lint` for detection.

### 4. Maintenance Burden

Native docs and ACP can still drift if users don't understand bridging:
- Update JSDoc but forget it bridges to ACP
- Change function signature but not docstring

**Mitigation:** `acp bridge lint` warnings; clear documentation.

### 5. Performance Overhead

Parsing multiple documentation formats adds indexing time:
- Additional regex/parsing passes
- More data in cache
- Larger memory footprint

**Mitigation:** Lazy parsing; caching; incremental updates.

---

## Alternatives

### Alternative 1: No Bridging (Status Quo)

Keep ACP completely separate from native documentation systems.

**Pros:**
- Simpler implementation
- No format detection complexity
- Clear separation of concerns

**Cons:**
- Forces documentation duplication
- Higher adoption barrier
- Comment bloat

**Decision:** Rejected because adoption friction is too high.

### Alternative 2: Full Replacement

ACP replaces all native documentation systems entirely.

**Pros:**
- Single source of truth
- No bridging needed
- Consistent format

**Cons:**
- Loses IDE integration (hover, autocomplete)
- Breaks doc generators (Sphinx, TypeDoc)
- Breaks type checkers (mypy, tsc)
- Massive migration effort

**Decision:** Rejected because it's impractical and loses existing tooling value.

### Alternative 3: Decorator/Attribute-Based

Use language-native decorators instead of comments:

```python
from acp import param, lock

@lock("restricted")
@param("token", directive="Sanitize before logging")
def validate(token: str):
    """Validate a token."""
```

**Pros:**
- Language-native syntax
- Runtime accessible
- Type-checkable

**Cons:**
- Requires import (not comment-based)
- Different syntax per language
- Not compatible with ACP's comment-based design

**Decision:** Rejected because it conflicts with ACP's core design as comment-based annotations.

### Alternative 4: External Mapping File

Keep ACP annotations in a separate file that maps to source:

```yaml
# .acp.annotations.yaml
src/auth/session.ts:
  validateSession:
    lock: restricted
    params:
      token:
        directive: "Sanitize before logging"
```

**Pros:**
- No source code changes
- Single file to manage
- Easy to diff

**Cons:**
- Disconnected from code (easy to drift)
- Extra file to maintain
- Harder to review in PRs

**Decision:** Rejected because annotations should live with code.

---

## Backward Compatibility

### Existing ACP Users

- **No breaking changes**: Existing `@acp:*` annotations continue to work
- **Bridging is opt-in**: Must enable in config
- **Precedence configurable**: Choose `acp-first` to preserve current behavior

### Migration Path

1. **Phase 1**: Add config with `bridge.enabled: false` (default)
2. **Phase 2**: Run `acp bridge lint` to assess codebase
3. **Phase 3**: Enable bridging with `bridge.precedence: "acp-first"`
4. **Phase 4**: Remove redundant ACP annotations
5. **Phase 5**: Optionally switch to `merge` or `native-first`

### Schema Changes

- New fields are additive (`source`, `sourceFormat`, `typeSource`)
- Old caches remain valid
- New fields have sensible defaults

---

## Implementation

### Implementation Strategy

#### Phase 1: JSDoc Bridging (v1.3.0)

1. JSDoc parser for JavaScript/TypeScript
2. Basic precedence rules (acp-first)
3. Config schema additions
4. `acp bridge status` command

**Effort:** ~2 weeks

#### Phase 2: Python Docstring Bridging (v1.4.0)

1. Google/NumPy/Sphinx format parsers
2. Format auto-detection
3. Type hint extraction
4. `acp bridge lint` command

**Effort:** ~3 weeks

#### Phase 3: Additional Languages (v1.5.0)

1. Rust doc comment parsing
2. Javadoc parsing
3. Go doc parsing
4. `acp bridge convert` command

**Effort:** ~2 weeks

#### Phase 4: Advanced Features (v1.6.0)

1. Merge mode with intelligent conflict resolution
2. `@acp:bridge-*` control annotations
3. Performance optimizations
4. IDE extension integration

**Effort:** ~2 weeks

### Affected Components

| Component                         | Change Type         | Priority   |
|-----------------------------------|---------------------|------------|
| `cli/src/parse/`                  | Major (new parsers) | P0         |
| `cli/src/bridge/`                 | New module          | P0         |
| `schemas/v1/cache.schema.json`    | Additive            | P0         |
| `schemas/v1/config.schema.json`   | Additive            | P0         |
| `spec/chapters/05-annotations.md` | Update              | P1         |
| `spec/chapters/XX-bridging.md`    | New chapter         | P1         |
| `docs/LANGUAGE-SUPPORT.md`        | Update              | P2         |

### Testing Requirements

1. **Unit tests** for each format parser
2. **Integration tests** with real-world codebases
3. **Edge case tests** for format detection
4. **Performance benchmarks** for large codebases
5. **Regression tests** for existing ACP behavior

---

## Tooling Impact

| Tool              | Impact   | Required Changes              |
|-------------------|----------|-------------------------------|
| ACP CLI           | High     | New parsers, commands, config |
| VS Code Extension | Medium   | Display bridging provenance   |
| Language Server   | Medium   | Enhanced hover info           |
| Third-party tools | Low      | Cache format is additive      |

---

## Rollout Plan

1. **Phase 1** (Alpha): Implement behind `--experimental-bridge` flag
2. **Phase 2** (Beta): Enable by default with `bridge.enabled: false`
3. **Phase 3** (RC): Documentation and migration guides
4. **Phase 4** (Release): Full release with `bridge.enabled: true` default

---

## Open Questions

*All open questions have been resolved. See Resolved Questions below.*

---

## Resolved Questions

1. **Q**: Should bridging be enabled by default in new projects?
   **A**: No. Bridging should be disabled by default (`bridge.enabled: false`). The `acp init` command should prompt the user and explain what enabling bridging will do before they opt in.

2. **Q**: How should we handle malformed documentation?
   **A**: Configurable strictness via `bridge.strictness`. Options: `"permissive"` (default, best-effort extraction) or `"strict"` (skip and warn). Users can choose based on their documentation quality.

3. **Q**: Should we support custom documentation formats?
   **A**: Defer to RFC-0007 (ACP Complete Documentation Solution). Custom formats add significant complexity and are out of scope for the initial bridging implementation.

4. **Q**: How deep should type extraction go?
   **A**: Start simple, expand later. Phase 1-3 will extract simple types (`str`, `int`, `List[str]`, etc.). Phase 4 will add support for complex types (`Callable`, `Awaitable`, nested generics).

5. **Q**: Should we parse runtime type annotations (Pydantic, attrs)?
   **A**: No, out of scope. Focus on documentation-level information.

6. **Q**: Should bridged annotations count toward "coverage" metrics?
   **A**: Yes, but track separately (`convertedCount` vs `explicitCount`).

7. **Q**: What happens if native docs are in a different language (i18n)?
   **A**: Extract as-is; translation is out of scope.

---

## Performance Benchmark Expectations

### Target Metrics

| Metric | Target | Rationale |
|--------|--------|-----------|
| **Indexing overhead** | < 15% increase | Typical for adding parsing pass |
| **Per-file parsing** | < 5ms average | Regex-based, not AST |
| **Memory overhead** | < 20% increase | Additional strings in cache |
| **Cache size increase** | < 30% | Adding provenance fields |

### Per-Format Targets

| Format | Target Parse Time | Notes |
|--------|-------------------|-------|
| JSDoc | < 3ms per doc block | Existing converter |
| Google docstring | < 3ms per doc block | Existing converter |
| NumPy docstring | < 4ms per doc block | More complex format |
| Format detection | < 1ms per file | Heuristic regex |

### Benchmark Test Matrix

1. **Codebase Size Tests** (compare baseline vs +bridging):
   - Small project (< 100 files)
   - Medium project (100-1000 files)
   - Large project (1000+ files)

2. **Regression Tests** (ensure no slowdown):
   - `acp index` without bridging
   - `acp annotate`

### Performance Safeguards

1. **Lazy parsing**: Parse doc blocks on demand
2. **Format caching**: Remember detected format per-file
3. **Incremental updates**: Only re-parse changed files
4. **Early exit**: Skip files without doc blocks

---

## References

- [JSDoc Documentation](https://jsdoc.app/)
- [TSDoc Specification](https://tsdoc.org/)
- [Google Python Style Guide](https://google.github.io/styleguide/pyguide.html#38-comments-and-docstrings)
- [NumPy Docstring Standard](https://numpydoc.readthedocs.io/en/latest/format.html)
- [Sphinx reStructuredText Primer](https://www.sphinx-doc.org/en/master/usage/restructuredtext/basics.html)
- [PEP 484 - Type Hints](https://peps.python.org/pep-0484/)
- [PEP 257 - Docstring Conventions](https://peps.python.org/pep-0257/)
- [Rust Documentation Comments](https://doc.rust-lang.org/rustdoc/how-to-write-documentation.html)
- [Javadoc Tool](https://docs.oracle.com/javase/8/docs/technotes/tools/windows/javadoc.html)
- RFC-0001: Self-Documenting Annotations
- RFC-0003: Annotation Provenance Tracking

---

## Appendix

### A. Complete Tag Mapping Reference

#### A.1 JSDoc → ACP

| JSDoc Tag                | ACP Equivalent           | Notes                     |
|--------------------------|--------------------------|---------------------------|
| `@param {T} name - desc` | `@acp:param name - desc` | Type extracted separately |
| `@returns {T} desc`      | `@acp:returns - desc`    | Type extracted separately |
| `@throws {T} desc`       | `@acp:throws T - desc`   |                           |
| `@deprecated msg`        | `@acp:deprecated - msg`  |                           |
| `@example code`          | `@acp:example - code`    |                           |
| `@see ref`               | `@acp:ref ref`           | Auto-generate directive   |
| `@todo task`             | `@acp:todo task`         | Auto-generate directive   |
| `@description text`      | `@acp:fn - text`         |                           |
| `@author name`           | `@acp:owner name`        |                           |
| `@version ver`           | *(metadata only)*        | Store in cache            |
| `@since ver`             | *(metadata only)*        | Store in cache            |
| `@type {T}`              | *(skip)*                 | Type only, no directive   |
| `@typedef`               | *(skip)*                 | Type only                 |
| `@template T`            | *(skip)*                 | Type only                 |
| `@callback`              | *(skip)*                 | Type only                 |
| `@this {T}`              | *(skip)*                 | Type only                 |
| `@access public/private` | *(metadata only)*        | Store visibility          |
| `@readonly`              | *(metadata only)*        | Store as flag             |
| `@abstract`              | *(metadata only)*        | Store as flag             |
| `@override`              | *(metadata only)*        | Store as flag             |

#### A.2 Python Docstring → ACP

| Docstring Section                        | ACP Equivalent    | Styles               |
|------------------------------------------|-------------------|----------------------|
| First paragraph                          | `@acp:fn`         | All                  |
| `Args:` / `Parameters` / `:param:`       | `@acp:param`      | Google/NumPy/Sphinx  |
| `Returns:` / `:returns:`                 | `@acp:returns`    | All                  |
| `Raises:` / `:raises:`                   | `@acp:throws`     | All                  |
| `Yields:` / `:yields:`                   | `@acp:returns`    | All (for generators) |
| `Example:` / `Examples`                  | `@acp:example`    | Google/NumPy         |
| `See Also:` / `.. seealso::`             | `@acp:ref`        | NumPy/Sphinx         |
| `Note:` / `Notes` / `.. note::`          | *(informational)* | All                  |
| `Warning:` / `Warnings` / `.. warning::` | `@acp:critical`   | All                  |
| `.. deprecated::`                        | `@acp:deprecated` | Sphinx               |
| `.. todo::`                              | `@acp:todo`       | Sphinx               |
| `Attributes`                             | *(skip)*          | NumPy                |
| `References`                             | *(skip)*          | NumPy                |

#### A.3 Rust → ACP

| Rust Section    | ACP Equivalent                          |
|-----------------|-----------------------------------------|
| First paragraph | `@acp:fn`                               |
| `# Arguments`   | `@acp:param` (each bullet)              |
| `# Returns`     | `@acp:returns`                          |
| `# Errors`      | `@acp:throws`                           |
| `# Panics`      | `@acp:throws` (with exception: "panic") |
| `# Examples`    | `@acp:example`                          |
| `# Safety`      | `@acp:critical`                         |
| `# See also`    | `@acp:ref`                              |

### B. Format Detection Heuristics

#### B.1 Python Docstring Style Detection

```python
def detect_docstring_style(docstring: str) -> str:
    """
    Detect the docstring style based on content patterns.
    
    Returns: "google" | "numpy" | "sphinx" | "epytext" | "unknown"
    """
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

### C. Performance Considerations

1. **Lazy Parsing**: Only parse documentation blocks when needed
2. **Caching**: Cache parsed results per file hash
3. **Incremental Updates**: Only re-parse changed files
4. **Parallel Processing**: Parse multiple files concurrently
5. **Memory Limits**: Stream large files instead of loading entirely

### D. Security Considerations

1. **Path Traversal**: Validate all file paths in bridge configuration
2. **Code Injection**: Don't execute any code from documentation
3. **Resource Limits**: Limit regex complexity to prevent ReDoS
4. **Sensitive Data**: Don't log documentation content that may contain secrets

---

## Changelog

| Date       | Change        |
|------------|---------------|
| 2025-12-22 | Initial draft |
| 2025-12-23 | Resolved all open questions; added performance benchmarks; status changed to Accepted |

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
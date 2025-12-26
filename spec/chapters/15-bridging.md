# Chapter 15: Documentation System Bridging

## 15.1 Overview

Documentation System Bridging enables ACP to leverage existing documentation from native documentation systems such as JSDoc, Python docstrings, Rust doc comments, and Javadoc. This chapter specifies how bridging works, configuration options, precedence rules, and provenance tracking.

### 15.1.1 Core Principle

**ACP should leverage what exists, then layer on AI-specific guidance.**

Instead of requiring developers to write documentation twice (once in native format, once in ACP), bridging automatically extracts documentation from existing sources and merges it with ACP annotations.

### 15.1.2 Supported Documentation Systems

| Language       | Documentation System | Styles Supported          |
|----------------|----------------------|---------------------------|
| JavaScript/TypeScript | JSDoc/TSDoc     | Standard JSDoc            |
| Python         | Docstrings           | Google, NumPy, Sphinx     |
| Rust           | Doc comments         | Rustdoc conventions       |
| Java/Kotlin    | Javadoc              | Standard Javadoc          |
| Go             | Doc comments         | Godoc conventions         |

## 15.2 Configuration

Bridging is configured in `.acp.config.json` under the `bridge` section:

```json
{
  "bridge": {
    "enabled": false,
    "precedence": "acp-first",
    "strictness": "permissive",
    "jsdoc": {
      "enabled": true,
      "extractTypes": true,
      "convertTags": ["param", "returns", "throws", "deprecated", "example", "see"]
    },
    "python": {
      "enabled": true,
      "docstringStyle": "auto",
      "extractTypeHints": true,
      "convertSections": ["Args", "Parameters", "Returns", "Raises", "Example", "Yields"]
    },
    "rust": {
      "enabled": true,
      "convertSections": ["Arguments", "Returns", "Panics", "Errors", "Examples", "Safety"]
    },
    "provenance": {
      "markConverted": true,
      "includeSourceFormat": true
    }
  }
}
```

### 15.2.1 Top-Level Options

| Option | Type | Default | Description |
|--------|------|---------|-------------|
| `enabled` | boolean | `false` | Enable documentation bridging |
| `precedence` | string | `"acp-first"` | Precedence mode when both exist |
| `strictness` | string | `"permissive"` | How to handle malformed docs |

### 15.2.2 Precedence Modes

- **`acp-first`**: ACP annotations take precedence. Native docs fill gaps.
- **`native-first`**: Native docs are authoritative. ACP adds directives only.
- **`merge`**: Intelligently combine both sources.

### 15.2.3 Strictness Modes

- **`permissive`**: Best-effort extraction; skip malformed documentation
- **`strict`**: Reject and warn on malformed documentation

## 15.3 Precedence Rules

When both native documentation and ACP annotations exist for the same concept, the following rules apply:

### 15.3.1 Description Resolution

| Scenario | `acp-first` | `native-first` | `merge` |
|----------|-------------|----------------|---------|
| Both have description | Use native | Use native | Use native |
| Only ACP exists | Use ACP | Use ACP | Use ACP |
| Only native exists | Use native | Use native | Use native |

### 15.3.2 Directive Resolution

| Scenario | `acp-first` | `native-first` | `merge` |
|----------|-------------|----------------|---------|
| Both have directive | Use ACP | Use ACP | Concatenate |
| Only ACP exists | Use ACP | Use ACP | Use ACP |
| Only native exists | Use native | Use native | Use native |

### 15.3.3 Type Resolution

When types are available from multiple sources:

1. Inline type annotations (TypeScript, Python type hints) take priority
2. Documentation types (JSDoc `@param {Type}`, Sphinx `:type:`) are secondary
3. Inferred types are lowest priority

## 15.4 Supported Formats

### 15.4.1 JSDoc Tag Mapping

| JSDoc Tag | ACP Equivalent | Notes |
|-----------|----------------|-------|
| `@param {T} name - desc` | `@acp:param name - desc` | Type extracted separately |
| `@returns {T} desc` | `@acp:returns - desc` | Type extracted separately |
| `@throws {T} desc` | `@acp:throws T - desc` | |
| `@deprecated msg` | `@acp:deprecated - msg` | |
| `@example code` | `@acp:example - code` | |
| `@see ref` | `@acp:ref ref` | |
| First line | `@acp:fn` / `@acp:summary` | Function description |

### 15.4.2 Python Docstring Section Mapping

| Docstring Section | ACP Equivalent | Styles |
|-------------------|----------------|--------|
| `Args:` | `@acp:param` | Google |
| `Parameters` | `@acp:param` | NumPy |
| `:param name:` | `@acp:param name` | Sphinx |
| `Returns:` | `@acp:returns` | Google/NumPy |
| `:returns:` | `@acp:returns` | Sphinx |
| `Raises:` | `@acp:throws` | Google/NumPy |
| `:raises Exception:` | `@acp:throws Exception` | Sphinx |
| `Example:` | `@acp:example` | All |
| First paragraph | `@acp:fn` / `@acp:summary` | All |

### 15.4.3 Rust Doc Section Mapping

| Rust Section | ACP Equivalent |
|--------------|----------------|
| First paragraph | `@acp:fn` / `@acp:summary` |
| `# Arguments` | `@acp:param` (each bullet) |
| `# Returns` | `@acp:returns` |
| `# Errors` | `@acp:throws` |
| `# Panics` | `@acp:throws` (exception: "panic") |
| `# Examples` | `@acp:example` |
| `# Safety` | `@acp:critical` |

## 15.5 Format Detection

### 15.5.1 Python Docstring Style Detection

The style is auto-detected based on content patterns:

```
NumPy:   Section headers with underlines (Parameters\n----------)
Sphinx:  :param:, :returns:, :raises: tags
Google:  Args:, Returns:, Raises: sections
```

Detection priority:
1. Explicit `@acp:bridge-style` annotation
2. Configuration `docstringStyle` setting
3. Auto-detection heuristics

### 15.5.2 Detection Heuristics

```python
# NumPy: Section headers with underlines
if matches(r'\n\s*Parameters\s*\n\s*-{3,}', docstring):
    return "numpy"

# Sphinx: :param:, :returns:, :raises: tags
if matches(r':(param|returns?|raises?|type)\s+', docstring):
    return "sphinx"

# Google: Args:, Returns:, Raises: sections
if matches(r'\n\s*(Args|Returns|Raises|Yields|Examples?):', docstring):
    return "google"
```

## 15.6 Provenance Tracking

Bridged annotations are tracked with provenance information per RFC-0003.

### 15.6.1 Source Types

| Source | Description |
|--------|-------------|
| `explicit` | Pure ACP annotation (human-written) |
| `converted` | Converted from native documentation |
| `merged` | Combined from native + ACP |
| `heuristic` | Auto-generated through inference |

### 15.6.2 Source Formats

The `sourceFormat` field indicates the original documentation system:

- `jsdoc` - JSDoc/TSDoc
- `docstring:google` - Google-style Python docstring
- `docstring:numpy` - NumPy-style Python docstring
- `docstring:sphinx` - Sphinx/reST-style Python docstring
- `rustdoc` - Rust doc comments
- `javadoc` - Javadoc comments
- `acp` - Pure ACP annotation

### 15.6.3 Cache Schema

Parameter entries include provenance:

```json
{
  "name": "userId",
  "type": "string",
  "typeSource": "type_hint",
  "description": "The user's unique identifier",
  "directive": "MUST validate UUID format before query",
  "source": "merged",
  "sourceFormats": ["jsdoc", "acp"]
}
```

## 15.7 Bridge Control Annotations

Fine-grained control over bridging behavior:

### 15.7.1 `@acp:bridge` - Enable/Disable Bridging

```typescript
// Enable bridging for this file
// @acp:bridge enabled

// Disable bridging (use only explicit @acp: annotations)
// @acp:bridge disabled

// Enable specific format
// @acp:bridge jsdoc
```

### 15.7.2 `@acp:bridge-style` - Specify Docstring Style

```python
"""
@acp:bridge-style google - Parse as Google-style docstring
"""
```

### 15.7.3 `@acp:bridge-skip` - Skip Specific Tags

```typescript
/**
 * @param internal - Implementation detail
 * @acp:bridge-skip param:internal
 */
```

### 15.7.4 `@acp:bridge-only` - Convert Only Specified Tags

```typescript
/**
 * @acp:bridge-only returns,throws
 * @param hidden - Won't be converted
 * @returns {User} - Will be converted
 */
```

## 15.8 CLI Commands

### 15.8.1 Index with Bridging

```bash
# Enable bridging during indexing
acp index --bridge

# Disable bridging (override config)
acp index --no-bridge
```

### 15.8.2 Bridge Status

```bash
# Show bridging configuration and statistics
acp bridge status

# Output:
# Configuration:
#   Precedence: acp-first
#   JSDoc: enabled
#   Python: enabled (style: auto-detect)
#
# Statistics:
#   Total annotations: 847
#     Explicit (ACP only): 234 (28%)
#     Converted (from native): 412 (49%)
#     Merged (ACP + native): 156 (18%)
```

### 15.8.3 Bridge Status JSON Output

```bash
acp bridge status --json
```

Returns:
```json
{
  "enabled": true,
  "precedence": "acp-first",
  "summary": {
    "totalAnnotations": 847,
    "explicitCount": 234,
    "convertedCount": 412,
    "mergedCount": 156
  },
  "byFormat": {
    "jsdoc": 312,
    "docstring:google": 187,
    "docstring:numpy": 45
  }
}
```

## 15.9 Examples

### 15.9.1 JSDoc with Minimal ACP Enhancement

**Before (bloated):**
```typescript
/**
 * @param {string} userId - User identifier
 * @acp:param userId - User identifier  // DUPLICATE
 */
```

**After (with bridging):**
```typescript
/**
 * @param {string} userId - User identifier
 * @acp:param userId - MUST validate UUID format
 */
```

The description comes from JSDoc; ACP adds only the directive.

### 15.9.2 Python with Type Hints + Docstring + ACP

```python
def search_users(
    query: str,
    limit: int = 50
) -> Optional[QueryResult]:
    """Search for users matching a query string.

    Args:
        query: Search query string. Supports wildcards.
        limit: Maximum results per page.

    Returns:
        QueryResult with matching users, or None if no matches.

    @acp:param query - Sanitize for SQL injection
    @acp:returns - Results may contain PII; filter by access level
    """
```

**Extracted cache entry:**
```json
{
  "name": "search_users",
  "params": [
    {
      "name": "query",
      "type": "str",
      "typeSource": "type_hint",
      "description": "Search query string. Supports wildcards.",
      "directive": "Sanitize for SQL injection",
      "source": "merged",
      "sourceFormats": ["type_hint", "docstring:google", "acp"]
    },
    {
      "name": "limit",
      "type": "int",
      "typeSource": "type_hint",
      "default": "50",
      "description": "Maximum results per page.",
      "source": "converted",
      "sourceFormat": "docstring:google"
    }
  ]
}
```

## 15.10 Performance Considerations

### 15.10.1 Benchmark Targets

| Metric | Target |
|--------|--------|
| Indexing overhead | < 15% increase |
| Per-file parsing | < 5ms average |
| Memory overhead | < 20% increase |
| Cache size increase | < 30% |

### 15.10.2 Optimization Strategies

1. **Lazy parsing**: Parse doc blocks on demand
2. **Format caching**: Remember detected format per-file
3. **Incremental updates**: Only re-parse changed files
4. **Early exit**: Skip files without doc blocks

## 15.11 Security Considerations

1. **Path traversal**: Validate all file paths in bridge configuration
2. **Code injection**: Never execute code from documentation
3. **Resource limits**: Limit regex complexity to prevent ReDoS
4. **Sensitive data**: Don't log documentation that may contain secrets

## 15.12 Backward Compatibility

- Bridging is **opt-in** (disabled by default)
- Existing `@acp:*` annotations continue to work
- Schema changes are additive
- Old caches remain valid

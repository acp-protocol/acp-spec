# RFC-0009: Extended Annotation Types

- **RFC ID**: 0009
- **Title**: Extended Annotation Types
- **Author**: David (ACP Protocol)
- **Status**: Implemented
- **Created**: 2025-12-23
- **Updated**: 2025-12-25
- **Discussion**: [Pending GitHub Discussion]
- **Parent**: RFC-0007 (ACP Complete Documentation Solution)
- **Related**: RFC-0001, RFC-0008, RFC-0010 (annotations rendered in generated documentation)

---

## Summary

This RFC extends the ACP annotation vocabulary with 30+ new annotation types covering:

1. **File-level annotations**: `@acp:purpose`, `@acp:owner`, `@acp:layer`, `@acp:stability`
2. **Symbol-level annotations**: `@acp:class`, `@acp:interface`, `@acp:type`, `@acp:enum`
3. **Behavioral annotations**: `@acp:pure`, `@acp:idempotent`, `@acp:memoized`, `@acp:async`
4. **Lifecycle annotations**: `@acp:experimental`, `@acp:beta`, `@acp:internal`
5. **Documentation annotations**: `@acp:example`, `@acp:see`, `@acp:link`, `@acp:note`

These enable ACP to fully document any codebase without relying on native documentation systems.

These annotations are stored in the cache and rendered by `acp docs` (RFC-0010) as badges, sections, metadata tables, and highlighted callouts.

---

## Motivation

### Problem Statement

Current ACP focuses primarily on AI guidance (`@acp:lock`, `@acp:critical`, `@acp:ai-hint`). For complete documentation, users need:

- File/module-level metadata
- Class and interface documentation
- Behavioral characteristics (pure, async, etc.)
- Lifecycle status (deprecated, experimental)
- Cross-references and examples

### Goals

1. Define complete annotation vocabulary for standalone documentation
2. Cover all documentation needs expressible in JSDoc/docstrings
3. Maintain consistency with existing ACP patterns
4. Enable parity with native documentation systems

### Non-Goals

1. Replacing existing constraint annotations
2. Changing annotation syntax
3. Requiring new annotations (all optional)

---

## Detailed Design

### 1. File-Level Annotations

| Annotation       | Purpose            | Example                                     |
|------------------|--------------------|---------------------------------------------|
| `@acp:purpose`   | File purpose       | `@acp:purpose - Authentication utilities`   |
| `@acp:module`    | Module name        | `@acp:module "AuthService"`                 |
| `@acp:domain`    | Business domain    | `@acp:domain authentication`                |
| `@acp:owner`     | Team ownership     | `@acp:owner "Security Team"`                |
| `@acp:layer`     | Architecture layer | `@acp:layer service`                        |
| `@acp:stability` | API stability      | `@acp:stability stable`                     |
| `@acp:version`   | Version info       | `@acp:version "2.1.0"`                      |
| `@acp:since`     | Introduced version | `@acp:since "1.5.0"`                        |
| `@acp:license`   | License            | `@acp:license "MIT"`                        |
| `@acp:author`    | Author             | `@acp:author "Jane Doe <jane@example.com>"` |

**Example:**
```typescript
/**
 * @acp:module "UserService" - Core user management functionality
 * @acp:domain users - User management domain
 * @acp:layer service - Business logic layer
 * @acp:owner "Platform Team" - Contact for questions
 * @acp:stability stable - Production ready API
 */
```

### 2. Symbol-Level Annotations

| Annotation       | Purpose                | Example                                        |
|------------------|------------------------|------------------------------------------------|
| `@acp:fn`        | Function description   | `@acp:fn - Validates user credentials`         |
| `@acp:class`     | Class description      | `@acp:class - Manages user sessions`           |
| `@acp:method`    | Method description     | `@acp:method - Refreshes the session token`    |
| `@acp:interface` | Interface description  | `@acp:interface - Contract for auth providers` |
| `@acp:type`      | Type alias description | `@acp:type - User identifier type`             |
| `@acp:enum`      | Enum description       | `@acp:enum - Possible user roles`              |
| `@acp:const`     | Constant description   | `@acp:const - Maximum retry attempts`          |
| `@acp:var`       | Variable description   | `@acp:var - Current session instance`          |
| `@acp:property`  | Property description   | `@acp:property {string} - User's display name` |

### 3. Signature Annotations

| Annotation       | Purpose          | Example                                           |
|------------------|------------------|---------------------------------------------------|
| `@acp:param`     | Parameter        | `@acp:param {string} id - User ID`                |
| `@acp:returns`   | Return value     | `@acp:returns {User} - The user object`           |
| `@acp:throws`    | Exception        | `@acp:throws {NotFoundError} - When user missing` |
| `@acp:yields`    | Generator yield  | `@acp:yields {Item} - Each item in sequence`      |
| `@acp:async`     | Async marker     | `@acp:async - This function is asynchronous`      |
| `@acp:generator` | Generator marker | `@acp:generator - This is a generator function`   |
| `@acp:template`  | Type parameter   | `@acp:template T - Element type`                  |

### 4. Behavioral Annotations

| Annotation           | Purpose        | Example                                           |
|----------------------|----------------|---------------------------------------------------|
| `@acp:pure`          | Pure function  | `@acp:pure - No side effects`                     |
| `@acp:idempotent`    | Idempotent     | `@acp:idempotent - Safe to retry`                 |
| `@acp:memoized`      | Cached         | `@acp:memoized - Results are cached`              |
| `@acp:throttled`     | Rate limited   | `@acp:throttled "100/min" - Rate limit applies`   |
| `@acp:transactional` | DB transaction | `@acp:transactional - Runs in transaction`        |
| `@acp:side-effects`  | Has effects    | `@acp:side-effects "db,network" - Modifies state` |

**Example:**
```python
def get_user_by_id(user_id: str) -> User:
    """
    @acp:fn - Retrieves user from cache or database
    @acp:param {string} user_id - The user's unique identifier
    @acp:returns {User} - The user object
    @acp:memoized - Results cached for 5 minutes
    @acp:idempotent - Safe to call multiple times
    @acp:throws {NotFoundError} - User doesn't exist
    """
    pass
```

### 5. Lifecycle Annotations

| Annotation          | Purpose       | Example                                     |
|---------------------|---------------|---------------------------------------------|
| `@acp:deprecated`   | Deprecation   | `@acp:deprecated "2.0" - Use newFn instead` |
| `@acp:experimental` | Unstable      | `@acp:experimental - API may change`        |
| `@acp:beta`         | Beta feature  | `@acp:beta - Feature in beta testing`       |
| `@acp:internal`     | Internal only | `@acp:internal - Not for external use`      |
| `@acp:public-api`   | Public API    | `@acp:public-api - Stable public interface` |

### 6. Documentation Annotations

| Annotation     | Purpose      | Example                                     |
|----------------|--------------|---------------------------------------------|
| `@acp:example` | Code example | `@acp:example - const result = fn()`        |
| `@acp:see`     | Reference    | `@acp:see OtherClass - Related class`       |
| `@acp:link`    | URL link     | `@acp:link "https://..." - External docs`   |
| `@acp:note`    | Note         | `@acp:note - Important consideration`       |
| `@acp:warning` | Warning      | `@acp:warning - Security sensitive`         |
| `@acp:todo`    | Todo item    | `@acp:todo "Add validation" - Pending work` |

### 7. Performance Annotations

| Annotation    | Purpose          | Example                                      |
|---------------|------------------|----------------------------------------------|
| `@acp:perf`   | Performance note | `@acp:perf "O(n)" - Linear time complexity`  |
| `@acp:memory` | Memory usage     | `@acp:memory "O(1)" - Constant space`        |
| `@acp:cached` | Caching info     | `@acp:cached "5min" - Result cached 5 mins`  |

---

## Cache Schema Extensions

Add new annotation types to cache:

```json
{
  "symbol_entry": {
    "annotations": {
      "type": "object",
      "properties": {
        "behavioral": {
          "type": "object",
          "properties": {
            "pure": { "type": "boolean" },
            "idempotent": { "type": "boolean" },
            "memoized": { "type": "boolean" },
            "async": { "type": "boolean" },
            "generator": { "type": "boolean" },
            "sideEffects": { "type": "array", "items": { "type": "string" } }
          }
        },
        "lifecycle": {
          "type": "object",
          "properties": {
            "deprecated": { "type": "string" },
            "experimental": { "type": "boolean" },
            "internal": { "type": "boolean" },
            "since": { "type": "string" }
          }
        },
        "documentation": {
          "type": "object",
          "properties": {
            "examples": { "type": "array", "items": { "type": "string" } },
            "seeAlso": { "type": "array", "items": { "type": "string" } },
            "links": { "type": "array", "items": { "type": "string" } },
            "notes": { "type": "array", "items": { "type": "string" } },
            "warnings": { "type": "array", "items": { "type": "string" } }
          }
        },
        "performance": {
          "type": "object",
          "properties": {
            "complexity": { "type": "string" },
            "memory": { "type": "string" },
            "cached": { "type": "string" }
          }
        }
      }
    }
  }
}
```

---

## Examples

### Complete Class Documentation

```python
"""
@acp:module "DataProcessor" - Data transformation pipeline
@acp:domain data-pipeline - ETL domain
@acp:layer service - Processing layer
@acp:stability stable - Production ready
"""

class DataProcessor:
    """
    @acp:class - Processes and transforms data batches
    @acp:template T - Input data type
    @acp:template U - Output data type

    @acp:property {Callable[[T], U]} transformer - Transformation function
    @acp:property {int} batch_size - Items per batch; MUST be positive

    @acp:example -
        processor = DataProcessor(
            transformer=lambda x: x.upper(),
            batch_size=100
        )
        results = processor.process(["a", "b", "c"])
    """

    def process(self, items: List[T]) -> List[U]:
        """
        @acp:method - Process all items through the transformer
        @acp:param {List[T]} items - Items to process; may be empty
        @acp:returns {List[U]} - Transformed items in same order
        @acp:perf "O(n)" - Linear time complexity
        @acp:pure - No side effects; safe to parallelize
        """
        pass
```

---

## Documentation Rendering

Extended annotations from RFC-0009 are rendered by RFC-0010 templates as visual elements:

| Annotation Category                                | Rendering                |
|----------------------------------------------------|--------------------------|
| Behavioral (`@acp:pure`, `@acp:idempotent`)        | Colored badges in header |
| Lifecycle (`@acp:deprecated`, `@acp:experimental`) | Warning callouts         |
| Documentation (`@acp:example`, `@acp:see`)         | Dedicated sections       |
| Performance (`@acp:perf`, `@acp:cached`)           | Metadata table rows      |

Example template:

```jinja
<article class="symbol">
  <header>
    <h2>{{ sym.name }}</h2>
    
    {# Behavioral badges #}
    {% if sym.annotations.behavioral.pure %}
      <span class="badge badge-pure">Pure</span>
    {% endif %}
    {% if sym.annotations.behavioral.idempotent %}
      <span class="badge badge-idempotent">Idempotent</span>
    {% endif %}
    {% if sym.annotations.behavioral.memoized %}
      <span class="badge badge-memoized">Memoized</span>
    {% endif %}
    {% if sym.annotations.behavioral.async %}
      <span class="badge badge-async">Async</span>
    {% endif %}
  </header>
  
  {# Lifecycle warnings #}
  {% if sym.annotations.lifecycle.deprecated %}
    <aside class="callout callout-warning">
      <strong>⚠️ Deprecated:</strong> {{ sym.annotations.lifecycle.deprecated }}
    </aside>
  {% endif %}
  {% if sym.annotations.lifecycle.experimental %}
    <aside class="callout callout-info">
      <strong>🧪 Experimental:</strong> This API may change without notice.
    </aside>
  {% endif %}
  {% if sym.annotations.lifecycle.internal %}
    <aside class="callout callout-caution">
      <strong>🔒 Internal:</strong> Not intended for external use.
    </aside>
  {% endif %}
  
  {# Performance metadata #}
  {% if sym.annotations.performance %}
    {% set perf = sym.annotations.performance %}
    <dl class="performance-meta">
      {% if perf.complexity %}
        <dt>Time Complexity</dt><dd><code>{{ perf.complexity }}</code></dd>
      {% endif %}
      {% if perf.memory %}
        <dt>Space Complexity</dt><dd><code>{{ perf.memory }}</code></dd>
      {% endif %}
      {% if perf.cached %}
        <dt>Cache Duration</dt><dd>{{ perf.cached }}</dd>
      {% endif %}
    </dl>
  {% endif %}
  
  {# Examples section #}
  {% if sym.annotations.documentation.examples %}
    <section class="examples">
      <h3>Examples</h3>
      {% for example in sym.annotations.documentation.examples %}
        <pre><code>{{ example }}</code></pre>
      {% endfor %}
    </section>
  {% endif %}
  
  {# See also links #}
  {% if sym.annotations.documentation.seeAlso %}
    <section class="see-also">
      <h3>See Also</h3>
      <ul>
      {% for ref in sym.annotations.documentation.seeAlso %}
        <li><a href="#{{ ref }}">{{ ref }}</a></li>
      {% endfor %}
      </ul>
    </section>
  {% endif %}
  
  {# Notes and warnings #}
  {% for note in sym.annotations.documentation.notes %}
    <aside class="callout callout-note">
      <strong>📝 Note:</strong> {{ note }}
    </aside>
  {% endfor %}
  {% for warning in sym.annotations.documentation.warnings %}
    <aside class="callout callout-warning">
      <strong>⚠️ Warning:</strong> {{ warning }}
    </aside>
  {% endfor %}
</article>
```

---

## Drawbacks

1. **Annotation proliferation**: 30+ new annotations to learn
   - *Mitigation*: IDE completions guide usage; all annotations are optional

2. **Overlap with existing**: Some overlap with `@acp:summary`
   - *Mitigation*: Clear guidelines on when to use each

3. **Cache size increase**: More annotation types means larger cache files
   - *Mitigation*: Only store annotations that are present; sparse representation

---

## Implementation

### Phase 1: Schema Design (3 days)

1. Add `BehavioralAnnotations`, `LifecycleAnnotations`, `DocumentationAnnotations`, `PerformanceAnnotations` to cache.schema.json
2. Extend `symbol_entry` with behavioral, lifecycle, documentation, performance fields
3. Extend `file_entry` with version, since, license, author fields
4. Schema validation tests

### Phase 2: Parser Extension (4 days)

1. Add regex patterns for all 30+ annotation types
2. Parse file-level: `@acp:purpose`, `@acp:module`, `@acp:version`, `@acp:since`, `@acp:license`, `@acp:author`
3. Parse symbol-level: `@acp:fn`, `@acp:class`, `@acp:method`, `@acp:interface`, `@acp:type`, `@acp:enum`
4. Parse behavioral: `@acp:pure`, `@acp:idempotent`, `@acp:memoized`, `@acp:throttled`, `@acp:transactional`
5. Parse lifecycle: `@acp:deprecated`, `@acp:experimental`, `@acp:beta`, `@acp:internal`
6. Parse documentation: `@acp:example`, `@acp:see`, `@acp:link`, `@acp:note`, `@acp:warning`, `@acp:todo`
7. Parse performance: `@acp:perf`, `@acp:memory`, `@acp:cached`
8. Parser unit tests

### Phase 3: Cache Integration (3 days)

1. Define Rust types for new annotation categories
2. Update `FileEntry` and `SymbolEntry` structs
3. Indexer integration for annotation extraction
4. JSON serialization tests

### Phase 4: Specification Documentation (2 days)

1. Update Chapter 05 (Annotations) with Sections 7.4-7.9
2. Update Chapter 03 (Cache Format) with new fields
3. Update Appendix A (Annotation Reference)

### Phase 5: Testing (2 days)

1. Parser unit tests (~30 tests)
2. Integration tests for full file parsing
3. Regression tests

**Total Effort**: ~14 days

See `.claude/memory/rfc-plan-0009.md` and `.claude/memory/rfc-tasks-0009.md` for detailed breakdown.

---

## Changelog

| Date         | Change                                                                                    |
|--------------|-------------------------------------------------------------------------------------------|
| 2025-12-23   | Split from RFC-0007; initial draft                                                        |
| 2025-12-24   | Added RFC-0010 relationship; added Documentation Rendering section with template examples |
| 2025-12-24   | Refined implementation plan with 5 phases, 34 tasks; created plan and tasks files         |

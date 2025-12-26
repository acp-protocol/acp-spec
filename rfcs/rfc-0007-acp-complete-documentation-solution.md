# RFC-0007: ACP as Complete Documentation Solution

- **RFC ID**: 0007
- **Title**: ACP as Complete Documentation Solution
- **Author**: David (ACP Protocol)
- **Status**: Draft
- **Created**: 2025-12-22
- **Updated**: 2025-12-23
- **Discussion**: [Pending GitHub Discussion]
- **Depends On**: RFC-0006 (Documentation System Bridging)
- **Related**: RFC-0001, RFC-0003
- **Child RFCs**: RFC-0008, RFC-0009, RFC-0010, RFC-0011

---

## Summary

This RFC serves as the **umbrella document** for extending ACP to be a complete, standalone documentation solution. The original proposal has been split into four focused RFCs for easier implementation and review:

| RFC                                               | Title                     | Scope                      | Effort   |
|---------------------------------------------------|---------------------------|----------------------------|----------|
| [RFC-0008](rfc-0008-acp-type-annotations.md)      | ACP Type Annotations      | Type syntax in annotations | ~4 weeks |
| [RFC-0009](rfc-0009-extended-annotation-types.md) | Extended Annotation Types | 30+ new annotation types   | ~2 weeks |
| [RFC-0010](rfc-0010-documentation-generator.md)   | Documentation Generator   | `acp docs` command         | ~5 weeks |
| [RFC-0011](rfc-0011-ide-lsp-integration.md)       | IDE and LSP Integration   | Language Server Protocol   | ~6 weeks |

**Total Estimated Effort**: ~17 weeks

The core principle remains: **If you're going to document, document once with ACP.**

---

## Motivation

### Problem Statement

RFC-0006 addresses bridging for users with existing documentation. But what about:

1. **Greenfield projects**: No existing JSDoc/docstrings to bridge
2. **Polyglot codebases**: Different doc standards per language is confusing
3. **AI-first development**: Want documentation optimized for AI from the start
4. **Simplicity seekers**: Don't want to learn JSDoc AND ACP AND type hints

These users face a choice:
- **Use native docs + ACP**: Duplication and bridging complexity
- **Use ACP only**: Missing type information and tooling

### Value Proposition

After implementing all four child RFCs, ACP will provide:

1. **ACP Type Annotations** (RFC-0008): Optional `{type}` syntax within `@acp:param` and `@acp:returns`
2. **Extended Annotation Types** (RFC-0009): Complete annotation coverage for all documentation needs
3. **Documentation Generator** (RFC-0010): `acp docs` command generates HTML/Markdown documentation
4. **IDE Integration** (RFC-0011): LSP protocol for hover, autocomplete, and diagnostics

### Gap Analysis (Current State)

| Need                  | JSDoc       | Python Docstring  | ACP (Current)        | ACP (After RFC-0007) |
|-----------------------|-------------|-------------------|----------------------|----------------------|
| Parameter description | Y           | Y                 | Y                    | Y                    |
| Return description    | Y           | Y                 | Y                    | Y                    |
| Exception docs        | Y           | Y                 | Y                    | Y                    |
| Type annotations      | Y `{Type}`  | Partial `:type:`  | No                   | Y (RFC-0008)         |
| IDE hover             | Y Native    | Y Native          | No                   | Y (RFC-0011)         |
| Doc generation        | Y JSDoc CLI | Y Sphinx/mkdocs   | No                   | Y (RFC-0010)         |
| Autocomplete hints    | Y Native    | Y Pylance         | No                   | Y (RFC-0011)         |
| Deprecation warnings  | Y IDE shows | Y IDE shows       | No                   | Y (RFC-0011)         |
| Behavioral markers    | Partial     | No                | No                   | Y (RFC-0009)         |
| Lifecycle annotations | Y           | Partial           | No                   | Y (RFC-0009)         |

---

## Child RFC Overview

### RFC-0008: ACP Type Annotations

**Scope**: Add optional type syntax within ACP annotations.

**Key Features**:
- Type syntax: `@acp:param {string} userId - directive`
- Full grammar: primitives, generics, unions, intersections, conditional types
- Language-agnostic type mapping table
- Optional lint via `acp lint --check-types`

**Example**:
```typescript
// @acp:param {string | null} name - Name or null if anonymous
// @acp:param {Array<User>} users - List of users to process
// @acp:returns {Promise<Result<User, Error>>} - Async result type
// @acp:template T extends Serializable - Must be serializable
```

### RFC-0009: Extended Annotation Types

**Scope**: Define 30+ new annotation types for complete documentation.

**Categories**:
- **File-level**: `@acp:purpose`, `@acp:owner`, `@acp:layer`, `@acp:stability`
- **Symbol-level**: `@acp:class`, `@acp:interface`, `@acp:type`, `@acp:enum`
- **Behavioral**: `@acp:pure`, `@acp:idempotent`, `@acp:memoized`, `@acp:async`
- **Lifecycle**: `@acp:deprecated`, `@acp:experimental`, `@acp:internal`
- **Documentation**: `@acp:example`, `@acp:see`, `@acp:link`, `@acp:note`

**Example**:
```python
"""
@acp:module "DataProcessor" - Data transformation pipeline
@acp:domain data-pipeline - ETL domain
@acp:layer service - Processing layer
@acp:stability stable - Production ready
"""
```

### RFC-0010: Documentation Generator

**Scope**: `acp docs` command for generating documentation.

**Key Features**:
- Output formats: HTML, Markdown, JSON
- Full plugin API for custom themes and formats
- Built-in themes: default, minimal, api
- Search index generation
- Watch mode for development

**Example**:
```bash
$ acp docs --output ./docs --format html --theme default
```

### RFC-0011: IDE and LSP Integration

**Scope**: Language Server Protocol specification and VS Code extension.

**Capabilities**:
- Hover documentation
- Auto-completion for annotations
- Diagnostics (errors, warnings)
- Go-to-definition for cross-references
- Semantic tokens for syntax highlighting
- Code actions for quick fixes

**Example hover**:
```
validateToken(token: string, options?: ValidateOptions)
 Promise<Session | null>

Validates a JWT token and returns the associated session.

 RESTRICTED - Security-critical; changes require review

Parameters:
  token: The JWT token to validate
          MUST sanitize before logging
```

---

## Implementation Order

### Recommended Sequence

1. **RFC-0006** (Prerequisite) - Documentation System Bridging
   - Must be implemented first; RFC-0007 children depend on it
   - Status: Accepted

2. **RFC-0008** - Type Annotations
   - Can start after RFC-0006
   - Foundation for type-aware features

3. **RFC-0009** - Extended Annotations
   - Can be implemented in parallel with RFC-0008
   - Quick win; mostly schema and validation additions

4. **RFC-0010** - Documentation Generator
   - Depends on RFC-0008 and RFC-0009
   - Consumes type and annotation data

5. **RFC-0011** - IDE/LSP Integration
   - Can start in parallel with RFC-0010
   - Most complex; can be phased independently

### Dependency Graph

```
RFC-0006 (Bridging)
    |
    v
RFC-0008 (Types) ----+----> RFC-0010 (Doc Gen)
    |                |
    v                |
RFC-0009 (Annots) ---+
    |
    +----------------> RFC-0011 (IDE/LSP)
```

---

## Resolved Open Questions

The original RFC-0007 had four open questions. These have been resolved:

| #   | Question                                      | Resolution                                                                                     | Child RFC  |
|-----|-----------------------------------------------|------------------------------------------------------------------------------------------------|------------|
| 1   | Should ACP types support generic constraints? | **Yes - Full support** including conditional types like `T extends Array<infer U> ? U : never` | RFC-0008   |
| 2   | Should doc generator support plugins?         | **Yes - Full plugin API** for custom themes, output formats, and rendering hooks               | RFC-0010   |
| 3   | How detailed should LSP spec be?              | **Full protocol specification** with reference VS Code extension                               | RFC-0011   |
| 4   | Should types be validated against source?     | **Optional lint rule** via `acp lint --check-types` (opt-in)                                   | RFC-0008   |

---

## Goals

1. **Type syntax in ACP**: `@acp:param {string} userId - directive`
2. **Complete parity**: Everything expressible in JSDoc should be expressible in ACP
3. **Doc generation**: `acp docs` command generates HTML/Markdown documentation
4. **IDE protocol**: Define how IDEs should present ACP information
5. **Standalone viability**: ACP alone is sufficient for fully documented code
6. **Backward compatible**: Type syntax is optional; existing annotations work

## Non-Goals

1. **Runtime type checking**: ACP doesn't validate types at runtime
2. **Static type analysis**: ACP doesn't replace TypeScript/mypy
3. **Full IDE replacement**: IDEs still use native type systems; ACP supplements
4. **Enforced adoption**: Type syntax is entirely optional

---

## Drawbacks

### 1. Scope Complexity

Even split into four RFCs, this is a substantial undertaking (~17 weeks).

**Mitigation**: Each RFC can be accepted/rejected independently. Partial implementation provides value.

### 2. Duplication with Language Type Systems

ACP types duplicate information available in TypeScript/Python type hints.

**Mitigation**: Types are optional; RFC-0006 bridging extracts types automatically for existing codebases.

### 3. Tooling Investment

Requires IDE extensions, doc generators, LSP server.

**Mitigation**: Core functionality works without IDE integration. Doc generator is single CLI tool.

### 4. Maintenance Overhead

More annotations and tools to maintain.

**Mitigation**: `acp annotate` auto-generates baseline; `acp lint` catches drift.

---

## Alternatives Considered

### Alternative 1: JSDoc/Sphinx as Canon

Make JSDoc/Sphinx the primary format; ACP only adds directives.

**Rejected**: Doesn't serve greenfield or polyglot use cases.

### Alternative 2: Type-Only Mode

Only extract types from native systems; don't include type syntax in ACP.

**Rejected**: Standalone users need complete solution without native type systems.

### Alternative 3: Keep as Single RFC

Implement all features in one monolithic RFC.

**Rejected**: Split allows faster iteration, independent acceptance/rejection, and clearer scope per RFC.

---

## Backward Compatibility

- All existing `@acp:*` annotations continue to work
- Type syntax `{T}` is optional everywhere
- New annotations don't affect existing ones
- Schema changes are additive
- Old caches remain valid

---

## Success Criteria

RFC-0007 is considered complete when:

1. [ ] RFC-0008 implemented: Type syntax in annotations
2. [ ] RFC-0009 implemented: Extended annotation types
3. [ ] RFC-0010 implemented: Documentation generator
4. [ ] RFC-0011 implemented: IDE/LSP integration
5. [ ] All child RFCs marked as Implemented

---

## References

- [RFC-0006: Documentation System Bridging](rfc-0006-documentation-system-bridging.md)
- [RFC-0008: ACP Type Annotations](rfc-0008-acp-type-annotations.md)
- [RFC-0009: Extended Annotation Types](rfc-0009-extended-annotation-types.md)
- [RFC-0010: Documentation Generator](rfc-0010-documentation-generator.md)
- [RFC-0011: IDE and LSP Integration](rfc-0011-ide-lsp-integration.md)
- [TypeDoc](https://typedoc.org/)
- [Sphinx](https://www.sphinx-doc.org/)
- [JSDoc](https://jsdoc.app/)
- [Language Server Protocol](https://microsoft.github.io/language-server-protocol/)

---

## Changelog

| Date       | Change                                                              |
|------------|---------------------------------------------------------------------|
| 2025-12-22 | Initial draft                                                       |
| 2025-12-23 | Split into 4 child RFCs (0008-0011); converted to umbrella document |

---

## Appendix: Complete Annotation Reference

After all child RFCs are implemented, ACP will support:

```
FILE LEVEL
  @acp:purpose    File purpose description
  @acp:module     Module name
  @acp:domain     Business domain
  @acp:owner      Team ownership
  @acp:layer      Architecture layer
  @acp:stability  API stability level

SYMBOL LEVEL
  @acp:fn         Function description
  @acp:class      Class description
  @acp:method     Method description
  @acp:interface  Interface description
  @acp:type       Type alias description
  @acp:const      Constant description

SIGNATURE
  @acp:param      {Type} name - Parameter (directive)
  @acp:returns    {Type} - Return value (directive)
  @acp:throws     {Exception} - When thrown (directive)
  @acp:yields     {Type} - Generator yield
  @acp:template   T - Type parameter

BEHAVIOR
  @acp:pure       No side effects
  @acp:async      Asynchronous function
  @acp:generator  Generator function
  @acp:idempotent Safe to retry
  @acp:memoized   Results cached

LIFECYCLE
  @acp:deprecated Version - Migration note
  @acp:experimental API may change
  @acp:internal   Not for external use
  @acp:since      Version introduced

CONSTRAINTS (existing)
  @acp:lock       frozen|restricted|normal
  @acp:critical   Security/business critical
  @acp:perf       Performance considerations

DOCUMENTATION
  @acp:example    Code example
  @acp:see        Cross-reference
  @acp:link       External URL
  @acp:note       Additional note
  @acp:warning    Important warning
  @acp:todo       Pending work
```

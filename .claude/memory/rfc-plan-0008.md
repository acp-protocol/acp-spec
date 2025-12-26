# Implementation Plan: RFC-0008 ACP Type Annotations

## Overview

RFC-0008 introduces optional type syntax within ACP annotations, enabling developers to document parameter and return types directly in `@acp:param` and `@acp:returns` annotations.

**Core syntax**: `@acp:param {Type} name - directive`

### Goals
1. Add optional `{type}` syntax to parameter and return annotations
2. Support simple types, generics, unions, and constraints
3. Store parsed types in cache schema
4. Maintain backward compatibility (types are optional)

### Non-Goals
1. Runtime type checking
2. Static type analysis (not replacing TypeScript/mypy)
3. Enforced type syntax

---

## Components Affected

| Component | Files | Changes |
|-----------|-------|---------|
| **Specification** | `spec/chapters/05-annotations.md` | Add type syntax documentation |
| | `spec/chapters/03-cache-format.md` | Document type_info field |
| **Schemas** | `schemas/v1/cache.schema.json` | Add type_info to symbol_entry |
| **CLI Parser** | `src/parse/mod.rs` | Type expression parser |
| | `src/parse/types.rs` | New file: type AST and parser |
| **CLI Cache** | `src/cache/types.rs` | TypeInfo struct |
| **CLI Indexer** | `src/index/indexer.rs` | Extract types during indexing |
| **Tests** | `tests/` | Type parsing tests |

---

## Dependencies

### Internal Dependencies
- **Schema before CLI**: Cache schema must be updated before CLI implementation
- **Parser before Indexer**: Type parser must exist before indexer integration

### External Dependencies
- None (standalone feature)

### Blocking Issues
- None identified

---

## Phase 1: Foundation (Specification & Schema)

**Purpose**: Establish the contract before implementation

### T1.1: Update Annotation Specification
**Files**: `spec/chapters/05-annotations.md`
**Estimated Time**: 2 hours

**Description**:
- Add Section 5.X: Type Annotations
- Document `{type}` syntax for @acp:param and @acp:returns
- Include EBNF grammar from RFC
- Add type mapping table (ACP -> TypeScript/Python/Rust/etc.)
- Document optional/default syntax: `[name]`, `[name=default]`

**Acceptance Criteria**:
- [ ] Type syntax documented with examples
- [ ] Grammar specification included
- [ ] Language mapping table complete

### T1.2: Update Cache Format Specification
**Files**: `spec/chapters/03-cache-format.md`
**Estimated Time**: 1 hour

**Description**:
- Document `type_info` field in symbol_entry
- Document `params[].type`, `returns.type` fields
- Document `typeSource` enum values
- Add example JSON showing type information

**Acceptance Criteria**:
- [ ] type_info structure documented
- [ ] Example JSON included
- [ ] Cross-references to Chapter 5 added

### T1.3: Update Cache Schema
**Files**: `schemas/v1/cache.schema.json`
**Estimated Time**: 1 hour

**Description**:
Add to symbol_entry $defs:

```json
"type_info": {
  "type": "object",
  "properties": {
    "params": {
      "type": "array",
      "items": {
        "type": "object",
        "properties": {
          "name": { "type": "string" },
          "type": { "type": "string" },
          "typeSource": { "enum": ["acp", "inferred", "native"] },
          "optional": { "type": "boolean" },
          "default": { "type": "string" }
        },
        "required": ["name"]
      }
    },
    "returns": {
      "type": "object",
      "properties": {
        "type": { "type": "string" },
        "typeSource": { "enum": ["acp", "inferred", "native"] }
      }
    },
    "typeParams": {
      "type": "array",
      "items": {
        "type": "object",
        "properties": {
          "name": { "type": "string" },
          "constraint": { "type": "string" }
        },
        "required": ["name"]
      }
    }
  }
}
```

**Acceptance Criteria**:
- [ ] type_info definition added to schema
- [ ] Schema validates successfully
- [ ] Test fixtures created

**Gate**: Spec and schemas reviewed and stable

---

## Phase 2: Implementation (Parser & Cache Types)

**Purpose**: Build the type parsing functionality

### T2.1: Create Type Parser Module
**Files**: `src/parse/types.rs` (new file)
**Estimated Time**: 6 hours

**Description**:
Create a recursive descent parser for type expressions:

```rust
pub struct TypeParser;

pub enum TypeExpression {
    Primitive(String),           // string, number, boolean, etc.
    Literal(LiteralType),        // "foo", 42, true
    Array(Box<TypeExpression>),  // T[]
    Tuple(Vec<TypeExpression>),  // [T, U]
    Union(Vec<TypeExpression>),  // T | U
    Intersection(Vec<TypeExpression>), // T & U
    Object(Vec<Property>),       // { name: string, age: number }
    Function { params: Vec<Param>, returns: Box<TypeExpression> },
    Generic { name: String, args: Vec<TypeExpression> },
    Reference(String),           // User, MyType
    Optional(Box<TypeExpression>), // T?
    Conditional { check: Box<TypeExpression>, extends: Box<TypeExpression>,
                  true_type: Box<TypeExpression>, false_type: Box<TypeExpression> },
}

impl TypeParser {
    pub fn parse(input: &str) -> Result<TypeExpression, TypeParseError>;
}
```

**Acceptance Criteria**:
- [ ] Parse primitive types: string, number, boolean, null, void, any, never, unknown
- [ ] Parse array types: `T[]`, `Array<T>`
- [ ] Parse union types: `T | U | V`
- [ ] Parse generic types: `Promise<T>`, `Map<K, V>`
- [ ] Parse object types: `{ name: string, age?: number }`
- [ ] Parse function types: `(x: number) => string`
- [ ] Parse tuple types: `[string, number]`
- [ ] Handle nested types: `Promise<Array<User | null>>`
- [ ] Error on malformed types

### T2.2: Extend Annotation Parser
**Files**: `src/parse/mod.rs`
**Estimated Time**: 3 hours

**Description**:
Modify `ANNOTATION_PATTERN` regex to capture type specs:

```rust
// New pattern: @acp:param {Type} name - directive
static PARAM_PATTERN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"@acp:param\s+(?:\{([^}]+)\})?\s*(\[)?(\w+)\]?(?:=([^\s-]+))?\s*(?:-\s+(.+))?$").unwrap()
});

static RETURNS_PATTERN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"@acp:returns?\s+(?:\{([^}]+)\})?\s*(?:-\s+(.+))?$").unwrap()
});

static TEMPLATE_PATTERN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"@acp:template\s+(\w+)(?:\s+extends\s+(.+?))?(?:\s+-\s+(.+))?$").unwrap()
});
```

Add parsing for:
- `@acp:param {Type} name - directive`
- `@acp:param {Type} [name] - optional param`
- `@acp:param {Type} [name=default] - param with default`
- `@acp:returns {Type} - directive`
- `@acp:template T extends Constraint - directive`

**Acceptance Criteria**:
- [ ] @acp:param with type parsed correctly
- [ ] @acp:returns with type parsed correctly
- [ ] @acp:template parsed correctly
- [ ] Optional/default syntax works
- [ ] Backward compatible (type is optional)

### T2.3: Add Cache Type Structures
**Files**: `src/cache/types.rs`
**Estimated Time**: 2 hours

**Description**:
Add types for storing parsed type information:

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypeInfo {
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub params: Vec<ParamInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub returns: Option<ReturnInfo>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub type_params: Vec<TypeParam>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParamInfo {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_expr: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_source: Option<TypeSource>,
    #[serde(skip_serializing_if = "is_false", default)]
    pub optional: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub directive: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReturnInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_expr: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_source: Option<TypeSource>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub directive: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypeParam {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub constraint: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub directive: Option<String>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TypeSource {
    Acp,      // From @acp:param {Type}
    Inferred, // From source code analysis
    Native,   // From JSDoc/docstring bridging
}
```

Add `type_info: Option<TypeInfo>` to `SymbolEntry`.

**Acceptance Criteria**:
- [ ] TypeInfo struct defined
- [ ] ParamInfo, ReturnInfo, TypeParam structs defined
- [ ] TypeSource enum defined
- [ ] SymbolEntry updated with type_info field
- [ ] Sparse serialization (skip empty)

### T2.4: Integrate with SymbolBuilder
**Files**: `src/parse/mod.rs`
**Estimated Time**: 2 hours

**Description**:
Update `SymbolBuilder` to accumulate type information:

```rust
struct SymbolBuilder {
    // ... existing fields ...
    type_info: TypeInfo,
}

// In annotation parsing loop:
"param" => {
    if let Some(cap) = PARAM_PATTERN.captures(line) {
        let type_expr = cap.get(1).map(|m| m.as_str().to_string());
        let optional = cap.get(2).is_some();
        let name = cap.get(3).unwrap().as_str().to_string();
        let default = cap.get(4).map(|m| m.as_str().to_string());
        let directive = cap.get(5).map(|m| m.as_str().to_string());

        builder.type_info.params.push(ParamInfo {
            name,
            type_expr,
            type_source: type_expr.as_ref().map(|_| TypeSource::Acp),
            optional,
            default,
            directive,
        });
    }
}
```

**Acceptance Criteria**:
- [ ] SymbolBuilder accumulates params
- [ ] SymbolBuilder accumulates returns
- [ ] SymbolBuilder accumulates type_params
- [ ] TypeInfo populated in built SymbolEntry

**Gate**: Implementation complete, compiles clean

---

## Phase 3: Validation (Testing)

**Purpose**: Ensure correctness

### T3.1: Unit Tests for Type Parser
**Files**: `src/parse/types.rs` (tests module)
**Estimated Time**: 3 hours

**Description**:
Comprehensive tests for type expression parsing:

```rust
#[cfg(test)]
mod tests {
    #[test] fn test_parse_primitive() { ... }
    #[test] fn test_parse_array() { ... }
    #[test] fn test_parse_union() { ... }
    #[test] fn test_parse_generic() { ... }
    #[test] fn test_parse_object() { ... }
    #[test] fn test_parse_function() { ... }
    #[test] fn test_parse_tuple() { ... }
    #[test] fn test_parse_nested() { ... }
    #[test] fn test_parse_optional() { ... }
    #[test] fn test_parse_error() { ... }
}
```

**Acceptance Criteria**:
- [ ] All primitive types tested
- [ ] All complex types tested
- [ ] Nested types tested
- [ ] Error cases tested
- [ ] 90%+ code coverage for type parser

### T3.2: Unit Tests for Annotation Parsing
**Files**: `src/parse/mod.rs` (tests module)
**Estimated Time**: 2 hours

**Description**:
Tests for @acp:param, @acp:returns, @acp:template parsing:

```rust
#[test] fn test_param_with_type() { ... }
#[test] fn test_param_optional() { ... }
#[test] fn test_param_with_default() { ... }
#[test] fn test_returns_with_type() { ... }
#[test] fn test_template_with_constraint() { ... }
#[test] fn test_backward_compat_no_type() { ... }
```

**Acceptance Criteria**:
- [ ] All annotation patterns tested
- [ ] Optional/default syntax tested
- [ ] Backward compatibility verified
- [ ] Edge cases covered

### T3.3: Integration Tests
**Files**: `tests/type_annotation_tests.rs` (new file)
**Estimated Time**: 2 hours

**Description**:
End-to-end tests: parse file -> check cache has type_info:

```rust
#[test]
fn test_index_file_with_types() {
    // Create test file with typed annotations
    // Run indexer
    // Verify cache contains type_info
}
```

**Acceptance Criteria**:
- [ ] Full indexing workflow tested
- [ ] Cache contains correct type_info
- [ ] Schema validation passes

### T3.4: Schema Validation Tests
**Files**: `tests/fixtures/schemas/cache/valid/with-types.json`
**Estimated Time**: 1 hour

**Description**:
Add test fixtures for cache with type_info:

**Acceptance Criteria**:
- [ ] Valid fixture with type_info validates
- [ ] Invalid fixtures rejected correctly

**Gate**: All tests pass, coverage meets threshold

---

## Phase 4: Documentation (Docs)

**Purpose**: Enable users

### T4.1: Update CLI README
**Files**: `README.md`
**Estimated Time**: 1 hour

**Description**:
- Add section on type annotations
- Document @acp:param, @acp:returns, @acp:template syntax
- Add examples

**Acceptance Criteria**:
- [ ] Type syntax documented
- [ ] Examples provided

### T4.2: Update RFC Status
**Files**: `rfcs/rfc-0008-acp-type-annotations.md`, `rfcs/README.md`
**Estimated Time**: 30 minutes

**Description**:
- Change status from Draft to Implemented
- Add implementation notes
- Update rfcs/README.md index

**Acceptance Criteria**:
- [ ] RFC status updated
- [ ] Implementation date recorded

**Gate**: Documentation complete and reviewed

---

## Phase 5: Release (Integration)

### T5.1: Version Bump and Changelog
**Files**: `CHANGELOG.md`
**Estimated Time**: 30 minutes

**Description**:
- Add changelog entry for RFC-0008
- Version bump (0.7.0)

**Acceptance Criteria**:
- [ ] Changelog updated
- [ ] Version consistent across files

---

## Effort Estimation Summary

| Phase | Tasks | Estimated Time |
|-------|-------|----------------|
| Phase 1: Foundation | 3 | 4 hours |
| Phase 2: Implementation | 4 | 13 hours |
| Phase 3: Validation | 4 | 8 hours |
| Phase 4: Documentation | 2 | 1.5 hours |
| Phase 5: Release | 1 | 0.5 hours |
| **Total** | **14** | **~27 hours** |

---

## Risk Mitigation

| Risk | Likelihood | Impact | Mitigation |
|------|------------|--------|------------|
| Type parser complexity | Medium | Medium | Start with core types, add advanced features iteratively |
| Regex pattern conflicts | Low | Medium | Thorough testing of existing patterns |
| Performance impact | Low | Low | Lazy parsing, only parse when type syntax detected |
| Breaking changes | Low | High | Types are optional, backward compatible |

---

## Success Criteria

1. [ ] `@acp:param {string} name - directive` parses correctly
2. [ ] `@acp:returns {Promise<User>} - directive` parses correctly
3. [ ] `@acp:template T extends BaseType - directive` parses correctly
4. [ ] Cache contains `type_info` with correct structure
5. [ ] All existing tests still pass
6. [ ] New type-related tests pass
7. [ ] Schema validates cache with type_info
8. [ ] Documentation updated

---

## Ready for Implementation

Start with Phase 1: Foundation
- T1.1: Update spec/chapters/05-annotations.md
- T1.2: Update spec/chapters/03-cache-format.md
- T1.3: Update schemas/v1/cache.schema.json

No blockers identified. Estimated total: ~27 hours.

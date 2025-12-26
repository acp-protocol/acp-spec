# RFC-0008 Task Breakdown

## Task Index

| Task ID | Phase | Description | Depends On | Est. Time | Status |
|---------|-------|-------------|------------|-----------|--------|
| T1.1 | 1 | Update annotation spec with type syntax | - | 2h | Pending |
| T1.2 | 1 | Update cache format spec | T1.1 | 1h | Pending |
| T1.3 | 1 | Update cache.schema.json | T1.1 | 1h | Pending |
| T2.1 | 2 | Create type parser module | T1.3 | 6h | Pending |
| T2.2 | 2 | Extend annotation parser | T2.1 | 3h | Pending |
| T2.3 | 2 | Add cache type structures | T1.3 | 2h | Pending |
| T2.4 | 2 | Integrate with SymbolBuilder | T2.2, T2.3 | 2h | Pending |
| T3.1 | 3 | Unit tests for type parser | T2.1 | 3h | Pending |
| T3.2 | 3 | Unit tests for annotation parsing | T2.2 | 2h | Pending |
| T3.3 | 3 | Integration tests | T2.4 | 2h | Pending |
| T3.4 | 3 | Schema validation tests | T1.3 | 1h | Pending |
| T4.1 | 4 | Update CLI README | T2.4 | 1h | Pending |
| T4.2 | 4 | Update RFC status | T3.3 | 0.5h | Pending |
| T5.1 | 5 | Version bump and changelog | T4.2 | 0.5h | Pending |

---

## Phase 1: Foundation

### T1.1: Update Annotation Specification

**Phase**: 1
**Component**: spec
**Files**: `spec/chapters/05-annotations.md`
**Depends On**: -
**Estimated Time**: 2 hours

**Description**:
Add new section documenting type annotation syntax. Include:
1. EBNF grammar for type expressions
2. Syntax for @acp:param, @acp:returns, @acp:template
3. Type mapping table (ACP -> TypeScript/Python/Rust/Go/Java)
4. Examples for all type forms

**Content to add**:
```markdown
## 5.X Type Annotations

### Syntax
@acp:param {Type} name - directive
@acp:param {Type} [name] - optional parameter
@acp:param {Type} [name=default] - parameter with default
@acp:returns {Type} - directive
@acp:template T extends Constraint - type parameter

### Supported Types
- Primitives: string, number, boolean, null, undefined, void, any, never, unknown
- Arrays: T[], Array<T>
- Unions: T | U
- Generics: Promise<T>, Map<K, V>
- Objects: { name: string, age?: number }
- Functions: (x: number) => string
- Tuples: [string, number]
```

**Acceptance Criteria**:
- [ ] Section 5.X added with complete syntax documentation
- [ ] EBNF grammar included
- [ ] Type mapping table complete (5+ languages)
- [ ] 10+ examples covering all type forms
- [ ] Cross-reference to cache format chapter

---

### T1.2: Update Cache Format Specification

**Phase**: 1
**Component**: spec
**Files**: `spec/chapters/03-cache-format.md`
**Depends On**: T1.1
**Estimated Time**: 1 hour

**Description**:
Document the type_info field in symbol_entry. Add:
1. Field description in symbol_entry table
2. Nested structure documentation (params, returns, typeParams)
3. TypeSource enum documentation
4. Example JSON with type_info populated

**Content to add**:
```markdown
### type_info (optional)
Type information extracted from @acp:param, @acp:returns, @acp:template annotations.

| Field | Type | Description |
|-------|------|-------------|
| params | ParamInfo[] | Parameter type information |
| returns | ReturnInfo | Return type information |
| typeParams | TypeParam[] | Generic type parameters |

#### ParamInfo
| Field | Type | Description |
|-------|------|-------------|
| name | string | Parameter name |
| type | string | Type expression |
| typeSource | "acp" \| "inferred" \| "native" | Source of type |
| optional | boolean | Whether optional |
| default | string | Default value if any |
```

**Acceptance Criteria**:
- [ ] type_info documented in symbol_entry section
- [ ] All nested structures documented
- [ ] TypeSource enum values explained
- [ ] Example JSON included

---

### T1.3: Update Cache Schema

**Phase**: 1
**Component**: schemas
**Files**: `schemas/v1/cache.schema.json`
**Depends On**: T1.1
**Estimated Time**: 1 hour

**Description**:
Add type_info definition to cache schema $defs and reference it in symbol_entry.

**Changes**:
1. Add `type_info` to $defs
2. Add `param_info` to $defs
3. Add `return_info` to $defs
4. Add `type_param` to $defs
5. Add `type_source` enum to $defs
6. Reference type_info in symbol_entry properties

**Acceptance Criteria**:
- [ ] type_info schema definition complete
- [ ] All nested types defined
- [ ] symbol_entry references type_info
- [ ] Schema validates with ajv/jsonschema
- [ ] Test fixture created

---

## Phase 2: Implementation

### T2.1: Create Type Parser Module

**Phase**: 2
**Component**: cli
**Files**: `src/parse/types.rs` (new)
**Depends On**: T1.3
**Estimated Time**: 6 hours

**Description**:
Implement a recursive descent parser for type expressions.

**Key types**:
```rust
pub enum TypeExpression {
    Primitive(PrimitiveType),
    Literal(LiteralValue),
    Array(Box<TypeExpression>),
    Tuple(Vec<TypeExpression>),
    Union(Vec<TypeExpression>),
    Intersection(Vec<TypeExpression>),
    Object(Vec<ObjectProperty>),
    Function { params: Vec<FunctionParam>, returns: Box<TypeExpression> },
    Generic { name: String, args: Vec<TypeExpression> },
    Reference(String),
    Optional(Box<TypeExpression>),
    Conditional { ... },
}

pub enum PrimitiveType {
    String, Number, Boolean, Null, Undefined, Void, Any, Never, Unknown
}

pub struct TypeParser<'a> {
    input: &'a str,
    pos: usize,
}

impl<'a> TypeParser<'a> {
    pub fn parse(input: &str) -> Result<TypeExpression, TypeParseError>;
}
```

**Implementation approach**:
1. Tokenizer: split into tokens (identifiers, operators, brackets)
2. Parser: recursive descent with precedence for union/intersection
3. Error recovery: return partial parse with error location

**Acceptance Criteria**:
- [ ] Parse all primitive types
- [ ] Parse array types (T[] and Array<T>)
- [ ] Parse union types (T | U)
- [ ] Parse intersection types (T & U)
- [ ] Parse generic types (Promise<T>)
- [ ] Parse object types ({ key: Type })
- [ ] Parse function types ((x: T) => U)
- [ ] Parse tuple types ([T, U])
- [ ] Parse nested combinations
- [ ] Handle malformed input gracefully

---

### T2.2: Extend Annotation Parser

**Phase**: 2
**Component**: cli
**Files**: `src/parse/mod.rs`
**Depends On**: T2.1
**Estimated Time**: 3 hours

**Description**:
Add regex patterns and parsing logic for typed annotations.

**New patterns**:
```rust
// @acp:param {Type} name - directive
// @acp:param {Type} [name] - optional
// @acp:param {Type} [name=default] - with default
static PARAM_PATTERN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"@acp:param\s+(?:\{([^}]+)\})?\s*(\[)?(\w+)\]?(?:=([^\s-]+))?\s*(?:-\s+(.+))?$").unwrap()
});

// @acp:returns {Type} - directive
static RETURNS_PATTERN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"@acp:returns?\s+(?:\{([^}]+)\})?\s*(?:-\s+(.+))?$").unwrap()
});

// @acp:template T extends Constraint - directive
static TEMPLATE_PATTERN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"@acp:template\s+(\w+)(?:\s+extends\s+(.+?))?(?:\s+-\s+(.+))?$").unwrap()
});
```

**Changes to parse_annotations()**:
- Add matching for param, returns, template patterns
- Extract type expression string
- Pass to TypeParser for validation (optional, can defer)
- Store in annotation result

**Acceptance Criteria**:
- [ ] PARAM_PATTERN matches all variants
- [ ] RETURNS_PATTERN matches correctly
- [ ] TEMPLATE_PATTERN matches correctly
- [ ] Backward compatible (no type = no break)
- [ ] Type string extracted correctly

---

### T2.3: Add Cache Type Structures

**Phase**: 2
**Component**: cli
**Files**: `src/cache/types.rs`
**Depends On**: T1.3
**Estimated Time**: 2 hours

**Description**:
Add Rust structs matching the schema definition.

**New types**:
```rust
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TypeInfo {
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub params: Vec<ParamInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub returns: Option<ReturnInfo>,
    #[serde(rename = "typeParams", skip_serializing_if = "Vec::is_empty", default)]
    pub type_params: Vec<TypeParam>,
}

impl TypeInfo {
    pub fn is_empty(&self) -> bool {
        self.params.is_empty() && self.returns.is_none() && self.type_params.is_empty()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParamInfo {
    pub name: String,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_expr: Option<String>,
    #[serde(rename = "typeSource", skip_serializing_if = "Option::is_none")]
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
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_expr: Option<String>,
    #[serde(rename = "typeSource", skip_serializing_if = "Option::is_none")]
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

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum TypeSource {
    #[default]
    Acp,
    Inferred,
    Native,
}
```

**Update SymbolEntry**:
```rust
pub struct SymbolEntry {
    // ... existing fields ...

    /// RFC-0008: Type information from annotations
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_info: Option<TypeInfo>,
}
```

**Acceptance Criteria**:
- [ ] All structs defined
- [ ] Serialization matches schema
- [ ] Sparse serialization (skip empty)
- [ ] SymbolEntry updated

---

### T2.4: Integrate with SymbolBuilder

**Phase**: 2
**Component**: cli
**Files**: `src/parse/mod.rs`
**Depends On**: T2.2, T2.3
**Estimated Time**: 2 hours

**Description**:
Update SymbolBuilder to accumulate type information.

**Changes**:
1. Add `type_info: TypeInfo` to SymbolBuilder
2. In annotation loop, match "param", "returns", "template"
3. Parse and store type information
4. In build(), include type_info in SymbolEntry

**Code outline**:
```rust
struct SymbolBuilder {
    // ... existing ...
    type_info: TypeInfo,
}

// In parse loop:
"param" => {
    if let Some(cap) = PARAM_PATTERN.captures(line) {
        builder.type_info.params.push(ParamInfo {
            name: cap.get(3).unwrap().as_str().to_string(),
            type_expr: cap.get(1).map(|m| m.as_str().to_string()),
            type_source: cap.get(1).map(|_| TypeSource::Acp),
            optional: cap.get(2).is_some(),
            default: cap.get(4).map(|m| m.as_str().to_string()),
            directive: cap.get(5).map(|m| m.as_str().to_string()),
        });
    }
}
```

**Acceptance Criteria**:
- [ ] SymbolBuilder has type_info field
- [ ] @acp:param populates params
- [ ] @acp:returns populates returns
- [ ] @acp:template populates type_params
- [ ] build() includes type_info in SymbolEntry

---

## Phase 3: Validation

### T3.1: Unit Tests for Type Parser

**Phase**: 3
**Component**: cli
**Files**: `src/parse/types.rs`
**Depends On**: T2.1
**Estimated Time**: 3 hours

**Test cases**:
```rust
#[test] fn test_primitive_string() { assert_eq!(parse("string"), Primitive(String)); }
#[test] fn test_primitive_number() { ... }
#[test] fn test_primitive_boolean() { ... }
#[test] fn test_primitive_void() { ... }
#[test] fn test_primitive_any() { ... }
#[test] fn test_array_simple() { assert_eq!(parse("string[]"), Array(Primitive(String))); }
#[test] fn test_array_generic() { assert_eq!(parse("Array<number>"), Array(Primitive(Number))); }
#[test] fn test_union_two() { assert_eq!(parse("string | null"), Union([Primitive(String), Primitive(Null)])); }
#[test] fn test_union_three() { ... }
#[test] fn test_generic_promise() { assert_eq!(parse("Promise<User>"), Generic { name: "Promise", args: [Reference("User")] }); }
#[test] fn test_generic_map() { assert_eq!(parse("Map<string, number>"), ...); }
#[test] fn test_object_simple() { assert_eq!(parse("{ name: string }"), Object([...])); }
#[test] fn test_object_optional() { assert_eq!(parse("{ name?: string }"), ...); }
#[test] fn test_function_simple() { assert_eq!(parse("(x: number) => string"), Function { ... }); }
#[test] fn test_tuple() { assert_eq!(parse("[string, number]"), Tuple([...])); }
#[test] fn test_nested_complex() { assert_eq!(parse("Promise<Array<User | null>>"), ...); }
#[test] fn test_error_unclosed_brace() { assert!(parse("{ name: string").is_err()); }
#[test] fn test_error_invalid_token() { assert!(parse("!!!").is_err()); }
```

**Acceptance Criteria**:
- [ ] All primitive types tested
- [ ] Arrays tested (both syntaxes)
- [ ] Unions tested
- [ ] Generics tested
- [ ] Objects tested
- [ ] Functions tested
- [ ] Tuples tested
- [ ] Nested types tested
- [ ] Error cases tested

---

### T3.2: Unit Tests for Annotation Parsing

**Phase**: 3
**Component**: cli
**Files**: `src/parse/mod.rs`
**Depends On**: T2.2
**Estimated Time**: 2 hours

**Test cases**:
```rust
#[test]
fn test_param_with_type() {
    let content = "// @acp:param {string} userId - The user ID";
    let anns = parser.parse_annotations(content);
    // Verify param captured with type
}

#[test]
fn test_param_optional() {
    let content = "// @acp:param {number} [limit] - Optional limit";
    // Verify optional=true
}

#[test]
fn test_param_with_default() {
    let content = "// @acp:param {number} [limit=10] - Limit with default";
    // Verify default="10"
}

#[test]
fn test_returns_with_type() {
    let content = "// @acp:returns {Promise<User>} - The created user";
    // Verify returns captured
}

#[test]
fn test_template_with_constraint() {
    let content = "// @acp:template T extends BaseUser - User type param";
    // Verify template captured with constraint
}

#[test]
fn test_backward_compat_no_type() {
    let content = "// @acp:param userId - The user ID";
    // Verify still works without type
}
```

**Acceptance Criteria**:
- [ ] @acp:param variants tested
- [ ] @acp:returns tested
- [ ] @acp:template tested
- [ ] Backward compatibility verified

---

### T3.3: Integration Tests

**Phase**: 3
**Component**: cli
**Files**: `tests/type_annotation_tests.rs` (new)
**Depends On**: T2.4
**Estimated Time**: 2 hours

**Test scenario**:
1. Create temporary file with typed annotations
2. Run indexer
3. Verify cache contains correct type_info

```rust
#[test]
fn test_index_file_with_types() {
    let content = r#"
        // @acp:fn createUser - Creates a user
        // @acp:template T extends BaseUser - User type
        // @acp:param {string} email - User email
        // @acp:param {string} password - User password
        // @acp:param {Partial<T>} [profile={}] - Optional profile
        // @acp:returns {Promise<T>} - Created user
        async function createUser(email, password, profile = {}) { }
    "#;

    // Write to temp file, index, check cache
}
```

**Acceptance Criteria**:
- [ ] End-to-end workflow tested
- [ ] Cache contains type_info
- [ ] All param types present
- [ ] Returns type present
- [ ] Template type present

---

### T3.4: Schema Validation Tests

**Phase**: 3
**Component**: tests
**Files**: `tests/fixtures/schemas/cache/valid/with-types.json`
**Depends On**: T1.3
**Estimated Time**: 1 hour

**Create fixture**:
```json
{
  "version": "1.0.0",
  "generated_at": "2025-12-25T00:00:00Z",
  "project": { "name": "test", "root": "/test" },
  "stats": { "files": 1, "symbols": 1, "lines": 10 },
  "source_files": {},
  "files": {},
  "symbols": {
    "createUser": {
      "name": "createUser",
      "qualified_name": "test.ts:createUser",
      "type": "function",
      "file": "test.ts",
      "lines": [1, 10],
      "exported": true,
      "type_info": {
        "params": [
          { "name": "email", "type": "string", "typeSource": "acp" },
          { "name": "password", "type": "string", "typeSource": "acp" },
          { "name": "profile", "type": "Partial<T>", "typeSource": "acp", "optional": true, "default": "{}" }
        ],
        "returns": { "type": "Promise<T>", "typeSource": "acp" },
        "typeParams": [
          { "name": "T", "constraint": "BaseUser" }
        ]
      }
    }
  }
}
```

**Acceptance Criteria**:
- [ ] Valid fixture validates
- [ ] Schema test passes

---

## Phase 4: Documentation

### T4.1: Update CLI README

**Phase**: 4
**Component**: docs
**Files**: `README.md`
**Depends On**: T2.4
**Estimated Time**: 1 hour

**Add section**:
```markdown
## Type Annotations (RFC-0008)

Document parameter and return types directly in ACP annotations:

### Syntax
\`\`\`typescript
// @acp:param {string} userId - The user's unique identifier
// @acp:param {number} [limit=10] - Optional limit with default
// @acp:returns {Promise<User>} - The fetched user
// @acp:template T extends BaseUser - Generic type parameter
\`\`\`

### Supported Types
- Primitives: `string`, `number`, `boolean`, `null`, `void`, `any`
- Arrays: `T[]`, `Array<T>`
- Unions: `T | U | null`
- Generics: `Promise<T>`, `Map<K, V>`
- Objects: `{ name: string, age?: number }`
- Functions: `(x: number) => string`
```

**Acceptance Criteria**:
- [ ] Section added to README
- [ ] Syntax documented
- [ ] Examples provided

---

### T4.2: Update RFC Status

**Phase**: 4
**Component**: docs
**Files**: `rfcs/rfc-0008-*.md`, `rfcs/README.md`
**Depends On**: T3.3
**Estimated Time**: 30 minutes

**Changes**:
1. Update RFC-0008 status to Implemented
2. Add implementation date
3. Add implementation notes
4. Update rfcs/README.md index

**Acceptance Criteria**:
- [ ] Status changed to Implemented
- [ ] Implementation date added
- [ ] README index updated

---

## Phase 5: Release

### T5.1: Version Bump and Changelog

**Phase**: 5
**Component**: docs
**Files**: `CHANGELOG.md`
**Depends On**: T4.2
**Estimated Time**: 30 minutes

**Changelog entry**:
```markdown
## [0.7.0] - 2025-XX-XX

### Added - RFC-0008: ACP Type Annotations

- Optional type syntax in annotations: `@acp:param {Type} name - directive`
- Support for primitives, arrays, unions, generics, objects, functions, tuples
- `@acp:returns {Type}` for return type documentation
- `@acp:template T extends Constraint` for generic type parameters
- `type_info` field in cache symbol entries
- Type parser with comprehensive type expression support
```

**Acceptance Criteria**:
- [ ] Changelog entry added
- [ ] Version bumped to 0.7.0

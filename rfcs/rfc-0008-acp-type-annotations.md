# RFC-0008: ACP Type Annotations

- **RFC ID**: 0008
- **Title**: ACP Type Annotations
- **Author**: David (ACP Protocol)
- **Status**: Draft
- **Created**: 2025-12-23
- **Updated**: 2025-12-24
- **Discussion**: [Pending GitHub Discussion]
- **Parent**: RFC-0007 (ACP Complete Documentation Solution)
- **Related**: RFC-0001, RFC-0006, RFC-0010 (types rendered in generated documentation)

---

## Summary

This RFC introduces optional type syntax within ACP annotations, allowing developers to document parameter and return types directly in `@acp:param` and `@acp:returns` annotations. The type syntax supports simple types, generics, unions, and complex constraints.

**Core syntax**: `@acp:param {Type} name - directive`

This enables ACP to serve as a complete documentation solution for greenfield projects and polyglot codebases without requiring native documentation systems.

Types stored in the cache are rendered by `acp docs` (RFC-0010) in parameter tables, return type displays, and type signature headers.

---

## Motivation

### Problem Statement

Currently, ACP annotations can describe *what* parameters do but not their types:

```typescript
// Current: No type information
// @acp:param userId - The user's unique identifier

// Desired: Type included
// @acp:param {string} userId - The user's unique identifier
```

Users wanting complete documentation must use:
- ACP for directives + JSDoc for types (JavaScript)
- ACP for directives + docstrings for types (Python)
- Two different syntaxes for one concept

### Goals

1. Add optional `{type}` syntax to parameter and return annotations
2. Support simple types, generics, unions, and full constraints
3. Define language-agnostic type mapping
4. Enable type extraction during indexing
5. Maintain backward compatibility (types are optional)

### Non-Goals

1. Runtime type checking (documentation only)
2. Static type analysis (not replacing TypeScript/mypy)
3. Enforced type syntax (remains optional)

---

## Detailed Design

### 1. Grammar Extension

Extend the EBNF grammar from the ACP specification:

```ebnf
(* Extended ACP Annotation Grammar with Optional Types *)

annotation     = "@acp:" , namespace , [ ":" , sub_namespace ] ,
                 [ type_spec ] , [ whitespace , value ] ,
                 " - " , directive ;

type_spec      = whitespace , "{" , type_expression , "}" ;

type_expression = union_type ;

union_type      = intersection_type , { "|" , intersection_type } ;

intersection_type = primary_type , { "&" , primary_type } ;

primary_type    = primitive_type
                | literal_type
                | array_type
                | tuple_type
                | object_type
                | function_type
                | generic_type
                | conditional_type
                | type_reference
                | "(" , type_expression , ")" ;

primitive_type  = "string" | "number" | "boolean" | "null"
                | "undefined" | "void" | "any" | "never" | "unknown" ;

literal_type    = string_literal | number_literal | "true" | "false" ;

array_type      = primary_type , "[" , "]"
                | "Array" , "<" , type_expression , ">" ;

tuple_type      = "[" , [ type_list ] , "]" ;

object_type     = "{" , [ property_list ] , "}" ;

function_type   = "(" , [ param_list ] , ")" , whitespace , "=>" , whitespace , type_expression ;

generic_type    = type_reference , "<" , type_list , ">" ;

(* Full constraint support *)
constraint      = "extends" , type_expression ;
conditional_type = type_expression , "extends" , type_expression ,
                   "?" , type_expression , ":" , type_expression ;

type_reference  = identifier , { "." , identifier } ;

type_list       = type_expression , { "," , type_expression } ;

param_list      = param , { "," , param } ;

param           = [ identifier , ":" ] , type_expression ;

property_list   = property , { "," , property } ;

property        = [ "readonly" ] , property_name , [ "?" ] , ":" , type_expression ;

property_name   = identifier | string_literal ;
```

### 2. Type Syntax Examples

**Basic types:**
```typescript
// @acp:param {string} userId - The user's unique identifier
// @acp:param {number} age - User's age in years; MUST be positive
// @acp:param {boolean} active - Whether the user is active
// @acp:returns {void} - No return value
```

**Complex types:**
```typescript
// @acp:param {string | null} name - Name or null if anonymous
// @acp:param {Array<User>} users - List of users to process
// @acp:param {Map<string, number>} scores - Score map by user ID
// @acp:param {Promise<Result<User, Error>>} - Async result type
// @acp:returns {[string, number]} - Tuple of name and count
```

**Optional and default:**
```typescript
// @acp:param {string} [prefix] - Optional prefix string
// @acp:param {number} [limit=10] - Limit with default of 10
// @acp:param {Options} [options={}] - Options with empty default
```

**Function types:**
```typescript
// @acp:param {(item: T) => boolean} predicate - Filter function
// @acp:param {(a: number, b: number) => number} comparator - Sort comparator
// @acp:returns {() => void} - Cleanup function
```

**Object shapes:**
```typescript
// @acp:param {{name: string, age: number}} user - User object
// @acp:param {{id: string, data?: any}} request - Request with optional data
```

**Generic constraints (full support):**
```typescript
// @acp:template T extends Serializable - Must be serializable
// @acp:template K extends keyof User - Must be a User property key
// @acp:param {T extends Array<infer U> ? U : never} element - Extracted element type
```

### 3. Language-Agnostic Type Mapping

ACP types use a universal syntax that maps to language-specific types:

| ACP Type     | TypeScript   | Python         | Rust           | Java          | Go             |
|--------------|--------------|----------------|----------------|---------------|----------------|
| `string`     | `string`     | `str`          | `String`       | `String`      | `string`       |
| `number`     | `number`     | `int\|float`   | `i64\|f64`     | `Number`      | `int\|float64` |
| `boolean`    | `boolean`    | `bool`         | `bool`         | `Boolean`     | `bool`         |
| `null`       | `null`       | `None`         | `None`         | `null`        | `nil`          |
| `void`       | `void`       | `None`         | `()`           | `void`        | `-`            |
| `any`        | `any`        | `Any`          | `dyn Any`      | `Object`      | `any`          |
| `Array<T>`   | `T[]`        | `List[T]`      | `Vec<T>`       | `List<T>`     | `[]T`          |
| `Map<K,V>`   | `Map<K,V>`   | `Dict[K,V]`    | `HashMap<K,V>` | `Map<K,V>`    | `map[K]V`      |
| `T \| null`  | `T \| null`  | `Optional[T]`  | `Option<T>`    | `@Nullable T` | `*T`           |
| `Promise<T>` | `Promise<T>` | `Awaitable[T]` | `Future<T>`    | `Future<T>`   | `<-chan T`     |

### 4. Cache Schema Extensions

Add type fields to cache schema:

```json
{
  "symbol_entry": {
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
              "typeSource": {
                "type": "string",
                "enum": ["acp", "inferred", "native"]
              },
              "optional": { "type": "boolean" },
              "default": { "type": "string" }
            }
          }
        },
        "returns": {
          "type": "object",
          "properties": {
            "type": { "type": "string" },
            "typeSource": { "type": "string" }
          }
        },
        "typeParams": {
          "type": "array",
          "items": {
            "type": "object",
            "properties": {
              "name": { "type": "string" },
              "constraint": { "type": "string" }
            }
          }
        }
      }
    }
  }
}
```

### 5. Type Validation (Optional Lint)

Add opt-in type checking via `acp lint`:

```bash
# Check ACP types against source code types
acp lint --check-types src/

# Output:
# src/auth.ts:45 - Type mismatch: ACP declares {string} but code has number
# src/users.py:23 - Type mismatch: ACP declares {List[User]} but code has List[str]
```

Configuration:
```json
{
  "lint": {
    "checkTypes": true,
    "typeStrictness": "warning"  // "error" | "warning" | "off"
  }
}
```

---

## Examples

### Complete Function Documentation

```typescript
/**
 * @acp:fn - Creates a new user account with validation
 * @acp:template T extends BaseUser - User type parameter
 * @acp:param {string} email - User's email address; MUST validate format
 * @acp:param {string} password - Password to hash; MUST be 8+ chars
 * @acp:param {Partial<T>} [profile={}] - Optional profile data
 * @acp:returns {Promise<T>} - Created user with generated ID
 * @acp:throws {ValidationError} - Invalid email or password format
 */
async function createUser<T extends BaseUser>(
  email: string,
  password: string,
  profile: Partial<T> = {}
): Promise<T> {
  // Implementation
}
```

---

## Documentation Rendering

Types defined via RFC-0008 syntax are stored in the cache and rendered by RFC-0010 templates:

```jinja
{# Template example showing type rendering #}
{% if sym.params %}
<table class="params">
  <thead><tr><th>Name</th><th>Type</th><th>Description</th></tr></thead>
  <tbody>
  {% for param in sym.params %}
    <tr>
      <td><code>{{ param.name }}</code></td>
      <td><code>{{ param.type }}</code></td>
      <td>{{ param.description }}</td>
    </tr>
  {% endfor %}
  </tbody>
</table>
{% endif %}

{% if sym.returns %}
<p><strong>Returns:</strong> <code>{{ sym.returns.type }}</code>
  {% if sym.returns.description %} - {{ sym.returns.description }}{% endif %}
</p>
{% endif %}
```

This enables complete API documentation with type information from a single annotation system.

---

## Drawbacks

1. **Duplication with native types**: Types may duplicate TypeScript/Python hints
   - *Mitigation*: Types are optional; RFC-0006 bridging extracts native types

2. **Parser complexity**: Full constraint support adds parsing complexity
   - *Mitigation*: Use recursive descent parser; well-defined grammar

3. **Learning curve**: Another type syntax to learn
   - *Mitigation*: Syntax matches TypeScript (familiar to many)

---

## Alternatives

### Alternative 1: Simple Types Only
Only support basic types like `string`, `number`, etc.

**Rejected**: Users need generics and constraints for real-world documentation.

### Alternative 2: TypeScript Subset
Only support TypeScript-compatible syntax.

**Rejected**: ACP is language-agnostic; needs universal mapping.

---

## Implementation

### Phase 1: Parser (2 weeks)

1. Extend annotation parser for `{type}` syntax
2. Implement type expression parser
3. Add type fields to AST
4. Unit tests for all type forms

### Phase 2: Cache Integration (1 week)

1. Update cache schema with type fields
2. Extract types during indexing
3. Store type metadata in symbols

### Phase 3: Lint Integration (1 week)

1. Implement `--check-types` flag
2. Cross-reference with source types
3. Generate type mismatch warnings

**Total Effort**: ~4 weeks

---

## Changelog

| Date       | Change                                                             |
|------------|--------------------------------------------------------------------|
| 2025-12-23 | Split from RFC-0007; initial draft                                 |
| 2025-12-24 | Added RFC-0010 relationship; added Documentation Rendering section |

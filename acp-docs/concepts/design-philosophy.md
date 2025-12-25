# ACP Design Philosophy

**Document Type**: Explanation  
**Audience**: Developers, Protocol implementers  
**Reading Time**: 12 minutes

---

## Overview

ACP is built on five core principles that guide every design decision. Understanding these principles helps you use ACP effectively and anticipate how the protocol will evolve.

---

## Principle 1: Advisory, Not Enforced

> Constraints guide AI behavior but don't provide runtime enforcement.

### The Principle

ACP constraints are signals, not walls. When you mark code as `@acp:lock frozen`, you're telling AI tools "this code should not be modified"—but nothing technically prevents modification.

```python
# @acp:lock frozen - Security critical authentication
def validate_token(token: str) -> bool:
    # AI sees this constraint and respects it
    # But a developer can still modify if needed
    ...
```

### Why Advisory?

| Reason | Explanation |
|--------|-------------|
| **Flexibility** | Developers can override when necessary |
| **No overhead** | No runtime enforcement layer |
| **Simple** | No language-specific enforcement needed |
| **Trust-based** | Collaboration between humans and AI |

### The Trust Model

```
┌─────────────────────────────────────────────────────────────────┐
│                      Trust Relationship                          │
│                                                                  │
│   Developer ───────────────────────────────────────▶ AI Tool     │
│      │                                                  │        │
│      │  "I trust you to respect                        │        │
│      │   these constraints"                            │        │
│      │                                                  │        │
│      │                                                  ▼        │
│      │                                           ┌─────────┐    │
│      │                                           │ Respects│    │
│      └──────────────────────────────────────────▶│ @acp:   │    │
│                                                   │ lock    │    │
│         But can override when                     └─────────┘    │
│         explicitly needed                                        │
│                                                                  │
└─────────────────────────────────────────────────────────────────┘
```

### Implications

- AI tools claiming ACP conformance MUST respect constraints
- Nothing technically prevents violation—this is intentional
- Humans remain in control and can override when necessary
- The system relies on good-faith participation

---

## Principle 2: Token Efficiency

> AI systems have context limits. ACP optimizes for minimal token usage.

### The Problem

AI models have finite context windows. Every token spent on metadata is a token not available for actual work.

```
┌─────────────────────────────────────────────────────────────────┐
│                    Context Window                                │
├─────────────────────────────────────────────────────────────────┤
│                                                                  │
│  ████████████████████████████████░░░░░░░░░░░░░░░░░░░░░░░░░░░░░  │
│  ▲                               ▲                               │
│  │                               │                               │
│  Code + Context                  Available for                   │
│                                  Response                        │
│                                                                  │
│  More efficient context = More room for useful work              │
│                                                                  │
└─────────────────────────────────────────────────────────────────┘
```

### ACP's Approach

| Mechanism | How It Saves Tokens |
|-----------|---------------------|
| **Pre-computed cache** | Query, don't analyze entire codebase |
| **Variable references** | `$SYM_AUTH` expands to full context on demand |
| **Summarized metadata** | Concise descriptions, not full code |
| **Selective querying** | Fetch only what's needed |
| **Self-documenting annotations** | Directive is the explanation |

### Variables Example

Instead of repeating full paths and descriptions:

```json
// Without variables: 847 tokens
{
  "context": "The function src/auth/session.ts:SessionService.validateSession (lines 45-89) validates JWT tokens and returns session data. It's called by src/api/middleware.ts:AuthMiddleware.authenticate..."
}

// With variables: 12 tokens
{
  "context": "$SYM_VALIDATE_SESSION is called by $SYM_AUTH_MIDDLEWARE"
}
```

Variables expand to full context only when needed.

### Self-Documenting Annotations

Before (RFC-001):
```python
# @acp:lock frozen
# Separate comment explaining why this is frozen
# and what the implications are
```

After (RFC-001):
```python
# @acp:lock frozen - Security critical, DO NOT modify without security review
```

The directive IS the explanation—no duplication needed.

---

## Principle 3: Language Agnostic

> ACP works across programming languages.

### The Challenge

Codebases are polyglot. A single project might have TypeScript, Python, Go, and SQL. Any context protocol must work across all of them.

### ACP's Solution

| Component | Language Independence |
|-----------|----------------------|
| **Annotations** | Live in comments (every language has comments) |
| **Cache format** | Standard JSON (universal) |
| **Parsing** | tree-sitter supports 200+ languages |
| **Queries** | jq works on any JSON |

### Comment Styles

ACP adapts to each language's comment syntax:

```typescript
// TypeScript: Single-line comment
// @acp:domain auth

/**
 * TypeScript: Block comment
 * @acp:fn "Validates user credentials"
 */
```

```python
# Python: Hash comment
# @acp:domain auth

"""
Python: Docstring
@acp:fn "Validates user credentials"
"""
```

```rust
// Rust: Line comment
// @acp:domain auth

/// Rust: Doc comment
/// @acp:fn "Validates user credentials"
```

### Cache Is Universal

Regardless of source language, the cache is always JSON:

```json
{
  "symbols": {
    "src/auth.ts:validateUser": { "type": "function", "language": "typescript" },
    "src/auth.py:validate_user": { "type": "function", "language": "python" },
    "src/auth.rs:validate_user": { "type": "function", "language": "rust" }
  }
}
```

---

## Principle 4: Incremental Adoption

> Teams can adopt ACP gradually.

### The Anti-Pattern

Some systems require all-or-nothing migration:
- "Convert your entire codebase before anything works"
- "All or nothing annotation coverage required"
- "Breaking changes on every version"

### ACP's Approach

```
Adoption Ladder

Level 4: Full Coverage
         ┌─────────────────────────────────────────┐
         │ Comprehensive annotations across        │
         │ codebase, all domains defined,         │
         │ variables, constraints everywhere       │
         └─────────────────────────────────────────┘
              ▲
Level 3: Constraints
         ┌─────────────────────────────────────────┐
         │ Add @acp:lock for critical code        │
         │ Define ownership with @acp:owner       │
         └─────────────────────────────────────────┘
              ▲
Level 2: Organization
         ┌─────────────────────────────────────────┐
         │ Add @acp:module, @acp:domain           │
         │ Define architectural layers            │
         └─────────────────────────────────────────┘
              ▲
Level 1: Zero Annotations
         ┌─────────────────────────────────────────┐
         │ Just run `acp index`                   │
         │ Basic cache useful for structure       │
         │ No annotations required                │
         └─────────────────────────────────────────┘
```

### Starting from Zero

Even with no annotations, `acp index` generates useful metadata:

```bash
$ acp index
✓ Indexed 247 files
✓ Found 1,842 symbols
✓ Built call graph with 3,201 edges
✓ Generated .acp.cache.json
```

The cache includes:
- File list and structure
- Symbol names and types
- Call graph relationships
- Basic language detection

This is immediately useful for AI navigation—no annotations required.

### Progressive Enhancement

Add annotations where they provide value:

```python
# Start here: Mark critical code
# @acp:lock frozen - Payment processing, security critical

# Later: Add domain organization
# @acp:domain payments

# Eventually: Full documentation
# @acp:fn "Processes credit card transactions"
# @acp:param amount "Transaction amount in cents"
# @acp:returns "Transaction result with ID"
```

---

## Principle 5: Standard-Based

> ACP follows established conventions.

### Standards Used

| Standard | Usage in ACP |
|----------|--------------|
| **RFC 2119** | Normative keywords (MUST, SHOULD, MAY) |
| **Semantic Versioning** | Protocol and file versioning |
| **EBNF Grammar** | Formal annotation syntax |
| **JSON Schema** | File format validation |
| **ISO 8601** | Timestamp formats |

### Why Standards Matter

1. **Predictability**: Implementers know what to expect
2. **Tooling**: Existing tools work (jq, JSON validators)
3. **Interoperability**: Different implementations work together
4. **Validation**: Schemas catch errors early

### RFC 2119 Example

The specification uses precise normative language:

> Implementations **MUST** respect the `frozen` lock level.
> Implementations **SHOULD** warn before modifying `restricted` code.
> Implementations **MAY** provide additional lock levels.

This removes ambiguity about requirements vs. suggestions.

### Semantic Versioning

ACP files include version information:

```json
{
  "version": "1.0.0",
  "generated_at": "2024-12-17T15:30:00Z"
}
```

Version compatibility rules:
- Same major version = Compatible
- Different major version = Breaking changes possible
- Minor/patch = Additive only

---

## How Principles Interact

The principles reinforce each other:

```
┌─────────────────────────────────────────────────────────────────┐
│                                                                  │
│         Advisory ◄─────────────────────▶ Incremental            │
│            │                                   │                 │
│            │    No enforcement means           │                 │
│            │    easy to start small            │                 │
│            │                                   │                 │
│            ▼                                   ▼                 │
│      Token Efficient ◄───────────────▶ Language Agnostic        │
│            │                                   │                 │
│            │    JSON cache works               │                 │
│            │    everywhere efficiently         │                 │
│            │                                   │                 │
│            └────────────▶ Standard-Based ◄─────┘                │
│                                                                  │
│            All supported by established standards                │
│                                                                  │
└─────────────────────────────────────────────────────────────────┘
```

---

## Design Decisions Explained

### Why Comments, Not Attributes?

**Decision**: Annotations in comments, not language-specific attributes.

**Rationale**:
- ✅ Works in every language
- ✅ No syntax changes to code
- ✅ Visible in any editor
- ❌ Can't enforce at compile time (but that's principle #1)

### Why JSON, Not YAML or TOML?

**Decision**: Cache and config in JSON format.

**Rationale**:
- ✅ Universal parsing support
- ✅ jq for querying
- ✅ JSON Schema for validation
- ✅ No parser ambiguity (unlike YAML)
- ❌ No comments (use `.acp.config.json5` if needed)

### Why Pre-Computed Cache?

**Decision**: Generate cache file rather than parse on demand.

**Rationale**:
- ✅ Fast queries (no parsing)
- ✅ Consistent state (point-in-time snapshot)
- ✅ Works offline
- ✅ Git-trackable
- ❌ Can become stale (but staleness detection handles this)

---

## Future Principles

As ACP evolves, new principles may emerge. Current candidates:

- **Safety-First**: Critical constraint information always preserved
- **Composable**: Annotations can be combined without conflicts
- **Observable**: Tooling can introspect ACP state

---

## Summary

| Principle | One-Liner |
|-----------|-----------|
| Advisory | Constraints guide, don't enforce |
| Token Efficient | Minimize context window usage |
| Language Agnostic | Works everywhere |
| Incremental | Start small, grow as needed |
| Standard-Based | Build on proven foundations |

These principles ensure ACP remains practical, adoptable, and valuable across diverse codebases and teams.

---

## Further Reading

- [Why ACP?](why-acp.md) — The problem ACP solves
- [ACP vs MCP](acp-vs-mcp.md) — Protocol comparison
- [Specification](../reference/specification.md) — Full protocol details

---

*This document is part of the ACP Documentation. [Report issues](https://github.com/acp-protocol/acp-spec/issues)*

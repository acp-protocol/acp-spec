# Introduction

**ACP Version**: 1.0.0
**Document Version**: 1.0.0
**Last Updated**: 2024-12-18
**Status**: Draft

---

## Table of Contents

1. [Overview](#1-overview)
2. [Purpose](#2-purpose)
3. [Key Features](#3-key-features)
4. [Non-Goals](#4-non-goals)
5. [Design Principles](#5-design-principles)
6. [How It Works](#6-how-it-works)
7. [Specification Structure](#7-specification-structure)
8. [Conformance](#8-conformance)

---

## 1. Overview

The AI Context Protocol (ACP) provides a standardized way to embed machine-readable documentation and constraints directly in source code. It enables AI systems to understand code structure, respect developer intentions, and navigate codebases efficiently.

ACP bridges the gap between developers and AI coding assistants by establishing a shared vocabulary and structured metadata format. Rather than relying on AI to infer intent from code alone, ACP allows developers to explicitly communicate:

- What code does and why it exists
- How code should (or should not) be modified
- Which parts of the codebase relate to each other
- What constraints and quality requirements apply

### 1.1 The Problem

AI coding assistants face several challenges when working with codebases:

| Challenge | Description |
|-----------|-------------|
| **Context Limits** | AI systems have token limits; sending entire codebases is impractical |
| **Intent Ambiguity** | Code shows *what* but not always *why* or *how to change it* |
| **Discovery Difficulty** | Finding relevant code requires understanding project structure |
| **Constraint Invisibility** | Critical restrictions (security, stability) aren't machine-readable |
| **Inconsistent Interaction** | Each AI tool reinvents codebase understanding |

### 1.2 The Solution

ACP addresses these challenges through:

1. **Structured Annotations** — Embed metadata in code comments using `@acp:` syntax
2. **Pre-computed Cache** — Index codebase structure once, query efficiently many times
3. **Advisory Constraints** — Communicate modification rules AI systems should respect
4. **Token-Efficient Variables** — Reference code elements without verbose inline descriptions
5. **Standardized Format** — One protocol works across languages, tools, and AI systems

---

## 2. Purpose

ACP addresses three core needs:

### 2.1 Context Discovery

Help AI systems find relevant code quickly.

**Without ACP:**
```
User: "Fix the authentication bug"
AI: *searches through hundreds of files trying to find auth-related code*
```

**With ACP:**
```
User: "Fix the authentication bug"
AI: *queries cache for authentication domain, immediately finds relevant files and symbols*
```

The cache file (`.acp.cache.json`) provides pre-indexed access to:
- File and module organization
- Symbol definitions and their relationships
- Domain classifications (authentication, billing, etc.)
- Call graphs showing what calls what

### 2.2 Intent Communication

Let developers specify how code should be modified.

**Without ACP:**
```javascript
// This function is critical for security - don't change it!
function validateToken(token) { ... }
```

**With ACP:**
```javascript
/**
 * @acp:lock frozen
 * @acp:lock-reason "Security critical - validated by external audit"
 * @acp:quality security-review
 */
function validateToken(token) { ... }
```

Constraints communicate:
- **Lock levels** — How freely code can be modified
- **Style requirements** — What conventions to follow
- **Behavior guidance** — How aggressive changes should be
- **Quality gates** — What reviews or tests are required

### 2.3 Token Efficiency

Minimize context sent to AI systems.

**Without ACP (high token cost):**
```
Please review the validateSession function in src/auth/session.ts
(lines 45-89) which validates JWT tokens by checking the signature
against our secret key, verifying expiration, and returning the
decoded session data or null if invalid...
```

**With ACP (low token cost):**
```
Please review $SYM_VALIDATE_SESSION
```

Variables expand to full context automatically, reducing prompt size by 50-90% while maintaining complete information.

---

## 3. Key Features

### 3.1 Annotations

Structured metadata embedded in source code comments.

```javascript
/**
 * @acp:module "Payment Processing"
 * @acp:domain billing
 * @acp:layer service
 * @acp:stability stable
 * @acp:lock restricted
 * @acp:summary "Handles all payment transactions via Stripe"
 */
export class PaymentService { ... }
```

Annotations are:
- **Language-agnostic** — Same syntax works in any language with comments
- **Human-readable** — Developers can understand them at a glance
- **Machine-parseable** — Unambiguous format for reliable extraction
- **Non-intrusive** — Live in comments, don't affect code execution

### 3.2 Constraints

Advisory rules for AI behavior.

| Constraint Type | Purpose | Example |
|-----------------|---------|---------|
| **Lock** | Control modification freedom | `@acp:lock frozen` |
| **Style** | Enforce coding conventions | `@acp:style google-typescript` |
| **Behavior** | Guide AI decision-making | `@acp:behavior conservative` |
| **Quality** | Require specific checks | `@acp:quality security-review` |

Constraints are **advisory** — they guide AI behavior but don't provide runtime enforcement. AI systems claiming ACP conformance MUST respect these constraints.

### 3.3 Variables

Token-efficient references to code elements.

```json
{
  "SYM_VALIDATE_SESSION": {
    "type": "symbol",
    "value": "src/auth/session.ts:SessionService.validateSession",
    "description": "Validates JWT tokens and returns session"
  }
}
```

Variable types:
- `SYM_*` — Symbol references (functions, classes, methods)
- `FILE_*` — File references
- `DOM_*` — Domain references

### 3.4 Cache

Pre-computed codebase index for efficient querying.

```json
{
  "version": "1.0.0",
  "generated_at": "2024-12-18T15:30:00Z",
  "project": { "name": "my-app", "root": "." },
  "files": { ... },
  "symbols": { ... },
  "graph": { "forward": { ... }, "reverse": { ... } },
  "domains": { ... },
  "constraints": { ... }
}
```

The cache enables:
- **Fast queries** — Find symbols, files, domains without parsing
- **Call graph traversal** — Understand what calls what
- **Constraint lookup** — Check rules before modifying code
- **Statistics** — Understand codebase size and composition

### 3.5 Query Interface

Flexible ways to access metadata.

```bash
# CLI queries
acp query '.domains.authentication.files'
acp constraints src/auth/session.ts

# jq queries
jq '.symbols | to_entries | map(select(.value.type == "function"))' .acp.cache.json

# MCP tools (for AI assistants)
acp_query(type="domain", name="authentication")
acp_constraints(file="src/auth/session.ts")
```

---

## 4. Non-Goals

ACP is explicitly **NOT**:

### 4.1 A Runtime Enforcement Mechanism

ACP constraints are advisory. There is no runtime code that prevents violations.

**Why:** Runtime enforcement would require language-specific implementations, add overhead, and reduce flexibility. The advisory model enables trust-based collaboration between developers and AI.

### 4.2 A Type System or Static Analyzer

ACP does not validate code correctness, type safety, or catch bugs.

**Why:** Excellent tools already exist for static analysis (TypeScript, ESLint, mypy). ACP complements these tools by adding AI-specific metadata, not replacing them.

### 4.3 A Replacement for Existing Documentation

ACP supplements, not replaces, READMEs, API docs, and code comments.

**Why:** Human documentation serves different purposes. ACP provides machine-optimized metadata; traditional documentation provides human-optimized explanation.

### 4.4 A Security or Access Control System

ACP cannot enforce access restrictions or prevent unauthorized modifications.

**Why:** Security requires proper authentication, authorization, and access control systems. ACP's `@acp:lock frozen` is a signal to AI, not a security boundary.

---

## 5. Design Principles

### 5.1 Advisory, Not Enforced

Constraints guide AI behavior but don't provide runtime enforcement.

**Benefits:**
- Flexibility for AI decision-making
- No runtime overhead
- Simple implementation
- Trust-based collaboration

**Implication:** AI systems claiming ACP conformance MUST respect constraints, but nothing technically prevents violation. This is by design.

### 5.2 Token Efficiency

AI systems have context limits. ACP optimizes for minimal token usage.

**Mechanisms:**
- Pre-computed cache files (query, don't analyze)
- Variable references (compact identifiers expand to full context)
- Summarized metadata (concise descriptions, not full code)
- Selective querying (fetch only what's needed)

### 5.3 Language Agnostic

ACP works across programming languages.

**How:**
- Annotations live in comments (every language has comments)
- Cache format is JSON (universal)
- No language-specific syntax requirements
- Extensible language support via tree-sitter

### 5.4 Incremental Adoption

Teams can adopt ACP gradually.

**Adoption path:**
1. **Zero annotations** — Basic cache still useful for structure
2. **Module annotations** — Add `@acp:module`, `@acp:domain` for organization
3. **Constraint annotations** — Add `@acp:lock` for critical code
4. **Full coverage** — Comprehensive annotations across codebase

No all-or-nothing migration required.

### 5.5 Standard-Based

ACP follows established conventions.

| Convention | Usage |
|------------|-------|
| **RFC 2119** | Keywords (MUST, SHOULD, MAY) for requirements |
| **Semantic Versioning** | Protocol and file versioning |
| **EBNF Grammar** | Formal annotation syntax definition |
| **JSON Schema** | File format validation |
| **ISO 8601** | Timestamp formats |

---

## 6. How It Works

### 6.1 Workflow Overview

```
┌─────────────────┐     ┌─────────────────┐     ┌─────────────────┐
│   Your Code     │     │   ACP CLI       │     │   AI Tools      │
│                 │     │                 │     │                 │
│  @acp:domain X  │────▶│  acp index      │────▶│  Read cache     │
│  @acp:lock Y    │     │                 │     │  Respect rules  │
│  @acp:summary Z │     │  .acp.cache.json│     │  Better context │
└─────────────────┘     └─────────────────┘     └─────────────────┘
```

1. **Annotate** — Add `@acp:` annotations in code comments
2. **Index** — Run `acp index` to generate the cache
3. **Consume** — AI tools read the cache and respect constraints

### 6.2 Example Session

**Step 1: Developer adds annotations**

```typescript
// src/auth/session.ts

/**
 * @acp:module "Session Management"
 * @acp:domain authentication
 * @acp:stability stable
 */

/**
 * @acp:summary "Validates JWT tokens and returns session data"
 * @acp:lock restricted
 * @acp:lock-reason "Security critical - requires review"
 */
export function validateSession(token: string): Session | null {
  // Implementation...
}
```

**Step 2: Generate cache**

```bash
$ acp index
Indexing... done
Generated .acp.cache.json (127 files, 523 symbols)
```

**Step 3: AI queries before modification**

```
AI: Before modifying src/auth/session.ts, let me check constraints...
    [calls acp_constraints(file="src/auth/session.ts")]
    
    This file has lock level "restricted" with reason "Security critical - 
    requires review". I should explain proposed changes before making them.
```

**Step 4: AI respects constraints**

```
AI: I found the issue in validateSession. This function is marked as 
    restricted, so let me explain what I'd like to change:
    
    Current: The token expiration check happens after signature validation
    Proposed: Move expiration check before signature validation for efficiency
    
    Would you like me to proceed with this change?
```

---

## 7. Specification Structure

This specification is organized into the following chapters:

| Chapter | Title            | Description                                |
|---------|------------------|--------------------------------------------|
| 01      | Introduction     | Overview, goals, non-goals (this document) |
| 02      | Terminology      | Definitions and RFC 2119 keywords          |
| 03      | Cache Format     | `.acp.cache.json` structure and fields     |
| 04      | Config Format    | `.acp.config.json` structure and options   |
| 05      | Annotations      | Annotation syntax and semantics            |
| 06      | Constraints      | Lock levels, style, behavior, quality      |
| 07      | Variables        | Variable system and expansion              |
| 08      | Inheritance      | Cascade and override rules                 |
| 09      | Discovery        | File discovery algorithm                   |
| 10      | Querying         | Query interface and patterns               |
| 11      | Tool Integration | Transparent AI tool context distribution   |
| 12      | Versioning       | Protocol versioning and compatibility      |
| 13      | Debug Sessions   | Hack markers and debug tracking            |

### 7.1 How to Read This Specification

**For implementers:**
- Start with Chapter 02 (Terminology) for definitions
- Read Chapter 03-04 for file format details
- Implement Chapter 09 (Discovery) for cache generation
- Use JSON Schemas for validation

**For users:**
- Start with this chapter (Introduction)
- Read Chapter 05 (Annotations) for syntax
- Read Chapter 06 (Constraints) for lock levels
- See examples in each chapter

**For tool authors:**
- Read Chapter 10 (Querying) for interface patterns
- Review Conformance Levels (Section 8 below)
- Implement appropriate level for your tool

---

## 8. Conformance

### 8.1 Conformance Levels

ACP defines three conformance levels:

| Level         | Name     | Capabilities                                   |
|---------------|----------|------------------------------------------------|
| **Level 1**   | Reader   | Parse and query cache files                    |
| **Level 2**   | Standard | Generate cache, variables, enforce constraints |
| **Level 3**   | Full     | MCP integration, debug sessions, watch mode    |

### 8.2 Level 1: Reader

**Requirements:**
- MUST parse `.acp.cache.json` conforming to schema
- MUST handle all required fields
- SHOULD handle optional fields gracefully
- MAY ignore unknown fields (forward compatibility)

**Use cases:** AI assistants, IDE plugins, query tools

### 8.3 Level 2: Standard

**Requirements:**
- MUST meet all Level 1 requirements
- MUST generate valid cache files from source
- MUST implement file discovery algorithm
- MUST implement annotation parsing
- MUST implement constraint resolution
- SHOULD implement variable generation
- SHOULD implement staleness detection

**Use cases:** CLI tools, CI/CD integration

### 8.4 Level 3: Full

**Requirements:**
- MUST meet all Level 2 requirements
- MUST implement MCP server interface
- MUST implement debug session tracking
- MUST implement hack marker management
- SHOULD implement watch mode for incremental updates
- SHOULD implement constraint violation logging

**Use cases:** Full-featured development environments

### 8.5 Claiming Conformance

Tools claiming ACP conformance:
- MUST specify which level they implement
- MUST pass conformance tests for that level
- SHOULD document any limitations or extensions

**Example:**
```
ACP CLI v1.0.0 - Level 3 Conformant
```

---

## Appendix A: Quick Start

### A.1 Installation

```bash
# Install ACP CLI (when available)
cargo install acp-cli

# Or via npm
npm install -g @acp/cli
```

### A.2 Initialize Project

```bash
cd your-project
acp init
```

Creates `.acp.config.json` with sensible defaults.

### A.3 Add Annotations

```python
# @acp:module "User Authentication"
# @acp:domain auth
# @acp:lock restricted

def validate_token(token: str) -> bool:
    """Validates JWT tokens - security critical."""
    ...
```

### A.4 Generate Cache

```bash
acp index
```

Creates `.acp.cache.json` with indexed codebase.

### A.5 Query

```bash
# List domains
acp query '.domains | keys'

# Check constraints
acp constraints src/auth/session.py

# Search symbols
acp query '.symbols | to_entries | map(select(.key | contains("validate")))'
```

---

## Appendix B: Related Documents

- [Terminology](02-terminology.md) — Definitions and keywords
- [Cache Format](03-cache-format.md) — Cache file specification
- [Config Format](04-config-format.md) — Configuration options
- [Annotations](05-annotations.md) — Annotation syntax
- [Constraints](06-constraints.md) — Constraint system
- [Variables](07-variables.md) — Variable system
- [Inheritance](08-inheritance.md) — 
- [Discovery](09-discovery.md)
- [Querying](10-querying.md)
- [Tool Integration](11-tool-integrations.md)
- [Versioning](12-versioning.md)
- [Debug Sessions](13-debug-sessions.md)
- [Main Specification](../ACP-1.0.md) — Complete protocol specification

---

*End of Introduction*
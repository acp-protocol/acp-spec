# How to Annotate Your Codebase

**Document Type**: How-To Guide  
**Status**: OUTLINE — Content to be added  
**Goal**: Add ACP annotations to an existing codebase  
**Prerequisites**: ACP CLI installed, project initialized  
**Time**: 30-60 minutes (depending on codebase size)

---

## The Problem

You have an existing codebase and want to add ACP annotations so AI assistants understand your code's structure, constraints, and intent.

---

## Solution Overview

1. Identify critical code that needs protection
2. Map your domain boundaries
3. Add file-level annotations
4. Add symbol-level annotations
5. Generate and verify the cache

---

## Step 1: Identify Critical Code

> **TODO**: Expand with detailed methodology

### Questions to Ask

- Which files should never be modified by AI?
- Which files require approval before changes?
- Which files contain security-sensitive logic?
- Which files are auto-generated?

### Create a Criticality Map

```markdown
## Critical Code Map

### Frozen (Never Modify)
- src/auth/crypto.ts - Cryptographic operations
- src/payments/processor.ts - Payment processing

### Restricted (Approval Required)  
- src/api/public/*.ts - Public API surface
- src/db/migrations/*.ts - Database migrations

### Normal
- Everything else
```

---

## Step 2: Map Domain Boundaries

> **TODO**: Expand with examples

### Identify Domains

Domains are logical groupings by business function:

| Domain | Description | Files |
|--------|-------------|-------|
| `auth` | Authentication/authorization | `src/auth/*` |
| `payments` | Payment processing | `src/payments/*` |
| `users` | User management | `src/users/*` |
| `api` | Public API | `src/api/*` |

### Identify Layers

Layers are architectural groupings:

| Layer | Description | Pattern |
|-------|-------------|---------|
| `api` | API endpoints | `src/api/*` |
| `service` | Business logic | `src/services/*` |
| `data` | Data access | `src/repositories/*` |

---

## Step 3: Add File-Level Annotations

> **TODO**: Add examples for each language

### Template

```typescript
// @acp:domain [domain] - [description]
// @acp:lock [level] - [reason]
// @acp:owner [team] - [contact info]
// @acp:module "[Human-Readable Name]"
```

### Example

```typescript
// src/auth/session.ts
// @acp:domain auth - Authentication and session management
// @acp:lock frozen - Security critical, DO NOT modify without security review
// @acp:owner security-team - Contact @security-lead before changes
// @acp:module "Session Management"
// @acp:stability stable - Public API, maintain backwards compatibility

export class SessionService {
  // ...
}
```

---

## Step 4: Add Symbol-Level Annotations

> **TODO**: Add detailed examples

### For Functions

```typescript
// @acp:fn "Brief description of what this function does"
// @acp:param paramName "Description of parameter"
// @acp:returns "Description of return value"
// @acp:throws "Description of exceptions"
```

### For Classes

```typescript
// @acp:class "Brief description of this class"
```

### For Methods

```typescript
// @acp:method "Brief description of this method"
```

### Example

```typescript
export class SessionService {
  // @acp:fn "Validates JWT token and returns session data if valid"
  // @acp:param token "JWT token string to validate"
  // @acp:returns "Session object or null if invalid"
  // @acp:throws "TokenExpiredError if token has expired"
  validateToken(token: string): Session | null {
    // ...
  }
}
```

---

## Step 5: Generate and Verify Cache

> **TODO**: Add verification steps

### Generate Cache

```bash
acp index
```

### Verify Annotations

```bash
# Check that your annotations are in the cache
acp query '.constraints.by_lock_level'

# Verify domain mapping
acp query '.domains | keys'

# Check specific file
acp constraints src/auth/session.ts
```

---

## Annotation Priority Guide

> **TODO**: Expand recommendations

Start with high-value annotations:

| Priority | Annotation | Why |
|----------|------------|-----|
| 1 | `@acp:lock frozen` | Protects critical code |
| 2 | `@acp:domain` | Enables domain-aware AI |
| 3 | `@acp:owner` | Establishes accountability |
| 4 | `@acp:fn` | Documents functions |
| 5 | `@acp:layer` | Architectural clarity |

---

## Incremental Approach

> **TODO**: Add timeline recommendations

### Week 1: Critical Code
- Add `@acp:lock frozen` to all critical files
- Generate cache, test with AI tool

### Week 2: Domains
- Add `@acp:domain` to all files
- Verify domain boundaries

### Week 3: Ownership
- Add `@acp:owner` to team-owned areas
- Document ownership in team wiki

### Week 4+: Documentation
- Add `@acp:fn` to public APIs
- Add `@acp:class` to key classes

---

## Verification

### Check Cache Coverage

```bash
acp query '.stats'
```

Expected output:
```json
{
  "files": 247,
  "symbols": 1842,
  "annotated_files": 89,
  "frozen_files": 12,
  "domains": 8
}
```

### Test with AI Tool

1. Start your AI coding assistant
2. Ask it to modify a frozen file
3. Verify it respects the constraint

---

## Troubleshooting

> **TODO**: Add common issues

### Annotations Not Detected

- Check comment syntax for your language
- Verify annotation format (`@acp:`, not `@acp -`)
- Run `acp index --verbose`

### Cache Not Updating

- Run `acp index --force`
- Check file is in `include` patterns
- Verify file isn't in `exclude` patterns

---

## Related

- [Protecting Critical Code](protecting-critical-code.md)
- [Defining Domain Boundaries](defining-domains.md)
- [Annotation Reference](../reference/annotations.md)

---

*This guide is an outline. [Contribute content →](https://github.com/acp-protocol/docs)*

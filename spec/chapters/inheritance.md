# Inheritance & Cascade Specification

**ACP Version**: 1.0.0-revised
**Document Version**: 1.0.0
**Last Updated**: 2024-12-17
**Status**: Revised Draft

---

## Table of Contents

1. [Overview](#1-overview)
2. [Precedence Levels](#2-precedence-levels)
3. [Merging Rules](#3-merging-rules)
4. [Examples](#4-examples)

---

## 1. Overview

### 1.1 Purpose

Constraints can be defined at multiple levels and cascade to more specific scopes. This specification defines how constraints from different sources are combined and which takes precedence.

**From main specification Section 7 (Lines 911-1051):**

The inheritance and cascade system determines which constraints apply when multiple levels define overlapping constraints.

### 1.2 Design Principles

- **More specific wins**: Symbol-level overrides file-level
- **Most restrictive wins**: For lock levels
- **Accumulation**: For quality requirements
- **Last wins**: For equal specificity

### 1.3 Conformance

The key words "MUST", "MUST NOT", "REQUIRED", "SHALL", "SHALL NOT", "SHOULD", "SHOULD NOT", "RECOMMENDED", "MAY", and "OPTIONAL" in this document are to be interpreted as described in [RFC 2119](https://datatracker.ietf.org/doc/html/rfc2119).

---

## 2. Precedence Levels

From main specification Section 7.1 (Lines 916-924):

Constraints cascade from general to specific. More specific levels override less specific ones.

### 2.1 Precedence Hierarchy

**From most to least specific:**

1. **Symbol-level annotations** - Apply to specific function/class
2. **File-level annotations** - Apply to entire file
3. **Directory-level config** (`.acp.dir.json`) - Apply to directory tree
4. **Project-level config** (`.acp.config.json`) - Apply to entire project

### 2.2 Override Behavior

More specific constraints override less specific constraints.

**Example:**
- Project config sets `lock: normal`
- File annotation sets `lock: restricted`
- Symbol annotation sets `lock: frozen`
- **Result for symbol:** `frozen` (most specific)
- **Result for other symbols in file:** `restricted` (file-level)
- **Result for other files:** `normal` (project default)

---

## 3. Merging Rules

From main specification Section 7.2 (Lines 926-1051):

When constraints exist at multiple precedence levels, they merge according to specific rules depending on the constraint type.

### 3.1 Lock Levels

**Rule:** Most restrictive wins

**From specification Lines 932-943:**

When multiple lock levels apply, the most restrictive level takes precedence.

**Example:**
- Project default: `normal`
- Directory: `approval-required`
- File: `restricted`
- **Result:** `restricted` (most restrictive)

**Precedence** (most to least restrictive):
```
frozen > restricted > approval-required > tests-required > docs-required > normal > experimental
```

**Complete Example:**
```json
// .acp.config.json (project level)
{
  "constraints": {
    "defaults": {
      "lock": "normal"
    }
  }
}
```

```javascript
// File annotation
/**
 * @acp:lock restricted
 */

// Symbol annotation
/**
 * @acp:lock frozen  // Overrides to frozen for this function
 */
function criticalFunction() { }

/**
 * No additional constraints - inherits file-level restricted
 */
function normalFunction() { }
```

### 3.2 Style Guides

**Rule:** Most specific **guide** wins, but rules from all levels **accumulate**

**From specification Lines 945-958:**

The base style guide comes from the most specific level, but custom rules accumulate from all levels.

**Example:**
- Project: `google-typescript`
- Directory: custom rule `max-line-length=100`
- File: custom rule `indent=2`
- **Result:**
  - Base guide: `google-typescript` (from project)
  - Additional rules: `max-line-length=100`, `indent=2` (accumulated)

**Rationale:** File-level custom rules augment project-level guide, not replace.

**Complete Example:**
```json
// Project level
{
  "constraints": {
    "defaults": {
      "style": "google-typescript"
    }
  }
}
```

```javascript
// Directory level (conceptual)
/**
 * @acp:style-rules max-line-length=100
 */

// File level
/**
 * @acp:style-rules indent=2, no-any
 */
// Result: google-typescript base + max-line-length=100 + indent=2 + no-any
```

### 3.3 Behavior Constraints

**Rule:** Most specific wins completely (no merging)

**From specification Lines 960-967:**

Behavior constraints do not accumulate. The most specific level completely overrides.

**Example:**
- Project: `approach=conservative`
- File: `approach=aggressive`
- **Result:** `aggressive` (file-level overrides completely)

**Complete Example:**
```json
// Project level
{
  "constraints": {
    "defaults": {
      "behavior": "conservative"
    }
  }
}
```

```javascript
// File level
/**
 * @acp:behavior aggressive  // Completely overrides project default
 */
```

### 3.4 Quality Requirements

**Rule:** Requirements accumulate (all levels apply)

**From specification Lines 969-976:**

Quality requirements from all levels are combined. All requirements must be met.

**Example:**
- Project: `tests-required`
- File: `security-review`
- **Result:** Both requirements apply

**Complete Example:**
```json
// Project level
{
  "constraints": {
    "defaults": {
      "quality": ["tests-required"]
    }
  }
}
```

```javascript
// File level
/**
 * @acp:quality security-review, performance-test
 */
// Result: tests-required + security-review + performance-test
```

### 3.5 Equal Specificity

**From specification Lines 978-998:**

If two constraints at the **same precedence level** conflict:

**Rule:** Last defined wins

**Example** (both in same file):
```javascript
/**
 * @acp:lock frozen
 */

// ... later in same file ...

/**
 * @acp:lock normal
 */
```
**Result:** `normal` (last wins)

**Recommendation:** Avoid this. Use linter to detect conflicts.

---

## 4. Examples

From main specification Section 7.3 (Lines 1000-1051):

### 4.1 Multi-Level Inheritance

**Setup:**
- `.acp.config.json`: `{"constraints": {"defaults": {"lock": "normal"}}}`
- `.acp.dir.json` in `src/auth/`: `{"lock": "approval-required"}`
- `src/auth/session.ts` file annotation: `@acp:lock restricted`
- `validateSession` symbol annotation: `@acp:lock frozen`

**Resolution:**

| Element | Effective Lock Level | Reason |
|---------|---------------------|--------|
| `src/utils/helper.ts` | `normal` | Project default |
| `src/auth/token.ts` | `approval-required` | Directory override |
| `src/auth/session.ts` (file) | `restricted` | File annotation |
| `src/auth/session.ts::validateSession` | `frozen` | Symbol annotation (most specific) |

**Complete Code:**
```javascript
// src/auth/session.ts

/**
 * @acp:lock restricted
 */
export class SessionService {

  /**
   * @acp:lock frozen
   */
  validateSession(token) {
    // This function has lock:frozen (symbol level)
  }

  createSession(user) {
    // This function has lock:restricted (inherited from file)
  }
}
```

### 4.2 Style Guide Accumulation

**Setup:**
- Project: `@acp:style google-typescript`
- Directory `src/api/`: `@acp:style-rules max-params=4, async-required`
- File `src/api/users.ts`: `@acp:style-rules no-any`

**Resolution for `src/api/users.ts`:**
- Base guide: `google-typescript`
- Additional rules: `max-params=4`, `async-required`, `no-any`
- All rules apply together

**Complete Code:**
```typescript
// src/api/users.ts

/**
 * @acp:style-rules no-any
 */

// Effective constraints:
// - Base: google-typescript
// - Rules: max-params=4, async-required, no-any

export async function getUsers() { // Must be async (from dir rules)
  // Cannot use 'any' type (from file rules)
  // Max 4 parameters (from dir rules)
}
```

### 4.3 Conflict Resolution

**Setup** (same file):
```typescript
/**
 * @acp:lock restricted
 * @acp:behavior conservative
 */

/**
 * @acp:lock normal
 */
function dangerousOperation() { }
```

**Resolution:**
- File level: `lock=restricted`, `behavior=conservative`
- Symbol level: `lock=normal`
- For `dangerousOperation`:
  - `lock=normal` (symbol override)
  - `behavior=conservative` (inherited from file)

### 4.4 Quality Accumulation

**Setup:**
```json
// Project config
{
  "constraints": {
    "defaults": {
      "quality": ["tests-required"]
    }
  }
}
```

```typescript
// File annotation
/**
 * @acp:quality security-review, performance-test
 */
export class PaymentService {
  processPayment() {
    // Must meet ALL requirements:
    // - tests-required (from project)
    // - security-review (from file)
    // - performance-test (from file)
  }
}
```

### 4.5 Complex Scenario

**Setup:**
```json
// .acp.config.json
{
  "constraints": {
    "defaults": {
      "lock": "normal",
      "style": "prettier",
      "behavior": "balanced",
      "quality": ["tests-required"]
    }
  }
}
```

```typescript
// src/auth/session.ts
/**
 * @acp:lock restricted
 * @acp:style google-typescript
 * @acp:style-rules max-line-length=100
 * @acp:behavior conservative
 * @acp:quality security-review
 */

export class SessionService {
  /**
   * @acp:lock frozen
   * @acp:quality performance-test
   */
  validateSession(token: string) {
    // Effective constraints:
    // - lock: frozen (symbol > file > project)
    // - style: google-typescript base (file > project)
    //         + max-line-length=100 (accumulated from file)
    // - behavior: conservative (file > project)
    // - quality: tests-required (project)
    //          + security-review (file)
    //          + performance-test (symbol)
    //   = All three quality requirements apply
  }

  createSession(user: User) {
    // Effective constraints:
    // - lock: restricted (file > project)
    // - style: google-typescript base (file > project)
    //         + max-line-length=100 (accumulated from file)
    // - behavior: conservative (file > project)
    // - quality: tests-required (project)
    //          + security-review (file)
  }
}
```

---

## Appendix A: Quick Reference

### Precedence Levels (Most to Least Specific)

1. Symbol-level annotations
2. File-level annotations
3. Directory-level config
4. Project-level config

### Merging Rules by Constraint Type

| Constraint Type | Merging Rule |
|----------------|--------------|
| Lock Levels | Most restrictive wins |
| Style Guides | Most specific guide + accumulated rules |
| Behavior | Most specific wins (no merge) |
| Quality | All requirements accumulate |
| Equal Specificity | Last defined wins |

### Lock Level Restrictiveness

Most to least restrictive:
```
frozen > restricted > approval-required > tests-required >
docs-required > normal > experimental
```

---

## Appendix B: Related Documents

- [Annotation Syntax](annotations.md) - How to write annotations
- [Constraint System](constraints.md) - Constraint definitions
- [Configuration](config.md) - Project-level configuration
- [Cache Format](cache.md) - How constraints are stored

---

*End of Inheritance & Cascade Specification*

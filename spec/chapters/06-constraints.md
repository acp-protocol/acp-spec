# Constraint System Specification

**ACP Version**: 1.0.0-revised
**Document Version**: 1.0.0
**Last Updated**: 2024-12-17
**Status**: Revised Draft

---

## Table of Contents

1. [Overview](#1-overview)
2. [Lock Constraints](#2-lock-constraints)
3. [Style Constraints](#3-style-constraints)
4. [Behavior Constraints](#4-behavior-constraints)
5. [Quality Constraints](#5-quality-constraints)
6. [Constraint Violations](#6-constraint-violations)
7. [Constraint Merging](#7-constraint-merging)
8. [Examples](#8-examples)

---

## 1. Overview

### 1.1 Purpose

Constraints define rules and guardrails for how AI systems should interact with code. They enable developers to:

- Protect critical code from unintended modifications
- Enforce coding standards and style guides
- Require specific behaviors during AI-assisted development
- Set quality gates for changes

Constraints are **advisory** to AI systems, meaning ACP cannot enforce them through access control or runtime checks.

### 1.2 Advisory Nature

**Constraints are advisory**. There is no mechanism to prevent an AI from violating constraints.

However, AI systems that claim ACP conformance (see main specification Section 10) **MUST** respect constraint semantics as specified in this document. The MUST/SHOULD language in constraint behavior tables applies to **conformant AI systems**.

**Rationale:** Advisory model enables:
- Flexibility for AI decision-making
- No runtime overhead
- Simple implementation
- Trust-based collaboration

AI systems MAY log constraint violations if tracking is enabled (Section 6).

### 1.3 Constraint Scope

Constraints can be applied at multiple levels:

| Scope | Applies To | Example |
|-------|------------|---------|
| Project | All files | Default lock level in config |
| Directory | Files in directory | Pattern-based constraints |
| File | Single file | File header annotation |
| Symbol | Single function/class | Symbol annotation |

See [Inheritance & Cascade Specification](inheritance.md) for precedence rules.

### 1.4 Conformance

The key words "MUST", "MUST NOT", "REQUIRED", "SHALL", "SHALL NOT", "SHOULD", "SHOULD NOT", "RECOMMENDED", "MAY", and "OPTIONAL" in this document are to be interpreted as described in [RFC 2119](https://datatracker.ietf.org/doc/html/rfc2119).

---

## 2. Lock Constraints

Lock constraints control how freely AI can modify code.

### 2.1 Default Lock Level

**Files and symbols without explicit `@acp:lock` annotation default to `normal` level.**

This means:
- Absence of lock annotation permits unrestricted modification
- To restrict access, explicit annotation is required
- Default is permissive by design

### 2.2 Lock Levels

**Lock Levels (most to least restrictive):**

| Level | AI Behavior | Use Case |
|-------|-------------|----------|
| `frozen` | MUST NOT modify under any circumstances | Production config, security critical |
| `restricted` | MUST get explicit user approval before any change | Authentication, payment processing |
| `approval-required` | SHOULD get user approval for significant changes | Core business logic |
| `tests-required` | MUST add/update tests with changes | Complex logic, regression-prone code |
| `docs-required` | MUST update documentation with changes | Public APIs, user-facing features |
| `normal` | No special restrictions (default) | Most code |
| `experimental` | Encourage aggressive changes | Prototypes, proofs-of-concept |

### 2.3 Lock Annotation Syntax

#### Basic Lock

```
@acp:lock <level>
```

#### With Reason

```
@acp:lock <level>
@acp:lock-reason <explanation>
```

### 2.4 Lock Level Details

#### `frozen`

Code MUST NOT be modified under any circumstances.

**When to use:**
- Generated code (will be overwritten)
- Vendor/third-party code
- Cryptographic constants
- Legal/compliance text
- Production configuration

**AI Behavior:**
- MUST NOT suggest any changes
- MUST NOT include in refactoring
- MAY explain code if asked
- MAY suggest alternatives elsewhere

**Example:**
```javascript
/**
 * @acp:lock frozen
 * @acp:lock-reason "Production database credentials"
 */
const DB_CONFIG = { /* ... */ };
```

#### `restricted`

Limited modifications with explanation required. MUST get explicit user approval before any change.

**When to use:**
- Security-critical code
- Payment/financial processing
- Core business logic
- High-traffic hot paths

**AI Behavior:**
- MUST explain proposed changes before making them
- MUST get explicit user approval
- MUST use conservative approach
- SHOULD preserve existing patterns

**Example:**
```javascript
/**
 * @acp:lock restricted
 * @acp:lock-reason "Security-critical authentication logic"
 */
export function validateToken(token: string): boolean {
  // Security-critical code
}
```

#### `approval-required`

Changes need explicit user approval, but may be proposed proactively.

**When to use:**
- Shared utilities used across teams
- Public API contracts
- Configuration defaults

**AI Behavior:**
- SHOULD get user approval for significant changes
- MAY make minor changes without approval
- MUST explain reasoning

#### `tests-required`

All changes MUST include corresponding tests.

**When to use:**
- Core business logic
- Public APIs
- Functions with complex logic

**AI Behavior:**
- MUST add/update tests with any modification
- SHOULD update existing tests if behavior changes
- MAY create new test file if none exists

#### `docs-required`

All changes MUST update documentation.

**When to use:**
- Public APIs
- User-facing features
- Configuration options

**AI Behavior:**
- MUST update documentation with changes
- SHOULD update inline comments
- MAY update external documentation

#### `normal` (default)

No special restrictions.

**AI Behavior:**
- MAY modify freely based on user request
- SHOULD still follow best practices
- SHOULD still maintain code quality

#### `experimental`

Encourage aggressive changes. Changes are expected to be reversible.

**When to use:**
- Prototypes
- Proofs-of-concept
- Experimental features

**AI Behavior:**
- MAY make aggressive optimizations
- MAY try novel approaches
- SHOULD track changes for reversal

---

## 3. Style Constraints

Style constraints guide code formatting and conventions.

### 3.1 Style Annotation Syntax

#### Named Style Guide

```
@acp:style <guide-name> - <directive>
```

#### With Custom Rules

```
@acp:style <guide-name> - <directive>
@acp:style-rules <rule1>, <rule2>, ... - <directive>
```

#### With Style Inheritance (RFC-0002)

```
@acp:style <custom-guide> - <directive>
@acp:style-extends <parent-guide> - <directive>
```

### 3.2 Built-in Style Guides

The following style guide names are reserved and recognized by default:

| Guide Name          | Language   | Documentation URL                                            |
|---------------------|------------|--------------------------------------------------------------|
| `google-typescript` | TypeScript | https://google.github.io/styleguide/tsguide.html             |
| `google-javascript` | JavaScript | https://google.github.io/styleguide/jsguide.html             |
| `google-python`     | Python     | https://google.github.io/styleguide/pyguide.html             |
| `google-java`       | Java       | https://google.github.io/styleguide/javaguide.html           |
| `google-cpp`        | C++        | https://google.github.io/styleguide/cppguide.html            |
| `google-go`         | Go         | https://go.dev/doc/effective_go                              |
| `airbnb-javascript` | JavaScript | https://github.com/airbnb/javascript                         |
| `airbnb-react`      | React      | https://github.com/airbnb/javascript/tree/master/react       |
| `pep8`              | Python     | https://peps.python.org/pep-0008/                            |
| `black`             | Python     | https://black.readthedocs.io/en/stable/the_black_code_style/ |
| `prettier`          | Multi      | https://prettier.io/docs/en/options.html                     |
| `rustfmt`           | Rust       | https://rust-lang.github.io/rustfmt/                         |
| `standardjs`        | JavaScript | https://standardjs.com/rules.html                            |
| `tailwindcss-v3`    | CSS        | https://v2.tailwindcss.com/docs                              |

### 3.3 Custom Style Guides (RFC-0002)

Custom style guides can be defined in `.acp.config.json`:

```json
{
  "documentation": {
    "styleGuides": {
      "company-react": {
        "extends": "airbnb-react",
        "description": "Company React conventions",
        "rules": ["prefer-function-components", "max-line-length=120"],
        "filePatterns": ["src/components/**/*.tsx"]
      }
    }
  }
}
```

See [Config File Specification](config.md) Section 9 for complete style guide configuration.

### 3.4 Style Inheritance (RFC-0002)

The `@acp:style-extends` annotation specifies a parent style guide:

```typescript
/**
 * @acp:style company-standard - Follow our company coding standards
 * @acp:style-extends google-typescript - Based on Google TypeScript style
 * @acp:style-rules max-line-length=120 - Override specific rules
 */
```

**Inheritance rules:**
- Rules from parent guide apply first
- Child guide rules override parent rules
- Explicit `@acp:style-rules` override both

### 3.5 AI Behavior with Style

| Style Setting | AI Behavior |
|---------------|-------------|
| `@acp:style <guide>` | MUST follow specified style for new code |
| `@acp:style-extends <parent>` | MUST apply parent rules, then current rules |
| `@acp:style-rules <rules>` | MUST apply specific rules |
| No style specified | SHOULD follow surrounding code patterns |
| Conflicting styles | Symbol-level takes precedence over file-level |

**General Rules:**
- MUST follow specified style guide for new code
- SHOULD NOT reformat existing code unless asked
- MUST maintain consistency with surrounding code
- SHOULD note style violations in existing code

**Example:**
```javascript
/**
 * @acp:style google-typescript - Follow Google TypeScript style guide
 * @acp:style-rules max-line-length=100, no-any - Apply specific overrides
 */
```

---

## 4. Behavior Constraints

Behavior constraints modify how AI approaches code changes.

### 4.1 Behavior Annotation Syntax

```
@acp:behavior <approach>
```

### 4.2 Behavior Approaches

| Approach | Description | AI Behavior |
|----------|-------------|-------------|
| `conservative` | Minimize changes | Smallest working change, preserve patterns |
| `balanced` | Balance safety and functionality (default) | Reasonable changes, consider tradeoffs |
| `aggressive` | Optimize freely | May refactor, optimize, modernize |

**Example:**
```javascript
/**
 * @acp:behavior conservative
 */
```

---

## 5. Quality Constraints

Quality constraints specify additional checks needed.

### 5.1 Quality Annotation Syntax

```
@acp:quality <requirement1>, <requirement2>
```

### 5.2 Quality Requirements

**Common Requirements:**
- `security-review`: Requires security audit
- `performance-test`: Requires performance validation
- `manual-test`: Requires manual testing
- `regression-test`: Requires regression testing

**Example:**
```javascript
/**
 * @acp:quality security-review, performance-test
 */
```

---

## 6. Constraint Violations

### 6.1 Violation Tracking (Optional)

Implementations MAY provide optional constraint violation logging.

**Configuration** (in `.acp.config.json`):
```json
{
  "constraints": {
    "track_violations": true,
    "audit_file": ".acp.violations.log"
  }
}
```

**Log Format** (if enabled):
```json
{
  "timestamp": "2024-12-17T15:30:00Z",
  "file": "src/auth/session.ts",
  "constraint": "lock:frozen",
  "action": "modify",
  "user_override": false,
  "context": "AI attempted modification of frozen file"
}
```

Implementations MAY provide `--audit` flag to enable tracking for a session.

**Note**: Violation tracking is OPTIONAL. Implementations claiming conformance are not required to provide this feature.

---

## 7. Constraint Merging

When constraints exist at multiple precedence levels, they merge according to these rules:

### 7.1 Lock Levels

**Rule:** Most restrictive wins

**Example:**
- Project default: `normal`
- Directory: `approval-required`
- File: `restricted`
- **Result:** `restricted` (most restrictive)

**Precedence** (most to least restrictive):
```
frozen > restricted > approval-required > tests-required > docs-required > normal > experimental
```

### 7.2 Style Guides

**Rule:** Most specific **guide** wins, but rules from all levels **accumulate**

**Example:**
- Project: `google-typescript`
- Directory: custom rule `max-line-length=100`
- File: custom rule `indent=2`
- **Result:**
  - Base guide: `google-typescript` (from project)
  - Additional rules: `max-line-length=100`, `indent=2` (accumulated)

### 7.3 Behavior Constraints

**Rule:** Most specific wins completely (no merging)

**Example:**
- Project: `conservative`
- File: `aggressive`
- **Result:** `aggressive` (file-level overrides)

### 7.4 Quality Requirements

**Rule:** Requirements accumulate (all levels apply)

**Example:**
- Project: `tests-required`
- File: `security-review`
- **Result:** Both requirements apply

See [Inheritance & Cascade Specification](inheritance.md) for complete merging details.

---

## 8. Examples

### 8.1 Security-Critical File

```typescript
/**
 * @acp:module "Authentication Core"
 * @acp:domain authentication, security
 * @acp:lock restricted
 * @acp:lock-reason "Security-critical code. All changes require security review."
 * @acp:behavior conservative
 * @acp:quality security-review, tests-required
 */

export class AuthenticationService {
  /**
   * @acp:lock frozen
   * @acp:lock-reason "Cryptographic constant - never modify"
   */
  private static readonly HASH_ROUNDS = 12;

  /**
   * @acp:summary "Validates user credentials"
   */
  async validateCredentials(email: string, password: string): Promise<User | null> {
    // Implementation
  }
}
```

### 8.2 Generated Code

```typescript
/**
 * @acp:lock frozen
 * @acp:lock-reason "Auto-generated from schema.prisma. Edit schema instead."
 *
 * DO NOT EDIT - This file is generated by Prisma
 */

export interface User {
  id: string;
  email: string;
  // ...
}
```

### 8.3 Public API

```typescript
/**
 * @acp:module "Public API v2"
 * @acp:domain api
 * @acp:stability stable
 * @acp:lock approval-required
 * @acp:lock-reason "Public API - breaking changes affect customers"
 * @acp:quality documented
 */

export interface ApiResponse<T> {
  // Public API contract
}
```

---

## Appendix A: Quick Reference

| Constraint | Values | Default |
|------------|--------|---------|
| `@acp:lock` | frozen, restricted, approval-required, tests-required, docs-required, normal, experimental | normal |
| `@acp:style` | google-typescript, airbnb-javascript, prettier, pep8, custom, etc. | (none) |
| `@acp:style-rules` | key or key=value pairs | (none) |
| `@acp:style-extends` | parent style guide name (RFC-0002) | (none) |
| `@acp:behavior` | conservative, balanced, aggressive | balanced |
| `@acp:quality` | security-review, performance-test, manual-test, regression-test | (none) |

**Merging Rules:**
- Lock levels: Most restrictive wins
- Style guides: Most specific guide + accumulated rules
- Style extends: Parent rules first, child overrides
- Behavior: Most specific wins
- Quality: All requirements accumulate

---

## Appendix B: Related Documents

- [Annotation Syntax](annotations.md) - Annotation syntax definition
- [Cache Format](cache.md) - How constraints are indexed
- [Debug Sessions](debug-sessions.md) - Debug and hack tracking
- [Inheritance & Cascade](inheritance.md) - Constraint inheritance rules
- [Configuration](config.md) - Constraint configuration

---

*End of Constraint System Specification*

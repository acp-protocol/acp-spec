# Debug Sessions & Hack Tracking Specification

**ACP Version**: 1.0.0-revised
**Document Version**: 1.0.0
**Last Updated**: 2024-12-17
**Status**: Revised Draft

---

## Table of Contents

1. [Overview](#1-overview)
2. [Hack Markers](#2-hack-markers)
3. [Debug Sessions](#3-debug-sessions)
4. [Annotations](#4-annotations)
5. [Examples](#5-examples)

---

## 1. Overview

### 1.1 Purpose

Debug sessions and hack markers enable:

- **Reversible Changes**: Track experimental changes for easy rollback
- **Debugging History**: Record hypotheses, attempts, and outcomes
- **Temporary Code Tracking**: Mark and manage temporary fixes
- **Knowledge Preservation**: Document why changes were made

Debug sessions are persisted in `.acp/acp.attempts.json`. See [Section 3.4 of the main specification](../ACP-1.0.md#34-attempts-file-acpacpattemptsJson) for file format.

### 1.2 Use Cases

| Use Case | Feature | Benefit |
|----------|---------|---------|
| Debugging an issue | Debug Sessions | Track what was tried, what worked |
| Quick hotfix | Hack Markers | Remember to revisit and fix properly |
| Experimentation | Debug Sessions | Roll back failed experiments |
| Code review | Both | Show what changed and why |

### 1.3 Status

**Note**: Debug and hack feature detailed specifications are deferred to v1.1 per the main ACP specification changelog.

This document provides basic annotation syntax and usage. Full session management, rollback mechanisms, and storage formats will be specified in ACP v1.1.

### 1.4 Conformance

The key words "MUST", "MUST NOT", "REQUIRED", "SHALL", "SHALL NOT", "SHOULD", "SHOULD NOT", "RECOMMENDED", "MAY", and "OPTIONAL" in this document are to be interpreted as described in [RFC 2119](https://datatracker.ietf.org/doc/html/rfc2119).

---

## 2. Hack Markers

### 2.1 Purpose

Hack markers identify temporary, experimental, or workaround code that should be revisited.

**From the main specification (Section 5.6, Lines 1922-1929):**

Hack markers are annotations that flag temporary solutions or workarounds.

### 2.2 Hack Annotation

```
@acp:hack <reason>
```

**Example:**
```javascript
/**
 * @acp:hack "Workaround for API bug #123"
 */
function parseResponse(data) {
  // Workaround code
  if (isTuesday()) {
    return parseOldFormat(data);
  }
  return parseNewFormat(data);
}
```

### 2.3 Basic Usage

**Purpose:** Mark code that needs to be revisited or is a temporary solution.

**Common scenarios:**
- Workarounds for third-party bugs
- Quick fixes pending proper solution
- Experimental code
- Debug code to be removed

**Example:**
```python
# @acp:hack "Temporary fix until API is updated"
def process_data(input_data):
    # Workaround for legacy format
    if is_legacy_format(input_data):
        return convert_legacy(input_data)
    return process_normal(input_data)
```

---

## 3. Debug Sessions

### 3.1 Purpose

Debug sessions track the process of investigating and fixing an issue.

**From the main specification (Section 5.6, Lines 1910-1921):**

Debug sessions provide a way to track debugging attempts and hypotheses.

### 3.2 Debug Annotation

```
@acp:debug <session-identifier>
```

**Example:**
```javascript
/**
 * @acp:debug session-2024-12-17
 */
function validateToken(token) {
  console.log('Debug: validating token', token); // Debug code
  return verify(token);
}
```

### 3.3 Basic Usage

**Purpose:** Mark code added during debugging sessions.

**Common scenarios:**
- Temporary logging
- Debug assertions
- Test code
- Diagnostic checks

**Example:**
```typescript
/**
 * @acp:debug auth-investigation-001
 */
function debugAuthFlow(user: User) {
  console.log('User auth state:', {
    id: user.id,
    authenticated: user.authenticated,
    timestamp: Date.now()
  });
  // Debug investigation code
}
```

---

## 4. Annotations

### 4.1 Hack Annotations

From Appendix A of the main specification (Lines 1922-1929):

| Annotation | Parameters | Example | Description |
|------------|-----------|---------|-------------|
| `@acp:hack` | `<reason>` | `@acp:hack "Workaround for API bug #123"` | Explanation of temporary solution |

### 4.2 Debug Annotations

From Appendix A of the main specification (Lines 1910-1921):

| Annotation | Parameters | Example | Description |
|------------|-----------|---------|-------------|
| `@acp:debug` | `<session>` | `@acp:debug session-2024-12-17` | Debug session identifier |

### 4.3 Scope

Both annotations apply at **symbol level** (functions, classes, methods).

**Placement:**
- Before the symbol being marked
- As close to the temporary/debug code as possible

---

## 5. Examples

### 5.1 Hack Example

```javascript
/**
 * @acp:hack "API returns wrong timezone on weekends. Remove when API v2 deployed."
 */
function adjustTimezone(date) {
  const isWeekend = [0, 6].includes(date.getDay());
  if (isWeekend) {
    // Hack: Add 1 hour on weekends
    return new Date(date.getTime() + 3600000);
  }
  return date;
}
```

### 5.2 Debug Example

```typescript
/**
 * @acp:debug memory-leak-investigation
 */
function trackMemoryUsage() {
  if (process.env.DEBUG_MEMORY) {
    const used = process.memoryUsage();
    console.log('Memory:', {
      rss: Math.round(used.rss / 1024 / 1024) + 'MB',
      heapUsed: Math.round(used.heapUsed / 1024 / 1024) + 'MB'
    });
  }
}
```

### 5.3 Combined Example

```python
class PaymentProcessor:
    def process_payment(self, amount, currency):
        """
        @acp:debug payment-failures-2024-12
        @acp:hack "Extra validation until payment gateway bug fixed"
        """
        # Debug logging
        logger.debug(f"Processing payment: {amount} {currency}")

        # Hack: Extra validation for specific currency
        if currency == "EUR" and amount > 10000:
            # Workaround for gateway bug with large EUR amounts
            return self._process_in_batches(amount, currency)

        return self._process_normal(amount, currency)
```

### 5.4 Cache Storage (v1.1)

**Note**: Cache storage format will be fully specified in v1.1. Basic structure:

```json
{
  "constraints": {
    "hacks": [
      {
        "id": "hack-001",
        "file": "src/api/client.ts",
        "line": 45,
        "reason": "Workaround for API bug #123"
      }
    ],
    "debug_sessions": [
      {
        "id": "session-2024-12-17",
        "files": ["src/auth/session.ts"],
        "status": "active"
      }
    ]
  }
}
```

---

## Appendix A: Quick Reference

| Annotation | Level | Description |
|------------|-------|-------------|
| `@acp:hack <reason>` | Symbol | Mark temporary solution with explanation |
| `@acp:debug <session>` | Symbol | Mark debug code with session identifier |

**Best Practices:**
- Always provide a reason for hacks
- Use descriptive session identifiers for debug
- Remove debug code before production
- Track hack removal in issue tracker
- Document why the hack is necessary

---

## Appendix B: Related Documents

- [Annotation Syntax](annotations.md) - Annotation syntax definition
- [Constraint System](constraints.md) - How debug/hack relates to constraints
- [Cache Format](cache.md) - Storage in cache (v1.1)

---

## Appendix C: Future Enhancements (v1.1)

The following features are planned for ACP v1.1:

- **Hack Tracking**:
  - Expiration dates
  - Related tickets
  - Revert instructions
  - Automatic cleanup

- **Debug Sessions**:
  - Session lifecycle management
  - Attempt tracking
  - Rollback mechanisms
  - Result recording
  - Resolution tracking

- **CLI Support**:
  - `acp hacks list`
  - `acp debug start`
  - `acp debug resolve`
  - `acp hacks cleanup`

- **Cache Integration**:
  - Full session storage
  - Attempt history
  - Rollback metadata
  - Query interface

---

*End of Debug Sessions & Hack Tracking Specification*

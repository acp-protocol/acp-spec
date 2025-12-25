# How to Protect Critical Code

**Document Type**: How-To Guide  
**Status**: OUTLINE — Content to be added  
**Goal**: Use ACP lock levels to protect important code from AI modification  
**Prerequisites**: ACP CLI installed, basic annotation knowledge  
**Time**: 20-30 minutes

---

## The Problem

You have code that's critical to your application—security logic, payment processing, core algorithms—and you want to ensure AI coding assistants don't accidentally modify it.

---

## Solution Overview

1. Identify critical code categories
2. Choose appropriate lock levels
3. Apply annotations
4. Verify protection
5. Set up violation tracking (optional)

---

## Step 1: Identify Critical Code

> **TODO**: Add decision framework

### Category 1: Security-Critical

Code that handles:
- Authentication and authorization
- Cryptographic operations
- Session management
- Access control

**Recommendation**: `@acp:lock frozen`

### Category 2: Financial-Critical

Code that handles:
- Payment processing
- Billing calculations
- Financial reporting
- Audit trails

**Recommendation**: `@acp:lock frozen` or `restricted`

### Category 3: Compliance-Critical

Code that handles:
- GDPR/privacy compliance
- Healthcare (HIPAA)
- Regulatory reporting
- Legal requirements

**Recommendation**: `@acp:lock frozen`

### Category 4: Infrastructure-Critical

Code that handles:
- Database migrations
- Configuration management
- Deployment scripts
- Backup/restore

**Recommendation**: `@acp:lock restricted`

### Category 5: API-Critical

Code that handles:
- Public API contracts
- SDK interfaces
- Integration points

**Recommendation**: `@acp:lock restricted` with `@acp:stability stable`

---

## Step 2: Understand Lock Levels

> **TODO**: Add detailed behavior descriptions

| Level | Meaning | AI Behavior |
|-------|---------|-------------|
| `frozen` | Never modify | Skip entirely, don't suggest changes |
| `restricted` | Approval required | Warn before suggesting, flag for review |
| `approval-required` | Change review needed | Suggest with review request |
| `tests-required` | Must have tests | Verify test coverage before changing |
| `docs-required` | Must document | Ensure documentation exists |
| `review-required` | Code review needed | Flag for human review |
| `normal` | Standard code | Normal AI behavior |
| `experimental` | Expect changes | Full AI flexibility |

### Lock Level Selection Guide

```
Is this code security-critical?
├─ Yes → frozen
└─ No
   └─ Is unauthorized change catastrophic?
      ├─ Yes → restricted
      └─ No
         └─ Does it need human review?
            ├─ Yes → review-required
            └─ No → normal
```

---

## Step 3: Apply Annotations

> **TODO**: Add language-specific examples

### File-Level Protection

```typescript
// src/auth/session.ts
// @acp:lock frozen - Session management, security critical
// @acp:owner security-team - All changes require security review

export class SessionService {
  // ...
}
```

### Function-Level Protection

```typescript
// src/utils/helpers.ts
// @acp:lock normal - General utilities

export function formatDate(date: Date): string {
  // Normal code, AI can modify
}

// @acp:lock frozen - Cryptographic hash, security critical
export function hashPassword(password: string): string {
  // Frozen, AI must not modify
}
```

### Block-Level Protection

```typescript
// @acp:lock frozen - BEGIN payment calculation
function calculateTotal(items: Item[]): Money {
  // Critical calculation logic
}
// @acp:lock frozen - END
```

---

## Step 4: Add Context with Directives

> **TODO**: Expand with examples

The directive after the lock level explains WHY:

```typescript
// @acp:lock frozen - PCI-DSS compliant, audited annually
// @acp:lock frozen - Legacy edge case handling from PROD-4521
// @acp:lock frozen - Performance optimized, benchmarked
// @acp:lock restricted - Public API, breaking change requires deprecation
```

---

## Step 5: Verify Protection

> **TODO**: Add verification steps

### Check Frozen Files

```bash
acp query '.constraints.by_lock_level.frozen'
```

### Check Specific File

```bash
acp constraints src/auth/session.ts
```

Expected output:
```
File: src/auth/session.ts
━━━━━━━━━━━━━━━━━━━━━━━━

Lock Level: frozen
Reason: Session management, security critical
Owner: security-team

⚠️  This file should NOT be modified by AI assistants.
   Contact: @security-lead
```

### Test with AI

1. Open AI tool with ACP integration
2. Ask: "Refactor the SessionService class"
3. Expected: AI should decline or acknowledge constraint

---

## Step 6: Track Violations (Optional)

> **TODO**: Expand violation tracking

### Enable Tracking

```json
// .acp.config.json
{
  "constraints": {
    "track_violations": true,
    "audit_file": ".acp/violations.log"
  }
}
```

### Review Violations

```bash
cat .acp/violations.log
```

```
[2024-12-17T15:30:00Z] VIOLATION: Attempted modify src/auth/session.ts (frozen)
  Tool: cursor
  Operation: edit
  User: developer@example.com
```

---

## Patterns and Best Practices

> **TODO**: Expand each pattern

### Pattern 1: Frozen Core, Normal Periphery

```
src/payments/
├── core/
│   ├── processor.ts      # frozen
│   └── calculator.ts     # frozen
├── adapters/
│   ├── stripe.ts         # restricted
│   └── paypal.ts         # restricted
└── utils/
    └── formatting.ts     # normal
```

### Pattern 2: Graduated Permissions

```typescript
// @acp:lock frozen
export const CRITICAL_CONFIG = { ... };  // Never change

// @acp:lock restricted
export function processPayment() { ... } // Needs approval

// @acp:lock normal
export function formatReceipt() { ... }  // AI can modify
```

### Pattern 3: Time-Based Locks

```typescript
// @acp:lock frozen - Freeze until 2025-01-15 (audit period)
export function auditReport() { ... }
```

---

## Common Mistakes

> **TODO**: Add examples for each

### Mistake 1: Over-Freezing

Freezing too much code limits AI helpfulness.

**Solution**: Only freeze truly critical code.

### Mistake 2: Missing Directives

Lock without explanation:
```typescript
// @acp:lock frozen
```

**Solution**: Always explain why:
```typescript
// @acp:lock frozen - Handles PII, GDPR compliance requirement
```

### Mistake 3: Inconsistent Levels

Same type of code with different locks.

**Solution**: Create a lock level policy document.

---

## Verification Checklist

- [ ] All security-critical code is `frozen`
- [ ] All financial-critical code is `frozen` or `restricted`
- [ ] All compliance-critical code is `frozen`
- [ ] All public APIs are `restricted` with stability markers
- [ ] Directives explain WHY for all locks
- [ ] Owners are assigned for all restricted+ code
- [ ] Cache is regenerated after annotations
- [ ] AI tool respects constraints (tested)

---

## Related

- [Annotation Reference](../reference/annotations.md)
- [Tracking Constraint Violations](tracking-violations.md)
- [Setting Up Code Ownership](setting-up-ownership.md)

---

*This guide is an outline. [Contribute content →](https://github.com/acp-protocol/docs)*

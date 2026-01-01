# Complete ACP Example

**Updated for ACP 1.3.0 (RFC-0015)**

This document demonstrates a full-featured ACP setup with all major features.

---

## Project Overview

This example shows a typical web application with:
- Multiple domains (authentication, billing, api)
- Various constraint levels
- Complete annotation coverage
- Variables for token efficiency
- Full configuration

---

## Project Structure

```
ecommerce-api/
├── src/
│   ├── auth/
│   │   ├── session.ts        # Restricted - security critical
│   │   ├── jwt.ts            # Restricted - security critical
│   │   └── oauth.ts          # Approval required
│   ├── billing/
│   │   ├── payment.ts        # Frozen - payment processing
│   │   ├── subscription.ts   # Restricted
│   │   └── invoice.ts        # Normal
│   ├── api/
│   │   ├── routes.ts         # Normal
│   │   ├── middleware.ts     # Tests required
│   │   └── handlers.ts       # Normal
│   ├── db/
│   │   ├── connection.ts     # Frozen - database config
│   │   ├── models.ts         # Docs required
│   │   └── migrations.ts     # Frozen
│   └── utils/
│       ├── helpers.ts        # Experimental
│       └── validation.ts     # Normal
├── .acp.config.json
├── .acp.cache.json           # Generated
└── .acp.vars.json            # Generated
```

---

## Source Files with Annotations

### Authentication Domain

**`src/auth/session.ts`**
```typescript
/**
 * @acp:module "Session Management"
 * @acp:summary Handles user session lifecycle and validation
 * @acp:domain authentication
 * @acp:domain security
 * @acp:layer service
 * @acp:stability stable
 * @acp:lock restricted
 * @acp:lock-reason "Security critical - all changes require security review"
 * @acp:style google-typescript
 * @acp:behavior conservative
 * @acp:quality security-review
 * @acp:ref https://docs.example.com/auth/sessions
 */

import { verifyToken } from './jwt';
import { findSession, createSession } from '../db/sessions';

/**
 * @acp:summary Validates JWT token and returns session data
 */
export async function validateSession(token: string): Promise<Session | null> {
  const payload = await verifyToken(token);
  if (!payload) return null;
  return findSession(payload.sessionId);
}

/**
 * @acp:summary Creates a new user session
 * @acp:quality security-review
 */
export async function startSession(userId: string): Promise<Session> {
  return createSession({ userId, createdAt: new Date() });
}

/**
 * @acp:deprecated "Use invalidateAllSessions instead for better security"
 */
export async function endSession(sessionId: string): Promise<void> {
  // Legacy implementation
}

/**
 * @acp:summary Invalidates all sessions for a user
 */
export async function invalidateAllSessions(userId: string): Promise<void> {
  // Implementation
}
```

**`src/auth/jwt.ts`**
```typescript
/**
 * @acp:module "JWT Utilities"
 * @acp:summary JSON Web Token creation and verification
 * @acp:domain authentication
 * @acp:domain security
 * @acp:layer utility
 * @acp:stability stable
 * @acp:lock restricted
 * @acp:lock-reason "Cryptographic operations - requires expert review"
 * @acp:behavior conservative
 * @acp:quality security-review
 */

import * as jwt from 'jsonwebtoken';

/**
 * @acp:summary Verifies JWT token signature and expiration
 * @acp:lock frozen
 * @acp:lock-reason "Verified implementation - do not modify"
 */
export async function verifyToken(token: string): Promise<TokenPayload | null> {
  try {
    return jwt.verify(token, process.env.JWT_SECRET) as TokenPayload;
  } catch {
    return null;
  }
}

/**
 * @acp:summary Creates signed JWT token
 */
export function createToken(payload: TokenPayload): string {
  return jwt.sign(payload, process.env.JWT_SECRET, { expiresIn: '24h' });
}
```

### Billing Domain

**`src/billing/payment.ts`**
```typescript
/**
 * @acp:module "Payment Processing"
 * @acp:summary Handles payment transactions and refunds
 * @acp:domain billing
 * @acp:domain financial
 * @acp:layer service
 * @acp:stability stable
 * @acp:lock frozen
 * @acp:lock-reason "PCI-compliant payment processing - certified implementation"
 * @acp:style google-typescript
 * @acp:behavior conservative
 * @acp:quality security-review
 * @acp:quality manual-test
 * @acp:ref https://docs.stripe.com/api
 */

import Stripe from 'stripe';

const stripe = new Stripe(process.env.STRIPE_KEY);

/**
 * @acp:summary Processes a payment charge
 * @acp:lock frozen
 */
export async function processPayment(
  amount: number,
  currency: string,
  customerId: string
): Promise<PaymentResult> {
  // Implementation - DO NOT MODIFY
}

/**
 * @acp:summary Issues a refund for a payment
 * @acp:lock frozen
 */
export async function processRefund(
  paymentId: string,
  amount?: number
): Promise<RefundResult> {
  // Implementation - DO NOT MODIFY
}
```

**`src/billing/subscription.ts`**
```typescript
/**
 * @acp:module "Subscription Management"
 * @acp:summary Handles subscription lifecycle
 * @acp:domain billing
 * @acp:layer service
 * @acp:stability stable
 * @acp:lock restricted
 * @acp:lock-reason "Billing logic - requires finance team approval"
 * @acp:behavior conservative
 */

/**
 * @acp:summary Creates a new subscription
 */
export async function createSubscription(
  customerId: string,
  planId: string
): Promise<Subscription> {
  // Implementation
}

/**
 * @acp:summary Cancels an active subscription
 */
export async function cancelSubscription(
  subscriptionId: string,
  immediate: boolean = false
): Promise<void> {
  // Implementation
}
```

### API Layer

**`src/api/middleware.ts`**
```typescript
/**
 * @acp:module "API Middleware"
 * @acp:summary Request processing middleware
 * @acp:domain api
 * @acp:layer handler
 * @acp:stability stable
 * @acp:lock tests-required
 * @acp:lock-reason "Critical request path - changes must include tests"
 * @acp:style google-typescript
 */

import { validateSession } from '../auth/session';

/**
 * @acp:summary Authentication middleware
 * @acp:quality regression-test
 */
export async function authMiddleware(
  req: Request,
  res: Response,
  next: NextFunction
): Promise<void> {
  const token = req.headers.authorization?.replace('Bearer ', '');
  if (!token) {
    res.status(401).json({ error: 'No token provided' });
    return;
  }
  
  const session = await validateSession(token);
  if (!session) {
    res.status(401).json({ error: 'Invalid token' });
    return;
  }
  
  req.session = session;
  next();
}

/**
 * @acp:summary Rate limiting middleware
 */
export function rateLimitMiddleware(
  limit: number,
  windowMs: number
): Middleware {
  // Implementation
}
```

### Database Layer

**`src/db/connection.ts`**
```typescript
/**
 * @acp:module "Database Connection"
 * @acp:summary Database connection pool management
 * @acp:domain database
 * @acp:layer repository
 * @acp:stability stable
 * @acp:lock frozen
 * @acp:lock-reason "Production database configuration - never modify"
 * @acp:behavior conservative
 */

import { Pool } from 'pg';

/**
 * @acp:lock frozen
 */
export const pool = new Pool({
  connectionString: process.env.DATABASE_URL,
  max: 20,
  idleTimeoutMillis: 30000,
});

/**
 * @acp:lock frozen
 */
export async function query(sql: string, params?: any[]): Promise<QueryResult> {
  return pool.query(sql, params);
}
```

### Utilities

**`src/utils/helpers.ts`**
```typescript
/**
 * @acp:module "Helper Utilities"
 * @acp:summary General utility functions
 * @acp:domain utilities
 * @acp:layer utility
 * @acp:stability experimental
 * @acp:lock experimental
 * @acp:behavior aggressive
 */

/**
 * @acp:summary Formats date to ISO string
 * @acp:hack "Temporary workaround for timezone bug"
 * @acp:hack-ticket JIRA-456
 */
export function formatDate(date: Date): string {
  // Workaround implementation
  return date.toISOString().split('T')[0];
}

/**
 * @acp:summary Generates random ID
 */
export function generateId(): string {
  return Math.random().toString(36).substring(2);
}
```

---

## Configuration File

**`.acp.config.json`**
```json
{
  "version": "1.0.0",
  "include": [
    "src/**/*.ts"
  ],
  "exclude": [
    "**/*.test.ts",
    "**/*.spec.ts",
    "node_modules/**",
    "dist/**",
    "coverage/**"
  ],
  "error_handling": {
    "strictness": "permissive",
    "max_errors": 100,
    "auto_correct": false
  },
  "constraints": {
    "defaults": {
      "lock": "normal"
    },
    "track_violations": true,
    "audit_file": ".acp.violations.log"
  },
  "domains": {
    "authentication": {
      "patterns": ["src/auth/**"],
      "description": "User authentication and session management"
    },
    "billing": {
      "patterns": ["src/billing/**"],
      "description": "Payment and subscription processing"
    },
    "api": {
      "patterns": ["src/api/**"],
      "description": "API routes and middleware"
    },
    "database": {
      "patterns": ["src/db/**"],
      "description": "Database access layer"
    }
  },
  "call_graph": {
    "include_stdlib": false,
    "max_depth": null,
    "exclude_patterns": ["**/test/**"]
  },
  "limits": {
    "max_file_size_mb": 10,
    "max_files": 100000,
    "max_annotations_per_file": 1000,
    "max_cache_size_mb": 100
  }
}
```

---

## Generated Variables File

**`.acp.vars.json`**
```json
{
  "version": "1.0.0",
  "variables": {
    "SYM_VALIDATE_SESSION": {
      "type": "symbol",
      "value": "src/auth/session.ts:validateSession",
      "description": "Validates JWT token and returns session data"
    },
    "SYM_VERIFY_TOKEN": {
      "type": "symbol",
      "value": "src/auth/jwt.ts:verifyToken",
      "description": "Verifies JWT token signature and expiration"
    },
    "SYM_PROCESS_PAYMENT": {
      "type": "symbol",
      "value": "src/billing/payment.ts:processPayment",
      "description": "Processes a payment charge"
    },
    "SYM_AUTH_MIDDLEWARE": {
      "type": "symbol",
      "value": "src/api/middleware.ts:authMiddleware",
      "description": "Authentication middleware"
    },
    "FILE_SESSION": {
      "type": "file",
      "value": "src/auth/session.ts",
      "description": "Session Management - user session lifecycle"
    },
    "FILE_PAYMENT": {
      "type": "file",
      "value": "src/billing/payment.ts",
      "description": "Payment Processing - transactions and refunds"
    },
    "DOM_AUTH": {
      "type": "domain",
      "value": "authentication",
      "description": "User authentication and session management"
    },
    "DOM_BILLING": {
      "type": "domain",
      "value": "billing",
      "description": "Payment and subscription processing"
    }
  }
}
```

---

## Generated Cache File (Excerpt)

**`.acp.cache.json`** (partial)
```json
{
  "version": "1.1.0",
  "generated_at": "2026-01-01T15:30:00Z",
  "git_commit": "abc123def456789",
  "project": {
    "name": "ecommerce-api",
    "root": "/home/user/ecommerce-api",
    "description": "E-commerce API with authentication and billing"
  },
  "stats": {
    "files": 10,
    "symbols": 25,
    "lines": 450,
    "primary_language": "TypeScript",
    "languages": [
      { "name": "TypeScript", "files": 10, "percentage": 100 }
    ]
  },
  "conventions": {
    "file_naming": [
      { "directory": "src/auth", "pattern": "*.ts", "confidence": 1.0 },
      { "directory": "src/billing", "pattern": "*.ts", "confidence": 1.0 },
      { "directory": "src/api", "pattern": "*.ts", "confidence": 1.0 }
    ],
    "imports": {
      "module_system": "esm",
      "path_style": "relative"
    }
  },
  "source_files": {
    "src/auth/session.ts": "2026-01-01T14:00:00Z",
    "src/auth/jwt.ts": "2026-01-01T14:00:00Z",
    "src/billing/payment.ts": "2025-12-15T10:00:00Z",
    "src/api/middleware.ts": "2026-01-01T12:00:00Z"
  },
  "files": {
    "src/auth/session.ts": {
      "path": "src/auth/session.ts",
      "module": "Session Management",
      "summary": "Handles user session lifecycle and validation",
      "lines": 45,
      "language": "typescript",
      "domains": ["authentication", "security"],
      "layer": "service",
      "stability": "stable",
      "exports": [
        "src/auth/session.ts:validateSession",
        "src/auth/session.ts:startSession",
        "src/auth/session.ts:endSession",
        "src/auth/session.ts:invalidateAllSessions"
      ],
      "imports": ["./jwt", "../db/sessions"],
      "imported_by": ["src/api/middleware.ts"]
    },
    "src/billing/payment.ts": {
      "path": "src/billing/payment.ts",
      "module": "Payment Processing",
      "summary": "Handles payment transactions and refunds",
      "lines": 60,
      "language": "typescript",
      "domains": ["billing", "financial"],
      "layer": "service",
      "stability": "stable",
      "exports": [
        "src/billing/payment.ts:processPayment",
        "src/billing/payment.ts:processRefund"
      ],
      "imports": ["stripe"],
      "imported_by": ["src/api/handlers.ts"]
    }
  },
  "symbols": {
    "src/auth/session.ts:validateSession": {
      "name": "validateSession",
      "qualified_name": "src/auth/session.ts:validateSession",
      "type": "function",
      "file": "src/auth/session.ts",
      "lines": [18, 23],
      "signature": "(token: string) => Promise<Session | null>",
      "summary": "Validates JWT token and returns session data",
      "async": true,
      "exported": true,
      "visibility": "public",
      "calls": [
        "src/auth/jwt.ts:verifyToken",
        "src/db/sessions.ts:findSession"
      ],
      "called_by": [
        "src/api/middleware.ts:authMiddleware"
      ]
    },
    "src/auth/jwt.ts:verifyToken": {
      "name": "verifyToken",
      "qualified_name": "src/auth/jwt.ts:verifyToken",
      "type": "function",
      "file": "src/auth/jwt.ts",
      "lines": [20, 28],
      "signature": "(token: string) => Promise<TokenPayload | null>",
      "summary": "Verifies JWT token signature and expiration",
      "async": true,
      "exported": true,
      "visibility": "public",
      "calls": [],
      "called_by": [
        "src/auth/session.ts:validateSession"
      ]
    }
  },
  "graph": {
    "forward": {
      "src/auth/session.ts:validateSession": [
        "src/auth/jwt.ts:verifyToken",
        "src/db/sessions.ts:findSession"
      ],
      "src/api/middleware.ts:authMiddleware": [
        "src/auth/session.ts:validateSession"
      ]
    },
    "reverse": {
      "src/auth/jwt.ts:verifyToken": [
        "src/auth/session.ts:validateSession"
      ],
      "src/auth/session.ts:validateSession": [
        "src/api/middleware.ts:authMiddleware"
      ],
      "src/db/sessions.ts:findSession": [
        "src/auth/session.ts:validateSession"
      ]
    }
  },
  "domains": {
    "authentication": {
      "name": "authentication",
      "description": "User authentication and session management",
      "files": [
        "src/auth/session.ts",
        "src/auth/jwt.ts",
        "src/auth/oauth.ts"
      ],
      "symbols": [
        "src/auth/session.ts:validateSession",
        "src/auth/session.ts:startSession",
        "src/auth/jwt.ts:verifyToken",
        "src/auth/jwt.ts:createToken"
      ]
    },
    "billing": {
      "name": "billing",
      "description": "Payment and subscription processing",
      "files": [
        "src/billing/payment.ts",
        "src/billing/subscription.ts",
        "src/billing/invoice.ts"
      ],
      "symbols": [
        "src/billing/payment.ts:processPayment",
        "src/billing/payment.ts:processRefund",
        "src/billing/subscription.ts:createSubscription"
      ]
    }
  },
  "constraints": {
    "by_file": {
      "src/auth/session.ts": {
        "lock_level": "restricted",
        "lock_reason": "Security critical - all changes require security review",
        "style": "google-typescript",
        "behavior": "conservative",
        "quality": ["security-review"]
      },
      "src/billing/payment.ts": {
        "lock_level": "frozen",
        "lock_reason": "PCI-compliant payment processing - certified implementation",
        "style": "google-typescript",
        "behavior": "conservative",
        "quality": ["security-review", "manual-test"]
      },
      "src/db/connection.ts": {
        "lock_level": "frozen",
        "lock_reason": "Production database configuration - never modify"
      },
      "src/api/middleware.ts": {
        "lock_level": "tests-required",
        "lock_reason": "Critical request path - changes must include tests",
        "style": "google-typescript"
      },
      "src/utils/helpers.ts": {
        "lock_level": "experimental",
        "behavior": "aggressive"
      }
    },
    "by_lock_level": {
      "frozen": [
        "src/billing/payment.ts",
        "src/db/connection.ts",
        "src/db/migrations.ts"
      ],
      "restricted": [
        "src/auth/session.ts",
        "src/auth/jwt.ts",
        "src/billing/subscription.ts"
      ],
      "tests-required": [
        "src/api/middleware.ts"
      ],
      "experimental": [
        "src/utils/helpers.ts"
      ]
    }
  }
}
```

---

## Query Examples

### Constraint Queries

```bash
# Get all frozen files
jq '.constraints.by_lock_level.frozen' .acp.cache.json
# ["src/billing/payment.ts", "src/db/connection.ts", "src/db/migrations.ts"]

# Check before modifying a file
jq '.constraints.by_file["src/auth/session.ts"]' .acp.cache.json
# {
#   "lock_level": "restricted",
#   "lock_reason": "Security critical - all changes require security review",
#   "style": "google-typescript",
#   "behavior": "conservative",
#   "quality": ["security-review"]
# }

# Find files requiring tests
jq '.constraints.by_lock_level["tests-required"]' .acp.cache.json
```

### Domain Queries

```bash
# Get authentication domain files
jq '.domains["authentication"].files' .acp.cache.json

# Get all billing symbols
jq '.domains["billing"].symbols' .acp.cache.json
```

### Call Graph Queries

```bash
# What does validateSession call?
jq '.graph.forward["src/auth/session.ts:validateSession"]' .acp.cache.json
# ["src/auth/jwt.ts:verifyToken", "src/db/sessions.ts:findSession"]

# What calls verifyToken?
jq '.graph.reverse["src/auth/jwt.ts:verifyToken"]' .acp.cache.json
# ["src/auth/session.ts:validateSession"]
```

### Variable Usage

Using variables in prompts:

```
Fix the bug in $SYM_VALIDATE_SESSION that causes 401 errors.
Check if $SYM_AUTH_MIDDLEWARE is calling it correctly.
```

Expands to:

```
Fix the bug in validateSession (src/auth/session.ts:18-23) - Validates JWT token
that causes 401 errors.
Check if authMiddleware (src/api/middleware.ts:15-35) - Authentication middleware
is calling it correctly.
```

---

## RFC-0015: Primer and Context Commands

### Generate Tiered Primer

```bash
# Get standard tier AI context
acp primer --budget 500

# Get full tier with JSON output
acp primer --budget 800 --json

# Preview tier selection
acp primer --budget 600 --explain --preview
```

### Operation-Specific Context

```bash
# Before creating a new file - get naming conventions
acp context create --directory src/auth

# Before modifying a file - get constraints and dependencies
acp context modify --file src/auth/session.ts

# Debug session - get related files and symbols
acp context debug --file src/auth/session.ts

# Explore project structure
acp context explore
```

### Example Context Output

```bash
$ acp context modify --file src/auth/session.ts
```

```
File: src/auth/session.ts
Lock Level: restricted
Lock Reason: Security critical - all changes require security review
Style: google-typescript
Behavior: conservative

Imported By:
  - src/api/middleware.ts

Related Symbols:
  - validateSession (lines 18-23)
  - startSession (lines 26-29)

Action Required: Explain proposed changes and wait for approval
```

---

## AI Workflow Example

*(Updated for RFC-0015)*

1. **User Request:** "Fix the authentication bug causing 401 errors"

2. **AI Gets Context (RFC-0015):**
   ```bash
   acp context modify --file src/auth/session.ts
   ```

3. **AI Checks Response:**
   - `lock_level: restricted`
   - `imported_by: ["src/api/middleware.ts"]`
   - `action_required: approval`

4. **AI Response:**
   ```
   I found the authentication domain files. Before modifying src/auth/session.ts,
   I need to note that this file is marked as "restricted" because it's
   security-critical code.

   This file is imported by src/api/middleware.ts, so changes may affect
   the authentication middleware.

   I'll explain the proposed changes first:
   [detailed explanation]

   Do you approve these changes?
   ```

---

*See [Edge Cases](edge-cases.md) for unusual scenarios and gotchas.*
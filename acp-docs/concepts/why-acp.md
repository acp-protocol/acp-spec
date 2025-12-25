# Why ACP? Understanding the AI Context Problem

**Document Type**: Explanation  
**Audience**: Developers evaluating ACP, Decision-makers  
**Reading Time**: 15 minutes

---

## Executive Summary

AI coding assistants are fundamentally limited by their inability to understand the *context* of your codebase—which code is critical, how components relate, and what constraints apply. 

Existing solutions like semantic search (Cursor's codebase indexing, Greptile, RAG) use **probabilistic retrieval**—they might find relevant context, but can't guarantee it. For constraints like "don't modify this security-critical file," you need **deterministic guarantees**.

ACP solves this by providing structured, pre-computed context that's always available when needed—not retrieved by similarity, but queried by exact match.

---

## The Context Gap

### What AI Assistants See

When an AI coding assistant analyzes your codebase, it sees:
- Syntax and structure
- Variable names and comments
- Import relationships
- Test coverage (maybe)

### What AI Assistants Don't See

But it misses the crucial context that lives in your team's heads:

| Context Type | Example | What AI Misses |
|--------------|---------|----------------|
| **Criticality** | Payment processing code | "This code handles $2M daily—don't touch without review" |
| **Ownership** | Security module | "Security team must approve all changes" |
| **Stability** | Public API | "This is stable API—breaking changes require deprecation cycle" |
| **Architecture** | Domain boundaries | "Auth and Billing should never directly call each other" |
| **History** | Legacy code | "This looks ugly but handles 47 edge cases we found in production" |

### The Consequences

Without this context, AI assistants make predictable mistakes:

1. **Dangerous Suggestions** — Proposing changes to frozen, production-critical code
2. **Architecture Violations** — Ignoring layer boundaries and domain separation
3. **Lost Knowledge** — Suggesting "improvements" that break carefully-designed edge case handling
4. **Wasted Time** — Developers constantly correcting AI suggestions

---

## The Alternatives Landscape

Before diving into how ACP solves this, let's examine the existing approaches and their limitations.

### Alternative 1: Cursor's Codebase Indexing

**How it works**: Cursor indexes your codebase using embeddings, then retrieves "relevant" code chunks via semantic similarity search when you ask questions.

**The appeal**: Automatic, no annotation required, finds related code.

**The limitations**:

| Issue | Description |
|-------|-------------|
| **Similarity ≠ Relevance** | The code most similar to your query isn't necessarily the code with constraints you need to respect |
| **No constraint guarantee** | If you ask "help me optimize auth," the frozen `session.ts` might not be in the top-k results |
| **Code looks alike** | `validateUser()`, `validatePayment()`, `validateOrder()` embed nearly identically |
| **Constraints are metadata** | "Don't modify this file" isn't semantically similar to the code itself |

**Example failure case**:
```
User: "Help me refactor the authentication flow"

Cursor retrieves: auth/login.ts, auth/register.ts, auth/password-reset.ts
Cursor misses: auth/session.ts (frozen, security-critical)

Why? session.ts handles JWT validation—semantically distant from 
"authentication flow" even though it's the most critical file to avoid.
```

### Alternative 2: Greptile

**How it works**: External service that indexes your repo, provides semantic code search and Q&A capabilities.

**The appeal**: Powerful search, understands code relationships, works across repos.

**The limitations**:

| Issue | Description |
|-------|-------------|
| **Same embedding problem** | Semantic similarity doesn't surface constraints |
| **External dependency** | Your codebase context lives on someone else's servers |
| **No constraint layer** | Greptile finds code, but doesn't understand "frozen" vs "modifiable" |
| **Latency** | External API calls on every query |
| **Cost scaling** | Pricing tied to repo size and query volume |

**Example failure case**:
```
User: "What code handles payment processing?"

Greptile returns: payments/processor.ts, payments/gateway.ts, billing/invoice.ts

Missing context: processor.ts is @acp:lock frozen with PCI-DSS compliance 
requirements. Greptile found the code but not the constraint.
```

### Alternative 3: Generic RAG (Retrieval-Augmented Generation)

**How it works**: Chunk your codebase, embed chunks, store in vector database, retrieve top-k similar chunks per query.

**The appeal**: Flexible, works with any content, well-understood technology.

**The limitations**:

| Issue | Description |
|-------|-------------|
| **Chunking destroys context** | File-level constraints split across chunks |
| **Embedding collapse** | Structurally similar code (common in codebases) clusters together |
| **Top-k misses** | Critical constraint might be at position 15, you retrieve top 10 |
| **Token overhead** | Every request incurs retrieval + context injection cost |
| **No prioritization** | "Frozen file" and "normal file" weighted equally by similarity |

**The math problem**:
```
Your codebase: 1,000 files
Frozen files: 10 (1%)
Top-k retrieval: 20 chunks

Probability frozen file is retrieved for random query: ~2%
Probability it's missed: ~98%

For constraints, 98% miss rate is unacceptable.
```

### Alternative 4: MCP (Model Context Protocol)

**How it works**: Servers provide resources, tools, and prompts that AI assistants can access dynamically.

**The appeal**: Standardized protocol, rich integrations, dynamic data access.

**The repurposing trend**: Developers have started building MCP servers to inject coding rules, documentation constraints, and "don't touch these files" directives—essentially using MCP as a context/guardrails system.

**The limitations for this repurposed use**:

| Issue | Description |
|-------|-------------|
| **Always-on injection** | Full context injected every request, even when irrelevant |
| **No constraint semantics** | MCP provides data, not rules—AI must interpret what "frozen" means |
| **Custom formats** | Every rules server uses different data structures |
| **No versioning** | State is ephemeral, not git-tracked |
| **Token overhead** | 2,000 tokens of rules injected on "what's the weather?" |

**Example**:
```
MCP rules server injects on every request:
- Coding standards (500 tokens)
- Frozen files list (200 tokens)  
- Architecture docs (1,000 tokens)
- Style guide (300 tokens)

User asks: "Write a haiku about cats"
AI receives: [2,000 tokens of irrelevant rules] + query

vs ACP:
AI receives: [40 token bootstrap] + query
(AI can request full context when actually doing code work)
```

**Note**: MCP is excellent for its intended purpose (dynamic capabilities). The issue is repurposing it for static constraints. See [ACP vs MCP](acp-vs-mcp.md) for the full comparison.

### Alternative 5: @codebase / Include Everything

**How it works**: Include your entire codebase (or large chunks) in the context window.

**The appeal**: Simple, comprehensive, no retrieval errors.

**The limitations**:

| Issue | Description |
|-------|-------------|
| **Token explosion** | 100k LOC = millions of tokens, exceeds context limits |
| **No prioritization** | Critical constraints buried in noise |
| **Cost** | Massive token usage per request |
| **Latency** | Processing time scales with context size |
| **Still no structure** | Raw code doesn't convey "frozen" or "restricted" |

---

## Why Existing Solutions Fall Short: The Fundamental Problem

All similarity-based approaches share a fundamental flaw for constraint handling:

```
┌─────────────────────────────────────────────────────────────────┐
│                    THE RETRIEVAL PROBLEM                         │
├─────────────────────────────────────────────────────────────────┤
│                                                                  │
│  Query: "Help me improve the authentication system"              │
│                                                                  │
│  Semantic Similarity Ranking:                                    │
│  ┌────────────────────────────────────────────────────────────┐ │
│  │ 1. auth/login.ts           (0.92 similarity)  - normal     │ │
│  │ 2. auth/register.ts        (0.89 similarity)  - normal     │ │
│  │ 3. auth/password-reset.ts  (0.87 similarity)  - normal     │ │
│  │ 4. auth/oauth.ts           (0.85 similarity)  - normal     │ │
│  │ 5. auth/middleware.ts      (0.83 similarity)  - restricted │ │
│  │ ...                                                         │ │
│  │ 12. auth/session.ts        (0.71 similarity)  - FROZEN     │ │
│  │ 15. auth/crypto.ts         (0.68 similarity)  - FROZEN     │ │
│  └────────────────────────────────────────────────────────────┘ │
│                                                                  │
│  Top-5 retrieval: ✓ Gets relevant code                          │
│                   ✗ Misses FROZEN constraints                    │
│                                                                  │
│  The most CRITICAL files have LOWER similarity because:          │
│  - They handle lower-level concerns (JWT, crypto)                │
│  - Their purpose is "validation" not "authentication flow"       │
│  - Constraint metadata isn't in the embedding                    │
│                                                                  │
└─────────────────────────────────────────────────────────────────┘
```

### The Code Similarity Problem

Code has unique characteristics that break semantic search assumptions:

**1. Structural Homogeneity**

```typescript
// These embed almost identically:
function validateUser(user: User): boolean { ... }
function validatePayment(payment: Payment): boolean { ... }
function validateOrder(order: Order): boolean { ... }

// But they have very different constraints:
// validateUser - normal
// validatePayment - FROZEN (PCI compliance)
// validateOrder - restricted
```

**2. Pattern Replication**

```typescript
// Service pattern repeated across domains:
class UserService { async create() { ... } async update() { ... } }
class PaymentService { async create() { ... } async update() { ... } }
class OrderService { async create() { ... } async update() { ... } }

// Embeddings cluster together, constraints differ wildly
```

**3. Naming Convention Collision**

```typescript
// All of these are "handlers":
authHandler.ts      // frozen
paymentHandler.ts   // frozen  
userHandler.ts      // normal
orderHandler.ts     // normal

// Semantic search can't distinguish criticality from names
```

**4. Constraints Are Orthogonal to Code Content**

The statement "this file is frozen" has no semantic relationship to the code itself. A file containing:
```typescript
export const JWT_SECRET = process.env.JWT_SECRET;
```

...should be frozen, but nothing about the code content indicates that. The constraint is *metadata about* the code, not *in* the code.

---

## How ACP Solves This

### The Core Insight: Deterministic > Probabilistic for Constraints

ACP takes a fundamentally different approach:

| Aspect | Similarity-Based | ACP |
|--------|-----------------|-----|
| **Retrieval** | Probabilistic (might find) | Deterministic (always available) |
| **Constraint visibility** | ~80% (top-k dependent) | 100% (structured query) |
| **Query type** | "Find similar" | "Get constraints for X" |
| **Token cost** | Per-request retrieval | One-time indexing |
| **Versioning** | Ephemeral | Git-integrated |

### Structured, Machine-Readable Annotations

ACP annotations are designed for exact-match queries, not similarity search:

```typescript
// @acp:lock frozen - Critical payment logic, DO NOT modify
// @acp:domain payments - Payment processing domain
// @acp:owner payments-team - Requires payments team approval

export class PaymentProcessor {
  // @acp:fn "Processes credit card transactions"
  processCard(card: CardDetails, amount: Money): Result {
    // ...
  }
}
```

### Efficient File Comprehension

ACP's annotation structure enables AI to understand files without reading all the code:

| Level | What AI Reads | What AI Learns |
|-------|---------------|----------------|
| **Header** | First 5-10 lines | File purpose, constraints, ownership, domain |
| **Inline markers** | `@acp:fn`, `@acp:sym` tags | Where important code lives |
| **Cache** | Pre-computed JSON | Everything, without reading the file |

**Without ACP**: AI reads 500 lines to understand a file.
**With ACP headers**: AI reads 10 lines and knows the file's role.
**With ACP cache**: AI already knows—goes directly to line 45.
```

### Pre-Computed Cache with Guaranteed Access

```json
{
  "constraints": {
    "by_lock_level": {
      "frozen": ["src/auth/session.ts", "src/payments/processor.ts"],
      "restricted": ["src/api/public/v1.ts"]
    }
  }
}
```

**Query**: "What files are frozen?"
**Result**: Exact list, 100% complete, zero retrieval uncertainty.

### On-Demand Context Injection

Unlike MCP (full context always) or RAG (per-query retrieval), ACP uses a minimal bootstrap plus on-demand expansion:

```
┌─────────────────────────────────────────────────────────────────┐
│                    CONTEXT INJECTION COMPARISON                  │
├─────────────────────────────────────────────────────────────────┤
│                                                                  │
│  MCP Approach (repurposed for rules):                            │
│  ┌──────────────────────────────────────────────────────────┐   │
│  │ Request 1: [ALL rules ~2000 tok] + query                 │   │
│  │ Request 2: [ALL rules ~2000 tok] + query                 │   │
│  │ Request 3: [ALL rules ~2000 tok] + query                 │   │
│  └──────────────────────────────────────────────────────────┘   │
│  Token cost: O(n) per request, regardless of relevance           │
│                                                                  │
│  RAG Approach:                                                   │
│  ┌──────────────────────────────────────────────────────────┐   │
│  │ Request 1: [retrieve] + [inject top-k] + query           │   │
│  │ Request 2: [retrieve] + [inject top-k] + query           │   │
│  │ Request 3: [retrieve] + [inject top-k] + query           │   │
│  └──────────────────────────────────────────────────────────┘   │
│  Token cost: O(k) per request + retrieval latency               │
│  Accuracy: probabilistic (may miss constraints)                  │
│                                                                  │
│  ACP Approach:                                                   │
│  ┌──────────────────────────────────────────────────────────┐   │
│  │ Bootstrap: [~40 tok] "ACP active. Frozen: x, y. Use      │   │
│  │            `acp primer` for details."                     │   │
│  │                                                           │   │
│  │ Request 1: [bootstrap ~40 tok] + "what's the weather?"   │   │
│  │            → AI knows ACP exists, doesn't need details   │   │
│  │                                                           │   │
│  │ Request 2: [bootstrap ~40 tok] + "refactor auth"         │   │
│  │            → AI requests: acp primer --domain auth       │   │
│  │            → Gets ~500 tok of relevant context           │   │
│  │                                                           │   │
│  │ Request 3: [bootstrap ~40 tok] + "explain $SYM_VALIDATOR"│   │
│  │            → AI expands variable on-demand               │   │
│  └──────────────────────────────────────────────────────────┘   │
│  Token cost: O(1) bootstrap + O(1) per expansion (when needed)  │
│  Accuracy: deterministic (AI always knows constraints exist)    │
│                                                                  │
└─────────────────────────────────────────────────────────────────┘
```

---

## The ACP Advantage: Detailed Comparison

### ACP vs Cursor Codebase Indexing

| Aspect | Cursor Codebase | ACP |
|--------|-----------------|-----|
| **Constraint visibility** | Probabilistic | Guaranteed |
| **Setup** | Automatic | Annotations required |
| **Accuracy for constraints** | ~80% | 100% |
| **Token efficiency** | Per-query retrieval | Pre-computed |
| **Git integration** | Limited | Native |
| **Customization** | Limited | Full control |

**When to use Cursor Codebase**: Finding similar code, exploring unfamiliar areas
**When to use ACP**: Protecting critical code, enforcing constraints

**Best practice**: Use both—Cursor for exploration, ACP for constraints.

### ACP vs Greptile

| Aspect | Greptile | ACP |
|--------|----------|-----|
| **Hosting** | External service | Local/self-hosted |
| **Constraint system** | None | Native |
| **Query type** | Natural language | Structured + NL |
| **Cost model** | Per-query/repo size | Free (open source) |
| **Offline support** | No | Yes |
| **Privacy** | Code sent to service | Code stays local |

**When to use Greptile**: Cross-repo search, team knowledge base
**When to use ACP**: Critical constraints, privacy-sensitive codebases

### ACP vs Generic RAG

| Aspect | RAG | ACP |
|--------|-----|-----|
| **Chunking** | Required (lossy) | Not needed |
| **Retrieval** | Top-k similarity | Exact-match query |
| **Constraint priority** | Equal to all content | First-class citizen |
| **Infrastructure** | Vector DB required | JSON file |
| **Maintenance** | Re-embed on changes | Re-index on changes |

**When to use RAG**: Documentation search, knowledge retrieval
**When to use ACP**: Code constraints, structure metadata

### ACP vs MCP (for constraints/context)

| Aspect | MCP (repurposed) | ACP |
|--------|------------------|-----|
| **Designed for** | Dynamic capabilities | Codebase constraints |
| **Injection model** | Full context, always | Bootstrap + on-demand |
| **Token efficiency** | ~2000 tok/request | ~40 tok + expansions |
| **Constraint semantics** | None (custom) | Native (standardized) |
| **Versioning** | Ephemeral | Git-integrated |

**When to use MCP**: File I/O, external APIs, command execution, database queries
**When to use ACP**: Codebase constraints, architecture awareness, ownership tracking

**Best practice**: Use MCP for capabilities, ACP for constraints—see [ACP vs MCP](acp-vs-mcp.md).

---

## The Complementary Ecosystem

ACP doesn't replace other tools—it fills the constraint gap they can't address:

```
┌─────────────────────────────────────────────────────────────────┐
│                    AI CODING ASSISTANT CONTEXT                   │
├─────────────────────────────────────────────────────────────────┤
│                                                                  │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐              │
│  │   Cursor    │  │  Greptile   │  │    MCP      │              │
│  │  Codebase   │  │             │  │             │              │
│  │             │  │ "Find code" │  │ "Do things" │              │
│  │ "Find       │  │             │  │             │              │
│  │  similar"   │  │ Semantic    │  │ External    │              │
│  │             │  │ search      │  │ tools       │              │
│  └──────┬──────┘  └──────┬──────┘  └──────┬──────┘              │
│         │                │                │                      │
│         └────────────────┼────────────────┘                      │
│                          │                                       │
│                          ▼                                       │
│                   ┌─────────────┐                                │
│                   │     ACP     │                                │
│                   │             │                                │
│                   │ "What are   │                                │
│                   │  the rules?"│                                │
│                   │             │                                │
│                   │ Constraints │                                │
│                   │ Domains     │                                │
│                   │ Ownership   │                                │
│                   └─────────────┘                                │
│                                                                  │
│  Together: AI that finds code, respects constraints,             │
│            and has dynamic capabilities                          │
│                                                                  │
└─────────────────────────────────────────────────────────────────┘
```

---

## Real-World Impact

### Before ACP

A developer asks the AI: "Help me optimize the authentication flow."

**With Cursor Codebase alone**:
- Retrieves: `login.ts`, `register.ts`, `oauth.ts` (high similarity)
- Misses: `session.ts` (frozen), `crypto.ts` (frozen)
- AI suggests refactoring session validation (violation!)
- Developer spends 30 minutes explaining why suggestions don't work

### After ACP

The same request with ACP:

**Primer includes**:
```
Frozen files (DO NOT MODIFY):
- src/auth/session.ts - Security critical
- src/auth/crypto.ts - Cryptographic operations
```

**AI behavior**:
- Sees constraint, avoids frozen files
- Suggests optimizations only in safe areas
- Flags when changes would require frozen file modifications

**Result**: Developer gets actionable suggestions immediately.

---

## Decision Framework

### Use ACP When:

✅ You have code that should never be modified by AI
✅ You need guaranteed constraint visibility (not probabilistic)
✅ You want versioned, git-tracked context
✅ You have domain boundaries AI should respect
✅ You need team ownership tracking

### Complement with Cursor/Greptile When:

✅ You need semantic code search
✅ You're exploring unfamiliar codebases
✅ You want to find similar implementations

### Complement with MCP When:

✅ You need external system integration
✅ You want AI to execute actions
✅ You need real-time data access

### The Optimal Stack:

```
ACP (constraints) + Cursor (exploration) + MCP (capabilities)
= Full-featured, safe AI coding assistant
```

---

## Getting Started

Ready to add deterministic constraints to your codebase?

1. **Install the CLI**: See [Installation Guide](../getting-started/installation.md)
2. **Add your first annotation**: See [Quickstart](../getting-started/quickstart.md)
3. **Protect critical code**: See [Protecting Critical Code](../guides/protecting-critical-code.md)

---

## Further Reading

- [Design Philosophy](design-philosophy.md) — Core principles behind ACP
- [ACP vs MCP](acp-vs-mcp.md) — Detailed protocol comparison
- [Specification](../reference/specification.md) — Full protocol specification

---

*This document is part of the ACP Documentation. [Report issues](https://github.com/acp-protocol/acp-spec/issues)*

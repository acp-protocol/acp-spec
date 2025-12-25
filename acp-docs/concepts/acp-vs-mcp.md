# ACP vs MCP: Purpose-Built Context vs Repurposed Capabilities

**Document Type**: Explanation  
**Audience**: Developers familiar with MCP, Integration architects  
**Reading Time**: 12 minutes

---

## Executive Summary

MCP (Model Context Protocol) was designed for dynamic capabilities—connecting AI to external systems, databases, and tools. But developers have started repurposing MCP servers to inject documentation rules, coding standards, and codebase constraints.

ACP is purpose-built for exactly what people are hacking MCP to do: structured codebase context and constraints.

| Protocol | Designed For | Injection Model |
|----------|--------------|-----------------|
| **MCP** | Dynamic capabilities | Full context, every request |
| **ACP** | Codebase constraints | Minimal bootstrap + on-demand expansion |

They're complementary for their intended purposes—but for constraints and codebase awareness, ACP is the right tool.

---

## The MCP Repurposing Problem

### What MCP Was Designed For

MCP provides AI assistants with dynamic capabilities:

```
┌──────────────┐     ┌──────────────┐     ┌──────────────┐
│  AI Model    │────▶│  MCP Server  │────▶│  External    │
│  (Claude)    │     │              │     │  System      │
│              │◀────│  Resources   │◀────│  (API, DB)   │
└──────────────┘     │  Tools       │     └──────────────┘
                     │  Prompts     │
                     └──────────────┘
```

**Intended use cases**:
- Read files from filesystem
- Query databases
- Call external APIs
- Execute git commands
- Search the web

### How People Are Repurposing MCP

Developers have discovered they can use MCP servers to inject arbitrary context, so they're building servers that:

| Repurposed Use | What They're Doing |
|----------------|-------------------|
| **Documentation enforcement** | MCP server that injects "always follow these docs" |
| **Coding standards** | MCP server that injects style guides and rules |
| **Codebase awareness** | MCP server that injects file structure and relationships |
| **Guardrails** | MCP server that injects "never modify these files" |
| **Custom rules** | MCP server wrapping `.cursorrules` or similar |

**This works**, but it's using a dynamic capabilities protocol for static context—like using a database to serve static files.

---

## Why ACP Is Better Suited for Context

### The Injection Model Difference

**MCP Injection**: Full context on every request

```
┌─────────────────────────────────────────────────────────────────┐
│                    MCP CONTEXT INJECTION                         │
├─────────────────────────────────────────────────────────────────┤
│                                                                  │
│  MCP Server: "codebase-rules"                                    │
│  Injects: [style guide] + [frozen files] + [architecture docs]  │
│                                                                  │
│  Request 1: "What's the weather?"                                │
│  Context: [ALL codebase rules] + query                           │
│  Tokens: ~2,000 injected (even though irrelevant)                │
│                                                                  │
│  Request 2: "Write a haiku"                                      │
│  Context: [ALL codebase rules] + query                           │
│  Tokens: ~2,000 injected (even though irrelevant)                │
│                                                                  │
│  Request 3: "Refactor the auth module"                           │
│  Context: [ALL codebase rules] + query                           │
│  Tokens: ~2,000 injected (appropriately relevant)                │
│                                                                  │
│  Total for 3 requests: ~6,000 tokens of context                  │
│                                                                  │
└─────────────────────────────────────────────────────────────────┘
```

**ACP Injection**: Minimal bootstrap + on-demand expansion

```
┌─────────────────────────────────────────────────────────────────┐
│                    ACP CONTEXT INJECTION                         │
├─────────────────────────────────────────────────────────────────┤
│                                                                  │
│  Bootstrap Primer (always injected, ~40 tokens):                 │
│  ┌──────────────────────────────────────────────────────────┐   │
│  │ This project uses ACP. Constraints in .acp.cache.json.   │   │
│  │ Run `acp primer` for full context or `acp query` for     │   │
│  │ specific constraints. Frozen files must not be modified. │   │
│  └──────────────────────────────────────────────────────────┘   │
│                                                                  │
│  Request 1: "What's the weather?"                                │
│  Context: [bootstrap only] + query                               │
│  Tokens: ~40 injected                                            │
│                                                                  │
│  Request 2: "Write a haiku"                                      │
│  Context: [bootstrap only] + query                               │
│  Tokens: ~40 injected                                            │
│                                                                  │
│  Request 3: "Refactor the auth module"                           │
│  Context: [bootstrap] + [AI requests: acp primer --domain auth]  │
│  Tokens: ~40 + ~500 (on-demand, only when needed)                │
│                                                                  │
│  Total for 3 requests: ~580 tokens of context                    │
│                                                                  │
└─────────────────────────────────────────────────────────────────┘
```

**The difference**: ACP's bootstrap tells the AI that context exists and how to get it. The AI then requests additional context only when needed—through commands (`acp primer`, `acp query`) or variable expansion (`$SYM_AUTH_VALIDATOR`).

### Native Constraint Semantics

MCP resources are unstructured data. The AI has to interpret what they mean:

```javascript
// MCP resource - just data, no semantics
{
  "type": "resource",
  "uri": "rules://frozen-files",
  "content": "session.ts\nprocessor.ts"
}
// AI must parse this and figure out what "frozen" means
```

ACP provides structured constraints with built-in semantics:

```json
// ACP cache - structured with semantics
{
  "constraints": {
    "by_lock_level": {
      "frozen": {
        "files": ["session.ts", "processor.ts"],
        "meaning": "MUST NOT modify under any circumstances"
      }
    }
  }
}
// AI knows exactly what "frozen" means per spec
```

### Git-Integrated Versioning

MCP state is ephemeral—whatever the server returns right now:

```
Request at 10:00 AM → Server returns rules v1
Request at 10:05 AM → Server returns rules v2 (someone changed it)
No tracking of which version was active when
```

ACP caches are versioned with git:

```json
{
  "version": "1.0.0",
  "generated_at": "2024-12-17T15:30:00Z",
  "git_commit": "abc123def456789...",
  "git_branch": "main"
}
```

You can always know what constraints were in effect at any commit.

### Standardized vs Custom

Every MCP "rules server" is custom:
- Different data formats
- Different resource URIs
- Different interpretations of "frozen" or "restricted"
- No interoperability between tools

ACP is a standard:
- Defined annotation syntax (`@acp:lock`, `@acp:domain`)
- Defined cache schema (JSON with spec)
- Defined semantics (RFC 2119 compliance)
- Tools can interoperate

---

## MCP's Actual Strengths (and ACP's File Comprehension)

To be clear: MCP is excellent for its intended purpose—dynamic capabilities. But let's also clarify what ACP provides for file understanding.

### MCP: Raw File Access

MCP provides the **capability** to read file contents:

```
AI → MCP filesystem server → Raw file bytes
```

The AI gets the complete file and must parse/understand it from scratch.

### ACP: Structured File Comprehension

ACP's annotation design enables **efficient file understanding** at multiple levels:

**Level 1: Header Annotations (File Overview)**
```typescript
// @acp:file "JWT session validation and token management"
// @acp:lock frozen - Security critical, requires security team review
// @acp:domain auth - Part of authentication domain  
// @acp:owner security-team
// @acp:layer core - Core infrastructure layer

import { verify } from 'jsonwebtoken';
// ... 500 lines of code ...
```

An AI reading this file understands its purpose, constraints, ownership, and architectural position **from the header alone**—without reading 500 lines of implementation.

**Level 2: Inline Annotations (Navigation Markers)**
```typescript
// @acp:fn "Validates JWT tokens - security critical, handles all token verification"
export function validateToken(token: string): Claims { ... }

// @acp:fn "Refreshes expired tokens - rate limited, audit logged"  
export function refreshToken(token: string): NewToken { ... }

// @acp:sym "Session timeout in seconds - changing affects all active sessions"
export const SESSION_TIMEOUT = 3600;
```

The AI can scan for `@acp:fn` and `@acp:sym` markers to jump directly to relevant code without reading everything in between.

**Level 3: Cache (Pre-Computed Understanding)**
```json
{
  "files": {
    "src/auth/session.ts": {
      "description": "JWT session validation and token management",
      "lock_level": "frozen",
      "domain": "auth",
      "symbols": [
        {"name": "validateToken", "line": 45, "description": "Validates JWT tokens..."},
        {"name": "refreshToken", "line": 89, "description": "Refreshes expired tokens..."}
      ]
    }
  }
}
```

With the cache, the AI **already understands the file** before reading it. It can go directly to line 45 for `validateToken` without any file scanning.

### The Comprehension Ladder

```
┌─────────────────────────────────────────────────────────────────┐
│                    FILE COMPREHENSION APPROACHES                 │
├─────────────────────────────────────────────────────────────────┤
│                                                                  │
│  MCP Only (raw read):                                            │
│  ┌──────────────────────────────────────────────────────────┐   │
│  │ 1. Read entire file (500 lines)                          │   │
│  │ 2. Parse and understand from scratch                     │   │
│  │ 3. Figure out what's important                           │   │
│  │ 4. Find the relevant section                             │   │
│  │                                                          │   │
│  │ Tokens consumed: ~5,000 (full file)                      │   │
│  │ Understanding: Built from scratch each time              │   │
│  └──────────────────────────────────────────────────────────┘   │
│                                                                  │
│  ACP Annotations (no cache):                                     │
│  ┌──────────────────────────────────────────────────────────┐   │
│  │ 1. Read header annotations (~10 lines)                   │   │
│  │ 2. Understand file purpose, constraints, ownership       │   │
│  │ 3. Scan for @acp:fn markers to find target              │   │
│  │ 4. Read only the relevant function                       │   │
│  │                                                          │   │
│  │ Tokens consumed: ~500 (header + target section)          │   │
│  │ Understanding: Immediate from structured metadata        │   │
│  └──────────────────────────────────────────────────────────┘   │
│                                                                  │
│  ACP Cache (pre-computed):                                       │
│  ┌──────────────────────────────────────────────────────────┐   │
│  │ 1. Already know file structure from cache                │   │
│  │ 2. Already know constraints, ownership, symbols          │   │
│  │ 3. Go directly to line 45 for validateToken             │   │
│  │ 4. Read only what's needed                               │   │
│  │                                                          │   │
│  │ Tokens consumed: ~100 (just the target code)             │   │
│  │ Understanding: Pre-computed, instant                     │   │
│  └──────────────────────────────────────────────────────────┘   │
│                                                                  │
└─────────────────────────────────────────────────────────────────┘
```

### Capability Comparison (Corrected)

| Use Case | MCP | ACP |
|----------|-----|-----|
| **Raw file read** | ✅ Provides capability | ❌ Not its purpose |
| **File comprehension** | ❌ AI figures it out | ✅ Structured metadata |
| **Navigate to symbol** | ❌ Search required | ✅ Direct line reference |
| **Understand constraints** | ❌ Not provided | ✅ Header annotations |
| **Execute git commands** | ✅ Provides capability | ❌ Not its purpose |
| **Query databases** | ✅ Provides capability | ❌ Not its purpose |
| **Call external APIs** | ✅ Provides capability | ❌ Not its purpose |

**Key insight**: MCP provides the **capability** to read files. ACP provides the **comprehension** to understand them efficiently. They're complementary—use MCP to fetch the bytes, use ACP to know what they mean and where to look.

---

## Side-by-Side Comparison

### For Codebase Awareness

| Aspect | MCP (repurposed) | ACP (purpose-built) |
|--------|------------------|---------------------|
| **Injection** | Full context, every request | Bootstrap + on-demand |
| **Token efficiency** | O(n) per request | O(1) bootstrap + O(1) expansions |
| **Constraint semantics** | Custom/undefined | Standardized (spec-defined) |
| **Versioning** | Ephemeral | Git-integrated |
| **Tooling** | Build your own | CLI, LSP, IDE extensions |
| **Interoperability** | None (custom servers) | Standard format |

### For Dynamic Capabilities

| Aspect | MCP | ACP |
|--------|-----|-----|
| **File I/O** | ✅ Native | ❌ Not designed for this |
| **External APIs** | ✅ Native | ❌ Not designed for this |
| **Database queries** | ✅ Native | ❌ Not designed for this |
| **Action execution** | ✅ Native | ❌ Not designed for this |

---

## How They Work Together

The optimal architecture uses both for their intended purposes:

```
┌─────────────────────────────────────────────────────────────────┐
│                        AI Coding Session                         │
├─────────────────────────────────────────────────────────────────┤
│                                                                  │
│   ┌─────────────────────────────────────────────────────────┐   │
│   │                    CONTEXT LAYER                         │   │
│   │                                                          │   │
│   │   ACP Bootstrap Primer (~40 tokens, always present):     │   │
│   │   "ACP active. Frozen: session.ts, processor.ts.         │   │
│   │    Use `acp query` for details."                         │   │
│   │                                                          │   │
│   │   → AI knows constraints exist                           │   │
│   │   → AI can request more context when needed              │   │
│   │                                                          │   │
│   └─────────────────────────────────────────────────────────┘   │
│                              │                                   │
│                              ▼                                   │
│   ┌─────────────────────────────────────────────────────────┐   │
│   │                  CAPABILITY LAYER                        │   │
│   │                                                          │   │
│   │   MCP Servers (available for AI to invoke):              │   │
│   │   • filesystem - read/write files                        │   │
│   │   • git - commits, branches, diffs                       │   │
│   │   • terminal - execute commands                          │   │
│   │                                                          │   │
│   │   → AI can take actions                                  │   │
│   │   → AI respects ACP constraints when acting              │   │
│   │                                                          │   │
│   └─────────────────────────────────────────────────────────┘   │
│                                                                  │
└─────────────────────────────────────────────────────────────────┘
```

### Example Flow: Constraint-Aware File Editing

```
1. Session starts
   └── ACP bootstrap injected: "session.ts is frozen"

2. User: "Refactor the auth module for better performance"

3. AI thinks: "I should check ACP constraints for auth"
   └── AI runs: acp primer --domain auth
   └── Receives: detailed constraints for auth domain

4. AI plans refactoring (respecting frozen session.ts)

5. AI uses MCP filesystem server to read auth/utils.ts

6. AI generates refactored code (avoiding session.ts)

7. AI uses MCP filesystem server to write changes

8. AI uses MCP git server to commit

Result: Capabilities (MCP) + Constraints (ACP) = Safe, effective assistance
```

---

## Migration: From MCP Rules Server to ACP

If you've built an MCP server for injecting rules, here's how to migrate:

### Before (MCP)

```javascript
// Custom MCP server
const server = new MCPServer({
  resources: [{
    uri: "rules://coding-standards",
    content: fs.readFileSync("./rules.md")
  }, {
    uri: "rules://frozen-files", 
    content: "session.ts\nprocessor.ts"
  }]
});
```

**Problems**:
- Injected on every request
- Custom format
- No versioning
- AI must interpret unstructured content

### After (ACP)

```typescript
// In your code files:
// @acp:lock frozen - Security critical, DO NOT modify
export class SessionValidator { ... }
```

```bash
# Generate cache
acp index

# Bootstrap primer auto-injected (~40 tokens)
# AI requests details via acp primer when needed
```

**Benefits**:
- Minimal bootstrap, on-demand expansion
- Standard format (spec-defined)
- Git-integrated versioning
- AI understands constraint semantics

---

## The ACP-MCP Bridge

For tools that only support MCP, ACP provides a bridge server:

```
┌─────────────────────────────────────────────────────────────────┐
│                      ACP-MCP Bridge Server                       │
├─────────────────────────────────────────────────────────────────┤
│                                                                  │
│   Exposes ACP through MCP primitives                             │
│                                                                  │
│   Resources:                                                     │
│   • acp://bootstrap     → Minimal primer                         │
│   • acp://constraints   → Full constraint data (on-demand)       │
│   • acp://domain/{name} → Domain-specific context                │
│                                                                  │
│   Tools:                                                         │
│   • acp_query           → Query the cache                        │
│   • acp_check_file      → Check constraints for a file           │
│   • acp_get_primer      → Get formatted primer                   │
│                                                                  │
│   Note: Bridge provides compatibility but native ACP             │
│   integration is more token-efficient                            │
│                                                                  │
└─────────────────────────────────────────────────────────────────┘
```

---

## Decision Guide

### Use ACP When You Need:

✅ Codebase constraints (frozen files, restricted areas)
✅ Architecture documentation (domains, layers, relationships)
✅ Team ownership tracking
✅ Token-efficient context (bootstrap + on-demand)
✅ Git-versioned constraints
✅ Standardized format across tools

### Use MCP When You Need:

✅ File read/write capabilities
✅ External API access
✅ Database queries
✅ Command execution
✅ Real-time data
✅ Dynamic capabilities

### Use Both When You Need:

✅ Full AI coding assistance with safety guardrails
✅ Capabilities that respect constraints
✅ Context-aware tool usage

---

## Summary

| Question | MCP | ACP |
|----------|-----|-----|
| **Primary purpose** | Dynamic capabilities | Codebase constraints |
| **Can be used for constraints?** | Yes (repurposed) | Yes (purpose-built) |
| **Injection model** | Full context, every request | Bootstrap + on-demand |
| **Token efficiency** | Lower (always-on) | Higher (selective) |
| **Constraint semantics** | None (custom) | Native (standardized) |
| **Versioning** | Ephemeral | Git-integrated |

**Bottom Line**: Stop hacking MCP to inject rules. Use MCP for capabilities, ACP for constraints. They're designed to work together.

---

## Further Reading

- [Why ACP?](why-acp.md) — Complete problem/solution analysis
- [Design Philosophy](design-philosophy.md) — ACP's core principles
- [MCP Documentation](https://modelcontextprotocol.io) — Official MCP docs
- [MCP Server Integration](../tooling/mcp-server.md) — ACP-MCP bridge setup

---

*This document is part of the ACP Documentation. [Report issues](https://github.com/acp-protocol/acp-spec/issues)*

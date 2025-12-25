# 5-Minute Quickstart

**Document Type**: Tutorial  
**Audience**: New ACP users  
**Time to Complete**: 5 minutes

---

## What You'll Learn

In this quickstart, you'll:
1. Install the ACP CLI
2. Add your first annotation
3. Generate the cache
4. Query your codebase metadata

---

## Prerequisites

- Node.js 18+ or Rust toolchain
- A codebase with at least a few source files
- Terminal access

---

## Step 1: Install the CLI

Choose your preferred installation method:

### npm (Recommended)

```bash
npm install -g @acp-protocol/cli
```

### Homebrew (macOS/Linux)

```bash
brew install acp-protocol/tap/acp
```

### Cargo (Rust)

```bash
cargo install acp-cli
```

### Verify Installation

```bash
acp --version
# Output: acp 0.3.0
```

---

## Step 2: Initialize Your Project

Navigate to your project root and initialize ACP:

```bash
cd your-project
acp init
```

This creates:
- `.acp.config.json` — Configuration file (optional customization)
- `.acp/` — Directory for ACP-generated files

**Output:**
```
✓ Created .acp.config.json
✓ Created .acp/ directory
✓ Ready to add annotations!
```

---

## Step 3: Add Your First Annotation

Open any source file and add an ACP annotation. Here's an example:

**Before:**
```typescript
// src/auth/session.ts

export class SessionService {
  validateToken(token: string): boolean {
    // Critical security logic
    return jwt.verify(token, SECRET_KEY);
  }
}
```

**After:**
```typescript
// src/auth/session.ts
// @acp:domain auth - Authentication and authorization
// @acp:lock frozen - Security critical, DO NOT modify without security review

export class SessionService {
  // @acp:fn "Validates JWT tokens and returns validity status"
  validateToken(token: string): boolean {
    // Critical security logic
    return jwt.verify(token, SECRET_KEY);
  }
}
```

### What We Added

| Annotation | Meaning |
|------------|---------|
| `@acp:domain auth` | This file belongs to the "auth" domain |
| `@acp:lock frozen` | This file should not be modified |
| `@acp:fn "..."` | Description of the function |

---

## Step 4: Generate the Cache

Run the indexer to generate your cache file:

```bash
acp index
```

**Output:**
```
Indexing your-project...
✓ Found 24 source files
✓ Parsed 156 symbols
✓ Detected 3 domains: auth, api, utils
✓ Found 2 frozen files
✓ Built call graph with 89 edges
✓ Generated .acp/acp.cache.json (12.4 KB)

Done in 0.34s
```

---

## Step 5: Query Your Codebase

Now you can query your codebase metadata:

### List All Domains

```bash
acp query '.domains | keys'
```

**Output:**
```json
["auth", "api", "utils"]
```

### Find Frozen Files

```bash
acp query '.constraints.by_lock_level.frozen'
```

**Output:**
```json
["src/auth/session.ts", "src/auth/crypto.ts"]
```

### Get File Information

```bash
acp query '.files["src/auth/session.ts"]'
```

**Output:**
```json
{
  "path": "src/auth/session.ts",
  "language": "typescript",
  "domain": "auth",
  "lock_level": "frozen",
  "lock_reason": "Security critical, DO NOT modify without security review",
  "symbols": ["SessionService", "SessionService.validateToken"]
}
```

### Check Constraints for a File

```bash
acp constraints src/auth/session.ts
```

**Output:**
```
File: src/auth/session.ts
━━━━━━━━━━━━━━━━━━━━━━━━

Lock Level: frozen
Reason: Security critical, DO NOT modify without security review

⚠️  This file should NOT be modified by AI assistants.
```

---

## Step 6: Integrate with AI Tools (Optional)

Generate context files for your AI coding assistant:

```bash
acp sync
```

This creates:
- `.cursorrules` — For Cursor IDE
- `CLAUDE.md` — For Claude Code
- `AGENTS.md` — Universal AI context

**Output:**
```
✓ Detected: Cursor
✓ Generated .cursorrules (487 tokens)
✓ Generated CLAUDE.md (523 tokens)
✓ Generated AGENTS.md (445 tokens)
```

Your AI assistant now knows about your constraints and codebase structure!

---

## What's Next?

### Add More Annotations

Start annotating your critical code:

```typescript
// @acp:lock restricted - Requires team lead approval
// @acp:owner platform-team - Contact before changes
// @acp:stability stable - Public API, maintain backwards compatibility
```

### Explore the Cache

The cache is just JSON—explore it directly:

```bash
cat .acp/acp.cache.json | jq '.stats'
```

### Learn More

- [First Project Tutorial](first-project.md) — Complete guided walkthrough
- [Annotation Reference](../reference/annotations.md) — All annotation types
- [CLI Reference](../tooling/cli-reference.md) — All commands
- [Integrating with Cursor](../guides/integrating-with-cursor.md) — IDE setup

---

## Quick Reference

### Common Annotations

| Annotation | Example | Purpose |
|------------|---------|---------|
| `@acp:domain` | `@acp:domain auth` | Assign to domain |
| `@acp:lock` | `@acp:lock frozen` | Set modification constraints |
| `@acp:owner` | `@acp:owner security-team` | Assign ownership |
| `@acp:fn` | `@acp:fn "Description"` | Document function |
| `@acp:module` | `@acp:module "User Auth"` | Human-readable name |

### Common Commands

| Command | Purpose |
|---------|---------|
| `acp init` | Initialize project |
| `acp index` | Generate cache |
| `acp query <jq-expr>` | Query cache |
| `acp constraints <file>` | Check file constraints |
| `acp sync` | Generate AI context files |

### Lock Levels

| Level | Meaning |
|-------|---------|
| `frozen` | Do not modify |
| `restricted` | Requires approval |
| `normal` | Standard code |
| `experimental` | Expect changes |

---

## Troubleshooting

### "Command not found: acp"

Ensure the CLI is in your PATH:
```bash
# npm
npm list -g @acp-protocol/cli

# Check PATH
which acp
```

### "No source files found"

Check your `.acp.config.json` include patterns:
```json
{
  "include": ["src/**/*.ts", "src/**/*.js"]
}
```

### Cache is stale

Regenerate after code changes:
```bash
acp index --force
```

---

*Congratulations! You've successfully set up ACP in your project. [Continue to the full tutorial →](first-project.md)*

# ACP (AI Context Protocol) - Complete System

A protocol for AI-friendly code documentation, context management, and behavioral guardrails.

## Quick Start

### 1. Index Your Codebase

```bash
# Using the Rust CLI
acp index . --vars

# This creates:
# .acp.cache.json - Codebase index
# .acp.vars.json  - Variable definitions
```

### 2. Add the Primer to Your AI

Choose one (see `docs/primers.md` for full options):

**Minimal (~150 tokens):**
```
This codebase uses ACP. Before modifying files:
1. Check `.acp.cache.json` for constraints
2. Respect @acp:lock levels: frozen=don't touch, restricted=ask first
3. Use @acp:hack markers for temporary fixes
Query: jq '.constraints.by_file["<path>"]' .acp.cache.json
```

**With MCP Tools (~200 tokens):**
```
This codebase uses ACP. You have tools:
- acp_constraints - Check before modifying any file
- acp_query - Query symbols, files, domains
- acp_debug - Track debugging attempts
- acp_hack - Mark temporary code
```

### 3. Add Constraints to Your Code

```typescript
/**
 * @acp:lock restricted
 * @acp:style google-typescript
 * @acp:behavior conservative
 * @acp:ref https://docs.example.com
 */
export function sensitiveFunction() {
  // AI will ask permission and explain changes
}
```

### 4. (Optional) Run the MCP Server

```bash
cd acp-mcp-server
npm install && npm run build

# Add to Claude Desktop config
```

## The Flow

```
┌─────────────────────────────────────────────────────────────────┐
│                        USER REQUEST                              │
│                  "Fix the auth bug"                             │
└─────────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│ AI checks constraints (from primer knowledge)                   │
│ > acp_constraints("src/auth/session.ts")                        │
│ Returns: { lock: "restricted", behavior: "conservative" }       │
└─────────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│ AI adapts behavior:                                             │
│ - Explains changes before making them                           │
│ - Uses conservative approach                                    │
│ - Tracks debugging attempts                                     │
└─────────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│ AI starts debug session:                                        │
│ > acp_debug("start", { problem: "401 errors" })                 │
│ > acp_debug("attempt", { hypothesis: "race condition" })        │
│                                                                 │
│ If fails: acp_debug("revert", { attempt: 1 })                   │
│ If works: acp_debug("resolve", { resolution: "..." })           │
└─────────────────────────────────────────────────────────────────┘
```

## Context Budget

| Component | Tokens | When |
|-----------|--------|------|
| Primer | 150-300 | Always |
| MCP tools | ~400 | Available |
| Tool calls | Variable | On demand |
| **Total** | **~500** | **vs ~8000 for full spec** |

## Key Annotations

| Annotation | Purpose | AI Behavior |
|------------|---------|-------------|
| `@acp:lock frozen` | Don't modify | Refuses all changes |
| `@acp:lock restricted` | Ask first | Explains, requests approval |
| `@acp:style tailwindcss-v4` | Follow style | Uses specified conventions |
| `@acp:ref <url>` | Documentation | Can fetch and consult |
| `@acp:hack` | Temporary code | Tracks for cleanup |
| `@acp:debug-session` | Debug tracking | Logs attempts for reversal |

## File Structure

```
your-project/
├── .acp.cache.json      # Codebase index (generated)
├── .acp.vars.json       # Variables (generated)
├── .acp.config.json     # Optional configuration
├── AGENTS.md            # Project-specific AI instructions
└── src/
    └── auth/
        └── session.ts   # Contains @acp:* annotations
```

## jq Quick Reference

```bash
# Check if you can modify a file
jq '.constraints.by_file["src/auth/session.ts"].mutation.level' .acp.cache.json

# Get all frozen files
jq '.constraints.by_lock_level.frozen' .acp.cache.json

# Find expired hacks
jq '.constraints.hacks | map(select(.expires < "2024-01-01"))' .acp.cache.json

# Get symbol info
jq '.symbols["validateSession"]' .acp.cache.json
```

## Package Contents

```
acp-complete.zip
├── acp-rust/                    # Rust core implementation
│   ├── src/                     # Library and CLI
│   ├── schemas/                 # JSON schemas
│   ├── docs/                    # Documentation
│   │   ├── primers.md           # System prompt templates
│   │   ├── constraints.md       # Constraint reference
│   │   ├── integration-guide.md # Setup guide
│   │   └── jq-queries.md        # Query reference
│   └── examples/                # Example files
│
└── acp-mcp-server/              # MCP server
    ├── src/index.ts             # Server implementation
    ├── package.json
    └── README.md
```

## Why ACP?

| Problem | ACP Solution |
|---------|--------------|
| AI doesn't understand codebase structure | Indexed in `.acp.cache.json` |
| AI ignores style guides | `@acp:style` with reference URLs |
| AI modifies critical code | `@acp:lock frozen/restricted` |
| AI fixes break other things | Debug session tracking |
| Temporary fixes become permanent | `@acp:hack` with expiration |
| Full context too expensive | Variables + tiered loading |

## Next Steps

1. **Index your codebase**: `acp index . --vars`
2. **Add primer to your AI**: Copy from `docs/primers.md`
3. **Annotate critical files**: Add `@acp:lock`, `@acp:style`
4. **Create AGENTS.md**: Project-specific instructions
5. **Optional**: Set up MCP server for tool access

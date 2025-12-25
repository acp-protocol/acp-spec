# AI Context Protocol (ACP)

> Embed machine-readable context directly in your codebase so AI coding assistants understand your code's structure, constraints, and intent.

---

## The Problem

AI coding assistants are powerful, but they don't understand *your* codebase:

- They don't know which files are frozen in production and must never be modified
- They can't see your domain boundaries or architectural layers
- They miss the "tribal knowledge" that lives in your team's heads
- They treat every line of code the same, regardless of criticality

**The result?** AI assistants suggest changes to code they shouldn't touch, miss important context, and require constant correction.

---

## The Solution

ACP provides a simple, standard way to annotate your code with machine-readable context that AI assistants can consume:

```typescript
// @acp:domain auth - Authentication and authorization
// @acp:lock frozen - Critical security code, DO NOT modify
// @acp:owner security-team - Contact before changes

export class SessionValidator {
  // @acp:fn "Validates JWT tokens - security critical"
  validateToken(token: string): boolean {
    // ...
  }
}
```

Run `acp index` to generate a cache file that any AI tool can read:

```bash
$ acp index
✓ Indexed 247 files
✓ Found 1,842 symbols  
✓ Detected 12 domains
✓ Generated .acp.cache.json
```

Now your AI assistant knows:
- Which code is frozen and why
- How your codebase is organized into domains
- Who owns what and who to ask
- What each function does at a glance

---

## Quick Links

### Get Started
- [5-Minute Quickstart](getting-started/quickstart.md) — Your first ACP-annotated project
- [Installation](getting-started/installation.md) — Install the CLI
- [First Project Tutorial](getting-started/first-project.md) — Complete guided walkthrough

### Understand ACP
- [Why ACP?](concepts/why-acp.md) — The problem ACP solves
- [ACP vs MCP](concepts/acp-vs-mcp.md) — How ACP complements Model Context Protocol
- [ACP vs RAG](concepts/acp-vs-rag.md) — Why deterministic context beats probabilistic retrieval
- [Design Philosophy](concepts/design-philosophy.md) — Core principles behind the protocol

### How-To Guides
- [Annotating Your Codebase](guides/annotating-your-codebase.md) — Add ACP to an existing project
- [Integrating with Cursor](guides/integrating-with-cursor.md) — Set up Cursor IDE integration
- [Integrating with Claude Code](guides/integrating-with-claude-code.md) — Set up Claude Code integration
- [Protecting Critical Code](guides/protecting-critical-code.md) — Use lock levels effectively

### Reference
- [Specification](reference/specification.md) — Complete protocol specification
- [Schema Reference](reference/schemas.md) — JSON Schema documentation
- [Annotation Reference](reference/annotations.md) — All annotation types
- [CLI Reference](tooling/cli-reference.md) — Command-line interface

### Tooling
- [CLI Documentation](tooling/cli.md) — Command-line tool
- [VS Code Extension](tooling/vscode.md) — IDE integration
- [MCP Server](tooling/mcp-server.md) — Model Context Protocol server
- [Daemon Service](tooling/daemon.md) — Real-time file watching

---

## Key Features

| Feature | Description |
|---------|-------------|
| **Constraints** | Mark code as `frozen`, `restricted`, or `approval-required` |
| **Domains** | Organize code by business domain (auth, payments, users) |
| **Layers** | Define architectural layers (api, service, data) |
| **Ownership** | Specify team ownership for accountability |
| **Variables** | Token-efficient references that expand to full context |
| **Call Graphs** | Understand what calls what across your codebase |

---

## How It Works

```
┌─────────────────┐     ┌─────────────────┐     ┌─────────────────┐
│   Your Code     │     │   ACP CLI       │     │   AI Tools      │
│                 │     │                 │     │                 │
│  @acp:domain X  │────▶│  acp index      │────▶│  Read cache     │
│  @acp:lock Y    │     │                 │     │  Respect rules  │
│  @acp:summary Z │     │  .acp.cache.json│     │  Better context │
└─────────────────┘     └─────────────────┘     └─────────────────┘
```

1. **Annotate** — Add `@acp:` annotations in your code comments
2. **Index** — Run `acp index` to generate the cache
3. **Integrate** — AI tools read the cache and respect your constraints

---

## Supported Languages

ACP works with any language that has comments. Built-in parsing support for:

| Language | Extensions | Parser |
|----------|------------|--------|
| TypeScript | `.ts`, `.tsx` | tree-sitter |
| JavaScript | `.js`, `.jsx`, `.mjs` | tree-sitter |
| Python | `.py` | tree-sitter |
| Rust | `.rs` | tree-sitter |
| Go | `.go` | tree-sitter |
| Java | `.java` | tree-sitter |

Other languages work with comment-based annotations (no AST parsing).

---

## Conformance Levels

Implementations can claim different levels of ACP conformance:

| Level | Name | Capabilities |
|-------|------|--------------|
| 1 | Reader | Parse cache files, respect constraints |
| 2 | Standard | Level 1 + CLI, cache generation |
| 3 | Full | Level 2 + MCP server, real-time sync |

---

## Community & Resources

- **GitHub**: [github.com/acp-protocol](https://github.com/acp-protocol)
- **Discord**: [discord.gg/acp-protocol](#)
- **Twitter**: [@acp_protocol](#)
- **Documentation**: [acp-protocol.dev](https://acp-protocol.dev)

---

## License

ACP is an open specification released under the MIT License. Implementations may have their own licenses.

---

*ACP Version 1.0.0 | Last Updated: December 2025*

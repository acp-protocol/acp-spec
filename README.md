# AI Context Protocol (ACP)

> An open standard for embedding machine-readable context in codebases for AI-assisted development.

[![Spec Version](https://img.shields.io/badge/spec-v1.0.0-blue.svg)](./spec/ACP-1.0.md)
[![License](https://img.shields.io/badge/license-MIT-green.svg)](./LICENSE)
[![Schema Store](https://img.shields.io/badge/schema-store-orange.svg)](https://www.schemastore.org/)

---

## The Problem

AI coding assistants are powerful, but they're blind to your codebase's structure and constraints. They don't know:

- Which files are security-critical and shouldn't be modified carelessly
- Your team's coding standards and conventions
- How your codebase is organized into domains and layers
- What temporary hacks exist that need proper fixes

The context is in your head. The AI can't read your mind.

## The Solution

ACP provides a simple, standardized way to annotate your code with machine-readable context:

```typescript
/**
 * @acp:module "Session Management"
 * @acp:domain authentication
 * @acp:lock restricted
 * @acp:summary "Validates JWT tokens and manages user sessions - security critical"
 */
export class SessionService {
  // AI tools now understand this is security-critical code
  // and will be more careful when suggesting changes
}
```

These annotations are indexed into a structured JSON cache that any AI tool can consume.

## Key Features

🔒 **Constraints** — Protect critical code with graduated lock levels (frozen, restricted, approval-required)

📝 **Annotations** — Document structure, domains, layers, and intent for AI consumption

🔤 **Variables** — Token-efficient references (`$SYM_VALIDATE_SESSION` expands to full context)

🔌 **Tool Agnostic** — Works with Claude, GPT, Copilot, Cursor, or any AI that can read JSON

🌐 **Language Agnostic** — TypeScript, Python, Rust, Go, Java, and any language with comments

---

## Quick Start

### 1. Install the CLI

```bash
# Homebrew (macOS/Linux)
brew tap acp-protocol/tap
brew install acp-cli

# Cargo (Rust)
cargo install acp-protocol-cli

# npm
npm install -g @acp-protocol/cli
```

### 2. Initialize Your Project

```bash
cd your-project
acp init
```

This creates `.acp.config.json` with sensible defaults.

### 3. Add Annotations to Your Code

```python
# @acp:module "User Authentication"
# @acp:domain auth
# @acp:lock restricted

def validate_token(token: str) -> bool:
    """Validates JWT tokens - security critical."""
    # ...
```

### 4. Index Your Codebase

```bash
acp index
```

This generates `.acp.cache.json` with your codebase structure.

### 5. Query the Index

```bash
# List all domains
acp query '.domains | keys'

# Get authentication files
acp query '.domains.auth.files'

# Check constraints on a file
acp constraints src/auth/session.py
```

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

1. **Annotate** — Add `@acp:` annotations in code comments
2. **Index** — Run `acp index` to generate the cache
3. **Consume** — AI tools read the cache and respect your constraints

---

## Documentation

| Document | Description |
|----------|-------------|
| [Specification](./spec/ACP-1.0.md) | Complete protocol specification |
| [Getting Started](./docs/getting-started.md) | Step-by-step tutorial |
| [Annotation Reference](./spec/chapters/annotations.md) | All annotation types |
| [Constraint Reference](./spec/chapters/constraints.md) | Lock levels and rules |
| [Variable Reference](./spec/chapters/vars.md) | Variable system |
| [CLI Reference](./cli/README.md) | Command-line interface |

### Integrations

- [Claude Desktop (MCP)](./docs/integrations/claude-desktop.md)
- [Cursor](./docs/integrations/cursor.md)
- [GitHub Copilot](./docs/integrations/github-copilot.md)

---

## File Formats

ACP uses three JSON files:

| File | Purpose | Schema |
|------|---------|--------|
| `.acp.config.json` | Project configuration | [config.schema.json](./schemas/v1/config.schema.json) |
| `.acp.cache.json` | Indexed codebase cache | [cache.schema.json](./schemas/v1/cache.schema.json) |
| `.acp.vars.json` | Variable definitions | [vars.schema.json](./schemas/v1/vars.schema.json) |

All schemas are available in the [JSON Schema Store](https://www.schemastore.org/) for IDE autocomplete.

---

## Annotations at a Glance

### File/Module Level

```javascript
/**
 * @acp:module "Payment Processing"
 * @acp:domain billing
 * @acp:layer service
 * @acp:stability stable
 */
```

### Symbol Level

```javascript
/**
 * @acp:summary "Processes credit card payments via Stripe"
 * @acp:lock restricted
 * @acp:ref "https://stripe.com/docs/api"
 */
function processPayment(amount, card) { }
```

### Constraints

```javascript
// @acp:lock frozen — Never modify this code
// @acp:lock restricted — Explain changes before modifying
// @acp:lock approval-required — Ask before making changes
// @acp:lock tests-required — Include tests with any changes
```

### Temporary Code

```javascript
// @acp:hack ticket=JIRA-123 expires=2024-06-01
// @acp:debug session=auth-bug-42 status=active
```

See the [Annotation Reference](./spec/chapters/annotations.md) for the complete list.

---

## Constraint Levels

| Level | Meaning | AI Behavior |
|-------|---------|-------------|
| `frozen` | Never modify | Refuse all changes |
| `restricted` | Explain first | Describe changes before making them |
| `approval-required` | Ask permission | Wait for explicit approval |
| `tests-required` | Include tests | Must add/update tests |
| `docs-required` | Include docs | Must add/update documentation |
| `review-required` | Flag for review | Mark changes for human review |
| `normal` | No restrictions | Default behavior |
| `experimental` | Extra caution | Warn about instability |

---

## Variables

Variables provide token-efficient references to code elements:

```bash
# Instead of:
"Check the validateSession function in src/auth/session.ts on lines 45-89 
which handles JWT validation and session management..."

# Just use:
"Check $SYM_VALIDATE_SESSION"
```

Variables are defined in `.acp.vars.json` and expand to full context automatically.

---

## Roadmap

- [x] Core specification v1.0
- [x] JSON schemas
- [ ] Reference CLI implementation
- [ ] MCP server for Claude Desktop
- [ ] VS Code extension
- [ ] Language server protocol (LSP)
- [ ] GitHub Action for CI validation

See the [full roadmap](./docs/roadmap.md) for details.

---

## Contributing

We welcome contributions! See [CONTRIBUTING.md](./CONTRIBUTING.md) for guidelines.

### Ways to Contribute

- 🐛 Report bugs or suggest features via [Issues](https://github.com/acp-protocol/acp-spec/issues)
- 📝 Improve documentation
- 🔧 Submit pull requests
- 💬 Join the discussion on [Discord](https://discord.gg/acp-protocol)
- 📣 Spread the word

### RFC Process

Protocol changes go through an RFC process. See [rfcs/](./rfcs/) for details.

---

## Community

- **Discord**: [Join our server](https://discord.gg/acp-protocol)
- **GitHub Discussions**: [Discussions](https://github.com/acp-protocol/acp-spec/discussions)
- **Twitter/X**: [@acp_protocol](https://twitter.com/acp_protocol)

---

## License

MIT License — see [LICENSE](./LICENSE) for details.

---

## Acknowledgments

ACP draws inspiration from:

- [JSDoc](https://jsdoc.app/) — Documentation annotations for JavaScript
- [OpenAPI](https://www.openapis.org/) — API specification standard
- [Model Context Protocol](https://modelcontextprotocol.io/) — AI tool integration
- The developer community's collective frustration with AI tools ignoring context

---

<p align="center">
  <strong>Give your AI the context it needs.</strong><br>
  <a href="./docs/getting-started.md">Get Started</a> •
  <a href="./spec/ACP-1.0.md">Read the Spec</a> •
  <a href="https://discord.gg/acp-protocol">Join Discord</a>
</p>
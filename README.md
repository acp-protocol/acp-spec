![ACP Header](public/github-headerGitHub%20Header.svg)


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

🌳 **AST Parsing** — Tree-sitter based symbol extraction for accurate code analysis

📊 **Git Integration** — Blame tracking, file history, and contributor metadata per symbol

🔄 **Tool Sync** — Automatic synchronization to Cursor, Claude Code, Copilot, and more

🔌 **Tool Agnostic** — Works with Claude, GPT, Copilot, Cursor, or any AI that can read JSON

🌐 **Language Support** — TypeScript, JavaScript, Python, Rust, Go, and Java with full AST parsing

---

## Quick Start

### 1. Install the CLI

```bash
# From source (Rust required)
git clone https://github.com/acp-protocol/acp-spec.git
cd acp-spec/cli
cargo install --path .

# Or build directly
cargo build --release
```

### 2. Initialize Your Project

```bash
cd your-project
acp init
```

This auto-detects languages and creates `.acp.config.json` with sensible defaults.

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

This generates `.acp.cache.json` with your codebase structure, symbols, call graph, and git metadata.

### 5. Query the Index

```bash
# Look up a symbol
acp query symbol validate_token

# Find all callers of a function
acp query callers validate_token

# List all domains
acp query domains

# Check constraints on a file
acp check src/auth/session.py

# Expand variable references
acp expand "Check \$SYM_VALIDATE_TOKEN"
```

---

## How It Works

```
┌─────────────────┐     ┌─────────────────┐     ┌─────────────────┐
│   Your Code     │     │   ACP CLI       │     │   AI Tools      │
│                 │     │                 │     │                 │
│  @acp:domain X  │────▶│  Tree-sitter    │────▶│  Read cache     │
│  @acp:lock Y    │     │  AST parsing    │     │  Respect rules  │
│  @acp:summary Z │     │  Git metadata   │     │  Better context │
│                 │     │  .acp.cache.json│     │                 │
└─────────────────┘     └─────────────────┘     └─────────────────┘
```

1. **Annotate** — Add `@acp:` annotations in code comments
2. **Index** — Run `acp index` to parse AST, extract symbols, and generate the cache
3. **Consume** — AI tools read the cache and respect your constraints

---

## Documentation

### Specification Chapters

| Chapter | Description |
|---------|-------------|
| [Full Specification](./spec/ACP-1.0.md) | Complete protocol specification |
| [Introduction](./spec/chapters/01-introduction.md) | Overview and design principles |
| [Annotations](./spec/chapters/05-annotations.md) | All annotation types |
| [Constraints](./spec/chapters/06-constraints.md) | Lock levels and rules |
| [Variables](./spec/chapters/07-variables.md) | Variable system |
| [Querying](./spec/chapters/10-querying.md) | Query interfaces |
| [Tool Integration](./spec/chapters/11-tool-integration.md) | AI tool sync |
| [Debug Sessions](./spec/chapters/13-debug-sessions.md) | Attempt tracking |

### Additional Resources

| Resource | Description |
|----------|-------------|
| [CLI Reference](./cli/README.md) | Command-line interface |
| [Testing Guide](./cli/docs/TESTING_GUIDE.md) | Testing the CLI |
| [Cache Format](./spec/chapters/03-cache-format.md) | Cache file structure |
| [Config Format](./spec/chapters/04-config-format.md) | Configuration options |

---

## File Formats

ACP uses six JSON schemas:

| File | Purpose | Schema |
|------|---------|--------|
| `.acp.config.json` | Project configuration | [config.schema.json](./schemas/v1/config.schema.json) |
| `.acp.cache.json` | Indexed codebase cache | [cache.schema.json](./schemas/v1/cache.schema.json) |
| `.acp.vars.json` | Variable definitions | [vars.schema.json](./schemas/v1/vars.schema.json) |
| `.acp/acp.attempts.json` | Debug session tracking | [attempts.schema.json](./schemas/v1/attempts.schema.json) |
| `.acp/acp.sync.json` | Tool sync configuration | [sync.schema.json](./schemas/v1/sync.schema.json) |
| `primer.*.json` | AI context primers | [primer.schema.json](./schemas/v1/primer.schema.json) |

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

See the [Annotation Reference](./spec/chapters/05-annotations.md) for the complete list.

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

## CLI Commands

| Command | Description |
|---------|-------------|
| `acp init` | Initialize a new ACP project with auto-detected languages |
| `acp index` | Index the codebase using AST parsing and generate cache |
| `acp vars` | Generate variable definitions from cache |
| `acp query <subcommand>` | Query symbols, files, domains, callers, callees, stats |
| `acp expand <text>` | Expand variable references in text |
| `acp check <file>` | Check constraints and guardrails on a file |
| `acp validate <file>` | Validate JSON files against ACP schemas |
| `acp watch` | Watch for file changes and auto-update cache |
| `acp attempt <subcommand>` | Manage debug sessions (start, fail, verify, revert) |
| `acp chain <var>` | Show variable inheritance chains |

### Query Subcommands

```bash
acp query symbol <name>     # Look up a specific symbol
acp query file <path>       # Get file information
acp query callers <symbol>  # Find all callers of a function
acp query callees <symbol>  # Find all functions called by a symbol
acp query domains           # List all code domains
acp query domain <name>     # Get details of a specific domain
acp query hotpaths          # List frequently-called symbols
acp query stats             # Show aggregate statistics
```

### Debug Session Commands

```bash
acp attempt start <id>      # Begin a new debugging attempt
acp attempt fail <id>       # Mark attempt as failed
acp attempt verify <id>     # Mark attempt as successful
acp attempt revert <id>     # Revert changes from attempt
acp checkpoint <name>       # Create a named checkpoint
acp checkpoints             # List all checkpoints
acp restore <name>          # Restore to a checkpoint
```

---

## Roadmap

- [x] Core specification v1.0
- [x] JSON schemas (6 schemas)
- [x] Reference CLI implementation (Rust)
- [x] Tree-sitter AST parsing (6 languages)
- [x] Git2 integration (blame, history, contributors)
- [x] Schema validation with semantic checks
- [x] Debug session & checkpoint tracking
- [ ] MCP server for Claude Desktop
- [ ] VS Code extension
- [ ] Language server protocol (LSP)
- [ ] GitHub Action for CI validation
- [ ] Package distribution (Homebrew, npm, crates.io)

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
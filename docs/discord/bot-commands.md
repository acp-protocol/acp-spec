# Discord Bot Commands Reference

The ACP Discord bot provides helpful commands for accessing ACP information, CLI help, examples, and community utilities.

## Information Commands

### `/acp-spec`

Link to the latest ACP specification.

**Usage:**
```
/acp-spec
```

**Response:**
- Link to latest specification version
- Quick overview of what's included
- Link to specific chapters

**Example:**
```
📖 ACP Specification v1.0.0
🔗 https://github.com/acp-protocol/acp-spec/blob/main/spec/ACP-1.0.md

The complete protocol specification including:
• Annotations and syntax
• Constraint system
• Variable system
• Query interfaces
• Conformance levels
```

### `/cli-install`

Get installation instructions for the ACP CLI.

**Usage:**
```
/cli-install
```

**Response:**
- Installation methods (source, Homebrew, npm)
- System requirements
- Quick start steps
- Troubleshooting tips

**Example:**
```
🛠️ ACP CLI Installation

From Source (Rust required):
  git clone https://github.com/acp-protocol/acp-spec.git
  cd acp-spec/cli
  cargo install --path .

Quick Start:
  1. cd your-project
  2. acp init
  3. acp index

📚 Full guide: https://github.com/acp-protocol/acp-spec/blob/main/cli/README.md
```

### `/quickstart`

Get a quick start guide for ACP.

**Usage:**
```
/quickstart
```

**Response:**
- Step-by-step getting started guide
- First annotation example
- Common next steps
- Links to resources

### `/examples`

Show code examples and use cases.

**Usage:**
```
/examples [type]
```

**Parameters:**
- `type` (optional): Filter by type (annotations, constraints, variables, integration)

**Response:**
- Code examples based on type
- Language-specific examples
- Real-world use cases

**Example:**
```
/examples annotations

📝 Annotation Examples

TypeScript:
/**
 * @acp:module "Payment Processing"
 * @acp:domain billing
 * @acp:lock restricted
 */
export class PaymentService { ... }

Python:
# @acp:module "User Authentication"
# @acp:domain auth
# @acp:lock restricted
def validate_token(token: str) -> bool: ...
```

### `/roadmap`

Show the current development roadmap.

**Usage:**
```
/roadmap
```

**Response:**
- Current development status
- Upcoming features
- Completed items
- Link to roadmap channel

## Utility Commands

### `/acp-validate`

Validate annotation syntax in code snippets.

**Usage:**
```
/acp-validate <code>
```

**Parameters:**
- `code`: Code snippet to validate

**Response:**
- Validation results
- Syntax errors (if any)
- Suggestions for fixes

**Example:**
```
/acp-validate "@acp:module \"Test\" @acp:domain auth"

✅ Valid annotations found:
  • @acp:module "Test"
  • @acp:domain auth

💡 Tip: Consider adding @acp:summary for better context
```

### `/acp-example`

Show annotation examples by type.

**Usage:**
```
/acp-example <type>
```

**Parameters:**
- `type`: Example type (module, domain, lock, summary, style, behavior, quality)

**Response:**
- Example annotations
- Usage explanation
- Best practices

**Example:**
```
/acp-example lock

🔒 Lock Level Examples

Frozen (most restrictive):
  @acp:lock frozen
  @acp:lock-reason "Production config - never modify"

Restricted:
  @acp:lock restricted
  @acp:lock-reason "Security critical - requires review"

Normal (default):
  @acp:lock normal

See: https://github.com/acp-protocol/acp-spec/blob/main/spec/chapters/06-constraints.md
```

### `/constraints`

Explain constraint levels and usage.

**Usage:**
```
/constraints [level]
```

**Parameters:**
- `level` (optional): Specific lock level to explain

**Response:**
- Constraint level explanation
- AI behavior for each level
- When to use each level
- Examples

**Example:**
```
/constraints restricted

🔒 Restricted Lock Level

Meaning: AI must get explicit user approval before any change

AI Behavior:
  • Must explain proposed changes
  • Must wait for user approval
  • Cannot modify without permission

Use Cases:
  • Authentication code
  • Payment processing
  • Security-critical functions

Example:
  @acp:lock restricted
  @acp:lock-reason "Security critical - requires review"
```

### `/variables`

Explain the variable system.

**Usage:**
```
/variables
```

**Response:**
- Variable system overview
- Syntax and types
- Expansion modifiers
- Examples

**Example:**
```
🔤 ACP Variables System

Variables provide token-efficient references to code elements.

Syntax: $PREFIX_NAME[.modifier]

Types:
  • SYM_* - Symbol references (functions, classes)
  • FILE_* - File references
  • DOM_* - Domain references

Modifiers:
  • .full - Complete JSON expansion
  • .ref - File reference only
  • .signature - Signature only

Example:
  $SYM_VALIDATE_SESSION
  → "validateSession (src/auth/session.ts:45-89) validates JWT tokens"

See: https://github.com/acp-protocol/acp-spec/blob/main/spec/chapters/07-variables.md
```

## Community Commands

### `/contribute`

Get the contribution guide.

**Usage:**
```
/contribute
```

**Response:**
- How to contribute
- Contribution guidelines
- RFC process
- Links to resources

**Example:**
```
🤝 Contributing to ACP

Ways to Contribute:
  • Report bugs via GitHub Issues
  • Suggest features in #feature-requests
  • Submit pull requests
  • Improve documentation
  • Help others in #help

RFC Process:
  Major changes go through RFC process
  See: #rfc-discussions channel

📚 Full guide: https://github.com/acp-protocol/acp-spec/blob/main/CONTRIBUTING.md
```

### `/rfc`

Link to a specific RFC.

**Usage:**
```
/rfc <number>
```

**Parameters:**
- `number`: RFC number to link to

**Response:**
- Link to RFC
- RFC summary
- Discussion thread link

**Example:**
```
/rfc 1

📜 RFC-001: [RFC Title]

Status: [Draft/Active/Accepted/Rejected]
Author: [Author Name]

Summary: [Brief description]

🔗 Full RFC: https://github.com/acp-protocol/acp-spec/tree/main/rfcs/RFC-001.md
💬 Discussion: #rfc-discussions
```

### `/stats`

Show server and project statistics.

**Usage:**
```
/stats [type]
```

**Parameters:**
- `type` (optional): Stats type (server, project, cli)

**Response:**
- Server statistics (members, activity)
- Project statistics (stars, contributors, issues)
- CLI statistics (downloads, usage)

**Example:**
```
/stats

📊 ACP Statistics

Server:
  • Members: 1,234
  • Active this week: 456
  • Messages this week: 2,345

Project:
  • GitHub Stars: 567
  • Contributors: 89
  • Open Issues: 12
  • Open PRs: 5

CLI:
  • Total Downloads: 1,234
  • Active Users: 567
```

## Help Commands

### `/help`

Show available bot commands.

**Usage:**
```
/help [command]
```

**Parameters:**
- `command` (optional): Specific command to get help for

**Response:**
- List of all commands
- Command descriptions
- Usage examples

### `/docs`

Link to documentation resources.

**Usage:**
```
/docs [topic]
```

**Parameters:**
- `topic` (optional): Specific documentation topic

**Response:**
- Links to relevant documentation
- Quick reference guides
- Tutorial links

## Command Categories

### Information
- `/acp-spec` - Specification link
- `/cli-install` - Installation guide
- `/quickstart` - Quick start guide
- `/examples` - Code examples
- `/roadmap` - Development roadmap

### Utility
- `/acp-validate` - Validate annotations
- `/acp-example` - Show examples
- `/constraints` - Explain constraints
- `/variables` - Explain variables

### Community
- `/contribute` - Contribution guide
- `/rfc` - Link to RFC
- `/stats` - Statistics
- `/help` - Command help
- `/docs` - Documentation links

## Tips

- Use slash commands for quick access to information
- Commands are case-insensitive
- Most commands support optional parameters
- Use `/help` to discover new commands
- Commands work in any channel

## Feedback

Found a bug or have a suggestion for a new command?
- Report in `#bug-reports`
- Suggest in `#feature-requests`
- DM a moderator

---

*Last updated: 2024-12-19*


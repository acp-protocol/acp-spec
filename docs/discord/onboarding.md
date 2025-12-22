# Discord Server Onboarding Guide

Welcome to the ACP Discord server! This guide will help you get started and make the most of your experience.

## Welcome Flow

When you join the server, you'll go through the following onboarding process:

### Step 1: Welcome Message

You'll first see a welcome message in `#welcome` that includes:
- Server rules and guidelines
- Quick links to important resources
- Getting started guide
- Role selection instructions

### Step 2: Role Selection

Choose roles that match your interests and expertise:

**Interest Roles:**
- `Developer` - I'm a developer using ACP in my projects
- `AI Tool Creator` - I'm building AI tools that integrate with ACP
- `Contributor` - I want to contribute to the ACP project
- `Learner` - I'm learning about ACP

**Language Roles:**
- `TypeScript/JavaScript` - Working with TS/JS
- `Python` - Working with Python
- `Rust` - Working with Rust
- `Go` - Working with Go
- `Java` - Working with Java
- `Multi-language` - Working with multiple languages

**Experience Levels:**
- `New to ACP` - Just getting started
- `Active User` - Using ACP regularly
- `Expert` - Deep knowledge of ACP

### Step 3: Onboarding Checklist

Complete these steps to get the most out of the server:

- [ ] **Read the welcome message** - Understand server rules and structure
- [ ] **Select your roles** - Choose roles that match your interests
- [ ] **Review getting started guide** - Check `#getting-started` for tutorials
- [ ] **Install CLI tool** - Get the ACP CLI installed on your system
- [ ] **Create first cache** - Run `acp index` in a project
- [ ] **Share in showcase** - Post your first implementation in `#showcase`

## Getting Started with ACP

### 1. Install the CLI

```bash
# Via Homebrew (macOS/Linux)
brew install acp-protocol/tap/acp

# From source (Rust required)
git clone https://github.com/acp-protocol/acp-cli.git
cd acp-cli
cargo install --path .
```

### 2. Initialize Your Project

```bash
cd your-project
acp init
```

This creates `.acp.config.json` with sensible defaults.

### 3. Add Annotations

Add your first annotations to your code:

```typescript
/**
 * @acp:module "User Authentication"
 * @acp:domain authentication
 * @acp:lock restricted
 * @acp:summary "Validates JWT tokens - security critical"
 */
export function validateToken(token: string): boolean {
  // Implementation...
}
```

### 4. Index Your Codebase

```bash
acp index
```

This generates `.acp.cache.json` with your codebase structure.

### 5. Query the Cache

```bash
# Show stats
acp query stats

# Look up a symbol
acp query symbol validateToken

# List domains
acp query domains
```

## Educational Content Series

### Week 1: What is ACP and Why Use It?

**Topics:**
- The problem ACP solves
- How ACP works
- Key benefits
- Use cases

**Resources:**
- [Introduction Chapter](../../spec/chapters/01-introduction.md)
- [Main README](../../README.md)
- `#general` channel for discussions

### Week 2: Installing and Using the CLI

**Topics:**
- Installation methods
- Basic commands
- Configuration options
- Common workflows

**Resources:**
- [CLI Repository](https://github.com/acp-protocol/acp-cli)
- `#cli-discussion` channel
- Bot command: `/cli-install`

### Week 3: Adding Your First Annotations

**Topics:**
- Annotation syntax
- Common annotations
- Best practices
- Examples by language

**Resources:**
- [Annotations Chapter](../../spec/chapters/05-annotations.md)
- `#annotations` channel
- Bot command: `/acp-example`

### Week 4: Understanding Constraints

**Topics:**
- Lock levels
- Constraint types
- When to use constraints
- Inheritance rules

**Resources:**
- [Constraints Chapter](../../spec/chapters/06-constraints.md)
- `#constraints` channel
- Bot command: `/constraints`

### Week 5: Variables and Token Efficiency

**Topics:**
- Variable system
- Creating variables
- Expansion modifiers
- Token optimization

**Resources:**
- [Variables Chapter](../../spec/chapters/07-variables.md)
- `#general` channel for questions

## Regular Events

### Weekly Office Hours

**When:** Every Wednesday at 2 PM UTC
**Where:** `#general` channel
**What:** Q&A session with maintainers and experts

Bring your questions about:
- ACP usage
- CLI troubleshooting
- Best practices
- Integration help

### Monthly Showcase

**When:** First Friday of each month
**Where:** `#showcase` channel
**What:** Community project highlights

Share:
- Your ACP implementations
- Interesting use cases
- Success stories
- Tips and tricks

### Quarterly Roadmap Review

**When:** First Monday of each quarter
**Where:** `#roadmap` channel
**What:** Review development progress and upcoming features

Discuss:
- Completed features
- Upcoming releases
- Feature priorities
- Community feedback

## Resource Library

### Pinned Messages

Each channel has pinned messages with essential resources:

**`#getting-started`:**
- Installation guides
- First steps tutorial
- Common setup issues
- Quick reference

**`#cli-discussion`:**
- CLI command reference
- Configuration examples
- Tips and tricks
- Troubleshooting guide

**`#annotations`:**
- Annotation syntax reference
- Examples by language
- Best practices
- Common patterns

**`#constraints`:**
- Constraint level guide
- When to use each level
- Inheritance examples
- Behavior guidance

**`#integrations`:**
- Tool integration guides
- MCP server setup
- VS Code extension (when available)
- Cursor integration

## Progress Tracking

Track your onboarding progress with reaction roles:

1. ✅ **Welcome Read** - React to welcome message
2. 🎯 **Roles Selected** - Complete role selection
3. 📚 **Guide Reviewed** - Read getting started guide
4. 💻 **CLI Installed** - Install CLI tool
5. 📝 **First Cache** - Create first `.acp.cache.json`
6. 🎉 **Showcase Shared** - Share in `#showcase`

## Getting Help

### Where to Ask

- **General Questions** → `#help` channel
- **CLI Issues** → `#cli-discussion` channel
- **Annotation Questions** → `#annotations` channel
- **Constraint Questions** → `#constraints` channel
- **Integration Help** → `#integrations` channel

### How to Ask

1. **Search first** - Check if your question has been answered
2. **Be specific** - Provide context and code examples
3. **Use threads** - Keep discussions organized
4. **Be patient** - Community members are volunteers

### Bot Commands

Use bot commands for quick information:
- `/quickstart` - Quick start guide
- `/cli-install` - Installation instructions
- `/acp-example <type>` - Show examples
- `/help` - List all commands

## Next Steps

After completing onboarding:

1. **Explore channels** - Check out different channels based on your interests
2. **Join discussions** - Participate in `#general` and technical channels
3. **Share your work** - Post in `#showcase` when you have something to share
4. **Help others** - Answer questions in `#help` when you can
5. **Contribute** - Check `#contributing` for ways to contribute

## Tips for Success

- **Read pinned messages** - They contain essential information
- **Use threads** - Keep conversations organized
- **Be active** - Regular participation helps you learn faster
- **Ask questions** - No question is too basic
- **Share knowledge** - Help others and learn together

## Feedback

We're always improving! Share feedback:
- `#feedback` channel - General feedback
- `#suggestions` channel - Feature suggestions
- Direct DM to moderators - Sensitive feedback

---

*Welcome to the ACP community! We're excited to have you here.*


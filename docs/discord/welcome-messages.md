# Welcome Message Templates

This document contains templates for welcome messages, DMs, and onboarding communications.

## Main Welcome Message

**Channel:** `#welcome`
**Type:** Pinned message
**Update Frequency:** As needed

```markdown
# Welcome to the ACP Discord Server! 🎉

Welcome to the **AI Context Protocol** community! We're excited to have you here.

## What is ACP?

The AI Context Protocol (ACP) is an open standard for embedding machine-readable context in codebases for AI-assisted development. It helps AI tools understand your code structure, respect your constraints, and navigate your codebase efficiently.

## Quick Links

• **GitHub**: https://github.com/acp-protocol/acp-spec
• **Documentation**: https://github.com/acp-protocol/acp-spec/blob/main/spec/ACP-1.0.md
• **CLI Guide**: https://github.com/acp-protocol/acp-spec/blob/main/cli/README.md
• **Website**: https://acp-protocol.dev

## Getting Started

1. **Read the rules** - Check out our community guidelines below
2. **Select your roles** - React below to choose roles that match your interests
3. **Visit #getting-started** - Follow our step-by-step onboarding guide
4. **Install the CLI** - Get started with ACP tools (`/cli-install`)
5. **Join the discussion** - Introduce yourself in #general

## Role Selection

React to select your roles:

🎯 **Developer** - I'm a developer using ACP in my projects
🤖 **AI Tool Creator** - I'm building AI tools that integrate with ACP
🤝 **Contributor** - I want to contribute to the ACP project
📚 **Learner** - I'm learning about ACP

**Language Roles:**
🔷 TypeScript/JavaScript
🐍 Python
🦀 Rust
🐹 Go
☕ Java
🌐 Multi-language

**Experience Levels:**
🆕 New to ACP
⭐ Active User
🏆 Expert

## Server Rules

1. **Be respectful** - Treat everyone with kindness and respect
2. **Be helpful** - Share knowledge and help others learn
3. **Be constructive** - Provide helpful, actionable feedback
4. **Follow guidelines** - Read channel-specific rules in pinned messages
5. **No spam** - Keep self-promotion minimal and relevant

## Need Help?

• **General Questions** → #help
• **CLI Issues** → #cli-discussion
• **Technical Questions** → Relevant technical channel
• **Bot Commands** → Use `/help` to see all commands

## Resources

• **Getting Started Guide**: https://github.com/acp-protocol/acp-spec/blob/main/docs/discord/onboarding.md
• **Bot Commands**: https://github.com/acp-protocol/acp-spec/blob/main/docs/discord/bot-commands.md
• **Moderation Guide**: https://github.com/acp-protocol/acp-spec/blob/main/docs/discord/moderation-guide.md

Welcome aboard! We're here to help you succeed with ACP. 🚀
```

## Welcome DM

**Type:** Direct message
**Trigger:** New member joins
**Update Frequency:** As needed

```markdown
Hi! Welcome to the ACP Discord server! 👋

I'm the ACP bot, and I'm here to help you get started with the AI Context Protocol.

## Quick Start Checklist

Here's what you can do to get started:

- [ ] Read the welcome message in #welcome
- [ ] Select your roles (react in #welcome)
- [ ] Check out #getting-started for the onboarding guide
- [ ] Install the CLI tool (use `/cli-install` for instructions)
- [ ] Create your first `.acp.cache.json` (run `acp index`)
- [ ] Share your implementation in #showcase

## Useful Commands

Try these commands in any channel:

• `/quickstart` - Quick start guide
• `/cli-install` - Installation instructions
• `/acp-spec` - Link to specification
• `/examples` - Code examples
• `/help` - List all commands

## Resources

• **Getting Started**: https://github.com/acp-protocol/acp-spec/blob/main/docs/discord/onboarding.md
• **CLI Guide**: https://github.com/acp-protocol/acp-spec/blob/main/cli/README.md
• **Full Spec**: https://github.com/acp-protocol/acp-spec/blob/main/spec/ACP-1.0.md
• **Examples**: https://github.com/acp-protocol/acp-spec/tree/main/spec/examples

## Need Help?

• Ask in #help for general questions
• Use #cli-discussion for CLI-specific issues
• Check technical channels for specific topics
• Use `/help` to see all available commands

Welcome to the community! We're excited to have you here. If you have any questions, don't hesitate to ask! 🎉
```

## Getting Started Channel Message

**Channel:** `#getting-started`
**Type:** Pinned message
**Update Frequency:** As needed

```markdown
# Getting Started with ACP

Welcome to the getting started channel! This is your guide to using the AI Context Protocol.

## Step 1: Install the CLI

```bash
# From source (Rust required)
git clone https://github.com/acp-protocol/acp-spec.git
cd acp-spec/cli
cargo install --path .
```

Or use `/cli-install` for detailed instructions.

## Step 2: Initialize Your Project

```bash
cd your-project
acp init
```

This creates `.acp.config.json` with sensible defaults.

## Step 3: Add Annotations

Add your first annotations to your code:

**TypeScript:**
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

**Python:**
```python
# @acp:module "User Authentication"
# @acp:domain auth
# @acp:lock restricted
def validate_token(token: str) -> bool:
    """Validates JWT tokens - security critical."""
    # Implementation...
```

## Step 4: Index Your Codebase

```bash
acp index
```

This generates `.acp.cache.json` with your codebase structure.

## Step 5: Query the Cache

```bash
# Show stats
acp query stats

# Look up a symbol
acp query symbol validateToken

# List domains
acp query domains
```

## Next Steps

1. **Learn about annotations** → Check #annotations
2. **Understand constraints** → Check #constraints
3. **Explore integrations** → Check #integrations
4. **Share your work** → Post in #showcase

## Educational Series

We run a 5-week getting started series:

- **Week 1**: What is ACP and why use it?
- **Week 2**: Installing and using the CLI
- **Week 3**: Adding your first annotations
- **Week 4**: Understanding constraints
- **Week 5**: Variables and token efficiency

Check this channel every Thursday at 3 PM UTC for new content!

## Need Help?

• Ask questions in this channel
• Use `/quickstart` for quick reference
• Check #help for general questions
• Use `/help` to see all bot commands

Happy coding! 🚀
```

## Rules Message

**Channel:** `#welcome`
**Type:** Pinned message
**Update Frequency:** As needed

```markdown
# Server Rules

To ensure a welcoming and productive community, please follow these rules:

## 1. Be Respectful

Treat everyone with kindness and respect. No harassment, discrimination, or hate speech of any kind.

## 2. Be Helpful

Share knowledge and help others learn. We're all here to grow together.

## 3. Be Constructive

Provide constructive feedback. Focus on ideas, not individuals.

## 4. Follow Channel Purposes

• Use appropriate channels for topics
• Read pinned messages before posting
• Keep discussions on-topic

## 5. No Spam

• Minimal self-promotion
• No repeated messages
• No unauthorized bots
• Keep links relevant

## 6. Help Channel Etiquette

When asking for help:
• Search first
• Be specific
• Use threads
• Mark as solved

## 7. Contribution Guidelines

• Read CONTRIBUTING.md before contributing
• Follow RFC process for major changes
• Link to GitHub for bug reports
• Be patient with maintainers

## Enforcement

Violations may result in:
• Warnings
• Temporary mutes
• Kicks
• Bans (for severe violations)

See our [Moderation Guide](https://github.com/acp-protocol/acp-spec/blob/main/docs/discord/moderation-guide.md) for details.

## Questions?

DM a moderator or use #feedback for questions about rules or moderation.

Thank you for helping maintain a positive community! 🙏
```

## Role Selection Message

**Channel:** `#welcome`
**Type:** Regular message with reactions
**Update Frequency:** As needed

```markdown
# Select Your Roles

React below to choose roles that match your interests and expertise. You can select multiple roles!

## Interest Roles

🎯 **Developer** - I'm a developer using ACP in my projects
🤖 **AI Tool Creator** - I'm building AI tools that integrate with ACP
🤝 **Contributor** - I want to contribute to the ACP project
📚 **Learner** - I'm learning about ACP

## Language Roles

🔷 **TypeScript/JavaScript** - Working with TS/JS
🐍 **Python** - Working with Python
🦀 **Rust** - Working with Rust
🐹 **Go** - Working with Go
☕ **Java** - Working with Java
🌐 **Multi-language** - Working with multiple languages

## Experience Levels

🆕 **New to ACP** - Just getting started
⭐ **Active User** - Using ACP regularly
🏆 **Expert** - Deep knowledge of ACP

React to any roles that apply to you! This helps us tailor content and connect you with relevant community members.
```

## Progress Tracking Message

**Channel:** `#getting-started`
**Type:** Pinned message
**Update Frequency:** As needed

```markdown
# Onboarding Progress

Track your onboarding progress! React to mark completed steps:

✅ **Welcome Read** - I've read the welcome message
🎯 **Roles Selected** - I've selected my roles
📚 **Guide Reviewed** - I've reviewed the getting started guide
💻 **CLI Installed** - I've installed the CLI tool
📝 **First Cache** - I've created my first `.acp.cache.json`
🎉 **Showcase Shared** - I've shared something in #showcase

Complete all steps to unlock full server access and get the "Onboarded" role!
```

---

*Last updated: 2024-12-19*


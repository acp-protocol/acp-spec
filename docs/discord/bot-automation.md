# Discord Bot Automation Configuration

This document outlines the configuration for Discord MCP server automation, including welcome messages, GitHub integration, and help channel management.

## Discord MCP Server Setup

The ACP Discord server uses the Discord MCP server (`saseq/discord-mcp`) for automation. This server is already configured in your MCP settings.

### Configuration

The MCP server is configured in `~/.cursor/mcp.json`:

```json
{
  "mcpServers": {
    "mcp-server": {
      "command": "docker",
      "args": [
        "run", "--rm", "-i",
        "-e", "DISCORD_TOKEN=<your-token>",
        "saseq/discord-mcp:latest"
      ]
    }
  }
}
```

## Automated Welcome System

### Welcome Message Template

**Channel:** `#welcome`
**Trigger:** New member joins server

```markdown
# Welcome to the ACP Discord Server! 🎉

Welcome to the **AI Context Protocol** community! We're excited to have you here.

## Quick Links
• **GitHub**: https://github.com/acp-protocol/acp-spec
• **Documentation**: https://github.com/acp-protocol/acp-spec/blob/main/spec/ACP-1.0.md
• **CLI Guide**: https://github.com/acp-protocol/acp-spec/blob/main/cli/README.md
• **Join Discord**: https://discord.gg/acp-protocol

## Getting Started

1. **Read the rules** - Check out our community guidelines
2. **Select your roles** - React below to choose roles that match your interests
3. **Visit #getting-started** - Follow our onboarding guide
4. **Install the CLI** - Get started with ACP tools
5. **Join the discussion** - Introduce yourself in #general

## Role Selection

React to select your roles:

🎯 **Developer** - I'm a developer using ACP
🤖 **AI Tool Creator** - I'm building AI tools
🤝 **Contributor** - I want to contribute
📚 **Learner** - I'm learning about ACP

## Need Help?

• **General Questions** → #help
• **CLI Issues** → #cli-discussion
• **Technical Questions** → Relevant technical channel
• **Bot Commands** → Use `/help` to see all commands

Welcome aboard! 🚀
```

### Welcome DM Template

**Trigger:** Sent automatically to new members

```markdown
Hi! Welcome to the ACP Discord server! 👋

I'm here to help you get started with the AI Context Protocol.

## Quick Start Checklist

- [ ] Read the welcome message in #welcome
- [ ] Select your roles (react in #welcome)
- [ ] Check out #getting-started for the onboarding guide
- [ ] Install the CLI tool
- [ ] Create your first `.acp.cache.json`
- [ ] Share your implementation in #showcase

## Useful Commands

• `/quickstart` - Quick start guide
• `/cli-install` - Installation instructions
• `/acp-spec` - Link to specification
• `/help` - List all commands

## Resources

• Getting Started: https://github.com/acp-protocol/acp-spec/blob/main/docs/discord/onboarding.md
• CLI Guide: https://github.com/acp-protocol/acp-spec/blob/main/cli/README.md
• Full Spec: https://github.com/acp-protocol/acp-spec/blob/main/spec/ACP-1.0.md

Need help? Just ask in #help or use `/help` to see available commands!

Welcome to the community! 🎉
```

### Role Assignment

**Roles to Assign Based on Reactions:**

- 🎯 → `Developer` role
- 🤖 → `AI Tool Creator` role
- 🤝 → `Contributor` role
- 📚 → `Learner` role

**Language Roles:**
- TypeScript/JavaScript → `TypeScript/JavaScript` role
- Python → `Python` role
- Rust → `Rust` role
- Go → `Go` role
- Java → `Java` role

**Experience Levels:**
- New to ACP → `New to ACP` role
- Active User → `Active User` role
- Expert → `Expert` role

## GitHub Integration

### Release Announcements

**Channel:** `#announcements`
**Trigger:** GitHub release published

The GitHub Actions workflow (`.github/workflows/discord-webhook.yml`) automatically posts release announcements to Discord.

**Format:**
```
🎉 New Release: ACP CLI v[version]

Version: [tag]
What's New:
[Release notes]

Download:
• CLI Releases: [link]
• Installation Guide: [link]

Full Changelog: [link]
```

### Issue Notifications

**Channels:** Based on issue type
- Bug reports → `#bug-reports`
- Feature requests → `#feature-requests`
- General issues → `#general`

**Format:**
```
🐛 New Issue: [title]

Type: [bug/feature/general]
Author: [username]

[Issue description]

View Issue: [link]
Discuss in: [channel mention]
```

### Pull Request Notifications

**Channel:** `#contributing`
**Trigger:** PR opened or merged

**Format (Opened):**
```
🔧 New Pull Request: [title]

Author: [username]
Branch: [branch] → [base]

[PR description]

View PR: [link]
```

**Format (Merged):**
```
✅ Pull Request Merged: [title]

PR #[number] by [username] has been merged!

Changes: [additions] additions, [deletions] deletions

View PR: [link]

Thank you for contributing! 🎉
```

### Contributor Recognition

**Channel:** `#announcements`
**Trigger:** First contribution merged

**Format:**
```
🌟 Welcome New Contributor: [username]

[username] made their first contribution to ACP!

PR: [title]
View: [link]

Thank you for contributing to the ACP project! 🎉
```

## Help Channel Management

### Auto-Thread Creation

**Trigger:** Message in `#help` channel
**Action:** Automatically create thread for each help question

**Thread Naming:**
- Format: `[Topic] - [Brief Description]`
- Example: `CLI Installation - Error on macOS`

### Expert Tagging

**Based on Topic:**
- CLI questions → Tag `@CLI Experts`
- Annotation questions → Tag `@Annotation Experts`
- Constraint questions → Tag `@Constraint Experts`
- Integration questions → Tag `@Integration Experts`

### Resolution Tracking

**Reactions:**
- ✅ → Mark as solved
- 🔄 → In progress
- ❌ → Not solved

**Auto-Archive:**
- Solved threads archived after 7 days
- Unsolved threads remain active
- Weekly review of unresolved threads

## Activity Tracking

### Member Engagement Levels

**Track:**
- Message count
- Help provided
- Contributions made
- Activity frequency

**Levels:**
- **New** (0-10 messages) → Read-only access
- **Active** (10-50 messages) → Full community access
- **Expert** (50+ messages + contributions) → Contributor access

### Statistics

**Track:**
- New member growth rate
- Active member count
- Help channel response time
- Channel activity levels
- Most helpful members

**Reporting:**
- Weekly statistics in `#announcements`
- Monthly summary
- Quarterly review

## Bot Commands Implementation

### Command Handlers

Commands are implemented using Discord MCP server capabilities:

**Information Commands:**
- `/acp-spec` → Returns specification link
- `/cli-install` → Returns installation guide
- `/quickstart` → Returns quick start guide
- `/examples` → Returns code examples
- `/roadmap` → Returns roadmap link

**Utility Commands:**
- `/acp-validate <code>` → Validates annotation syntax
- `/acp-example <type>` → Shows annotation examples
- `/constraints` → Explains constraint levels
- `/variables` → Explains variable system

**Community Commands:**
- `/contribute` → Returns contribution guide
- `/rfc <number>` → Links to RFC
- `/stats` → Shows server statistics
- `/help` → Lists all commands

## Automation Scripts

### Welcome Flow Script

```javascript
// Pseudo-code for welcome flow
onMemberJoin(member) {
  // Send welcome message in #welcome
  sendWelcomeMessage(member);
  
  // Send welcome DM
  sendWelcomeDM(member);
  
  // Assign default role
  assignRole(member, 'New Member');
  
  // Track onboarding start
  trackOnboarding(member, 'started');
}
```

### Help Channel Script

```javascript
// Pseudo-code for help channel management
onHelpMessage(message) {
  // Create thread
  const thread = createThread(message, {
    name: extractTopic(message.content),
    autoArchive: 86400 // 24 hours
  });
  
  // Tag relevant experts
  const experts = findExperts(message.content);
  tagUsers(thread, experts);
  
  // Track question
  trackQuestion(message, thread);
}
```

## Configuration Files

### Discord Server Configuration

Create `discord-config.json` (not committed, contains sensitive data):

```json
{
  "server_id": "your-server-id",
  "channels": {
    "welcome": "channel-id",
    "announcements": "channel-id",
    "help": "channel-id",
    "contributing": "channel-id",
    "bug-reports": "channel-id",
    "feature-requests": "channel-id"
  },
  "roles": {
    "developer": "role-id",
    "ai_tool_creator": "role-id",
    "contributor": "role-id",
    "learner": "role-id",
    "new_member": "role-id",
    "active": "role-id",
    "expert": "role-id"
  },
  "webhooks": {
    "announcements": "webhook-url",
    "releases": "webhook-url"
  }
}
```

### Bot Command Configuration

Create `bot-commands.json`:

```json
{
  "commands": {
    "acp-spec": {
      "response": "https://github.com/acp-protocol/acp-spec/blob/main/spec/ACP-1.0.md",
      "description": "Link to latest ACP specification"
    },
    "cli-install": {
      "response": "file:docs/discord/bot-commands.md#cli-install",
      "description": "Installation instructions for CLI"
    }
  }
}
```

## Testing

### Test Welcome Flow

1. Join server with test account
2. Verify welcome message posted
3. Verify welcome DM received
4. Verify default role assigned
5. Test role selection reactions

### Test GitHub Integration

1. Create test release
2. Verify announcement posted
3. Create test issue
4. Verify notification posted
5. Create test PR
6. Verify PR notification posted

### Test Help Channel

1. Post question in `#help`
2. Verify thread created
3. Verify experts tagged
4. Mark as solved
5. Verify thread archived after delay

## Monitoring

### Logs

Track:
- Welcome messages sent
- DMs delivered
- Role assignments
- GitHub webhook deliveries
- Help threads created
- Command usage

### Metrics

Monitor:
- Welcome flow completion rate
- Help response time
- GitHub integration success rate
- Bot command usage
- Member engagement

## Troubleshooting

### Common Issues

**Welcome messages not sending:**
- Check MCP server connection
- Verify channel permissions
- Check bot token validity

**GitHub webhooks not working:**
- Verify webhook URL in secrets
- Check GitHub Actions workflow
- Verify webhook permissions

**Help threads not creating:**
- Check channel permissions
- Verify bot has thread creation permission
- Check message content parsing

## Security

### Token Management

- Store Discord token in environment variables
- Never commit tokens to repository
- Rotate tokens regularly
- Use least privilege permissions

### Permissions

**Bot Required Permissions:**
- Send Messages
- Manage Messages
- Create Threads
- Manage Threads
- Add Reactions
- Manage Roles (limited)
- Read Message History

## Resources

- [Discord MCP Server](https://github.com/saseq/discord-mcp)
- [Discord API Documentation](https://discord.com/developers/docs)
- [GitHub Actions Documentation](https://docs.github.com/en/actions)
- [Discord Bot Best Practices](https://discord.com/developers/docs/topics/community-resources)

---

*Last updated: 2024-12-19*


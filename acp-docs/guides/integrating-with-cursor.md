# How to Integrate ACP with Cursor

**Document Type**: How-To Guide  
**Status**: OUTLINE — Content to be added  
**Goal**: Set up Cursor IDE to use ACP context  
**Prerequisites**: ACP CLI installed, Cursor IDE installed  
**Time**: 10-15 minutes

---

## The Problem

You want Cursor to understand your codebase's constraints and structure so it generates better, safer code suggestions.

---

## Solution Overview

1. Initialize ACP in your project
2. Generate the `.cursorrules` file
3. Configure Cursor to use ACP context
4. Verify the integration

---

## Step 1: Initialize ACP

> **TODO**: Add screenshots

If you haven't already:

```bash
cd your-project
acp init
```

---

## Step 2: Add Annotations

Add at least critical annotations:

```typescript
// @acp:lock frozen - Critical code, DO NOT modify
// @acp:domain auth - Authentication domain
```

See [Annotating Your Codebase](annotating-your-codebase.md) for details.

---

## Step 3: Generate Cursor Rules

```bash
acp sync --tools cursor
```

This creates `.cursorrules` with:
- Frozen file list
- Domain boundaries
- Constraint summaries
- Project structure overview

### Customize the Output

```bash
# More detailed context
acp sync --tools cursor --budget 1000

# Safety-focused
acp sync --tools cursor --preset safe
```

---

## Step 4: Verify `.cursorrules`

> **TODO**: Add example content

Check the generated file:

```bash
cat .cursorrules
```

Expected structure:
```markdown
# Project Context

## Constraints

### Frozen Files (DO NOT MODIFY)
- src/auth/session.ts - Security critical
- src/payments/processor.ts - Payment processing

### Restricted Files (Require Approval)
- src/api/public/*.ts - Public API

## Domains
- auth: Authentication and authorization
- payments: Payment processing
- users: User management

## Architecture
[Domain relationships and layer structure]
```

---

## Step 5: Configure Cursor

> **TODO**: Add screenshots, version-specific instructions

### Automatic Detection

Cursor automatically reads `.cursorrules` from:
- Project root
- `.cursor/` directory

### Manual Configuration

If automatic detection doesn't work:

1. Open Cursor Settings
2. Navigate to AI → Context
3. Add `.cursorrules` path

---

## Step 6: Test the Integration

> **TODO**: Add test scenarios

### Test 1: Frozen File Respect

1. Open a frozen file in Cursor
2. Ask: "Refactor this function"
3. Expected: Cursor should acknowledge constraint

### Test 2: Domain Awareness

1. Ask: "Add a new feature to auth"
2. Expected: Cursor suggests files in auth domain only

### Test 3: Context Inclusion

1. Ask: "What are the main domains?"
2. Expected: Cursor lists domains from ACP

---

## Keeping Cursor Rules Updated

### Manual Update

```bash
acp sync --tools cursor
```

### Automatic Update

Use the ACP daemon for real-time updates:

```bash
acp daemon start
```

Or use the VS Code extension with auto-sync enabled.

### Git Hook (Optional)

```bash
# .git/hooks/post-commit
#!/bin/sh
acp index && acp sync --tools cursor
```

---

## Advanced Configuration

### Custom Token Budget

```json
// .acp.sync.json
{
  "tools": {
    "cursor": {
      "enabled": true,
      "budget": 750
    }
  }
}
```

### Custom Primer Weights

```json
{
  "primer": {
    "weights": {
      "safety": 2.0,
      "structure": 1.0,
      "efficiency": 0.5
    }
  }
}
```

### Multiple Cursor Profiles

> **TODO**: Document profile-specific rules

---

## Verification

### Check Cursor Is Using Rules

1. Open a new Cursor chat
2. Ask: "What files are frozen in this project?"
3. Cursor should list files from `.cursorrules`

### Debug Mode

```bash
acp sync --tools cursor --verbose
```

Shows what's included/excluded and why.

---

## Troubleshooting

### Cursor Not Reading Rules

1. Verify `.cursorrules` exists in project root
2. Restart Cursor
3. Check Cursor version (rules support added in X.X)

### Rules Seem Outdated

1. Run `acp sync --tools cursor`
2. Check `acp index` is up to date
3. Verify git commit matches cache

### Context Too Large

1. Reduce budget: `--budget 300`
2. Use `--preset minimal`
3. Exclude non-essential sections

### Context Too Small

1. Increase budget: `--budget 1000`
2. Use `--preset detailed`
3. Add specific sections in config

---

## Related

- [Integrating with Claude Code](integrating-with-claude-code.md)
- [Custom Primer Templates](custom-primers.md)
- [CLI Reference: sync command](../tooling/cli.md#acp-sync)

---

*This guide is an outline. [Contribute content →](https://github.com/acp-protocol/docs)*

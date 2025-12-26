# ACP Primers

This directory contains primer/bootstrap prompts for AI assistants working with ACP-enabled codebases.

## Primer Format

Each primer is a markdown file with YAML frontmatter:

```markdown
---
name: Primer Name
version: "1.0"
tokens: 50
description: What this primer does
tags: [tag1, tag2]
---

Actual primer content goes here...
```

## Frontmatter Fields

| Field | Required | Description |
|-------|----------|-------------|
| `name` | Yes | Human-readable primer name |
| `version` | Yes | Primer version string |
| `tokens` | No | Estimated token count (auto-calculated if missing) |
| `description` | Yes | Brief description of the primer's approach |
| `tags` | No | Tags for categorization and filtering |

## Available Primers

| Primer | Tokens | Description |
|--------|--------|-------------|
| `minimal` | ~40 | Bare minimum bootstrap |
| `v2-context` | ~65 | Explains what ACP caches |
| `v3-levels` | ~95 | Adds constraint level definitions |
| `v4-imperative` | ~110 | More direct, imperative instructions |
| `v5-examples` | ~140 | Includes concrete usage examples |
| `v6-strict` | ~130 | Emphasizes safety requirements |
| `v7-conditional` | ~85 | Minimal with self-documenting outputs |

## Custom Primers

Place custom primers in the `custom/` subdirectory. They will be loaded by name:

```bash
npx primer-eval --primer custom/my-primer
```

## Writing Effective Primers

1. **Be concise** - Every token costs money and context space
2. **Be imperative** - Use direct commands: "MUST", "ALWAYS", "NEVER"
3. **Define terms** - Explain what constraint levels mean
4. **Show examples** - Concrete examples aid comprehension
5. **Prioritize safety** - Frozen files should never be modified

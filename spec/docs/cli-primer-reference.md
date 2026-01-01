# ACP CLI Primer Command Reference

**Version**: 1.0.0
**Specification**: ACP 1.3.0 (RFC-0015)
**Last Updated**: 2026-01-01

---

## Overview

This document provides a complete reference for the ACP CLI `primer` and `context` commands, which implement RFC-0015 "Primer System Redesign: Accuracy-Focused, Context-Aware Bootstrap".

## Commands

### `acp primer`

Generate tiered AI bootstrap context based on token budget.

#### Synopsis

```bash
acp primer [OPTIONS]
```

#### Options

| Option              | Type     | Default   | Description                              |
|---------------------|----------|-----------|------------------------------------------|
| `--budget <N>`      | integer  | 200       | Token budget for primer output           |
| `--preset <NAME>`   | string   | *(none)*  | Weight preset (safe, efficient, accurate, balanced) |
| `--capabilities`    | string[] | *(none)*  | Required capabilities (e.g., "shell", "mcp") |
| `--cache <PATH>`    | path     | *(auto)*  | Path to cache file                       |
| `--primer-config`   | path     | *(auto)*  | Custom primer config file                |
| `--format`          | enum     | markdown  | Output format (markdown, text)           |
| `--json`            | flag     | false     | Output as JSON metadata                  |
| `--include <IDS>`   | string[] | *(none)*  | Force include section IDs                |
| `--exclude <IDS>`   | string[] | *(none)*  | Exclude section IDs                      |
| `--categories`      | string[] | *(none)*  | Filter by category IDs                   |
| `--no-dynamic`      | flag     | false     | Disable dynamic value modifiers          |
| `--explain`         | flag     | false     | Show selection reasoning                 |
| `--list-sections`   | flag     | false     | List available sections                  |
| `--list-presets`    | flag     | false     | List available presets                   |
| `--preview`         | flag     | false     | Preview selection without rendering      |
| `--standalone`      | flag     | false     | Include foundation prompt for raw API    |

#### Tier Selection

The budget determines which tier is automatically selected:

| Tier     | Budget Range | CLI Tokens | MCP Tokens | Use Case                    |
|----------|--------------|------------|------------|------------------------------|
| Micro    | <300         | ~250       | ~178       | Essential safety constraints |
| Minimal  | 300-449      | ~400       | ~285       | Core project context         |
| Standard | 450-699      | ~600       | ~427       | Balanced context             |
| Full     | >=700        | ~1400      | ~1000      | Complete understanding       |

#### Weight Presets

| Preset     | Safety | Efficiency | Accuracy | Base | Description                    |
|------------|--------|------------|----------|------|--------------------------------|
| `safe`     | 2.0    | 1.0        | 1.0      | 1.0  | Prioritize safety sections     |
| `efficient`| 1.0    | 2.0        | 1.0      | 1.0  | Minimize token usage           |
| `accurate` | 1.0    | 1.0        | 2.0      | 1.0  | Maximize accuracy context      |
| `balanced` | 1.0    | 1.0        | 1.0      | 1.0  | Equal weight (default)         |

#### Examples

```bash
# Standard tier with default settings
acp primer --budget 500

# Full tier with JSON output
acp primer --budget 800 --json

# Safe preset for security-critical projects
acp primer --budget 600 --preset safe

# MCP capability filtering
acp primer --budget 500 --capabilities mcp

# Preview selection without output
acp primer --budget 600 --explain --preview

# Standalone mode for raw API usage
acp primer --budget 500 --standalone
```

---

### `acp context`

Get operation-specific context for AI agents.

#### Synopsis

```bash
acp context <OPERATION> [OPTIONS]
```

#### Operations

##### `create`

Get context for creating new files. Returns naming conventions, anti-patterns, and import style guidance.

```bash
acp context create [--directory <PATH>]
```

| Option              | Type   | Description                       |
|---------------------|--------|-----------------------------------|
| `--directory <PATH>`| path   | Target directory for new file     |

**Output includes:**
- Naming conventions (patterns with confidence scores)
- Anti-patterns (what NOT to use)
- Import conventions (module system, path style)
- Related files in directory

##### `modify`

Get context for modifying existing files. Returns constraints, dependencies, and affected files.

```bash
acp context modify --file <PATH>
```

| Option         | Type   | Required | Description                |
|----------------|--------|----------|----------------------------|
| `--file <PATH>`| path   | Yes      | File to modify             |

**Output includes:**
- Lock level and lock reason
- Style constraints
- Files that import this file (`imported_by`)
- Related symbols and dependencies

##### `debug`

Get context for debugging issues. Returns file relationships, symbols, and domain context.

```bash
acp context debug --file <PATH>
```

| Option         | Type   | Required | Description                |
|----------------|--------|----------|----------------------------|
| `--file <PATH>`| path   | Yes      | File being debugged        |

**Output includes:**
- Symbol definitions and relationships
- Call graph (callers/callees)
- Related files in same domain
- Debug history (if attempts file exists)

##### `explore`

Get context for exploring the project. Returns project overview and domain structure.

```bash
acp context explore
```

**Output includes:**
- Project summary and primary language
- Domain hierarchy
- Key architectural patterns
- File statistics

#### Common Options

| Option         | Type   | Default | Description                |
|----------------|--------|---------|----------------------------|
| `--format`     | enum   | human   | Output format (human, json)|
| `--cache`      | path   | *(auto)*| Path to cache file         |

#### Examples

```bash
# Get naming conventions for creating a new component
acp context create --directory src/components

# Check constraints before modifying a file
acp context modify --file src/auth/login.ts

# Get debug context for troubleshooting
acp context debug --file src/utils/helpers.ts

# Explore project structure
acp context explore

# JSON output for MCP integration
acp context modify --file src/api/users.ts --format json
```

---

## MCP Integration

### `acp_primer` Tool

Generate primer context via MCP.

```json
{
  "name": "acp_primer",
  "arguments": {
    "budget": 500,
    "preset": "balanced"
  }
}
```

### `acp_context` Tool

Get operation-specific context via MCP.

```json
{
  "name": "acp_context",
  "arguments": {
    "operation": "modify",
    "file": "src/auth/login.ts"
  }
}
```

**Parameters:**

| Parameter   | Type   | Required | Description                           |
|-------------|--------|----------|---------------------------------------|
| `operation` | enum   | Yes      | One of: create, modify, debug, explore|
| `file`      | string | Depends  | Required for modify/debug operations  |
| `directory` | string | No       | Optional for create operation         |

---

## IDE Environment Detection

The CLI automatically detects IDE environments to provide appropriate warnings:

| Environment | Detection Method              |
|-------------|-------------------------------|
| Cursor      | `CURSOR_*` environment vars   |
| VS Code     | `VSCODE_*` environment vars   |
| Cline       | `CLINE_*` environment vars    |
| JetBrains   | `JETBRAINS_*` environment vars|
| Zed         | `ZED_*` environment vars      |
| Claude Code | `CLAUDE_CODE` environment var |

### Disabling Detection

Set `ACP_NO_IDE_DETECT=1` to disable IDE detection:

```bash
ACP_NO_IDE_DETECT=1 acp primer --standalone --budget 500
```

---

## Configuration

### Primer Configuration File

Custom primer configurations can be defined in `.acp.primer.json`:

```json
{
  "version": "1.0.0",
  "sections": {
    "custom-safety": {
      "content": "Custom safety instructions...",
      "tokens": 50,
      "value": {
        "safety": 90,
        "efficiency": 30,
        "accuracy": 50
      }
    }
  },
  "defaults": {
    "preset": "balanced",
    "budget": 500
  }
}
```

### Project State Integration

The primer system reads project state from the cache to apply dynamic value modifiers:

- Language-specific sections prioritized based on `stats.primary_language`
- Constraint-heavy sections elevated when many locks exist
- Domain sections included based on indexed domains

---

## See Also

- [Chapter 11: Tool Integration](../chapters/11-tool-integration.md) - Tiered system specification
- [Chapter 14: Bootstrap & AI Integration](../chapters/14-bootstrap.md) - Command details
- [Chapter 03: Cache Format](../chapters/03-cache-format.md) - Cache structure for context
- [RFC-0015](../../rfcs/rfc-0015-primer-redesign.md) - Full specification

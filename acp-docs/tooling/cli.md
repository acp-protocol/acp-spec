# ACP CLI Documentation

**Document Type**: Reference + How-To  
**Status**: OUTLINE — Content to be added  
**Last Updated**: December 2025

---

## Overview

The ACP CLI is the primary tool for working with the AI Context Protocol. It provides commands for initialization, indexing, querying, and integration with AI coding assistants.

---

## Installation

> **TODO**: Expand each section with detailed instructions

### npm
```bash
npm install -g @acp-protocol/cli
```

### Homebrew
```bash
brew install acp-protocol/tap/acp
```

### Cargo
```bash
cargo install acp-cli
```

### Binary Downloads
- [ ] Add download links for each platform
- [ ] Add verification instructions (checksums)
- [ ] Add shell completion setup

---

## Commands Reference

### `acp init`

Initialize ACP in a project.

**Synopsis**:
```bash
acp init [options]
```

**Options**:
| Flag | Description | Default |
|------|-------------|---------|
| `--force` | Overwrite existing config | `false` |
| `--template <name>` | Use a preset template | `default` |
| `--no-gitignore` | Don't update .gitignore | `false` |

> **TODO**: Add examples, common use cases, error handling

---

### `acp index`

Generate or update the cache file.

**Synopsis**:
```bash
acp index [options] [path]
```

**Options**:
| Flag | Description | Default |
|------|-------------|---------|
| `--force` | Regenerate from scratch | `false` |
| `--watch` | Watch for changes | `false` |
| `--output <path>` | Custom output path | `.acp/acp.cache.json` |
| `--stats` | Show detailed statistics | `false` |

> **TODO**: Add performance considerations, incremental indexing, large codebase handling

---

### `acp query`

Query the cache using jq expressions.

**Synopsis**:
```bash
acp query <expression> [options]
```

**Options**:
| Flag | Description | Default |
|------|-------------|---------|
| `--raw` | Raw output (no formatting) | `false` |
| `--cache <path>` | Custom cache path | `.acp/acp.cache.json` |

> **TODO**: Add common query patterns, examples, jq cheatsheet

---

### `acp constraints`

Check constraints for a file or symbol.

**Synopsis**:
```bash
acp constraints <target> [options]
```

**Options**:
| Flag | Description | Default |
|------|-------------|---------|
| `--json` | JSON output | `false` |
| `--verbose` | Include inheritance chain | `false` |

> **TODO**: Add output examples, integration with CI/CD

---

### `acp sync`

Synchronize ACP context to AI tool files.

**Synopsis**:
```bash
acp sync [options]
```

**Options**:
| Flag | Description | Default |
|------|-------------|---------|
| `--tools <list>` | Specific tools | Auto-detect |
| `--dry-run` | Preview without writing | `false` |
| `--budget <tokens>` | Token budget | `500` |

> **TODO**: Add tool-specific configurations, custom templates

---

### `acp primer`

Generate primer content for AI assistants.

**Synopsis**:
```bash
acp primer [options]
```

**Options**:
| Flag | Description | Default |
|------|-------------|---------|
| `--budget <tokens>` | Token budget | `500` |
| `--preset <name>` | Preset (safe, balanced, detailed) | `balanced` |
| `--preview` | Show selection matrix | `false` |
| `--compare <budgets>` | Compare multiple budgets | - |

> **TODO**: Add primer customization, weight tuning

---

### `acp validate`

Validate ACP files and annotations.

**Synopsis**:
```bash
acp validate [options] [files...]
```

**Options**:
| Flag | Description | Default |
|------|-------------|---------|
| `--strict` | Strict mode (fail on warnings) | `false` |
| `--fix` | Auto-fix issues | `false` |

> **TODO**: Add validation rules, common errors

---

### `acp start`

Start the ACP proxy server for AI tool integration.

**Synopsis**:
```bash
acp start [options]
```

**Options**:
| Flag | Description | Default |
|------|-------------|---------|
| `--port <port>` | Proxy port | `auto` |
| `--daemon` | Run in background | `false` |

> **TODO**: Add proxy architecture, tool configuration

---

## Configuration

> **TODO**: Expand with full `.acp.config.json` reference

### Minimal Configuration
```json
{
  "version": "1.0.0"
}
```

### Full Configuration Template
```json
{
  "version": "1.0.0",
  "include": [],
  "exclude": [],
  "error_handling": {},
  "constraints": {},
  "domains": {},
  "limits": {}
}
```

---

## Environment Variables

| Variable | Description | Default |
|----------|-------------|---------|
| `ACP_CONFIG` | Custom config path | `.acp.config.json` |
| `ACP_CACHE` | Custom cache path | `.acp/acp.cache.json` |
| `ACP_LOG_LEVEL` | Log verbosity | `info` |
| `ACP_NO_COLOR` | Disable color output | `false` |

---

## Exit Codes

| Code | Meaning |
|------|---------|
| `0` | Success |
| `1` | General error |
| `2` | Configuration error |
| `3` | Validation error |
| `4` | File not found |

---

## Sections to Add

- [ ] **Troubleshooting**: Common issues and solutions
- [ ] **Performance**: Optimizing for large codebases
- [ ] **CI/CD Integration**: GitHub Actions, GitLab CI examples
- [ ] **Shell Completion**: Bash, Zsh, Fish, PowerShell
- [ ] **Logging**: Debug logging, log file locations
- [ ] **Upgrade Guide**: Migrating between versions

---

*This document is an outline. [Contribute content →](https://github.com/acp-protocol/acp-cli)*

# RFC-0004: Tiered Interface Primers

- **RFC ID**: 0004
- **Title**: Tiered Interface Primers
- **Author**: ACP Protocol Team
- **Status**: Accepted
- **Created**: 2025-12-22
- **Updated**: 2025-12-22
- **Discussion**: [Pending GitHub Discussion]
- **Supersedes**: Portions of Chapter 11 (Tool Integration) primer system

---

## Summary

This RFC proposes a fundamental simplification of the ACP primer system. Building on RFC-001's insight that annotations are self-documenting directives, it eliminates "teaching" sections that explain protocol semantics and replaces them with tiered interface documentation. The primer's sole purpose becomes: (1) establishing that `@acp:*` tags are directives, (2) documenting available CLI/MCP/daemon commands at appropriate detail levels, and (3) providing the critical pre-edit workflow.

## Motivation

### Problem Statement

RFC-001 established that annotations should be self-documenting:

```typescript
// OLD: Requires bootstrap explanation
// @acp:lock frozen

// NEW: Self-explanatory  
// @acp:lock frozen - MUST NOT modify this file under any circumstances
```

However, the current primer system (Chapter 11) was designed *before* this insight and still contains ~35 sections that attempt to teach AI agents what ACP concepts mean. This creates several problems:

1. **Redundant Content**: Sections like `lock-frozen`, `lock-restricted`, `constraint-concept`, and `annotation-syntax` duplicate information that is now embedded in the directives themselves.

2. **Over-Engineering**: The multi-dimensional value scoring system (safety/efficiency/accuracy/base), dynamic modifiers, category budgets, and complex selection algorithms were designed to optimize which *teaching* content to include—a problem that no longer exists.

3. **Misplaced Complexity**: The current system has sophisticated machinery for selecting educational content but provides only basic documentation of the actual interface (CLI commands, MCP tools).

4. **Token Waste**: At typical budgets (200-500 tokens), significant space is consumed explaining concepts the AI can derive from directive text.

### Goals

- Eliminate all "teaching" sections from the primer system
- Provide tiered (minimal/standard/full) documentation for each CLI command, MCP tool, and daemon API endpoint
- Simplify the primer schema by removing multi-dimensional scoring machinery
- Reduce minimum viable bootstrap to ~40 tokens
- Enable budget-aware interface documentation depth

### Non-Goals

- Changing the annotation syntax or directive format (RFC-001 scope)
- Modifying the `acp sync` command behavior
- Changing tool adapter implementations
- Altering the cache format

## Detailed Design

### Overview

The new primer system has three components:

1. **Bootstrap Block** (~15-20 tokens): Always included, establishes ACP awareness and critical workflow
2. **Interface Documentation**: Tiered command/tool reference scaled to budget
3. **Project-Specific Warnings**: Dynamic content from cache (protected files, active sessions)

```
┌─────────────────────────────────────────────────────────────────────────┐
│                    Simplified Primer Architecture                       │
├─────────────────────────────────────────────────────────────────────────┤
│                                                                         │
│  BOOTSTRAP (~20 tokens) - Always included                               │
│  ├── Awareness: "@acp:* comments are directives for you"                │
│  ├── Workflow: "acp constraints <path> before editing"                  │
│  └── Expansion: "acp primer --budget N for more"                        │
│                                                                         │
│  INTERFACE (budget-scaled)                                              │
│  ├── Tier 1 (minimal): Command + one-line purpose                       │
│  ├── Tier 2 (standard): + key options + output shape                    │
│  └── Tier 3 (full): + examples + usage patterns                         │
│                                                                         │
│  PROJECT-SPECIFIC (if budget allows)                                    │
│  ├── Protected files list (from cache)                                  │
│  └── Active debug sessions (from cache)                                 │
│                                                                         │
└─────────────────────────────────────────────────────────────────────────┘
```

### Tier Definitions

| Tier         | Budget Range   | Content Depth                            | Use Case                                  |
|--------------|----------------|------------------------------------------|-------------------------------------------|
| **Minimal**  | <100 tokens    | Command + one-line purpose               | Extremely constrained; AI can self-expand |
| **Standard** | 100-350 tokens | + key options, output shape, when to use | Normal usage                              |
| **Full**     | 350+ tokens    | + examples, patterns, edge cases         | Dedicated agents, complex projects        |

### Bootstrap Block

The bootstrap block is **always included** regardless of budget:

```
This project uses ACP. @acp:* comments are directives for you.
Before editing: acp constraints <path>
More: acp primer --budget N
```

**Token count**: ~20 tokens

This is sufficient because:
- The AI knows to follow `@acp:*` directive text
- The AI knows the critical pre-edit workflow
- The AI knows how to get more context

### Interface Documentation Structure

Each command/tool has three tier definitions:

```json
{
  "name": "constraints",
  "critical": true,
  "capabilities": ["shell"],
  "tiers": {
    "minimal": {
      "tokens": 8,
      "template": "acp constraints <path> → Check before editing"
    },
    "standard": {
      "tokens": 35,
      "template": "acp constraints <path>\n  Returns: lock level + directive\n  Levels: frozen (refuse), restricted (ask), normal (proceed)"
    },
    "full": {
      "tokens": 80,
      "template": "acp constraints <path> [--format json|text]\n  Purpose: Check file modification permissions\n  Output: Lock level, directive text, owner, rules\n  Example:\n    $ acp constraints src/auth/session.ts\n    frozen - MUST NOT modify; security-critical\n  CRITICAL: Always run before modifying any file."
    }
  }
}
```

### Command Priority

Commands are included in priority order until budget exhausted:

| Priority   | Command        | Critical  | Rationale                      |
|------------|----------------|-----------|--------------------------------|
| 1          | `constraints`  | Yes       | Safety-critical pre-edit check |
| 2          | `query file`   | No        | Most common context retrieval  |
| 3          | `query symbol` | No        | Understanding function impact  |
| 4          | `query domain` | No        | Cross-file work                |
| 5          | `map`          | No        | Navigation                     |
| 6          | `knowledge`    | No        | Exploration                    |
| 7          | `attempt`      | No        | Debug tracking                 |
| 8          | `primer`       | No        | Self-expansion (implicit)      |

Critical commands are **always** included at minimum tier if budget allows bootstrap + critical commands.

### Schema Changes

#### Update Schema: `primer.v1.schema.json`

```json
{
  "$schema": "https://json-schema.org/draft-07/schema#",
  "$id": "https://acp-protocol.dev/schemas/v1/primer.schema.json",
  "title": "ACP Primer Schema v2",
  "description": "Tiered interface documentation for AI agents",
  
  "type": "object",
  "required": ["version", "bootstrap", "interface"],
  "additionalProperties": false,
  
  "properties": {
    "$schema": {
      "type": "string",
      "description": "Reference to this schema"
    },
    "version": {
      "type": "string",
      "pattern": "^2\\.\\d+\\.\\d+",
      "description": "Schema version (must be 2.x.x)"
    },
    "metadata": {
      "$ref": "#/$defs/metadata"
    },
    "bootstrap": {
      "$ref": "#/$defs/bootstrap"
    },
    "interface": {
      "$ref": "#/$defs/interface"
    },
    "project": {
      "$ref": "#/$defs/projectConfig"
    }
  },
  
  "$defs": {
    "metadata": {
      "type": "object",
      "properties": {
        "name": { "type": "string" },
        "description": { "type": "string" },
        "author": { "type": "string" },
        "license": { "type": "string" },
        "minAcpVersion": { 
          "type": "string",
          "pattern": "^\\d+\\.\\d+\\.\\d+"
        }
      }
    },
    
    "bootstrap": {
      "type": "object",
      "description": "Core awareness block - always included",
      "required": ["awareness", "workflow"],
      "properties": {
        "awareness": {
          "type": "string",
          "description": "Establishes ACP and directive pattern",
          "default": "This project uses ACP. @acp:* comments are directives for you."
        },
        "workflow": {
          "type": "string",
          "description": "Critical pre-edit workflow",
          "default": "Before editing: acp constraints <path>"
        },
        "expansion": {
          "type": "string",
          "description": "How to get more context",
          "default": "More: acp primer --budget N"
        },
        "tokens": {
          "type": "integer",
          "description": "Approximate token count for bootstrap",
          "default": 20
        }
      }
    },
    
    "interface": {
      "type": "object",
      "description": "Tiered command/tool documentation",
      "properties": {
        "cli": {
          "$ref": "#/$defs/commandSet"
        },
        "mcp": {
          "$ref": "#/$defs/commandSet"
        },
        "daemon": {
          "$ref": "#/$defs/commandSet"
        }
      }
    },
    
    "commandSet": {
      "type": "object",
      "properties": {
        "commands": {
          "type": "array",
          "items": { "$ref": "#/$defs/command" }
        }
      }
    },
    
    "command": {
      "type": "object",
      "required": ["name", "tiers"],
      "properties": {
        "name": {
          "type": "string",
          "description": "Command name (e.g., 'constraints', 'query file')"
        },
        "critical": {
          "type": "boolean",
          "description": "If true, always include at minimum tier",
          "default": false
        },
        "priority": {
          "type": "integer",
          "description": "Selection priority (lower = higher priority)",
          "minimum": 1,
          "maximum": 100
        },
        "capabilities": {
          "type": "array",
          "items": { 
            "type": "string",
            "enum": ["shell", "mcp", "file-read", "file-write", "http"]
          },
          "description": "Required tool capabilities"
        },
        "tiers": {
          "type": "object",
          "required": ["minimal"],
          "properties": {
            "minimal": { "$ref": "#/$defs/tierContent" },
            "standard": { "$ref": "#/$defs/tierContent" },
            "full": { "$ref": "#/$defs/tierContent" }
          }
        }
      }
    },
    
    "tierContent": {
      "type": "object",
      "required": ["tokens", "template"],
      "properties": {
        "tokens": {
          "type": "integer",
          "description": "Approximate token count",
          "minimum": 1
        },
        "template": {
          "type": "string",
          "description": "The actual content template"
        }
      }
    },
    
    "projectConfig": {
      "type": "object",
      "description": "Project-specific dynamic content",
      "properties": {
        "showProtectedFiles": {
          "type": "boolean",
          "description": "Include list of frozen/restricted files",
          "default": true
        },
        "maxProtectedFiles": {
          "type": "integer",
          "description": "Maximum protected files to list",
          "default": 5
        },
        "showActiveSessions": {
          "type": "boolean",
          "description": "Include active debug sessions",
          "default": true
        },
        "customRules": {
          "type": "string",
          "description": "Project-specific rules to append"
        }
      }
    }
  }
}
```

#### Deprecated Schema Elements

The following v1 schema elements are **deprecated**:

| Element                                | Replacement                   |
|----------------------------------------|-------------------------------|
| `categories`                           | Removed - no longer needed    |
| `sections[].value` (multi-dimensional) | Removed - use `priority`      |
| `sections[].value.modifiers`           | Removed                       |
| `selectionStrategy.weights`            | Removed                       |
| `outputFormats`                        | Simplified to tier templates  |
| `knowledgeStore`                       | Moved to separate concern     |
| `capabilities` registry                | Inline in command definitions |

### Selection Algorithm

The new algorithm is dramatically simpler:

```python
def generate_primer(budget: int, capabilities: list[str], project_state: dict) -> str:
    output = []
    remaining = budget
    
    # 1. Always include bootstrap
    output.append(BOOTSTRAP_TEMPLATE)
    remaining -= BOOTSTRAP_TOKENS  # ~20 tokens
    
    # 2. Determine tier based on remaining budget
    if remaining < 80:
        tier = "minimal"
    elif remaining < 300:
        tier = "standard"
    else:
        tier = "full"
    
    # 3. Get commands matching capabilities, sorted by priority
    commands = get_commands_for_capabilities(capabilities)
    commands.sort(key=lambda c: (not c.critical, c.priority))
    
    # 4. Add commands until budget exhausted
    for cmd in commands:
        content = cmd.tiers.get(tier) or cmd.tiers["minimal"]
        if content.tokens <= remaining:
            output.append(content.template)
            remaining -= content.tokens
        elif cmd.critical and cmd.tiers["minimal"].tokens <= remaining:
            # Critical commands get minimal tier as fallback
            output.append(cmd.tiers["minimal"].template)
            remaining -= cmd.tiers["minimal"].tokens
    
    # 5. Add project-specific warnings if budget allows
    if remaining > 40 and project_state.get("frozen_files"):
        files = project_state["frozen_files"][:5]
        output.append(f"⚠️ Protected: {', '.join(files)}")
        remaining -= 30
    
    if remaining > 30 and project_state.get("active_sessions"):
        sessions = project_state["active_sessions"]
        output.append(f"🐛 Active debug: {sessions[0]}")
    
    return "\n\n".join(output)
```

### Behavior

#### Tier Selection

| Remaining Budget   | Selected Tier  |
|--------------------|----------------|
| < 80 tokens        | minimal        |
| 80-299 tokens      | standard       |
| ≥ 300 tokens       | full           |

#### Budget Examples

**Budget: 60 tokens (Minimal)**
```
This project uses ACP. @acp:* comments are directives for you.
Before editing: acp constraints <path>
More: acp primer --budget N

acp constraints <path> → Check before editing
acp query file <path> → Get file context
acp query symbol <name> → Get symbol details
acp map [path] → Visual structure
```

**Budget: 200 tokens (Standard)**
```
This project uses ACP. @acp:* comments are directives for you.
Before editing: acp constraints <path>
More: acp primer --budget N

acp constraints <path>
  Returns: lock level + directive
  Levels: frozen (refuse), restricted (ask), normal (proceed)

acp query file <path> [--depth shallow|deep]
  Returns: purpose, constraints, symbols, dependencies
  Use: Understand file before working with it

acp query symbol <name>
  Returns: signature, location, callers, callees
  Use: Understanding function impact before modifying

acp query domain <name>
  Returns: domain files, boundaries, cross-domain deps
  Use: Working on features spanning multiple files

acp map [path] [--depth N]
  Returns: tree with constraint indicators
  Use: Navigation, understanding structure

acp knowledge "question"
  Returns: answer from indexed codebase
  Use: "how does auth work?", "where is X?"
```

**Budget: 500 tokens (Full)**

Includes all commands with examples, patterns, and edge cases, plus project-specific warnings.

### Error Handling

| Error Condition          | Behavior                                |
|--------------------------|-----------------------------------------|
| Budget < 40 tokens       | Error: "Budget too small for bootstrap" |
| No matching capabilities | Include only bootstrap + expansion hint |
| Missing tier definition  | Fall back to next lower tier            |
| Invalid template         | Skip command, log warning               |

### Examples

**Example 1: CLI-capable tool at standard budget**

```bash
$ acp primer --budget 200 --capabilities shell
```

Output:
```
This project uses ACP. @acp:* comments are directives for you.
Before editing: acp constraints <path>
More: acp primer --budget N

acp constraints <path>
  Returns: lock level + directive
  Levels: frozen (refuse), restricted (ask), normal (proceed)

acp query file <path> [--depth shallow|deep]
  Returns: purpose, constraints, symbols, dependencies
  Use: Understand file before working with it

[... additional commands at standard tier ...]
```

**Example 2: MCP-capable tool at minimal budget**

```bash
$ acp primer --budget 80 --capabilities mcp
```

Output:
```
This project uses ACP. @acp:* comments are directives for you.
Before editing: acp_constraints({ path })
More: acp_primer({ budget: N })

acp_constraints({ path }) → Check before editing
acp_query({ type, target }) → Query codebase
acp_knowledge({ query }) → Ask about codebase
```

**Example 3: Full budget with project warnings**

```bash
$ acp primer --budget 600 --capabilities shell
```

Output includes full tier documentation plus:
```
⚠️ Protected: src/auth/session.ts, src/billing/stripe.ts, config/secrets.ts
🐛 Active debug: session-timeout-fix (attempt 3)
```

## Drawbacks

1. **Breaking Change**: Existing `primer.defaults.json` format is incompatible with v2 schema. Projects with custom primer sections will need migration.

2. **Reduced Flexibility**: The multi-dimensional scoring allowed fine-grained control over section selection. The new priority-based system is simpler but less nuanced.

3. **MCP Documentation Gap**: Some MCP tools have complex parameter schemas that may not fit well into tiered templates.

4. **Learning Curve**: Users familiar with the v1 categories/weights system will need to learn the new tier-based approach.

## Alternatives

### Alternative A: Keep v1 with Pruned Sections

Simply remove teaching sections from v1 while keeping the scoring system.

**Rejected because**: The scoring machinery adds complexity without benefit once teaching sections are removed. The tier approach better matches the actual use case (interface documentation at varying depths).

### Alternative B: Single Detailed Level

Provide only full documentation, rely on AI to extract what it needs.

**Rejected because**: Token budgets are real constraints. A 60-token budget cannot fit full documentation for all commands.

### Alternative C: Per-Command Budgets

Allow budget allocation per command rather than global tiers.

**Deferred**: Could be added in v2.1 if needed, but adds complexity. The tier approach handles most use cases.

## Adoption Strategy

### Backward Compatibility

- v1 schema remains supported for one major version
- `acp primer` auto-detects schema version from `version` field
- Warning emitted when using v1 schema

### Migration Path

1. **Phase 1**: Implement v2 schema and selection algorithm
2. **Phase 2**: Create `primer.defaults.v2.json` with tiered commands
3. **Phase 3**: Update `acp sync` to prefer v2 when available
4. **Phase 4**: Deprecation warnings for v1
5. **Phase 5**: Remove v1 support in next major version

### Tooling Impact

| Tool               | Impact  | Required Changes                           |
|--------------------|---------|--------------------------------------------|
| CLI (`acp primer`) | Medium  | New selection algorithm, v2 schema support |
| CLI (`acp sync`)   | Low     | Pass-through to primer                     |
| MCP Server         | Low     | Update `acp_primer` tool                   |
| VS Code Extension  | None    | Consumes primer output unchanged           |

## Rollout Plan

1. **Phase 1** (v0.4.0): Implement v2 schema and algorithm behind `--schema-version 2` flag
2. **Phase 2** (v0.4.x): Community feedback, iterate on command templates
3. **Phase 3** (v0.5.0): Default to v2, deprecate v1
4. **Phase 4** (v1.0.0): Remove v1 support

## Open Questions

1. **Should daemon API be a separate interface type?** Currently grouped with CLI/MCP, but may have different documentation needs.

2. **How should custom project commands be added?** The v1 system allowed arbitrary sections. Should v2 support custom command definitions?

3. **Should tier selection be configurable?** Currently hardcoded budget thresholds. Should users be able to override?

4. **MCP parameter documentation**: MCP tools have typed parameters. Should the schema support richer parameter documentation than templates allow?

## Resolved Questions

1. **Q**: Should we keep any teaching sections?
   **A**: No. RFC-001's self-documenting directives make them redundant. The AI follows directive text directly.

2. **Q**: Should we keep multi-dimensional scoring?
   **A**: No. It was designed to optimize teaching content selection. Simple priority ordering suffices for interface documentation.

3. **Q**: Should critical commands always be included?
   **A**: Yes, at minimum tier. The `constraints` command is safety-critical and must be present.

## References

- [RFC-0001: Self-Documenting Annotations](./accepted/rfc-0001-self-documenting-annotations.md) - Foundation for this RFC
- [Chapter 11: Tool Integration](../spec/chapters/11-tool-integration.md) - Current primer system
- [Chapter 14: Bootstrap & AI Integration](../spec/chapters/14-bootstrap.md) - Bootstrap prompt guidance
- [primer.defaults.json](../primers/primer.defaults.json) - Current v1 section library

---

## Appendix

### A. Complete Command Reference (v1 Defaults)

#### CLI Commands

| Command        | Priority   | Critical   | Minimal Tokens  | Standard Tokens   | Full Tokens   |
|----------------|------------|------------|-----------------|-------------------|---------------|
| `constraints`  | 1          | Yes        | 8               | 35                | 80            |
| `query file`   | 2          | No         | 7               | 30                | 70            |
| `query symbol` | 3          | No         | 7               | 28                | 65            |
| `query domain` | 4          | No         | 7               | 25                | 55            |
| `map`          | 5          | No         | 6               | 25                | 60            |
| `knowledge`    | 6          | No         | 7               | 25                | 50            |
| `attempt`      | 7          | No         | 8               | 30                | 55            |
| `primer`       | 8          | No         | 8               | 20                | 40            |

#### MCP Tools

| Tool              | Priority   | Critical  | Minimal Tokens   | Standard Tokens   | Full Tokens   |
|-------------------|------------|-----------|------------------|-------------------|---------------|
| `acp_constraints` | 1          | Yes       | 10               | 30                | 60            |
| `acp_query`       | 2          | No        | 12               | 35                | 70            |
| `acp_knowledge`   | 3          | No        | 8                | 22                | 45            |
| `acp_map`         | 4          | No        | 8                | 20                | 45            |
| `acp_primer`      | 5          | No        | 10               | 18                | 35            |

### B. Token Budget Analysis

| Budget  | Bootstrap  | Interface Tier  | Commands Included   | Project Warnings   |
|---------|------------|-----------------|---------------------|--------------------|
| 40      | ✓          | —               | None (error)        | No                 |
| 60      | ✓          | Minimal         | ~4 commands         | No                 |
| 100     | ✓          | Standard        | ~3 commands         | No                 |
| 200     | ✓          | Standard        | ~6 commands         | No                 |
| 350     | ✓          | Full            | ~5 commands         | No                 |
| 500     | ✓          | Full            | All commands        | Yes                |
| 1000    | ✓          | Full            | All + patterns      | Yes                |

### C. Migration Guide

**From v1 custom sections:**

```json
// v1: Custom teaching section
{
  "id": "my-architecture",
  "category": "structure",
  "value": { "safety": 30, "efficiency": 60, "accuracy": 80, "base": 50 },
  "formats": {
    "markdown": { "template": "This is a microservices architecture." }
  }
}

// v2: Use project.customRules
{
  "project": {
    "customRules": "Architecture: microservices with gRPC. Check domain boundaries before cross-service calls."
  }
}
```

**From v1 weight presets:**

```json
// v1: Weight-based selection
{
  "selectionStrategy": {
    "weights": { "safety": 2.5, "efficiency": 0.8 }
  }
}

// v2: Not needed - safety-critical commands are marked `critical: true`
// For custom priority, override command priority in project primer
```

---

## Changelog

| Date       | Change        |
|------------|---------------|
| 2025-12-22 | Initial draft |

---

<!--
## RFC Process Checklist (for maintainers)

- [ ] RFC number assigned
- [ ] Added to proposed/
- [ ] Discussion link added
- [ ] Initial review complete
- [ ] Community feedback period (2+ weeks)
- [ ] FCP announced
- [ ] FCP complete (10 days)
- [ ] Decision made
- [ ] Moved to accepted/ or rejected/
-->
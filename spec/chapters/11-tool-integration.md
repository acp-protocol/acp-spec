# Tool Integration

**ACP Version**: 1.0.0
**Document Version**: 1.0.0
**Last Updated**: 2024-12-18
**Status**: Draft

---

## Table of Contents

1. [Overview](#1-overview)
2. [Tool Registry](#2-tool-registry)
3. [Primer System](#3-primer-system)
4. [Sync Command](#4-sync-command)
5. [Primer Command](#5-primer-command)
6. [Configuration](#6-configuration)
7. [Tool Adapters](#7-tool-adapters)
8. [File Ownership and Merging](#8-file-ownership-and-merging)
9. [Knowledge Store](#9-knowledge-store)
10. [Documentation References (RFC-0002)](#10-documentation-references-rfc-0002)
11. [Conformance](#11-conformance)
12. [Examples](#12-examples)

---

## 1. Overview

This chapter specifies how ACP context is automatically distributed to AI development tools without requiring manual user intervention. The goal is **zero-friction adoption**: after running `acp init`, context flows transparently to all supported tools.

### 1.1 Design Principles

1. **Transparency**: Users should not need to copy prompts, configure MCP servers, or remember tool-specific workflows
2. **Universality**: Work with any tool that has an auto-read mechanism, regardless of LLM provider
3. **Freshness**: Context stays synchronized as the codebase evolves
4. **Efficiency**: Respect token budgets through intelligent section selection
5. **Safety-First**: Critical constraint information always included

### 1.2 Architecture

```
┌─────────────────────────────────────────────────────────────────────────┐
│                          Primer System                                  │
├─────────────────────────────────────────────────────────────────────────┤
│                                                                         │
│  primer.schema.json          Defines WHAT a primer can contain          │
│         │                    • Section structure                        │
│         │                    • Value dimensions (safety/efficiency/etc) │
│         │                    • Capabilities registry                    │
│         │                    • Selection algorithms                     │
│         ▼                                                               │
│  primer.defaults.json        The DEFAULT section library                │
│         │                    • ~35 fine-grained sections                │
│         │                    • Multi-dimensional values                 │
│         │                    • Dynamic modifiers                        │
│         │                    • Format templates (markdown/compact/json) │
│         ▼                                                               │
│  acp primer CLI              ASSEMBLES primers on demand                │
│         │                    • --budget for token limits                │
│         │                    • --weights for dimension tuning           │
│         │                    • --preview/--compare for analysis         │
│         │                    • --tool for capability auto-detection     │
│         ▼                                                               │
│  acp sync                    DISTRIBUTES to tool configs                │
│         │                    • Calls primer internally per tool         │
│         │                    • Merges with existing files               │
│         │                    • Auto-runs on init/index                  │
│         ▼                                                               │
│  .cursorrules, CLAUDE.md,    Tool-specific output files                 │
│  copilot-instructions.md,    • Auto-read by each tool                   │
│  AGENTS.md, etc.             • User never copies/pastes                 │
│                                                                         │
└─────────────────────────────────────────────────────────────────────────┘
```

---

## 2. Tool Registry

### 2.1 Supported Tools

The following tools have defined adapters:

| Tool           | Auto-Read File                    | Capabilities                      | Default Budget  |
|----------------|-----------------------------------|-----------------------------------|-----------------|
| Cursor         | `.cursorrules`                    | shell, file-read, file-write      | 2000            |
| Claude Code    | `CLAUDE.md`                       | shell, mcp, file-read, file-write | 4000            |
| GitHub Copilot | `.github/copilot-instructions.md` | file-read                         | 1500            |
| Continue.dev   | `.continue/config.json`           | shell, mcp, file-read             | 2000            |
| Windsurf       | `.windsurfrules`                  | shell, file-read, file-write      | 2000            |
| Cline          | `.clinerules`                     | shell, mcp, file-read, file-write | 2000            |
| Aider          | `.aider.conf.yml`                 | shell                             | 1000            |
| Generic        | `AGENTS.md`                       | (none assumed)                    | unlimited       |

### 2.2 Capabilities

Capabilities determine which primer sections are applicable to each tool:

| Capability   | Description                    | Enables Sections           |
|--------------|--------------------------------|----------------------------|
| `shell`      | Can execute terminal commands  | CLI commands, jq queries   |
| `mcp`        | Model Context Protocol support | MCP tool references        |
| `file-read`  | Can read filesystem            | Direct cache access        |
| `file-write` | Can modify files               | File modification guidance |
| `http`       | Can make HTTP requests         | API references             |

### 2.3 Tool Detection

Tools are detected in the following order:

1. **Explicit Configuration**: Tools listed in `.acp/acp.sync.json` under `tools`
2. **Marker Files**: Presence of tool-specific configuration files
3. **Environment Detection**: CLI tools in PATH, running processes
4. **Default Set**: If no tools detected, generate for common tools

---

## 3. Primer System

The primer system replaces static content tiers with intelligent, value-optimized section assembly.

### 3.1 Section Structure

Each section in the primer library has:

```json
{
  "id": "lock-frozen",
  "name": "Lock Level: Frozen",
  "category": "constraints",
  "priority": 11,
  "tokens": 18,
  "required": false,
  "requiredIf": "constraints.frozenCount > 0",
  "capabilities": ["shell"],
  "dependsOn": ["constraint-concept"],
  "value": {
    "safety": 100,
    "efficiency": 30,
    "accuracy": 40,
    "base": 60,
    "modifiers": [
      {
        "condition": "constraints.frozenCount > 0",
        "add": 40,
        "reason": "Project has frozen files"
      }
    ]
  },
  "formats": {
    "markdown": { "template": "**`frozen`**: Never modify under any circumstances." },
    "compact": { "template": "frozen=NEVER modify" },
    "json": null
  }
}
```

### 3.2 Multi-Dimensional Value Scoring

Each section is scored across four dimensions:

| Dimension      | Description                 | Weight Default   |
|----------------|-----------------------------|------------------|
| **Safety**     | Prevents harmful AI actions | 1.5              |
| **Efficiency** | Saves future tokens/queries | 1.0              |
| **Accuracy**   | Improves response quality   | 1.0              |
| **Base**       | Baseline importance         | 1.0              |

Final score calculation:
```
score = (safety × w_safety) + (efficiency × w_efficiency) + 
        (accuracy × w_accuracy) + (base × w_base)
```

### 3.3 Dynamic Value Modifiers

Section values adjust based on project state:

```json
{
  "modifiers": [
    {
      "condition": "hacks.count == 0",
      "multiply": 0,
      "reason": "No hacks exist, section worthless"
    },
    {
      "condition": "hacks.expiredCount > 0",
      "add": 50,
      "dimension": "safety",
      "reason": "Expired hacks need attention"
    }
  ]
}
```

### 3.4 Selection Algorithm

```
function selectSections(budget, projectState, capabilities, weights):
    selected = []
    remaining = budget
    
    // Phase 1: Required sections (non-negotiable)
    for section in sections.filter(s => isRequired(s, projectState)):
        if section.tokens > remaining:
            ERROR("Budget too small for required sections")
        selected.push(section)
        remaining -= section.tokens
    
    // Phase 2: Conditionally required
    for section in sections.filter(s => isRequiredIf(s, projectState)):
        if meetsCondition(s.requiredIf, projectState):
            selected.push(section)
            remaining -= section.tokens
    
    // Phase 3: Safety-critical (safety >= 80)
    candidates = sections.filter(s => 
        !selected.includes(s) && 
        meetsCapabilities(s, capabilities) &&
        s.value.safety >= 80
    )
    for section in candidates.sortBy(s => calculateScore(s, weights)):
        if section.tokens <= remaining:
            selected.push(section)
            remaining -= section.tokens
    
    // Phase 4: Value-optimized (fill remaining budget)
    candidates = sections.filter(s => 
        !selected.includes(s) && 
        meetsCapabilities(s, capabilities) &&
        meetsDependencies(s, selected)
    )
    for section in candidates.sortBy(s => calculateScore(s, weights) / s.tokens):
        if section.tokens <= remaining:
            selected.push(section)
            remaining -= section.tokens
    
    return selected
```

### 3.5 Section Categories

| Category    | Icon   | Description                   | Budget Priority        |
|-------------|--------|-------------------------------|------------------------|
| bootstrap   | 🚀     | Core ACP awareness            | Highest (required)     |
| constraints | 🔒     | File protection rules         | High (safety-critical) |
| interface   | ⚡      | CLI/MCP commands              | Medium                 |
| annotations | 📝     | Code annotation syntax        | Medium                 |
| structure   | 🏗️    | Domains, layers, architecture | Medium                 |
| variables   | 🔤     | Token-efficient references    | Medium                 |
| debug       | 🐛     | Debug sessions, hacks         | Contextual             |
| knowledge   | 📚     | Self-expansion capabilities   | Low                    |

### 3.6 Output Formats

Each section defines templates for multiple formats:

| Format     | Use Case                             | Example                  |
|------------|--------------------------------------|--------------------------|
| `markdown` | Most tools (.cursorrules, CLAUDE.md) | Full formatting          |
| `compact`  | Token-constrained tools              | Single line, abbreviated |
| `json`     | API responses, MCP tools             | Structured data          |
| `text`     | Plain text config files              | No formatting            |

---

## 4. Sync Command

### 4.1 Usage

Synchronizes ACP context to all detected tool configuration files.

```bash
acp sync [options]
```

**Options**:

| Option             | Description                  | Default     |
|--------------------|------------------------------|-------------|
| `--tools <list>`   | Comma-separated tool names   | Auto-detect |
| `--exclude <list>` | Tools to skip                | none        |
| `--force`          | Overwrite without merge      | false       |
| `--dry-run`        | Show what would be generated | false       |
| `--watch`          | Continuous sync mode         | false       |
| `--quiet`          | Suppress output              | false       |

### 4.2 Integration with Other Commands

**`acp init`**: Automatically runs `acp sync` as final step
```bash
acp init
# Creates .acp.config.json
# Runs acp index
# Runs acp sync  ← automatic
```

**`acp index`**: Accepts `--sync` flag
```bash
acp index --sync
# Equivalent to: acp index && acp sync
```

**`acp watch`**: Auto-syncs on cache changes
```bash
acp watch --sync
# Re-syncs when cache updates
```

---

## 5. Primer Command

### 5.1 Usage

Generates context primers with configurable budget and weights.

```bash
acp primer [options]
```

**Key Options**:

| Option          | Description              | Example                              |
|-----------------|--------------------------|--------------------------------------|
| `--budget <N>`  | Token budget             | `--budget 1000`                      |
| `--weights <W>` | Dimension weights        | `--weights=safety:200,efficiency:50` |
| `--preset <P>`  | Named preset             | `--preset safe`                      |
| `--tool <T>`    | Auto-detect capabilities | `--tool cursor`                      |
| `--format <F>`  | Output format            | `--format compact`                   |
| `--preview`     | Show selection matrix    |                                      |
| `--compare <B>` | Compare budgets          | `--compare 250,500,1000`             |

### 5.2 Weight Presets

| Preset      | Safety   | Efficiency   | Accuracy   | Base   | Use Case           |
|-------------|----------|--------------|------------|--------|--------------------|
| `safe`      | 2.5      | 0.8          | 1.0        | 0.8    | Code modification  |
| `efficient` | 1.2      | 2.0          | 0.9        | 0.8    | Long conversations |
| `accurate`  | 1.2      | 0.9          | 2.0        | 0.8    | Complex analysis   |
| `balanced`  | 1.0      | 1.0          | 1.0        | 1.0    | General use        |

### 5.3 Preview Mode

```bash
$ acp primer --budget 500 --preview

┌─────────────────────────────────────────────────────────────────────────┐
│ Primer Budget: 500 tokens                                               │
├─────────────────────────────────────────────────────────────────────────┤
│  Category      Section                  Tokens   Score   V/T    Status  │
│  ─────────────────────────────────────────────────────────────────────  │
│  🚀 bootstrap  acp-exists                  15     245  16.33   ✓ req    │
│  🚀 bootstrap  acp-location                25     222   8.88   ✓ req    │
│  🔒 constraints constraint-concept         35     240   6.86   ✓ req    │
│  🔒 constraints lock-frozen                18     210  11.67   ✓ cond   │
│  🔒 constraints protected-files-list      120     189   1.58   ✓        │
│  ⚡ interface   cli-overview               35     142   4.06   ✓         │
│  ─────────────────────────────────────────────────────────────────────  │
│  TOTAL                                    355             USED          │
│  Remaining: 145 tokens                                                  │
└─────────────────────────────────────────────────────────────────────────┘
```

---

## 6. Configuration

### 6.1 Sync Configuration

In `.acp/acp.sync.json` or as `sync` section in `.acp.config.json`:

```json
{
  "$schema": "https://acp-protocol.dev/schemas/v1/sync.schema.json",
  "version": "1.0.0",
  "enabled": true,
  "auto": true,
  "tools": ["cursor", "claude-code", "generic"],
  "primer": {
    "defaultBudget": 500,
    "preset": "safe"
  },
  "custom": {
    "cursor": {
      "primer": {
        "budget": 2000,
        "capabilities": ["shell", "file-read", "file-write"]
      }
    },
    "claude-code": {
      "primer": {
        "budget": 4000,
        "capabilities": ["shell", "mcp", "file-read", "file-write"],
        "weights": { "safety": 1.5, "efficiency": 1.2 }
      }
    }
  }
}
```

### 6.2 Per-Tool Primer Configuration

| Field               | Type    | Description              |
|---------------------|---------|--------------------------|
| `budget`            | integer | Token budget (80-100000) |
| `format`            | string  | Output format            |
| `weights`           | object  | Dimension weights        |
| `preset`            | string  | Named weight preset      |
| `capabilities`      | array   | Tool capabilities        |
| `includeSections`   | array   | Section whitelist        |
| `excludeSections`   | array   | Section blacklist        |
| `includeCategories` | array   | Category whitelist       |
| `excludeCategories` | array   | Category blacklist       |
| `includeTags`       | array   | Tag filter               |
| `dynamicModifiers`  | boolean | Enable value modifiers   |

### 6.3 Project Primer Customization

Projects can extend the default primer library via `.acp/primer.json`:

```json
{
  "$schema": "https://acp-protocol.dev/schemas/v1/primer.schema.json",
  "version": "1.0.0",
  "sections": [
    {
      "id": "project-architecture",
      "category": "structure",
      "priority": 39,
      "tokens": 50,
      "value": { "safety": 30, "efficiency": 60, "accuracy": 80, "base": 50 },
      "formats": {
        "markdown": {
          "template": "This is a microservices architecture using gRPC."
        }
      }
    }
  ],
  "selectionStrategy": {
    "weights": { "safety": 2.0, "efficiency": 1.0, "accuracy": 1.5, "base": 0.8 }
  }
}
```

---

## 7. Tool Adapters

### 7.1 Adapter Interface

Each tool adapter implements:

```typescript
interface ToolAdapter {
    readonly name: string;
    readonly outputPath: string;
    readonly capabilities: Capability[];
    readonly defaultBudget: number;
    readonly format: OutputFormat;
    
    detect(projectRoot: string): Promise<boolean>;
    generate(primer: AssembledPrimer): Promise<string>;
    validate(content: string): Result<void, ValidationError>;
    merge(existing: string, generated: string): string;
}
```

### 7.2 Built-in Adapters

**Cursor Adapter**
- Output: `.cursorrules`
- Capabilities: shell, file-read, file-write
- Merge: Section-based with `<!-- ACP -->` markers

**Claude Code Adapter**
- Output: `CLAUDE.md`
- Capabilities: shell, mcp, file-read, file-write
- Special: Can reference `.acp/acp.cache.json` directly

**GitHub Copilot Adapter**
- Output: `.github/copilot-instructions.md`
- Capabilities: file-read
- Creates `.github/` directory if needed

**Continue.dev Adapter**
- Output: `.continue/config.json`
- Capabilities: shell, mcp, file-read
- Merge: JSON deep merge into `systemMessage`

**Generic Adapter**
- Output: `AGENTS.md`
- Capabilities: none (universal fallback)
- Always generated, full content

### 7.3 Custom Adapters

Define custom adapters in sync configuration:

```json
{
  "customAdapters": [
    {
      "name": "my-ide",
      "displayName": "My Custom IDE",
      "outputPath": ".my-ide/context.md",
      "format": "markdown",
      "detect": { "files": [".my-ide/config.json"] },
      "mergeStrategy": "section",
      "sectionMarker": "<!-- ACP-CONTEXT -->",
      "primer": {
        "budget": 2000,
        "capabilities": ["shell", "file-read"]
      }
    }
  ]
}
```

---

## 8. File Ownership and Merging

### 8.1 Section Markers

ACP-generated content is clearly delineated:

```markdown
<!-- BEGIN ACP GENERATED CONTENT - DO NOT EDIT -->
[generated primer content]
<!-- END ACP GENERATED CONTENT -->
```

### 8.2 Merge Strategies

| Strategy   | Description                 | When Used                   |
|------------|-----------------------------|-----------------------------|
| `replace`  | Replace entire file         | New files, `--force`        |
| `section`  | Replace marked section only | Existing files with markers |
| `append`   | Add to end of file          | No markers found            |
| `merge`    | Deep merge (JSON/YAML)      | Config files                |

### 8.3 Conflict Resolution

- **Within ACP section**: Regenerated content wins (user warned)
- **Outside ACP section**: User content preserved
- **Structural conflicts**: Fail with error, require `--force`

---

## 9. Knowledge Store

### 9.1 Overview

ACP can include a queryable knowledge base for self-expansion:

```bash
# Install knowledge store
acp knowledge install

# Query ACP documentation
acp knowledge "how do I check constraints"
```

### 9.2 Distribution

- **Core index** (~500KB): Ships with CLI, keyword-based lookup
- **Semantic DB** (~15MB): Optional, enables fuzzy queries

```bash
# Core only
npm install -g @acp-protocol/cli

# With semantic search
npm install -g @acp-protocol/cli @acp-protocol/knowledge
# OR
acp knowledge install
```

### 9.3 Bootstrap Integration

The primer includes self-expansion sections that teach the AI to query for more context:

```json
{
  "id": "acp-self-expand",
  "template": "For more context: `acp primer --budget N` or `acp knowledge \"question\"`"
}
```

---

## 10. Documentation References (RFC-0002)

This section specifies how AI tools should handle documentation references and style guides defined via `@acp:ref` and `@acp:style` annotations.

### 10.1 When to Consult References

AI tools SHOULD consult documentation references in these scenarios:

| Scenario | Action | Priority |
|----------|--------|----------|
| Modifying code with `@acp:ref-fetch true` | SHOULD fetch documentation proactively | High |
| User asks about code with `@acp:ref` | SHOULD mention reference exists | Medium |
| Adding new code in file with `@acp:ref` | MAY consult reference for patterns | Medium |
| Debugging code with `@acp:ref` | MAY consult for expected behavior | Low |
| Simple formatting changes | SHOULD NOT fetch (wasteful) | N/A |

### 10.2 When NOT to Fetch

AI tools SHOULD NOT fetch documentation when:

- The change is trivial (typos, formatting)
- The reference is marked `fetchable: false` in config
- The `@acp:ref-fetch` is explicitly `false`
- Network access is restricted or unavailable
- Documentation would exceed context limits

### 10.3 Reference Resolution

AI tools resolve `@acp:ref` annotations in this order:

1. **Source ID Resolution**: If reference uses a source ID (e.g., `react:hooks`):
   - Look up `id` in `documentation.approvedSources` in config
   - Construct URL from source `url` + section path (if `@acp:ref-section` present)
   - Apply version from `@acp:ref-version` if present

2. **Direct URL**: If reference is a full URL:
   - If `validation.requireApprovedSources` is `true`, reject and warn
   - Otherwise, use URL directly

3. **Fallback**: If reference cannot be resolved:
   - Log to `unresolvedRefs` in cache
   - Warn user but continue processing

### 10.4 Style Application

AI tools SHOULD apply style guides as follows:

| Style Setting | AI Behavior |
|---------------|-------------|
| `@acp:style <guide>` | Follow specified style for new/modified code |
| `@acp:style-extends <parent>` | Apply parent rules first, then override |
| `@acp:style-rules <rules>` | Apply specific rules (comma-separated) |
| Config `documentation.styleGuides` | Use for custom style definitions |
| Config `documentation.defaults.style` | Apply as fallback for files without annotation |
| No style specified | Follow surrounding code patterns |

**Precedence** (highest to lowest):
1. Symbol-level `@acp:style`
2. File-level `@acp:style`
3. Config `documentation.defaults.style`
4. Surrounding code patterns

### 10.5 Caching Recommendations

AI tools SHOULD:
- Cache fetched documentation during a session
- Use `documentation.sources` in cache for quick reference lookup
- Respect `lastVerified` timestamps for staleness
- Limit fetch frequency to avoid rate limiting

### 10.6 Error Handling

| Error | AI Behavior |
|-------|-------------|
| Source ID not found | Warn, continue without fetching |
| Network timeout | Continue without documentation, note in response |
| Invalid URL | Log to `unresolvedRefs`, continue |
| Unknown style guide | Warn if `validation.warnUnknownStyle` is true |

---

## 11. Conformance

### 11.1 Conformance Levels

| Level        | Requirements                                                |
|--------------|-------------------------------------------------------------|
| Sync Level 1 | Basic sync to at least one tool                             |
| Sync Level 2 | All built-in adapters, merge strategies, primer integration |
| Sync Level 3 | Custom adapters, templates, hooks, knowledge store          |

### 11.2 Required Behaviors

Implementations MUST:
1. Preserve user content outside ACP sections
2. Respect minimum budget for required sections
3. Support `--dry-run` for safe preview
4. Generate valid output for each tool's expected format
5. Include all `required: true` sections

Implementations SHOULD:
1. Detect tools automatically when not configured
2. Support dynamic value modifiers
3. Integrate with `acp init` and `acp index`

---

## 12. Examples

### 12.1 Basic Workflow

```bash
# Initialize project (includes sync)
acp init

# Files created:
# - .acp.config.json
# - .acp/acp.cache.json
# - .acp/acp.vars.json
# - .cursorrules (if Cursor detected)
# - CLAUDE.md (if Claude Code detected)
# - AGENTS.md (always)

# User opens any AI tool - context is already there
```

### 12.2 Custom Weights

```bash
# Safety-focused for security-critical project
acp sync --primer-preset safe

# Or configure in .acp/acp.sync.json
{
  "primer": {
    "weights": { "safety": 3.0, "efficiency": 0.5 }
  }
}
```

### 12.3 Budget Analysis

```bash
# Compare what different budgets include
acp primer --compare 250,500,1000,2000

# Preview specific budget
acp primer --budget 500 --preview --tool cursor
```

---

## Appendix A: Quick Reference

| Command | Description |
|---------|-------------|
| `acp sync` | Sync to all detected tools |
| `acp sync --tools cursor` | Sync to specific tool |
| `acp sync --dry-run` | Preview without writing |
| `acp primer --budget 500` | Generate primer with budget |
| `acp primer --preview` | Show selection matrix |
| `acp primer --compare 250,500,1000` | Compare budgets |

---

## Appendix B: Related Documents

- [Primer Schema](../schemas/v1/primer.schema.json)
- [Sync Schema](../schemas/v1/sync.schema.json)
- [Default Primer Library](../primers/primer.defaults.json)
- [CLI Primer Reference](../docs/cli-primer-reference.md)
- [Cache Format](03-cache-format.md) - How annotations are stored
- [Config Format](04-config-format.md) - Configuration options

---

*End of Tool Integration Specification*
---
description: Generate comprehensive documentation of the ACP Daemon architecture, including all components, data flows, and capabilities. Provides a detailed informational overview.
handoffs:
  - label: Evaluate Daemon Value
    agent: acp.daemon-evaluate
    prompt: Evaluate this daemon architecture against project goals
    send: true
  - label: Implementation Status
    agent: acp.daemon-status
    prompt: Check implementation status of daemon components
  - label: Architecture Comparison
    agent: acp.daemon-compare
    prompt: Compare daemon approach with alternatives
---

## User Input

```text
$ARGUMENTS
```

You **MUST** consider the user input before proceeding (if not empty). The user input may specify:
- `--component <n>` to focus on specific component (watch, proxy, sync, mcp)
- `--format <f>` output format (detailed, summary, diagram)
- `--include-code` to include relevant code snippets
- `--future` to include planned/future features

## Purpose

This command generates comprehensive documentation of the ACP Daemon architecture. The "daemon" is a conceptual architecture that encompasses several related components that work together to provide real-time codebase awareness to AI tools.

## Daemon Architecture Overview

The ACP Daemon is not a single process, but rather an **architectural pattern** consisting of:

```
┌──────────────────────────────────────────────────────────────────────────┐
│                         ACP Daemon Architecture                          │
├──────────────────────────────────────────────────────────────────────────┤
│                                                                          │
│  ┌─────────────────────────────────────────────────────────────────────┐ │
│  │                     Background Services                              │ │
│  ├─────────────────────────────────────────────────────────────────────┤ │
│  │                                                                      │ │
│  │  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐               │ │
│  │  │ File Watcher │  │ HTTP Proxy   │  │ MCP Server   │               │ │
│  │  │ (acp watch)  │  │ (acp proxy)  │  │ (acp-mcp)    │               │ │
│  │  │              │  │              │  │              │               │ │
│  │  │ • notify     │  │ • bootstrap  │  │ • tools      │               │ │
│  │  │ • debounce   │  │ • injection  │  │ • resources  │               │ │
│  │  │ • filter     │  │ • routing    │  │ • queries    │               │ │
│  │  └──────┬───────┘  └──────┬───────┘  └──────┬───────┘               │ │
│  │         │                 │                 │                        │ │
│  │         └────────────────┼────────────────┘                         │ │
│  │                          │                                          │ │
│  │                          ▼                                          │ │
│  │              ┌───────────────────────┐                              │ │
│  │              │   Shared Cache        │                              │ │
│  │              │   .acp.cache.json     │                              │ │
│  │              │   .acp.vars.json      │                              │ │
│  │              └───────────────────────┘                              │ │
│  │                          │                                          │ │
│  └──────────────────────────┼──────────────────────────────────────────┘ │
│                             │                                            │
│  ┌──────────────────────────┼──────────────────────────────────────────┐ │
│  │                     Client Tools                                     │ │
│  ├──────────────────────────┼──────────────────────────────────────────┤ │
│  │                          ▼                                          │ │
│  │  ┌──────────┐  ┌──────────┐  ┌──────────┐  ┌──────────┐            │ │
│  │  │  Cursor  │  │  Claude  │  │  Copilot │  │   Zed    │   ...      │ │
│  │  │          │  │  Code    │  │          │  │          │            │ │
│  │  └──────────┘  └──────────┘  └──────────┘  └──────────┘            │ │
│  │                                                                      │ │
│  └──────────────────────────────────────────────────────────────────────┘ │
│                                                                          │
└──────────────────────────────────────────────────────────────────────────┘
```

## Component Documentation

### 1. File Watcher (`acp watch`)

**Location**: `cli/src/watch.rs`

**Purpose**: Monitor filesystem changes and trigger incremental cache updates.

**Current Implementation**:
```rust
// Uses notify crate for cross-platform file watching
pub struct FileWatcher {
    config: AcpConfig,
}

impl FileWatcher {
    pub fn watch<P: AsRef<Path>>(&self, root: P) -> Result<()> {
        // Creates mpsc channel for events
        // Watches recursively from root
        // TODO: Incremental update based on event.kind
    }
}
```

**Capabilities**:
| Capability | Status | Description |
|------------|--------|-------------|
| Recursive watching | ✅ Implemented | Watches all subdirectories |
| Event detection | ✅ Implemented | Detects create/modify/delete |
| Incremental updates | ⏳ TODO | Only re-index changed files |
| Debouncing | ⏳ TODO | Coalesce rapid changes |
| Pattern filtering | ⏳ TODO | Respect include/exclude |
| Cache invalidation | ⏳ TODO | Smart cache partial updates |

**Data Flow**:
```
File System Events
       │
       ▼
┌─────────────────┐
│   notify crate  │
│   (inotify,     │
│   FSEvents,     │
│   ReadDir)      │
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│  Event Handler  │
│  (debounce,     │
│   filter)       │
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│  Incremental    │
│  Indexer        │
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│  Cache Update   │
│  (partial)      │
└─────────────────┘
```

---

### 2. HTTP Proxy (`acp proxy`)

**Location**: `cli/src/commands/proxy.rs` (planned)

**Purpose**: Intercept LLM API requests and inject ACP bootstrap context.

**Planned Implementation**:
```rust
pub struct AcpProxy {
    port: u16,
    upstream: HashMap<String, Url>,  // /anthropic -> api.anthropic.com
    bootstrap: String,
}

impl AcpProxy {
    pub async fn start(&self) -> Result<()> {
        // Bind to localhost:port
        // Route requests to appropriate upstream
        // Inject bootstrap into system prompt
        // Stream responses back
    }
}
```

**Capabilities**:
| Capability | Status | Description |
|------------|--------|-------------|
| HTTP server | ⏳ Planned | Bind to localhost |
| Provider routing | ⏳ Planned | Route /anthropic, /openai, /google |
| Bootstrap injection | ⏳ Planned | Prepend to system prompt |
| Streaming passthrough | ⏳ Planned | Handle SSE responses |
| Request logging | ⏳ Planned | Optional verbose mode |
| Custom templates | ⏳ Planned | Per-tool bootstrap |

**Data Flow**:
```
AI Tool (Cursor, etc.)
       │
       │  POST /anthropic/v1/messages
       ▼
┌─────────────────┐
│   ACP Proxy     │
│   localhost:    │
│   8080          │
└────────┬────────┘
         │
         │  1. Parse request
         │  2. Inject bootstrap
         │  3. Forward to upstream
         ▼
┌─────────────────┐
│   Upstream API  │
│   (Anthropic,   │
│    OpenAI,etc)  │
└────────┬────────┘
         │
         │  Stream response
         ▼
┌─────────────────┐
│   ACP Proxy     │
│   (passthrough) │
└────────┬────────┘
         │
         ▼
    AI Tool
```

**Bootstrap Injection Example**:
```json
// Original request
{
  "system": "You are a helpful assistant.",
  "messages": [...]
}

// After injection
{
  "system": "You are a helpful assistant.\n\n[ACP] Check `acp constraints <file>` before modifying. Protected: src/auth/session.ts (frozen). More: `acp primer --budget 500`",
  "messages": [...]
}
```

---

### 3. Sync Service (`acp sync`)

**Location**: `cli/src/commands/sync.rs` (skeleton)

**Purpose**: Distribute ACP context to tool-specific configuration files.

**Planned Implementation**:
```rust
pub struct SyncService {
    tools: Vec<Box<dyn ToolAdapter>>,
    primer_config: PrimerConfig,
}

pub trait ToolAdapter {
    fn name(&self) -> &str;
    fn detect(&self) -> bool;
    fn output_path(&self) -> PathBuf;
    fn format(&self) -> OutputFormat;
    fn write(&self, content: &str) -> Result<()>;
}
```

**Supported Tools**:
| Tool | Config File | Format | Detection |
|------|-------------|--------|-----------|
| Cursor | `.cursorrules` | Markdown | IDE markers |
| Claude Code | `CLAUDE.md` | Markdown | Project presence |
| GitHub Copilot | `.github/copilot-instructions.md` | Markdown | .github/ dir |
| Windsurf | `.windsurfrules` | Markdown | IDE markers |
| Zed | `AGENTS.md` | Markdown | Always |
| Continue.dev | `.continue/config.json` | JSON | .continue/ dir |

**Data Flow**:
```
┌─────────────────┐
│  acp sync       │
└────────┬────────┘
         │
         ├──────────────────┬──────────────────┐
         ▼                  ▼                  ▼
┌─────────────────┐ ┌─────────────────┐ ┌─────────────────┐
│ Detect Tools    │ │ Load Cache      │ │ Load Primer     │
│ (auto-detect)   │ │ (.acp.cache)    │ │ Config          │
└────────┬────────┘ └────────┬────────┘ └────────┬────────┘
         │                   │                   │
         └───────────────────┴───────────────────┘
                             │
                             ▼
                  ┌─────────────────┐
                  │ Generate Primer │
                  │ (per tool)      │
                  └────────┬────────┘
                           │
         ┌─────────────────┼─────────────────┐
         ▼                 ▼                 ▼
┌─────────────────┐ ┌─────────────────┐ ┌─────────────────┐
│ .cursorrules    │ │ CLAUDE.md       │ │ AGENTS.md       │
└─────────────────┘ └─────────────────┘ └─────────────────┘
```

---

### 4. MCP Server (`acp-mcp-server`)

**Location**: `docs/integrations/` (spec), separate package (implementation)

**Purpose**: Provide Model Context Protocol tools for direct AI assistant access.

**Tools Provided**:
| Tool | Purpose | Parameters |
|------|---------|------------|
| `acp_query` | Query codebase index | type, name |
| `acp_constraints` | Check file constraints | file |
| `acp_expand` | Expand variable references | text, mode |
| `acp_debug` | Manage debug sessions | action, data |
| `acp_hack` | Track temporary code | action, data |

**Resources Provided**:
| Resource | URI | Description |
|----------|-----|-------------|
| Cache | `acp://cache` | Full codebase index |
| Vars | `acp://vars` | Variable definitions |
| Constraints | `acp://constraints` | Constraints summary |

**Data Flow**:
```
AI Assistant (Claude, etc.)
       │
       │  MCP Tool Call
       ▼
┌─────────────────┐
│  MCP Server     │
│  (acp-mcp)      │
└────────┬────────┘
         │
         │  Query/Action
         ▼
┌─────────────────┐
│  Cache/Vars     │
│  Files          │
└────────┬────────┘
         │
         │  Response
         ▼
    AI Assistant
```

---

## Unified Daemon Vision (Future)

The current components are separate processes. The **unified daemon** vision would consolidate them:

```
┌─────────────────────────────────────────────────────────────────────────┐
│                         ACP Daemon (acpd)                                │
│                         Single Background Process                        │
├─────────────────────────────────────────────────────────────────────────┤
│                                                                          │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐  ┌─────────────┐  │
│  │ File Watcher │  │ HTTP Proxy   │  │ MCP Server   │  │ IPC Server  │  │
│  │ Thread       │  │ Thread       │  │ Thread       │  │ Thread      │  │
│  └──────┬───────┘  └──────┬───────┘  └──────┬───────┘  └──────┬──────┘  │
│         │                 │                 │                 │         │
│         └─────────────────┴─────────────────┴─────────────────┘         │
│                                   │                                      │
│                                   ▼                                      │
│                    ┌─────────────────────────────┐                       │
│                    │     In-Memory Cache         │                       │
│                    │     • File index            │                       │
│                    │     • Symbol table          │                       │
│                    │     • Constraint index      │                       │
│                    │     • Variable definitions  │                       │
│                    └─────────────────────────────┘                       │
│                                   │                                      │
│                                   ▼                                      │
│                    ┌─────────────────────────────┐                       │
│                    │     Persistence Layer       │                       │
│                    │     • .acp.cache.json       │                       │
│                    │     • .acp.vars.json        │                       │
│                    └─────────────────────────────┘                       │
│                                                                          │
└─────────────────────────────────────────────────────────────────────────┘
```

**Benefits of Unified Daemon**:
1. **Single process** - One background service instead of multiple
2. **Shared memory** - Cache loaded once, shared across interfaces
3. **Atomic updates** - Changes propagate instantly to all clients
4. **Resource efficient** - Lower memory footprint
5. **Simplified management** - `acp start` / `acp stop`

**Planned Command Interface**:
```bash
# Start unified daemon
acp start [--port 8080] [--mcp] [--watch]

# Stop daemon
acp stop

# Check status
acp status

# Restart on config change
acp restart
```

---

## Configuration

### Daemon Configuration (`.acp.config.json`)

```json
{
  "daemon": {
    "autostart": true,
    "port": 8080,
    "watch": {
      "enabled": true,
      "debounce_ms": 500,
      "ignore_patterns": ["**/node_modules/**", "**/.git/**"]
    },
    "proxy": {
      "enabled": true,
      "inject_mode": "system",
      "providers": {
        "anthropic": "https://api.anthropic.com",
        "openai": "https://api.openai.com"
      }
    },
    "mcp": {
      "enabled": true,
      "tools": ["acp_query", "acp_constraints", "acp_expand", "acp_debug"]
    },
    "sync": {
      "auto_sync": true,
      "tools": ["cursor", "claude-code", "agents"]
    }
  }
}
```

---

## Output Format

When generating this documentation, format as:

```markdown
# ACP Daemon Architecture

## Executive Summary
[Brief overview of what the daemon does]

## Components
### 1. File Watcher
[Details...]

### 2. HTTP Proxy
[Details...]

### 3. Sync Service
[Details...]

### 4. MCP Server
[Details...]

## Data Flows
[Diagrams and descriptions]

## Configuration
[Config options]

## Implementation Status
[Current vs planned]
```

## Completion Criteria

### Documentation Complete When:
- [ ] All four components documented
- [ ] Data flows explained with diagrams
- [ ] Current implementation status noted
- [ ] Future unified daemon vision described
- [ ] Configuration options documented
- [ ] Integration points identified

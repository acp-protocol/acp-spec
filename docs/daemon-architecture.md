# ACP Daemon Architecture

> Comprehensive documentation of the ACP Daemon (`acpd`) architecture, implementation status, and evaluation against project goals.

## Executive Summary

The ACP Daemon (`acpd`) is a **unified background service** that provides real-time codebase intelligence to AI tools. Unlike the conceptual multi-process architecture initially envisioned, the actual implementation is a **single Rust binary** that combines:

- **HTTP REST API** - Query codebase index, symbols, domains, constraints
- **MCP Server** - Model Context Protocol tools for direct AI assistant integration
- **Primer Generation** - Intelligent context optimization within token budgets
- **Process Lifecycle** - Start/stop/status management with PID file tracking

The daemon loads ACP cache, config, and vars files once, keeping them in shared memory for fast concurrent access.

---

## Architecture Overview

```
┌──────────────────────────────────────────────────────────────────────────────────┐
│                            ACP Daemon (acpd)                                     │
│                            Single Rust Binary                                    │
├──────────────────────────────────────────────────────────────────────────────────┤
│                                                                                  │
│  ┌─────────────────────────────────────────────────────────────────────────────┐ │
│  │                          Entry Points                                       │ │
│  ├─────────────────────────────────────────────────────────────────────────────┤ │
│  │                                                                              │ │
│  │  ┌──────────────────────┐    ┌──────────────────────┐                       │ │
│  │  │   acpd run           │    │   acpd mcp           │                       │ │
│  │  │   (HTTP Server)      │    │   (Stdio MCP)        │                       │ │
│  │  │                      │    │                      │                       │ │
│  │  │   Port: 9222         │    │   For Claude         │                       │ │
│  │  │   REST API           │    │   Desktop            │                       │ │
│  │  └──────────┬───────────┘    └──────────┬───────────┘                       │ │
│  │             │                           │                                    │ │
│  └─────────────┼───────────────────────────┼────────────────────────────────────┘ │
│                │                           │                                      │
│                └─────────────┬─────────────┘                                      │
│                              │                                                    │
│                              ▼                                                    │
│              ┌───────────────────────────────────────┐                            │
│              │          AppState                     │                            │
│              │    (Shared Thread-Safe State)         │                            │
│              ├───────────────────────────────────────┤                            │
│              │  • RwLock<Config>                     │                            │
│              │  • RwLock<Cache>                      │                            │
│              │  • RwLock<Option<VarsFile>>           │                            │
│              │  • project_root: PathBuf              │                            │
│              └───────────────────────────────────────┘                            │
│                              │                                                    │
│        ┌─────────────────────┼─────────────────────┐                              │
│        ▼                     ▼                     ▼                              │
│  ┌───────────────┐  ┌────────────────────┐  ┌────────────────┐                   │
│  │   REST API    │  │    MCP Service     │  │    Primer      │                   │
│  │   (Axum)      │  │    (rmcp SDK)      │  │    Generator   │                   │
│  │               │  │                    │  │                │                   │
│  │  /health      │  │  acp_get_*         │  │  Value-based   │                   │
│  │  /symbols     │  │  acp_check_*       │  │  optimization  │                   │
│  │  /files       │  │  acp_generate_     │  │  Multi-dim     │                   │
│  │  /domains     │  │    primer          │  │  scoring       │                   │
│  │  /constraints │  │                    │  │                │                   │
│  │  /callers     │  │                    │  │                │                   │
│  │  /callees     │  │                    │  │                │                   │
│  └───────────────┘  └────────────────────┘  └────────────────┘                   │
│                                                                                  │
└──────────────────────────────────────────────────────────────────────────────────┘
                                      │
                                      ▼
                   ┌──────────────────────────────────────┐
                   │         Persistence Layer            │
                   │   .acp/                              │
                   │   ├── acp.cache.json                 │
                   │   ├── acp.vars.json                  │
                   │   ├── daemon.pid                     │
                   │   └── daemon.log                     │
                   └──────────────────────────────────────┘
```

---

## Components

### 1. HTTP REST API

**Location**: See [acp-daemon repository](https://github.com/acp-protocol/acp-daemon) - `src/server.rs`, `src/api/`

**Purpose**: Provide a REST API for querying codebase intelligence over HTTP.

**Implementation**: Uses Axum web framework with tower middleware for CORS and tracing.

**Endpoints**:

| Endpoint | Method | Description |
|----------|--------|-------------|
| `/health` | GET | Health check with version info |
| `/cache` | GET | Full codebase cache |
| `/config` | GET | ACP configuration |
| `/vars` | GET | Variable definitions |
| `/symbols` | GET | List symbols (filterable) |
| `/symbols/{name}` | GET | Get specific symbol |
| `/files` | GET | List files |
| `/files/{*path}` | GET | Get specific file metadata |
| `/domains` | GET | List domains |
| `/domains/{name}` | GET | Get domain details |
| `/constraints/{*path}` | GET | Get file constraints |
| `/callers/{symbol}` | GET | Get symbol's callers |
| `/callees/{symbol}` | GET | Get symbol's callees |
| `/vars/{name}/expand` | GET | Expand variable reference |

**Query Parameters** (for `/symbols`):
- `file`: Filter by file path (contains match)
- `type`: Filter by symbol type (function, class, etc.)
- `exported`: Filter by exported status
- `limit`: Maximum results

**Status**: ✅ Fully Implemented

---

### 2. MCP Server

**Location**: See [acp-mcp repository](https://github.com/acp-protocol/acp-mcp) - `src/mcp/`

**Purpose**: Provide Model Context Protocol tools for direct AI assistant integration (Claude Desktop, etc.).

**Implementation**: Uses `rmcp` crate with stdio transport for Claude Desktop integration.

**MCP Tools**:

| Tool | Parameters | Description |
|------|------------|-------------|
| `acp_get_architecture` | (none) | Project overview: domains, files, symbols, languages |
| `acp_get_file_context` | `path` | File details: exports, imports, symbols, constraints |
| `acp_get_symbol_context` | `name` | Symbol details: definition, callers, callees |
| `acp_get_domain_files` | `name` | Files belonging to a domain |
| `acp_check_constraints` | `path` | Lock levels, style rules, behavior requirements |
| `acp_get_hotpaths` | (none) | Top 20 most frequently called symbols |
| `acp_expand_variable` | `name` | Expand $VAR_NAME to full context |
| `acp_generate_primer` | `token_budget`, `format`, `preset`, etc. | Generate optimized context primer |

**Primer Generation Parameters**:
- `token_budget`: Maximum tokens (default: 4000)
- `format`: "markdown", "compact", "json" (default: markdown)
- `preset`: "safe", "efficient", "accurate", "balanced" (default: balanced)
- `capabilities`: Tool capabilities (default: shell, file-read, file-write)
- `categories`: Filter by categories
- `tags`: Filter by tags
- `force_include`: Force include specific section IDs

**Usage with Claude Desktop**:
```json
{
  "mcpServers": {
    "acp": {
      "command": "acpd",
      "args": ["mcp", "-C", "/path/to/project"]
    }
  }
}
```

**Status**: ✅ Fully Implemented

---

### 3. Primer Generator

**Location**: See [acp-mcp repository](https://github.com/acp-protocol/acp-mcp) - `src/primer/`

**Purpose**: Generate intelligent context primers optimized for AI consumption within token budgets.

**Architecture**:

```
┌─────────────────────────────────────────────────────────────────────────┐
│                      Primer Generation Pipeline                         │
├─────────────────────────────────────────────────────────────────────────┤
│                                                                          │
│  ┌────────────────┐     ┌────────────────┐     ┌────────────────┐       │
│  │  PrimerDefaults│     │  ProjectState  │     │ DimensionWeights│      │
│  │  (embedded     │     │  (from cache)  │     │ (from preset)  │       │
│  │   JSON)        │     │                │     │                │       │
│  └───────┬────────┘     └───────┬────────┘     └───────┬────────┘       │
│          │                      │                      │                │
│          └──────────────────────┼──────────────────────┘                │
│                                 ▼                                       │
│                    ┌────────────────────────┐                           │
│                    │   score_sections()     │                           │
│                    │   Multi-dimensional    │                           │
│                    │   value scoring        │                           │
│                    └───────────┬────────────┘                           │
│                                │                                        │
│                                ▼                                        │
│                    ┌────────────────────────┐                           │
│                    │   select_sections()    │                           │
│                    │   Phase-based:         │                           │
│                    │   1. Required          │                           │
│                    │   2. Conditionally Req │                           │
│                    │   3. Safety-critical   │                           │
│                    │   4. Value-optimized   │                           │
│                    └───────────┬────────────┘                           │
│                                │                                        │
│                                ▼                                        │
│                    ┌────────────────────────┐                           │
│                    │   PrimerRenderer       │                           │
│                    │   Format output        │                           │
│                    │   (Handlebars)         │                           │
│                    └───────────┬────────────┘                           │
│                                │                                        │
│                                ▼                                        │
│                    ┌────────────────────────┐                           │
│                    │   PrimerResult         │                           │
│                    │   content, sections,   │                           │
│                    │   tokens_used          │                           │
│                    └────────────────────────┘                           │
│                                                                          │
└─────────────────────────────────────────────────────────────────────────┘
```

**Value Dimensions**:
- `safety`: How critical for preventing harmful AI actions (0-100)
- `efficiency`: How much it saves future tokens/queries (0-100)
- `accuracy`: How much it improves response quality (0-100)
- `base`: Baseline value independent of dimensions (0-100)

**Presets**:
| Preset | Safety | Efficiency | Accuracy | Base |
|--------|--------|------------|----------|------|
| Safe | 2.5 | 0.8 | 1.0 | 0.8 |
| Efficient | 1.2 | 2.0 | 0.9 | 0.8 |
| Accurate | 1.2 | 0.9 | 2.0 | 0.8 |
| Balanced | 1.5 | 1.0 | 1.0 | 1.0 |

**Output Formats**:
- **Markdown**: Full formatted output with headers and sections
- **Compact**: Minimal formatting for token efficiency
- **JSON**: Structured data for programmatic consumption

**Status**: ✅ Fully Implemented

---

### 4. File Watcher

**Location**: See [acp-cli repository](https://github.com/acp-protocol/acp-cli) - `src/watch.rs`

**Purpose**: Monitor filesystem changes for incremental cache updates.

**Current Implementation**:
```rust
pub struct FileWatcher {
    _config: AcpConfig,
}

impl FileWatcher {
    pub fn watch<P: AsRef<Path>>(&self, root: P) -> Result<()> {
        // Uses notify crate for cross-platform file watching
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

**Status**: ⚠️ Basic Implementation (incremental updates not yet implemented)

---

### 5. Process Lifecycle

**Location**: See [acp-daemon repository](https://github.com/acp-protocol/acp-daemon) - `src/lifecycle.rs`

**Purpose**: Manage daemon start/stop/status with PID file tracking.

**Commands**:
```bash
# Start daemon in background
acpd start [-f|--foreground] [--port 9222] [-C /path/to/project]

# Stop running daemon
acpd stop

# Check daemon status
acpd status

# Run in foreground (default)
acpd run

# Run MCP server over stdio
acpd mcp
```

**PID Management**:
- PID file: `.acp/daemon.pid`
- Log file: `.acp/daemon.log`
- Uses Unix signals (SIGTERM) for graceful shutdown

**Status**: ✅ Fully Implemented

---

### 6. Application State

**Location**: See [acp-daemon repository](https://github.com/acp-protocol/acp-daemon) - `src/state.rs`

**Purpose**: Thread-safe shared state for all daemon interfaces.

**Structure**:
```rust
pub struct AppState {
    inner: Arc<AppStateInner>,
}

struct AppStateInner {
    project_root: PathBuf,
    config: RwLock<Config>,
    cache: RwLock<Cache>,
    vars: RwLock<Option<VarsFile>>,
}
```

**Features**:
- Uses `tokio::sync::RwLock` for async concurrent access
- Hot-reload support via `reload_cache()` and `reload_vars()`
- Cloneable for sharing across handlers

**Status**: ✅ Fully Implemented

---

## Configuration

### Daemon CLI Options

```bash
acpd [OPTIONS] [COMMAND]

OPTIONS:
    -f, --foreground      Run in foreground mode (don't daemonize)
        --port <PORT>     HTTP server port [default: 9222]
    -C, --directory <DIR> Project root directory
        --log-level <LVL> Log level [default: info]

COMMANDS:
    start     Start the daemon
    stop      Stop the daemon
    status    Check daemon status
    run       Run daemon in foreground
    mcp       Run MCP server over stdio
```

### File Locations

| File | Purpose |
|------|---------|
| `.acp.config.json` | Project configuration |
| `.acp/acp.cache.json` | Codebase index (required) |
| `.acp/acp.vars.json` | Variable definitions (optional) |
| `.acp/daemon.pid` | Process ID for lifecycle |
| `.acp/daemon.log` | Daemon output log |

---

## Dependencies

From the daemon's `Cargo.toml`:

| Crate | Version | Purpose |
|-------|---------|---------|
| `axum` | 0.8 | HTTP server framework |
| `tower-http` | 0.6 | CORS and trace middleware |
| `tokio` | workspace | Async runtime |
| `rmcp` | 0.8 | Model Context Protocol SDK |
| `handlebars` | 6.3 | Template rendering for primers |
| `notify` | 8.2 | File watching (Phase 4) |
| `nix` | 0.29 | Unix signals for lifecycle |
| `daemonize` | 0.5 | Background process management |
| `acp` | path | Shared CLI library |

---

## Implementation Status

| Component | Status | Notes |
|-----------|--------|-------|
| HTTP REST API | ✅ Complete | All endpoints implemented |
| MCP Server | ✅ Complete | 8 tools available |
| Primer Generator | ✅ Complete | Multi-dimensional scoring |
| Process Lifecycle | ✅ Complete | Start/stop/status |
| Application State | ✅ Complete | Thread-safe shared state |
| File Watcher | ⚠️ Basic | Incremental updates TODO |
| HTTP Proxy | ⏳ Planned | Not yet implemented |
| Sync Service | ⏳ Planned | Not yet implemented |

---

## Future: HTTP Proxy (Planned)

The HTTP Proxy component is envisioned to intercept LLM API requests and inject ACP bootstrap context.

**Planned Capabilities**:
- Route requests to multiple providers (Anthropic, OpenAI, Google)
- Inject bootstrap into system prompt
- Handle SSE streaming passthrough
- Per-tool bootstrap templates

**Planned Data Flow**:
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
│   (Anthropic)   │
└────────┬────────┘
         │
         ▼
    AI Tool
```

---

## Future: Sync Service (Planned)

The Sync Service would distribute ACP context to tool-specific configuration files.

**Planned Tool Adapters**:
| Tool | Config File | Format |
|------|-------------|--------|
| Cursor | `.cursorrules` | Markdown |
| Claude Code | `CLAUDE.md` | Markdown |
| GitHub Copilot | `.github/copilot-instructions.md` | Markdown |
| Windsurf | `.windsurfrules` | Markdown |
| Zed | `AGENTS.md` | Markdown |
| Continue.dev | `.continue/config.json` | JSON |

---

# Evaluation Against Project Goals

## Goal Alignment Summary

| Goal | Description | Alignment | Score |
|------|-------------|-----------|-------|
| G1 | Token Efficiency | ✅ Strong | 5/5 |
| G2 | Deterministic Context | ✅ Strong | 5/5 |
| G3 | Version Controlled | ✅ Strong | 5/5 |
| G4 | Offline Capable | ⚠️ Partial | 3/5 |
| G5 | Tool Agnostic | ✅ Strong | 5/5 |
| G6 | Safety First | ✅ Strong | 5/5 |
| G7 | Transparent Integration | ⚠️ Partial | 3/5 |
| G8 | Query Optimized | ✅ Strong | 5/5 |
| **Overall** | | **Well Aligned** | **36/40 (90%)** |

## Design Principle Adherence

| Principle | Description | Score |
|-----------|-------------|-------|
| P1 | Transparency (No Manual Config) | 3/5 |
| P2 | Universality (Any Tool) | 4/5 |
| P3 | Freshness (Synchronized) | 4/5 |
| P4 | Efficiency (Token Budgets) | 5/5 |
| P5 | Safety-First (Constraints) | 5/5 |
| **Overall** | | **21/25 (84%)** |

## Gap Analysis

| Gap ID | Description | Severity | Recommendation |
|--------|-------------|----------|----------------|
| GAP-D1 | Daemon requires manual startup | Medium | Auto-start mechanism |
| GAP-D2 | MCP requires config file edit | Medium | `acp mcp-install` command |
| GAP-D3 | Watch not integrated with daemon | Medium | Integrate file watcher |
| GAP-D4 | HTTP Proxy not implemented | Low | Future phase |
| GAP-D5 | Sync service not implemented | Low | Future phase |

## Final Assessment

```
┌─────────────────────────────────────────────────────────────────────────┐
│                     DAEMON VALUE ASSESSMENT                             │
├─────────────────────────────────────────────────────────────────────────┤
│                                                                         │
│  Goal Alignment:      ████████████████████░░░░  90%  (36/40)           │
│                                                                         │
│  Principle Adherence: ████████████████░░░░░░░░  84%  (21/25)           │
│                                                                         │
│  Implementation:      ████████████████░░░░░░░░  75%  (4/5 complete)    │
│                                                                         │
│  ROI:                 ████████████████████░░░░  HIGH VALUE              │
│                                                                         │
└─────────────────────────────────────────────────────────────────────────┘

RECOMMENDATION: ✅ PROCEED with current architecture
                Focus on GAP-D1 (auto-start) and GAP-D3 (watch integration)
```

---

# AI Bootstrap Requirements

This section describes the bootstrap/primer content needed for AI tools to effectively leverage the ACP Daemon.

## The Bootstrap Problem

For an AI to use the daemon effectively, it needs to know:
1. **That the daemon exists** and what capabilities it provides
2. **How to invoke** daemon tools (MCP) or API endpoints (REST)
3. **When to use** each capability in its workflow
4. **What the data means** when it receives responses

Without this bootstrap knowledge, the AI has powerful tools available but no understanding of how to use them strategically.

## Bootstrap Delivery Methods

### Method 1: MCP Server Instructions (Current)

The MCP server provides instructions via the `get_info()` handler:

```rust
instructions: Some(
    "ACP (AI Context Protocol) server providing codebase context for AI agents. \
     Use acp_get_architecture first to understand the project structure, then \
     use other tools to explore specific files, symbols, and domains.".to_string()
)
```

**Limitation**: Brief, doesn't cover workflow integration or decision-making.

### Method 2: System Prompt Injection (Proxy - Planned)

The HTTP Proxy would inject bootstrap into every request:

```
[ACP] This project uses AI Context Protocol. Before modifying files:
1. Check constraints: `acp_check_constraints(path)`
2. Understand context: `acp_get_file_context(path)`
3. Review hotpaths: `acp_get_hotpaths()` for critical code

Protected files require approval. Use `acp_generate_primer` for full context.
```

**Limitation**: Token overhead on every request (~50-80 tokens).

### Method 3: Tool File Bootstrap (Sync - Planned)

Inject bootstrap into `.cursorrules`, `CLAUDE.md`, etc.:

```markdown
## ACP Integration

This project uses ACP for codebase intelligence. MCP tools available:
- `acp_get_architecture` - Start here to understand the project
- `acp_check_constraints` - ALWAYS check before modifying files
- `acp_generate_primer` - Get optimized context within token budget
```

**Limitation**: Static, may become stale.

### Method 4: Primer-Based Bootstrap (Recommended)

Use `acp_generate_primer` as the primary bootstrap mechanism:

```json
{
  "tool": "acp_generate_primer",
  "params": {
    "token_budget": 500,
    "preset": "safe",
    "format": "compact",
    "force_include": ["bootstrap", "constraints-summary", "hotpaths"]
  }
}
```

**Advantage**: Dynamic, token-efficient, always current.

---

## Recommended Bootstrap Content

### Minimal Bootstrap (~50 tokens)

For injection via proxy or minimal tool files:

```
[ACP] Check `acp_check_constraints <file>` before modifying.
Query architecture: `acp_get_architecture`.
Generate context: `acp_generate_primer --budget 2000`.
```

### Standard Bootstrap (~200 tokens)

For tool files like `CLAUDE.md`:

```markdown
## ACP Daemon Integration

This project runs an ACP daemon providing codebase intelligence.

### Before Modifying Code
1. **Check constraints**: `acp_check_constraints(path)` - Required for all file changes
2. **Understand context**: `acp_get_file_context(path)` - Get file metadata, imports, exports

### Understanding the Codebase
1. **Architecture overview**: `acp_get_architecture()` - Domains, file counts, languages
2. **Symbol lookup**: `acp_get_symbol_context(name)` - Definition, callers, callees
3. **Hotpaths**: `acp_get_hotpaths()` - Most critical code paths

### Getting Optimized Context
Use `acp_generate_primer` with appropriate settings:
- `preset: "safe"` - Prioritize constraint information
- `preset: "efficient"` - Minimize tokens
- `token_budget` - Stay within limits (default: 4000)
```

### Full Bootstrap (~500 tokens)

For comprehensive onboarding, included in primer `bootstrap` section:

```markdown
## ACP Daemon - AI Context Protocol

You have access to the ACP daemon, a codebase intelligence service.

### Available MCP Tools

| Tool | Purpose | When to Use |
|------|---------|-------------|
| `acp_get_architecture` | Project overview | First action on new tasks |
| `acp_get_file_context` | File metadata | Before reading/modifying files |
| `acp_get_symbol_context` | Symbol details | Understanding functions/classes |
| `acp_get_domain_files` | Domain membership | Finding related files |
| `acp_check_constraints` | File constraints | **ALWAYS before modifications** |
| `acp_get_hotpaths` | Critical code | Understanding impact |
| `acp_expand_variable` | Variable expansion | Resolving $VAR references |
| `acp_generate_primer` | Optimized context | Getting comprehensive overview |

### Workflow Integration

1. **New Task**: Call `acp_get_architecture()` to understand scope
2. **Before Reading**: Call `acp_get_file_context(path)` for metadata
3. **Before Modifying**: Call `acp_check_constraints(path)` - **MANDATORY**
4. **Understanding Impact**: Call `acp_get_hotpaths()` for critical paths
5. **Deep Dive**: Call `acp_generate_primer(budget=2000)` for full context

### Constraint Levels

| Level | Meaning | AI Action |
|-------|---------|-----------|
| `frozen` | Do not modify | Refuse changes, explain why |
| `restricted` | Approval required | Warn user, request confirmation |
| `normal` | Standard rules | Proceed with normal care |

### Safety Protocol

Before ANY file modification:
1. Check constraints exist for the file
2. If `frozen`: Stop, explain the constraint
3. If `restricted`: Warn user, await approval
4. If `normal`: Proceed with standard review
```

---

## Primer Section for Bootstrap

The daemon's primer generator should include a `bootstrap` section in `primers/primer.defaults.json`:

```json
{
  "id": "bootstrap",
  "name": "ACP Bootstrap",
  "description": "Instructions for AI to use ACP daemon effectively",
  "category": "required",
  "priority": 1,
  "tokens": 150,
  "required": true,
  "value": {
    "safety": 100,
    "efficiency": 80,
    "accuracy": 90,
    "base": 100
  },
  "formats": {
    "markdown": {
      "template": "## ACP Integration\n\nThis project uses ACP daemon for codebase intelligence.\n\n**Before modifying files**: `acp_check_constraints(path)`\n**Get context**: `acp_generate_primer(budget={{token_budget}})`\n**Understand structure**: `acp_get_architecture()`\n\n{{#if constraints.frozen_count}}**Warning**: {{constraints.frozen_count}} frozen files - check constraints.{{/if}}"
    },
    "compact": {
      "template": "[ACP] check:acp_check_constraints ctx:acp_generate_primer arch:acp_get_architecture{{#if constraints.frozen_count}} FROZEN:{{constraints.frozen_count}}{{/if}}"
    }
  }
}
```

---

## Integration Strategy

### For MCP-Based Integration (Claude Desktop)

1. **Server instructions** provide minimal bootstrap
2. **First tool call** should be `acp_get_architecture`
3. **AI learns** tool capabilities from responses
4. **Primer** provides comprehensive context on demand

Recommended Claude Desktop workflow:
```
User: "Help me fix the auth bug"

AI: [Calls acp_get_architecture]
    → Learns project has "auth" domain

AI: [Calls acp_get_domain_files(name="auth")]
    → Gets list of auth-related files

AI: [Calls acp_check_constraints(path="src/auth/session.ts")]
    → Sees "restricted" constraint

AI: "I found the auth files. Note that src/auth/session.ts has
     restricted constraints - I'll need your approval before
     modifying it. Let me examine the code..."
```

### For Proxy-Based Integration (Future)

1. **Bootstrap injection** on every request (~50 tokens)
2. **AI has context** without needing to call tools first
3. **Tools available** for deeper exploration
4. **Stateless** - each request is self-contained

### For File-Based Integration (Cursor, etc.)

1. **Tool file** contains standard bootstrap
2. **AI reads** on session start
3. **Must re-sync** when codebase changes
4. **Offline capable** - no daemon required for basic context

---

## Measuring Bootstrap Effectiveness

### Success Metrics

| Metric | Target | Measurement |
|--------|--------|-------------|
| Constraint Check Rate | 100% before modifications | Log tool calls |
| Architecture Query | First action on new tasks | Log tool order |
| Primer Usage | When context needed | Log primer calls |
| Frozen File Violations | 0% | Monitor constraint errors |

### Bootstrap Quality Indicators

1. **AI asks about constraints** before proposing changes
2. **AI uses domain terminology** from architecture
3. **AI references hotpaths** when discussing impact
4. **AI stays within token budgets** using primer

---

## Recommendations

1. **Implement `bootstrap` primer section** with required:true
2. **Enhance MCP instructions** with workflow guidance
3. **Create `acp primer --bootstrap`** command for quick injection
4. **Add bootstrap to sync templates** for all tool files
5. **Track bootstrap effectiveness** via usage analytics

The goal is an AI that naturally:
- Checks constraints before every modification
- Understands project architecture from first interaction
- Uses appropriate tools for each task type
- Respects token budgets through primer optimization
- Follows safety protocols without explicit reminders

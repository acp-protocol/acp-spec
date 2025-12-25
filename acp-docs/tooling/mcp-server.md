# ACP MCP Server

**Document Type**: Reference + How-To  
**Status**: OUTLINE — Content to be added  
**Last Updated**: December 2025

---

## Overview

The ACP MCP Server provides Model Context Protocol integration, allowing AI assistants with MCP support to access ACP metadata natively. This enables AI tools like Claude to query your codebase structure, constraints, and domains through standardized MCP resources and tools.

---

## Installation

> **TODO**: Expand with verification steps

### npm (Global)
```bash
npm install -g @acp-protocol/mcp
```

### npx (On-demand)
```bash
npx @acp-protocol/mcp
```

### From Source
```bash
git clone https://github.com/acp-protocol/acp-mcp
cd acp-mcp
npm install
npm run build
```

---

## Quick Start

> **TODO**: Add complete example session

### 1. Start the Server
```bash
acp-mcp --project /path/to/project
```

### 2. Configure Your MCP Client
```json
{
  "mcpServers": {
    "acp": {
      "command": "acp-mcp",
      "args": ["--project", "/path/to/project"]
    }
  }
}
```

### 3. Use in AI Assistant
```
User: What are the frozen files in my project?
Claude: [Uses acp_query tool to check constraints.by_lock_level.frozen]
```

---

## Resources

The MCP server exposes ACP data as resources:

### `acp://cache`

Full cache contents.

**URI**: `acp://cache`
**Returns**: Complete `.acp.cache.json` contents

> **TODO**: Add response example, use cases

---

### `acp://file/{path}`

File metadata for a specific file.

**URI**: `acp://file/src/auth/session.ts`
**Parameters**:
| Parameter | Type | Description |
|-----------|------|-------------|
| `path` | string | Relative file path |

**Returns**: FileEntry object

> **TODO**: Add response example

---

### `acp://symbol/{qualified_name}`

Symbol metadata for a specific symbol.

**URI**: `acp://symbol/src/auth/session.ts:SessionService.validateToken`
**Parameters**:
| Parameter | Type | Description |
|-----------|------|-------------|
| `qualified_name` | string | Fully qualified symbol name |

**Returns**: SymbolEntry object

> **TODO**: Add response example

---

### `acp://domain/{name}`

Domain information.

**URI**: `acp://domain/auth`
**Parameters**:
| Parameter | Type | Description |
|-----------|------|-------------|
| `name` | string | Domain name |

**Returns**: Domain metadata including files and symbols

> **TODO**: Add response example

---

### `acp://constraints`

All constraint data.

**URI**: `acp://constraints`
**Returns**: Complete constraints index

> **TODO**: Add response example

---

## Tools

The MCP server provides tools for querying and checking:

### `acp_query`

Execute a jq query against the cache.

**Parameters**:
| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `query` | string | Yes | jq expression |

**Example**:
```json
{
  "name": "acp_query",
  "arguments": {
    "query": ".domains | keys"
  }
}
```

> **TODO**: Add response format, common queries

---

### `acp_check_constraint`

Check if a file can be modified.

**Parameters**:
| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `file` | string | Yes | File path |
| `operation` | string | No | Operation type |

**Example**:
```json
{
  "name": "acp_check_constraint",
  "arguments": {
    "file": "src/auth/session.ts",
    "operation": "modify"
  }
}
```

**Returns**:
```json
{
  "allowed": false,
  "lock_level": "frozen",
  "reason": "Security critical, DO NOT modify",
  "suggestion": "Consider modifying src/auth/utils.ts instead"
}
```

> **TODO**: Expand with all operation types

---

### `acp_expand_variable`

Expand a variable reference.

**Parameters**:
| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `variable` | string | Yes | Variable name (e.g., `$SYM_AUTH`) |

**Returns**: Expanded variable data

> **TODO**: Add response example

---

### `acp_get_context`

Get relevant context for a task.

**Parameters**:
| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `domain` | string | No | Filter by domain |
| `file` | string | No | Context for specific file |
| `symbol` | string | No | Context for specific symbol |
| `include_graph` | boolean | No | Include call graph |

**Returns**: Curated context for AI consumption

> **TODO**: Add response format

---

## Configuration

### Server Options

```bash
acp-mcp [options]
```

| Option | Description | Default |
|--------|-------------|---------|
| `--project <path>` | Project root | Current directory |
| `--cache <path>` | Cache file path | `.acp/acp.cache.json` |
| `--port <port>` | Server port (HTTP mode) | `3000` |
| `--stdio` | Use stdio transport | `false` |
| `--watch` | Watch for cache changes | `true` |

### MCP Client Configuration

#### Claude Desktop

```json
{
  "mcpServers": {
    "acp": {
      "command": "acp-mcp",
      "args": ["--project", "/path/to/project", "--stdio"]
    }
  }
}
```

#### Continue.dev

```json
{
  "mcp": {
    "servers": [
      {
        "name": "acp",
        "command": "acp-mcp",
        "args": ["--project", ".", "--stdio"]
      }
    ]
  }
}
```

> **TODO**: Add more client configurations

---

## Prompts

The server provides reusable prompts:

### `acp-context`

Generates a context primer for the AI.

**Parameters**:
| Parameter | Type | Description |
|-----------|------|-------------|
| `budget` | number | Token budget |
| `focus` | string | Domain or file focus |

> **TODO**: Document all prompts

---

## Integration Patterns

> **TODO**: Expand each pattern with examples

### Pattern 1: Constraint-Aware Code Generation

1. AI receives code generation request
2. Uses `acp_check_constraint` before suggesting changes
3. Avoids frozen/restricted files
4. Suggests alternatives if needed

### Pattern 2: Domain-Scoped Operations

1. AI receives domain-specific request
2. Uses `acp://domain/{name}` to get relevant files
3. Operates only within domain boundaries

### Pattern 3: Context-Rich Assistance

1. AI needs to understand a symbol
2. Uses `acp://symbol/{name}` for metadata
3. Uses call graph for related context

---

## Sections to Add

- [ ] **Authentication**: Securing the MCP server
- [ ] **Rate Limiting**: Preventing abuse
- [ ] **Caching**: Server-side caching strategies
- [ ] **Logging**: Debug logging configuration
- [ ] **Metrics**: Prometheus/OpenTelemetry integration
- [ ] **Deployment**: Docker, Kubernetes examples

---

*This document is an outline. [Contribute content →](https://github.com/acp-protocol/acp-mcp)*

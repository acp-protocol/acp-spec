# RFC-0013: MCP Server Tool Completeness

**RFC Number**: 0013  
**Title**: MCP Server Tool Completeness  
**Author**: ACP Protocol Team  
**Status**: Draft  
**Created**: 2025-12-27  
**Updated**: 2025-12-27  
**Target Version**: 1.3.0  
**Requires**: RFC-001 (Self-Documenting Annotations)  
**Replaces**: None

---

## Table of Contents

1. [Summary](#1-summary)
2. [Motivation](#2-motivation)
3. [Research Basis](#3-research-basis)
4. [Specification](#4-specification)
5. [Tool Definitions](#5-tool-definitions)
6. [Implementation Guide](#6-implementation-guide)
7. [Backward Compatibility](#7-backward-compatibility)
8. [Security Considerations](#8-security-considerations)
9. [Testing Requirements](#9-testing-requirements)
10. [Migration Guide](#10-migration-guide)
11. [Alternatives Considered](#11-alternatives-considered)
12. [References](#12-references)

---

## 1. Summary

This RFC specifies the complete set of MCP (Model Context Protocol) tools that an ACP-conformant MCP server MUST, SHOULD, or MAY implement to achieve Level 3 (Full) conformance. It addresses gaps between the current `acp-mcp` implementation and the tools specified in Chapter 10 (Querying) of the ACP specification.

**Key additions:**
- `acp_query` - Unified query interface (consolidates multiple query patterns)
- `acp_expand` - Enhanced variable expansion with multi-variable and mode support
- `acp_debug` - Debug session management for reversible troubleshooting
- `acp_hack` - Hack tracking for temporary/experimental code

---

## 2. Motivation

### 2.1 Problem Statement

The current `acp-mcp` server implements 8 tools:
- `acp_get_architecture`
- `acp_get_file_context`
- `acp_get_symbol_context`
- `acp_get_domain_files`
- `acp_check_constraints`
- `acp_get_hotpaths`
- `acp_expand_variable`
- `acp_generate_primer`

However, the ACP specification (Chapter 10, Section 4.2) defines additional tools:
- `acp_query` - Unified query interface
- `acp_expand` - Enhanced variable expansion
- `acp_debug` - Debug session management
- `acp_hack` - Hack tracking

This gap creates:
1. **Spec non-conformance**: Level 3 (Full) conformance requires MCP server interface per spec
2. **Interoperability issues**: Agents expecting spec-defined tools will fail
3. **Feature incompleteness**: Debug sessions and hack tracking unavailable via MCP

### 2.2 Goals

1. Define complete MCP tool inventory for Level 3 conformance
2. Specify unified `acp_query` interface that complements existing granular tools
3. Enhance `acp_expand_variable` to full `acp_expand` capability
4. Add `acp_debug` and `acp_hack` tools for completeness
5. Maintain backward compatibility with existing tool names

### 2.3 Non-Goals

- Changing existing tool behavior (additive only)
- Defining MCP Resources (future RFC)
- Defining MCP Prompts (future RFC)
- Watch mode implementation (separate concern)

---

## 3. Research Basis

### 3.1 Zed Agent Client Protocol Integration

Research into Zed Industries' Agent Client Protocol (their "ACP") reveals that:

1. **MCP is the integration point**: Zed agents access external context via MCP servers
2. **Tool discoverability matters**: Agents enumerate tools via `tools/list`
3. **Unified interfaces preferred**: Agents benefit from fewer, more flexible tools

The `acp_query` tool provides a unified interface that agents can use without memorizing many tool names.

### 3.2 Gap Analysis (GAP-IDs)

| GAP-ID      | Gap                                           | Resolution              |
|-------------|-----------------------------------------------|-------------------------|
| GAP-MCP-001 | Missing `acp_query` unified interface         | Add `acp_query` tool    |
| GAP-MCP-002 | `acp_expand_variable` lacks multi-var support | Enhance to `acp_expand` |
| GAP-MCP-003 | No `acp_debug` for session tracking           | Add `acp_debug` tool    |
| GAP-MCP-004 | No `acp_hack` for hack tracking               | Add `acp_hack` tool     |
| GAP-MCP-005 | Missing search functionality                  | Add `search` query type |
| GAP-MCP-006 | Missing stats functionality                   | Add `stats` query type  |

### 3.3 Source References

- ACP Specification Chapter 10: Querying (Section 4.2 MCP Tools)
- ACP Specification Chapter 13: Debug Sessions
- Zed Agent Client Protocol: https://agentclientprotocol.com
- MCP Specification: https://modelcontextprotocol.io

---

## 4. Specification

### 4.1 Conformance Levels

| Tool                     | Level 1   | Level 2   | Level 3                               |
|--------------------------|-----------|-----------|---------------------------------------|
| `acp_get_architecture`   | -         | -         | SHOULD                                |
| `acp_get_file_context`   | -         | -         | SHOULD                                |
| `acp_get_symbol_context` | -         | -         | SHOULD                                |
| `acp_get_domain_files`   | -         | -         | SHOULD                                |
| `acp_check_constraints`  | -         | -         | **MUST**                              |
| `acp_get_hotpaths`       | -         | -         | MAY                                   |
| `acp_expand_variable`    | -         | -         | SHOULD (deprecated, use `acp_expand`) |
| `acp_generate_primer`    | -         | -         | SHOULD                                |
| **`acp_query`**          | -         | -         | **MUST**                              |
| **`acp_expand`**         | -         | -         | **MUST**                              |
| **`acp_debug`**          | -         | -         | **MUST**                              |
| **`acp_hack`**           | -         | -         | SHOULD                                |

### 4.2 Tool Naming Convention

All ACP MCP tools:
- MUST use `acp_` prefix
- MUST use snake_case
- SHOULD be descriptive of action (verb) or query (noun)

### 4.3 Error Handling

All tools MUST return MCP-compliant errors:

```json
{
  "code": -32602,
  "message": "Invalid params",
  "data": {
    "field": "name",
    "reason": "Symbol not found: validateSession"
  }
}
```

Standard error codes:

| Code   | Meaning          | When to Use                |
|--------|------------------|----------------------------|
| -32600 | Invalid Request  | Malformed JSON-RPC         |
| -32601 | Method not found | Unknown tool name          |
| -32602 | Invalid params   | Missing/invalid parameters |
| -32603 | Internal error   | Server-side failure        |
| -32000 | Cache not found  | No `.acp.cache.json`       |
| -32001 | Cache stale      | Cache needs regeneration   |

---

## 5. Tool Definitions

### 5.1 `acp_query` (NEW - MUST implement)

Unified query interface that consolidates multiple query patterns.

#### Parameters

```typescript
interface AcpQueryParams {
  /** Query type */
  type: "symbol" | "file" | "domain" | "callers" | "callees" | "search" | "stats";
  
  /** Name for symbol/file/domain/callers/callees queries */
  name?: string;
  
  /** Pattern for search queries (glob or regex) */
  pattern?: string;
  
  /** Maximum results for search (default: 20) */
  limit?: number;
}
```

#### JSON Schema

```json
{
  "$schema": "http://json-schema.org/draft-07/schema#",
  "type": "object",
  "required": ["type"],
  "properties": {
    "type": {
      "type": "string",
      "enum": ["symbol", "file", "domain", "callers", "callees", "search", "stats"],
      "description": "Query type"
    },
    "name": {
      "type": "string",
      "description": "Name for symbol/file/domain/callers/callees queries"
    },
    "pattern": {
      "type": "string",
      "description": "Search pattern (glob or regex)"
    },
    "limit": {
      "type": "integer",
      "minimum": 1,
      "maximum": 100,
      "default": 20,
      "description": "Maximum results for search"
    }
  }
}
```

#### Query Types

| Type      | Required Params   | Description              | Returns                          |
|-----------|-------------------|--------------------------|----------------------------------|
| `symbol`  | `name`            | Get symbol details       | SymbolEntry with callers/callees |
| `file`    | `name`            | Get file metadata        | FileEntry with symbols           |
| `domain`  | `name`            | Get domain info          | DomainEntry with files           |
| `callers` | `name`            | Get callers of symbol    | Array of qualified names         |
| `callees` | `name`            | Get callees of symbol    | Array of qualified names         |
| `search`  | `pattern`         | Search symbols and files | Array of matches                 |
| `stats`   | (none)            | Get codebase statistics  | Stats object                     |

#### Response Examples

**Symbol Query:**
```json
{
  "type": "symbol",
  "result": {
    "name": "validateSession",
    "qualified_name": "src/auth/session.ts:validateSession",
    "type": "function",
    "file": "src/auth/session.ts",
    "lines": [45, 89],
    "signature": "(token: string) => Session | null",
    "purpose": "Validates JWT tokens and returns session data",
    "constraints": {
      "lock_level": "restricted",
      "directive": "MUST explain changes before modifying"
    },
    "callers": ["src/middleware/auth.ts:authMiddleware"],
    "callees": ["src/auth/jwt.ts:verifyToken", "src/db/sessions.ts:getSession"]
  }
}
```

**Stats Query:**
```json
{
  "type": "stats",
  "result": {
    "files": 127,
    "symbols": 843,
    "lines": 24521,
    "domains": 5,
    "constraints": {
      "frozen": 3,
      "restricted": 12,
      "normal": 112
    },
    "languages": ["typescript", "javascript", "python"]
  }
}
```

**Search Query:**
```json
{
  "type": "search",
  "pattern": "validate*",
  "result": {
    "matches": [
      {"type": "symbol", "name": "validateSession", "file": "src/auth/session.ts"},
      {"type": "symbol", "name": "validateToken", "file": "src/auth/jwt.ts"},
      {"type": "file", "path": "src/validators/index.ts"}
    ],
    "total": 3,
    "truncated": false
  }
}
```

#### Error Cases

| Condition                  | Error Code   | Message                               |
|----------------------------|--------------|---------------------------------------|
| Missing `type`             | -32602       | "Missing required parameter: type"    |
| Invalid `type` value       | -32602       | "Invalid query type: {value}"         |
| `symbol` without `name`    | -32602       | "'name' required for symbol query"    |
| Symbol not found           | -32602       | "Symbol not found: {name}"            |
| `search` without `pattern` | -32602       | "'pattern' required for search query" |

---

### 5.2 `acp_expand` (NEW - MUST implement)

Enhanced variable expansion supporting multiple variables and expansion modes.

#### Parameters

```typescript
interface AcpExpandParams {
  /** Text containing $VAR references to expand */
  text: string;
  
  /** Expansion mode */
  mode?: "summary" | "full" | "inline" | "annotated";
}
```

#### JSON Schema

```json
{
  "$schema": "http://json-schema.org/draft-07/schema#",
  "type": "object",
  "required": ["text"],
  "properties": {
    "text": {
      "type": "string",
      "description": "Text containing $VAR references (e.g., $SYM_validateSession)"
    },
    "mode": {
      "type": "string",
      "enum": ["summary", "full", "inline", "annotated"],
      "default": "summary",
      "description": "Expansion mode"
    }
  }
}
```

#### Expansion Modes

| Mode        | Description             | Output                          |
|-------------|-------------------------|---------------------------------|
| `summary`   | Short summary (default) | Brief description with location |
| `full`      | Complete JSON           | Full cache entry as JSON        |
| `inline`    | Inline replacement      | Just the value, for embedding   |
| `annotated` | Shows both              | `$VAR → expansion` format       |

#### Response

```typescript
interface AcpExpandResponse {
  /** Original input text */
  original: string;
  
  /** Expanded text */
  expanded: string;
  
  /** Variables found in input */
  variables_found: string[];
  
  /** Variables successfully resolved */
  variables_resolved: string[];
  
  /** Variables that could not be resolved */
  variables_unresolved: string[];
  
  /** Expansion details per variable (for debugging) */
  expansions?: Record<string, {
    type: "symbol" | "file" | "domain";
    value: string;
    source: string;
  }>;
}
```

#### Example

**Request:**
```json
{
  "text": "Check $SYM_validateSession for the authentication bug in $FILE_session",
  "mode": "summary"
}
```

**Response:**
```json
{
  "original": "Check $SYM_validateSession for the authentication bug in $FILE_session",
  "expanded": "Check validateSession (src/auth/session.ts:45-89) - Validates JWT tokens for the authentication bug in src/auth/session.ts - Session management module",
  "variables_found": ["SYM_validateSession", "FILE_session"],
  "variables_resolved": ["SYM_validateSession", "FILE_session"],
  "variables_unresolved": []
}
```

#### Variable Resolution

Variables are resolved from `.acp.vars.json`:

| Prefix  | Type   | Example                | Resolution              |
|---------|--------|------------------------|-------------------------|
| `SYM_`  | Symbol | `$SYM_validateSession` | Symbol entry from cache |
| `FILE_` | File   | `$FILE_session`        | File entry from cache   |
| `DOM_`  | Domain | `$DOM_auth`            | Domain entry from cache |

---

### 5.3 `acp_debug` (NEW - MUST implement)

Debug session management for reversible troubleshooting.

#### Parameters

```typescript
interface AcpDebugParams {
  /** Action to perform */
  action: "start" | "attempt" | "result" | "revert" | "resolve" | "status" | "list";
  
  /** Action-specific data */
  data?: AcpDebugData;
}

interface AcpDebugData {
  // For 'start'
  problem?: string;
  hypothesis?: string;
  
  // For 'attempt'
  session_id?: string;
  description?: string;
  files?: string[];
  
  // For 'result'
  attempt_id?: string;
  success?: boolean;
  notes?: string;
  
  // For 'revert'
  // session_id and attempt_id required
  
  // For 'resolve'
  resolution?: string;
  
  // For 'status' and 'list'
  // session_id optional for status (returns current)
  // no data needed for list
}
```

#### JSON Schema

```json
{
  "$schema": "http://json-schema.org/draft-07/schema#",
  "type": "object",
  "required": ["action"],
  "properties": {
    "action": {
      "type": "string",
      "enum": ["start", "attempt", "result", "revert", "resolve", "status", "list"],
      "description": "Debug action to perform"
    },
    "data": {
      "type": "object",
      "properties": {
        "problem": {"type": "string"},
        "hypothesis": {"type": "string"},
        "session_id": {"type": "string"},
        "description": {"type": "string"},
        "files": {"type": "array", "items": {"type": "string"}},
        "attempt_id": {"type": "string"},
        "success": {"type": "boolean"},
        "notes": {"type": "string"},
        "resolution": {"type": "string"}
      }
    }
  }
}
```

#### Actions

| Action    | Required Data                         | Description                |
|-----------|---------------------------------------|----------------------------|
| `start`   | `problem`, `hypothesis`               | Begin new debug session    |
| `attempt` | `session_id`, `description`           | Log an attempt             |
| `result`  | `session_id`, `attempt_id`, `success` | Record outcome             |
| `revert`  | `session_id`, `attempt_id`            | Mark attempt as reverted   |
| `resolve` | `session_id`, `resolution`            | Close session successfully |
| `status`  | `session_id` (optional)               | Get session status         |
| `list`    | (none)                                | List all active sessions   |

#### Response Examples

**Start Session:**
```json
{
  "action": "start",
  "session_id": "debug-20251227-001",
  "created_at": "2025-12-27T10:30:00Z",
  "status": "active",
  "problem": "Authentication fails for expired tokens",
  "hypothesis": "Token expiration check happens after signature validation"
}
```

**Log Attempt:**
```json
{
  "action": "attempt",
  "session_id": "debug-20251227-001",
  "attempt_id": "attempt-001",
  "description": "Move expiration check before signature validation",
  "files": ["src/auth/jwt.ts"],
  "created_at": "2025-12-27T10:35:00Z"
}
```

**Session Status:**
```json
{
  "action": "status",
  "session": {
    "id": "debug-20251227-001",
    "status": "active",
    "problem": "Authentication fails for expired tokens",
    "hypothesis": "Token expiration check happens after signature validation",
    "created_at": "2025-12-27T10:30:00Z",
    "attempts": [
      {
        "id": "attempt-001",
        "description": "Move expiration check before signature validation",
        "files": ["src/auth/jwt.ts"],
        "status": "pending",
        "created_at": "2025-12-27T10:35:00Z"
      }
    ]
  }
}
```

#### Storage

Debug sessions are stored in `.acp/acp.attempts.json` per Chapter 13 specification.

---

### 5.4 `acp_hack` (NEW - SHOULD implement)

Track temporary/experimental code markers.

#### Parameters

```typescript
interface AcpHackParams {
  /** Action to perform */
  action: "mark" | "list" | "revert" | "cleanup";
  
  /** Action-specific data */
  data?: AcpHackData;
}

interface AcpHackData {
  // For 'mark'
  file?: string;
  line?: number;
  reason?: string;
  ticket?: string;
  expires?: string;  // ISO date
  
  // For 'list'
  file?: string;         // Filter by file (optional)
  expired_only?: boolean; // Only show expired (optional)
  
  // For 'revert'
  id?: string;  // Hack ID to remove
  
  // For 'cleanup'
  // No data needed - finds all expired hacks
}
```

#### JSON Schema

```json
{
  "$schema": "http://json-schema.org/draft-07/schema#",
  "type": "object",
  "required": ["action"],
  "properties": {
    "action": {
      "type": "string",
      "enum": ["mark", "list", "revert", "cleanup"],
      "description": "Hack tracking action"
    },
    "data": {
      "type": "object",
      "properties": {
        "file": {"type": "string"},
        "line": {"type": "integer", "minimum": 1},
        "reason": {"type": "string"},
        "ticket": {"type": "string"},
        "expires": {"type": "string", "format": "date"},
        "expired_only": {"type": "boolean"},
        "id": {"type": "string"}
      }
    }
  }
}
```

#### Response Examples

**List Hacks:**
```json
{
  "action": "list",
  "hacks": [
    {
      "id": "hack-001",
      "file": "src/api/legacy.ts",
      "line": 45,
      "reason": "Workaround for legacy API response format",
      "ticket": "PROJ-1234",
      "expires": "2026-01-15",
      "expired": false,
      "created_at": "2025-12-27T10:00:00Z"
    }
  ],
  "total": 1,
  "expired_count": 0
}
```

**Cleanup Expired:**
```json
{
  "action": "cleanup",
  "expired_hacks": [
    {
      "id": "hack-002",
      "file": "src/utils/temp.ts",
      "reason": "Quick fix for demo",
      "expires": "2025-12-01"
    }
  ],
  "count": 1,
  "message": "Found 1 expired hack(s). Use 'revert' to remove."
}
```

---

### 5.5 `acp_expand_variable` (DEPRECATED)

This tool is deprecated in favor of `acp_expand`. Implementations:
- SHOULD continue to support for backward compatibility
- MUST log deprecation warning when called
- SHOULD redirect to `acp_expand` internally

---

## 6. Implementation Guide

### 6.1 Rust Implementation

#### `acp_query` Handler

```rust
#[derive(Debug, Deserialize, JsonSchema)]
pub struct QueryParams {
    /// Query type
    #[serde(rename = "type")]
    pub query_type: String,
    
    /// Name for symbol/file/domain/callers/callees queries
    pub name: Option<String>,
    
    /// Pattern for search queries
    pub pattern: Option<String>,
    
    /// Maximum results for search
    #[serde(default = "default_limit")]
    pub limit: usize,
}

fn default_limit() -> usize { 20 }

impl AcpMcpService {
    async fn handle_query(&self, params: QueryParams) -> Result<CallToolResult, McpError> {
        match params.query_type.as_str() {
            "symbol" => {
                let name = params.name.ok_or_else(|| 
                    McpError::invalid_params("'name' required for symbol query", None))?;
                self.query_symbol(name).await
            }
            "file" => {
                let name = params.name.ok_or_else(|| 
                    McpError::invalid_params("'name' required for file query", None))?;
                self.query_file(name).await
            }
            "domain" => {
                let name = params.name.ok_or_else(|| 
                    McpError::invalid_params("'name' required for domain query", None))?;
                self.query_domain(name).await
            }
            "callers" => {
                let name = params.name.ok_or_else(|| 
                    McpError::invalid_params("'name' required for callers query", None))?;
                self.query_callers(name).await
            }
            "callees" => {
                let name = params.name.ok_or_else(|| 
                    McpError::invalid_params("'name' required for callees query", None))?;
                self.query_callees(name).await
            }
            "search" => {
                let pattern = params.pattern.ok_or_else(|| 
                    McpError::invalid_params("'pattern' required for search query", None))?;
                self.query_search(pattern, params.limit).await
            }
            "stats" => {
                self.query_stats().await
            }
            _ => {
                Err(McpError::invalid_params(
                    format!("Invalid query type: {}", params.query_type), 
                    None
                ))
            }
        }
    }
    
    async fn query_symbol(&self, name: String) -> Result<CallToolResult, McpError> {
        let cache = self.state.cache_async().await;
        
        let symbol = cache.symbols.get(&name)
            .ok_or_else(|| McpError::invalid_params(
                format!("Symbol not found: {}", name), None
            ))?;
        
        // Get callers/callees from graph
        let (callers, callees) = if let Some(ref graph) = cache.graph {
            (
                graph.reverse.get(&name).cloned().unwrap_or_default(),
                graph.forward.get(&name).cloned().unwrap_or_default(),
            )
        } else {
            (Vec::new(), Vec::new())
        };
        
        #[derive(Serialize)]
        struct SymbolQueryResult {
            #[serde(rename = "type")]
            result_type: String,
            result: SymbolWithRelations,
        }
        
        #[derive(Serialize)]
        struct SymbolWithRelations {
            #[serde(flatten)]
            symbol: acp::cache::SymbolEntry,
            callers: Vec<String>,
            callees: Vec<String>,
        }
        
        let result = SymbolQueryResult {
            result_type: "symbol".to_string(),
            result: SymbolWithRelations {
                symbol: symbol.clone(),
                callers,
                callees,
            },
        };
        
        let json = serde_json::to_string_pretty(&result)
            .map_err(|e| McpError::internal_error(format!("JSON error: {}", e), None))?;
        
        Ok(CallToolResult::success(vec![Content::text(json)]))
    }
    
    async fn query_stats(&self) -> Result<CallToolResult, McpError> {
        let cache = self.state.cache_async().await;
        
        // Count constraints by level
        let mut constraint_counts: HashMap<String, usize> = HashMap::new();
        if let Some(ref constraints) = cache.constraints {
            for (level, files) in &constraints.by_lock_level {
                constraint_counts.insert(level.clone(), files.len());
            }
        }
        
        // Get unique languages
        let languages: Vec<String> = cache.files.values()
            .map(|f| format!("{:?}", f.language).to_lowercase())
            .collect::<std::collections::HashSet<_>>()
            .into_iter()
            .collect();
        
        #[derive(Serialize)]
        struct StatsResult {
            #[serde(rename = "type")]
            result_type: String,
            result: Stats,
        }
        
        #[derive(Serialize)]
        struct Stats {
            files: usize,
            symbols: usize,
            lines: usize,
            domains: usize,
            constraints: HashMap<String, usize>,
            languages: Vec<String>,
        }
        
        let result = StatsResult {
            result_type: "stats".to_string(),
            result: Stats {
                files: cache.files.len(),
                symbols: cache.symbols.len(),
                lines: cache.stats.lines,
                domains: cache.domains.len(),
                constraints: constraint_counts,
                languages,
            },
        };
        
        let json = serde_json::to_string_pretty(&result)
            .map_err(|e| McpError::internal_error(format!("JSON error: {}", e), None))?;
        
        Ok(CallToolResult::success(vec![Content::text(json)]))
    }
    
    async fn query_search(&self, pattern: String, limit: usize) -> Result<CallToolResult, McpError> {
        let cache = self.state.cache_async().await;
        
        let mut matches = Vec::new();
        
        // Search symbols
        for (name, symbol) in &cache.symbols {
            if name.contains(&pattern) || 
               symbol.name.contains(&pattern) ||
               symbol.purpose.as_ref().map(|p| p.contains(&pattern)).unwrap_or(false) {
                matches.push(SearchMatch {
                    match_type: "symbol".to_string(),
                    name: symbol.name.clone(),
                    qualified_name: Some(name.clone()),
                    file: Some(symbol.file.clone()),
                    path: None,
                });
                if matches.len() >= limit {
                    break;
                }
            }
        }
        
        // Search files if under limit
        if matches.len() < limit {
            for (path, file) in &cache.files {
                if path.contains(&pattern) ||
                   file.purpose.as_ref().map(|p| p.contains(&pattern)).unwrap_or(false) {
                    matches.push(SearchMatch {
                        match_type: "file".to_string(),
                        name: path.split('/').last().unwrap_or(path).to_string(),
                        qualified_name: None,
                        file: None,
                        path: Some(path.clone()),
                    });
                    if matches.len() >= limit {
                        break;
                    }
                }
            }
        }
        
        let truncated = matches.len() >= limit;
        
        #[derive(Serialize)]
        struct SearchResult {
            #[serde(rename = "type")]
            result_type: String,
            pattern: String,
            result: SearchResultData,
        }
        
        #[derive(Serialize)]
        struct SearchResultData {
            matches: Vec<SearchMatch>,
            total: usize,
            truncated: bool,
        }
        
        #[derive(Serialize)]
        struct SearchMatch {
            #[serde(rename = "type")]
            match_type: String,
            name: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            qualified_name: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            file: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            path: Option<String>,
        }
        
        let total = matches.len();
        let result = SearchResult {
            result_type: "search".to_string(),
            pattern,
            result: SearchResultData {
                matches,
                total,
                truncated,
            },
        };
        
        let json = serde_json::to_string_pretty(&result)
            .map_err(|e| McpError::internal_error(format!("JSON error: {}", e), None))?;
        
        Ok(CallToolResult::success(vec![Content::text(json)]))
    }
}
```

#### `acp_expand` Handler

```rust
#[derive(Debug, Deserialize, JsonSchema)]
pub struct ExpandParams {
    /// Text containing $VAR references
    pub text: String,
    
    /// Expansion mode
    #[serde(default = "default_mode")]
    pub mode: String,
}

fn default_mode() -> String { "summary".to_string() }

impl AcpMcpService {
    async fn handle_expand(&self, params: ExpandParams) -> Result<CallToolResult, McpError> {
        let cache = self.state.cache_async().await;
        let vars = self.state.vars_async().await;
        
        // Find all $VAR patterns
        let var_pattern = regex::Regex::new(r"\$([A-Z_][A-Z0-9_]*)").unwrap();
        
        let mut variables_found = Vec::new();
        let mut variables_resolved = Vec::new();
        let mut variables_unresolved = Vec::new();
        let mut expansions = HashMap::new();
        
        let mut expanded = params.text.clone();
        
        for cap in var_pattern.captures_iter(&params.text) {
            let var_name = cap.get(1).unwrap().as_str();
            variables_found.push(var_name.to_string());
            
            if let Some(var_def) = vars.variables.get(var_name) {
                variables_resolved.push(var_name.to_string());
                
                let expansion = match params.mode.as_str() {
                    "full" => self.expand_full(&var_def, &cache),
                    "inline" => self.expand_inline(&var_def, &cache),
                    "annotated" => self.expand_annotated(var_name, &var_def, &cache),
                    _ => self.expand_summary(&var_def, &cache),  // "summary" default
                };
                
                expansions.insert(var_name.to_string(), ExpansionDetail {
                    var_type: var_def.var_type.clone(),
                    value: var_def.value.clone(),
                    source: expansion.clone(),
                });
                
                expanded = expanded.replace(&format!("${}", var_name), &expansion);
            } else {
                variables_unresolved.push(var_name.to_string());
            }
        }
        
        #[derive(Serialize)]
        struct ExpandResponse {
            original: String,
            expanded: String,
            variables_found: Vec<String>,
            variables_resolved: Vec<String>,
            variables_unresolved: Vec<String>,
            expansions: HashMap<String, ExpansionDetail>,
        }
        
        #[derive(Serialize)]
        struct ExpansionDetail {
            #[serde(rename = "type")]
            var_type: String,
            value: String,
            source: String,
        }
        
        let response = ExpandResponse {
            original: params.text,
            expanded,
            variables_found,
            variables_resolved,
            variables_unresolved,
            expansions,
        };
        
        let json = serde_json::to_string_pretty(&response)
            .map_err(|e| McpError::internal_error(format!("JSON error: {}", e), None))?;
        
        Ok(CallToolResult::success(vec![Content::text(json)]))
    }
    
    fn expand_summary(&self, var: &VarDefinition, cache: &Cache) -> String {
        match var.var_type.as_str() {
            "symbol" => {
                if let Some(sym) = cache.symbols.get(&var.value) {
                    format!("{} ({}:{}-{}) - {}", 
                        sym.name, 
                        sym.file, 
                        sym.lines[0], 
                        sym.lines[1],
                        sym.purpose.as_ref().unwrap_or(&"No description".to_string())
                    )
                } else {
                    var.value.clone()
                }
            }
            "file" => {
                if let Some(file) = cache.files.get(&var.value) {
                    format!("{} - {}", 
                        var.value,
                        file.purpose.as_ref().unwrap_or(&"No description".to_string())
                    )
                } else {
                    var.value.clone()
                }
            }
            "domain" => {
                if let Some(domain) = cache.domains.get(&var.value) {
                    format!("{} ({} files) - {}", 
                        var.value,
                        domain.files.len(),
                        domain.description.as_ref().unwrap_or(&"No description".to_string())
                    )
                } else {
                    var.value.clone()
                }
            }
            _ => var.value.clone()
        }
    }
}
```

### 6.2 Tool Registration

Update `build_tools()` to include new tools:

```rust
fn build_tools() -> Vec<Tool> {
    vec![
        // Existing tools...
        
        // NEW: acp_query
        Tool::new(
            "acp_query",
            "Unified query interface for symbols, files, domains, relationships, and search. \
             Use type='symbol' for symbol details, 'file' for file metadata, 'domain' for domain info, \
             'callers'/'callees' for relationships, 'search' for pattern matching, 'stats' for overview.",
            schema_to_json_object::<QueryParams>(),
        ),
        
        // NEW: acp_expand
        Tool::new(
            "acp_expand",
            "Expand text containing $VAR references (like $SYM_validateSession, $FILE_config). \
             Supports modes: 'summary' (default), 'full' (complete JSON), 'inline', 'annotated'.",
            schema_to_json_object::<ExpandParams>(),
        ),
        
        // NEW: acp_debug
        Tool::new(
            "acp_debug",
            "Manage debug sessions for reversible troubleshooting. \
             Actions: 'start' (new session), 'attempt' (log try), 'result' (record outcome), \
             'revert' (mark reverted), 'resolve' (close session), 'status', 'list'.",
            schema_to_json_object::<DebugParams>(),
        ),
        
        // NEW: acp_hack
        Tool::new(
            "acp_hack",
            "Track temporary/experimental code markers. \
             Actions: 'mark' (add hack), 'list' (show hacks), 'revert' (remove), 'cleanup' (find expired).",
            schema_to_json_object::<HackParams>(),
        ),
    ]
}
```

---

## 7. Backward Compatibility

### 7.1 Existing Tools

All existing tools remain unchanged:
- `acp_get_architecture` - No changes
- `acp_get_file_context` - No changes
- `acp_get_symbol_context` - No changes
- `acp_get_domain_files` - No changes
- `acp_check_constraints` - No changes
- `acp_get_hotpaths` - No changes
- `acp_generate_primer` - No changes

### 7.2 Deprecated Tools

| Tool                  | Status     | Migration                             |
|-----------------------|------------|---------------------------------------|
| `acp_expand_variable` | Deprecated | Use `acp_expand` with single variable |

Deprecation behavior:
1. Tool continues to work
2. Logs warning: "acp_expand_variable is deprecated, use acp_expand"
3. Internally redirects to `acp_expand`

### 7.3 New Tools

New tools are purely additive and don't affect existing functionality.

---

## 8. Security Considerations

### 8.1 File System Access

- `acp_debug` and `acp_hack` write to `.acp/acp.attempts.json`
- MUST NOT allow arbitrary file writes
- MUST validate all file paths are within project root

### 8.2 Query Injection

- `acp_query` with `search` MUST sanitize patterns
- MUST NOT allow regex denial of service
- SHOULD limit pattern complexity

### 8.3 Information Disclosure

- Tools MUST NOT expose files outside project root
- Tools MUST respect `.acp.config.json` exclude patterns

---

## 9. Testing Requirements

### 9.1 Unit Tests

Each tool MUST have tests for:
- Valid parameter handling
- Missing required parameters
- Invalid parameter values
- Not-found cases
- Empty result cases

### 9.2 Integration Tests

```rust
#[tokio::test]
async fn test_acp_query_symbol() {
    let service = create_test_service();
    let result = service.handle_query(QueryParams {
        query_type: "symbol".to_string(),
        name: Some("validateSession".to_string()),
        pattern: None,
        limit: 20,
    }).await;
    
    assert!(result.is_ok());
    let json: Value = serde_json::from_str(&result.unwrap().content[0].text).unwrap();
    assert_eq!(json["type"], "symbol");
    assert!(json["result"]["callers"].is_array());
}

#[tokio::test]
async fn test_acp_query_missing_name() {
    let service = create_test_service();
    let result = service.handle_query(QueryParams {
        query_type: "symbol".to_string(),
        name: None,
        pattern: None,
        limit: 20,
    }).await;
    
    assert!(result.is_err());
    assert!(result.unwrap_err().message.contains("'name' required"));
}

#[tokio::test]
async fn test_acp_expand_multi_var() {
    let service = create_test_service();
    let result = service.handle_expand(ExpandParams {
        text: "Check $SYM_auth and $FILE_config".to_string(),
        mode: "summary".to_string(),
    }).await;
    
    assert!(result.is_ok());
    let json: Value = serde_json::from_str(&result.unwrap().content[0].text).unwrap();
    assert_eq!(json["variables_found"].as_array().unwrap().len(), 2);
}
```

### 9.3 MCP Protocol Tests

Test with actual MCP client (e.g., Claude Desktop):

```bash
# List tools
echo '{"jsonrpc":"2.0","id":1,"method":"tools/list","params":{}}' | acp-mcp

# Call acp_query
echo '{"jsonrpc":"2.0","id":2,"method":"tools/call","params":{"name":"acp_query","arguments":{"type":"stats"}}}' | acp-mcp
```

---

## 10. Migration Guide

### 10.1 For MCP Server Implementers

1. Add new tool handlers (Section 6.1)
2. Register tools in `build_tools()` (Section 6.2)
3. Deprecate `acp_expand_variable` with warning
4. Run test suite (Section 9)

### 10.2 For Agent Developers

1. Prefer `acp_query` for unified interface
2. Migrate from `acp_expand_variable` to `acp_expand`
3. Use `acp_debug` for reversible troubleshooting
4. Use `acp_hack` to track temporary code

### 10.3 Timeline

| Phase          | Duration  | Actions                       |
|----------------|-----------|-------------------------------|
| Implementation | 1 week    | Add tools, tests              |
| Deprecation    | 3 months  | Warn on `acp_expand_variable` |
| Removal        | 6 months  | Remove deprecated tool        |

---

## 11. Alternatives Considered

### 11.1 Separate Tools vs Unified `acp_query`

**Considered**: Keep only separate tools (`acp_get_*`)

**Rejected because**:
- Agents must discover and remember many tool names
- Spec explicitly defines `acp_query` interface
- Unified interface enables pattern-based queries

**Decision**: Add `acp_query` while keeping existing tools for backward compatibility

### 11.2 Breaking Change to `acp_expand_variable`

**Considered**: Rename to `acp_expand` and change signature

**Rejected because**:
- Would break existing integrations
- Deprecation path is safer

**Decision**: Add `acp_expand` as new tool, deprecate old

### 11.3 Optional `acp_debug` and `acp_hack`

**Considered**: Make these MAY instead of MUST/SHOULD

**Rejected because**:
- Level 3 conformance requires debug session tracking
- Consistency with spec Chapter 13
- Important for enterprise use cases

**Decision**: `acp_debug` is MUST, `acp_hack` is SHOULD

---

## 12. References

1. **ACP Specification Chapter 10**: Querying - MCP Server Interface
2. **ACP Specification Chapter 13**: Debug Sessions
3. **MCP Specification**: https://modelcontextprotocol.io
4. **JSON-RPC 2.0**: https://www.jsonrpc.org/specification
5. **Zed Agent Client Protocol**: https://agentclientprotocol.com
6. **rmcp Rust SDK**: https://docs.rs/rmcp

---

## Appendix A: Complete Tool Inventory

After RFC-0013 implementation:

| Tool                     | Type      | Conformance  | Description           |
|--------------------------|-----------|--------------|-----------------------|
| `acp_get_architecture`   | Query     | SHOULD       | Project overview      |
| `acp_get_file_context`   | Query     | SHOULD       | File metadata         |
| `acp_get_symbol_context` | Query     | SHOULD       | Symbol with relations |
| `acp_get_domain_files`   | Query     | SHOULD       | Domain files          |
| `acp_check_constraints`  | Query     | **MUST**     | Constraint check      |
| `acp_get_hotpaths`       | Query     | MAY          | Critical symbols      |
| `acp_generate_primer`    | Query     | SHOULD       | Context primer        |
| **`acp_query`**          | Query     | **MUST**     | Unified query         |
| **`acp_expand`**         | Transform | **MUST**     | Variable expansion    |
| **`acp_debug`**          | Mutation  | **MUST**     | Debug sessions        |
| **`acp_hack`**           | Mutation  | SHOULD       | Hack tracking         |
| `acp_expand_variable`    | Transform | Deprecated   | Use `acp_expand`      |

---

## Appendix B: JSON Schema Bundle

Complete JSON Schema for all new tools is available at:
`https://acp-protocol.dev/schemas/v1/mcp-tools.schema.json`

---

*End of RFC-0013*
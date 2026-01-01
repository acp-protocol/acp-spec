# Query Interface Specification

**ACP Version**: 1.3.0
**Document Version**: 1.1.0
**Last Updated**: 2026-01-01
**Status**: RFC-0015 Compliant

---

## Table of Contents

1. [Overview](#1-overview)
2. [jq Queries](#2-jq-queries)
3. [CLI Queries](#3-cli-queries)
4. [MCP Server Interface](#4-mcp-server-interface)
5. [Programmatic Access](#5-programmatic-access)
6. [Query Patterns](#6-query-patterns)
7. [Performance Considerations](#7-performance-considerations)
8. [Query vs Context](#8-query-vs-context) *(RFC-0015)*

---

## 1. Overview

### 1.1 Purpose

The query interface provides flexible ways to access ACP metadata from the cache file. Queries enable:

- **AI Context Retrieval** — Get relevant code information for AI prompts
- **Constraint Checking** — Verify rules before modifying code
- **Navigation** — Find related code across domains and call graphs
- **Analysis** — Understand codebase structure and statistics

### 1.2 Query Methods

ACP supports three query interfaces:

| Interface | Use Case | Conformance Level |
|-----------|----------|-------------------|
| **jq** | Ad-hoc queries, scripting, universal access | Level 1+ |
| **CLI** | Developer tooling, automation | Level 2+ |
| **MCP** | AI assistant integration | Level 3 |

### 1.3 Cache Structure Reference

Queries operate on the cache file structure:

```json
{
  "version": "1.0.0",
  "generated_at": "2024-12-18T15:30:00Z",
  "project": { ... },
  "stats": { ... },
  "files": { "<path>": { ... } },
  "symbols": { "<qualified_name>": { ... } },
  "graph": { "forward": { ... }, "reverse": { ... } },
  "domains": { "<name>": { ... } },
  "constraints": { "by_file": { ... }, "by_lock_level": { ... } }
}
```

### 1.4 Conformance

The key words "MUST", "MUST NOT", "REQUIRED", "SHALL", "SHALL NOT", "SHOULD", "SHOULD NOT", "RECOMMENDED", "MAY", and "OPTIONAL" in this document are to be interpreted as described in [RFC 2119](https://datatracker.ietf.org/doc/html/rfc2119).

---

## 2. jq Queries

The cache is standard JSON, directly queryable with [jq](https://jqlang.github.io/jq/).

### 2.1 Basic Queries

#### List All Files

```bash
jq '.files | keys' .acp.cache.json
```

**Output:**
```json
[
  "src/auth/session.ts",
  "src/auth/jwt.ts",
  "src/utils/helpers.ts"
]
```

#### Get File Metadata

```bash
jq '.files["src/auth/session.ts"]' .acp.cache.json
```

**Output:**
```json
{
  "path": "src/auth/session.ts",
  "module": "Session Service",
  "summary": "User session management",
  "lines": 245,
  "language": "typescript",
  "domains": ["authentication"],
  "layer": "service",
  "stability": "stable"
}
```

#### Get Symbol Details

```bash
jq '.symbols["src/auth/session.ts:SessionService.validateSession"]' .acp.cache.json
```

**Output:**
```json
{
  "name": "validateSession",
  "qualified_name": "src/auth/session.ts:SessionService.validateSession",
  "type": "method",
  "file": "src/auth/session.ts",
  "lines": [45, 89],
  "signature": "(token: string) => Promise<Session | null>",
  "summary": "Validates JWT and returns session",
  "exported": true,
  "async": true
}
```

### 2.2 Constraint Queries

#### Check File Lock Level

```bash
jq '.constraints.by_file["src/auth/session.ts"].lock_level' .acp.cache.json
```

**Output:**
```json
"restricted"
```

#### Get All Frozen Files

```bash
jq '.constraints.by_lock_level.frozen[]' .acp.cache.json
```

**Output:**
```json
"src/config/production.ts"
"src/security/keys.ts"
```

#### Get All Restricted Files

```bash
jq '.constraints.by_lock_level.restricted' .acp.cache.json
```

**Output:**
```json
[
  "src/auth/session.ts",
  "src/billing/payment.ts"
]
```

#### Get Full Constraints for a File

```bash
jq '.constraints.by_file["src/auth/session.ts"]' .acp.cache.json
```

**Output:**
```json
{
  "lock_level": "restricted",
  "lock_reason": "Security critical",
  "style": "google-typescript",
  "quality": ["security-review"]
}
```

### 2.3 Domain Queries

#### List All Domains

```bash
jq '.domains | keys' .acp.cache.json
```

**Output:**
```json
["authentication", "billing", "database", "api"]
```

#### Get Domain Details

```bash
jq '.domains["authentication"]' .acp.cache.json
```

**Output:**
```json
{
  "name": "authentication",
  "description": "User authentication and session management",
  "files": ["src/auth/session.ts", "src/auth/jwt.ts"],
  "symbols": [
    "src/auth/session.ts:SessionService.validateSession",
    "src/auth/jwt.ts:verifyToken"
  ]
}
```

#### Get Files in a Domain

```bash
jq '.domains["authentication"].files[]' .acp.cache.json
```

**Output:**
```
"src/auth/session.ts"
"src/auth/jwt.ts"
```

### 2.4 Call Graph Queries

#### Get What a Function Calls (Callees)

```bash
jq '.graph.forward["src/auth/session.ts:SessionService.validateSession"]' .acp.cache.json
```

**Output:**
```json
[
  "src/auth/jwt.ts:verifyToken",
  "src/db/sessions.ts:findSession"
]
```

#### Get What Calls a Function (Callers)

```bash
jq '.graph.reverse["src/auth/jwt.ts:verifyToken"]' .acp.cache.json
```

**Output:**
```json
[
  "src/auth/session.ts:SessionService.validateSession",
  "src/api/middleware.ts:authMiddleware"
]
```

### 2.5 Search and Filter Queries

#### Find Symbols by Name Pattern

```bash
jq '.symbols | to_entries | map(select(.key | contains("validate"))) | .[].key' .acp.cache.json
```

**Output:**
```
"src/auth/session.ts:SessionService.validateSession"
"src/utils/validation.ts:validateEmail"
```

#### Get All TypeScript Files

```bash
jq '.files | to_entries | map(select(.value.language == "typescript")) | .[].key' .acp.cache.json
```

#### Get All Async Functions

```bash
jq '.symbols | to_entries | map(select(.value.async == true)) | .[].key' .acp.cache.json
```

#### Get Exported Symbols Only

```bash
jq '.symbols | to_entries | map(select(.value.exported == true)) | .[].value.name' .acp.cache.json
```

#### Find Files in a Layer

```bash
jq '.files | to_entries | map(select(.value.layer == "service")) | .[].key' .acp.cache.json
```

### 2.6 Statistics Queries

#### Get Codebase Stats

```bash
jq '.stats' .acp.cache.json
```

**Output:**
```json
{
  "files": 127,
  "symbols": 523,
  "lines": 15420,
  "annotation_coverage": 45.2
}
```

#### Count Symbols by Type

```bash
jq '.symbols | to_entries | group_by(.value.type) | map({type: .[0].value.type, count: length})' .acp.cache.json
```

**Output:**
```json
[
  {"type": "function", "count": 234},
  {"type": "method", "count": 189},
  {"type": "class", "count": 67},
  {"type": "const", "count": 33}
]
```

#### Count Files by Language

```bash
jq '.files | to_entries | group_by(.value.language) | map({language: .[0].value.language, count: length})' .acp.cache.json
```

### 2.7 Advanced Queries

#### Get Full Call Chain (2 Levels Deep)

```bash
jq --arg sym "src/auth/session.ts:SessionService.validateSession" '
  .graph.forward[$sym] as $callees |
  [$callees[] | . as $c | {callee: $c, calls: .graph.forward[$c]}]
' .acp.cache.json
```

#### Find Unrestricted Files in Security Domain

```bash
jq '
  .domains["authentication"].files as $auth_files |
  .constraints.by_file | to_entries |
  map(select(.key | IN($auth_files[]))) |
  map(select(.value.lock_level == "normal" or .value.lock_level == null)) |
  .[].key
' .acp.cache.json
```

#### Get Summary Report

```bash
jq '{
  project: .project.name,
  files: .stats.files,
  symbols: .stats.symbols,
  domains: (.domains | keys),
  frozen_count: (.constraints.by_lock_level.frozen | length),
  restricted_count: (.constraints.by_lock_level.restricted | length)
}' .acp.cache.json
```

---

## 3. CLI Queries

Level 2+ implementations SHOULD provide a command-line interface for queries.

### 3.1 Query Subcommands

#### Query Symbol

```bash
acp query symbol <name>
```

**Example:**
```bash
acp query symbol validateSession
```

**Output:**
```json
{
  "name": "validateSession",
  "qualified_name": "src/auth/session.ts:SessionService.validateSession",
  "type": "method",
  "file": "src/auth/session.ts",
  "lines": [45, 89],
  "signature": "(token: string) => Promise<Session | null>",
  "summary": "Validates JWT and returns session"
}
```

#### Query File

```bash
acp query file <path>
```

**Example:**
```bash
acp query file src/auth/session.ts
```

#### Query Callers

```bash
acp query callers <symbol>
```

**Example:**
```bash
acp query callers "src/auth/jwt.ts:verifyToken"
```

**Output:**
```
src/auth/session.ts:SessionService.validateSession
src/api/middleware.ts:authMiddleware
```

#### Query Callees

```bash
acp query callees <symbol>
```

**Example:**
```bash
acp query callees "src/auth/session.ts:SessionService.validateSession"
```

**Output:**
```
src/auth/jwt.ts:verifyToken
src/db/sessions.ts:findSession
```

#### List Domains

```bash
acp query domains
```

**Output:**
```
authentication: 5 files, 23 symbols
billing: 8 files, 45 symbols
database: 12 files, 67 symbols
api: 15 files, 89 symbols
```

#### Query Domain

```bash
acp query domain <name>
```

**Example:**
```bash
acp query domain authentication
```

#### Show Statistics

```bash
acp query stats
```

**Output:**
```
Files: 127
Symbols: 523
Lines: 15420
Coverage: 45.2%
Domains: 4
Layers: 6
```

### 3.2 Constraints Command

The `acp constraints` command is specifically designed for checking constraints before modifications.

#### Check File Constraints

```bash
acp constraints <path>
```

**Example:**
```bash
acp constraints src/auth/session.ts
```

**Output:**
```
File: src/auth/session.ts
Lock Level: restricted
Lock Reason: Security critical
Style: google-typescript
Quality Requirements:
  - security-review

⚠ This file requires approval before modification.
```

#### Check with JSON Output

```bash
acp constraints src/auth/session.ts --json
```

**Output:**
```json
{
  "file": "src/auth/session.ts",
  "lock_level": "restricted",
  "lock_reason": "Security critical",
  "style": "google-typescript",
  "quality": ["security-review"],
  "can_modify": false,
  "approval_needed": true
}
```

### 3.3 Direct jq Passthrough

Some implementations MAY support direct jq expression passthrough:

```bash
acp query '.domains | keys'
acp query '.constraints.by_lock_level.frozen'
```

### 3.4 Output Formats

Implementations SHOULD support multiple output formats:

| Flag | Format |
|------|--------|
| `--json` | JSON (default for programmatic use) |
| `--pretty` | Pretty-printed JSON (default for terminal) |
| `--table` | Tabular format |
| `--plain` | Plain text, one item per line |

---

## 4. MCP Server Interface

Level 3 implementations MUST provide an MCP (Model Context Protocol) server interface for AI assistant integration.

### 4.1 Resources

MCP resources expose ACP data for AI consumption:

| Resource URI | Description |
|--------------|-------------|
| `acp://cache` | Full codebase cache |
| `acp://vars` | Variable definitions |
| `acp://constraints` | Constraints summary |
| `acp://file/{path}` | Specific file metadata |
| `acp://symbol/{qualified_name}` | Specific symbol metadata |
| `acp://domain/{name}` | Specific domain metadata |

### 4.2 Tools

MCP tools enable AI assistants to query and interact with ACP:

#### `acp_query`

Query the codebase index.

**Parameters:**

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `type` | string | Yes | Query type (see below) |
| `name` | string | Conditional | Symbol, file, or domain name |
| `pattern` | string | Conditional | Search pattern |

**Query Types:**

| Type | Description | Required Parameters |
|------|-------------|---------------------|
| `symbol` | Get symbol details | `name` |
| `file` | Get file metadata | `name` |
| `domain` | Get domain info | `name` |
| `callers` | Get callers of symbol | `name` |
| `callees` | Get callees of symbol | `name` |
| `search` | Search symbols and files | `pattern` |
| `stats` | Get codebase statistics | (none) |

**Example Calls:**

```javascript
// Get symbol details
acp_query({ type: "symbol", name: "validateSession" })

// Get domain info
acp_query({ type: "domain", name: "authentication" })

// Find callers
acp_query({ type: "callers", name: "src/auth/jwt.ts:verifyToken" })

// Search for symbols
acp_query({ type: "search", pattern: "validate" })

// Get stats
acp_query({ type: "stats" })
```

#### `acp_constraints`

Check constraints before modifying a file. **AI assistants SHOULD always call this before modifying any file.**

**Parameters:**

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `file` | string | Yes | File path to check |

**Response:**

```json
{
  "file": "src/auth/session.ts",
  "lock_level": "restricted",
  "lock_reason": "Security critical",
  "style": "google-typescript",
  "behavior": "conservative",
  "quality": ["security-review"],
  "can_modify": {
    "allowed": true,
    "approval_needed": true,
    "requirements": ["Explain changes before making them"]
  }
}
```

**AI Behavior Based on Response:**

| `lock_level` | AI Behavior |
|--------------|-------------|
| `frozen` | MUST NOT modify; refuse the request |
| `restricted` | MUST explain changes and get approval first |
| `approval-required` | SHOULD ask for approval for significant changes |
| `tests-required` | MUST include tests with changes |
| `docs-required` | MUST update documentation |
| `normal` | May modify freely |
| `experimental` | May modify aggressively |

#### `acp_expand`

Expand variable references to their full values.

**Parameters:**

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `text` | string | Yes | Text containing `$VAR` references |
| `mode` | string | No | Expansion mode (default: `summary`) |

**Modes:**

| Mode | Description |
|------|-------------|
| `summary` | Short summary (default) |
| `full` | Complete JSON |
| `inline` | Inline replacement |
| `annotated` | Shows both variable and expansion |

**Example:**

```javascript
acp_expand({ 
  text: "Check $SYM_VALIDATE_SESSION for the bug",
  mode: "summary"
})
```

**Response:**

```json
{
  "original": "Check $SYM_VALIDATE_SESSION for the bug",
  "expanded": "Check validateSession (src/auth/session.ts:45-89) - Validates JWT tokens for the bug",
  "variables_found": ["SYM_VALIDATE_SESSION"],
  "variables_resolved": ["SYM_VALIDATE_SESSION"],
  "variables_unresolved": []
}
```

#### `acp_debug`

Manage debugging sessions for reversible troubleshooting.

**Parameters:**

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `action` | string | Yes | Action to perform |
| `data` | object | Conditional | Action-specific data |

**Actions:**

| Action | Description | Data Fields |
|--------|-------------|-------------|
| `start` | Begin new session | `problem`, `hypothesis` |
| `attempt` | Log an attempt | `session_id`, `description`, `files` |
| `result` | Record outcome | `session_id`, `attempt_id`, `success`, `notes` |
| `revert` | Mark as reverted | `session_id`, `attempt_id` |
| `resolve` | Close session | `session_id`, `resolution` |
| `status` | Get session status | `session_id` |

#### `acp_hack`

Track temporary/experimental code.

**Parameters:**

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `action` | string | Yes | Action to perform |
| `data` | object | Conditional | Action-specific data |

**Actions:**

| Action | Description | Data Fields |
|--------|-------------|-------------|
| `mark` | Mark code as hack | `file`, `line`, `reason`, `ticket`, `expires` |
| `list` | List all hacks | `file` (optional), `expired_only` (optional) |
| `revert` | Remove hack marker | `id` |
| `cleanup` | Find expired hacks | (none) |

### 4.3 MCP Server Configuration

**Claude Desktop Configuration Example:**

```json
{
  "mcpServers": {
    "acp": {
      "command": "acp-mcp-server",
      "args": ["--dir", "/path/to/your/project"]
    }
  }
}
```

**Or with Node.js:**

```json
{
  "mcpServers": {
    "acp": {
      "command": "node",
      "args": ["/path/to/acp-mcp-server/dist/index.js", "--dir", "/path/to/project"]
    }
  }
}
```

---

## 5. Programmatic Access

### 5.1 JavaScript/TypeScript

```typescript
import { readFileSync } from 'fs';

// Load cache
const cache = JSON.parse(readFileSync('.acp.cache.json', 'utf-8'));

// Query symbol
function getSymbol(name: string) {
  return Object.values(cache.symbols).find(s => s.name === name);
}

// Get callers
function getCallers(qualifiedName: string): string[] {
  return cache.graph.reverse[qualifiedName] || [];
}

// Check constraints
function canModify(filePath: string): { allowed: boolean; reason?: string } {
  const constraints = cache.constraints.by_file[filePath];
  if (!constraints) return { allowed: true };
  
  if (constraints.lock_level === 'frozen') {
    return { allowed: false, reason: constraints.lock_reason };
  }
  return { allowed: true };
}

// Get domain files
function getDomainFiles(domain: string): string[] {
  return cache.domains[domain]?.files || [];
}
```

### 5.2 Python

```python
import json

# Load cache
with open('.acp.cache.json') as f:
    cache = json.load(f)

# Query symbol
def get_symbol(name: str):
    for sym in cache['symbols'].values():
        if sym['name'] == name:
            return sym
    return None

# Get callers
def get_callers(qualified_name: str) -> list:
    return cache['graph']['reverse'].get(qualified_name, [])

# Check constraints
def can_modify(file_path: str) -> tuple[bool, str | None]:
    constraints = cache['constraints']['by_file'].get(file_path)
    if not constraints:
        return True, None
    
    if constraints.get('lock_level') == 'frozen':
        return False, constraints.get('lock_reason')
    return True, None

# Get domain files
def get_domain_files(domain: str) -> list:
    return cache['domains'].get(domain, {}).get('files', [])
```

### 5.3 Rust

```rust
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Deserialize)]
struct Cache {
    files: HashMap<String, FileEntry>,
    symbols: HashMap<String, SymbolEntry>,
    graph: CallGraph,
    domains: HashMap<String, DomainEntry>,
    constraints: ConstraintIndex,
}

impl Cache {
    fn get_symbol(&self, name: &str) -> Option<&SymbolEntry> {
        self.symbols.values().find(|s| s.name == name)
    }
    
    fn get_callers(&self, qualified_name: &str) -> Vec<&str> {
        self.graph.reverse
            .get(qualified_name)
            .map(|v| v.iter().map(|s| s.as_str()).collect())
            .unwrap_or_default()
    }
    
    fn can_modify(&self, file_path: &str) -> (bool, Option<&str>) {
        match self.constraints.by_file.get(file_path) {
            Some(c) if c.lock_level == Some("frozen".into()) => {
                (false, c.lock_reason.as_deref())
            }
            _ => (true, None)
        }
    }
}
```

---

## 6. Query Patterns

### 6.1 AI Workflow Pattern

Standard pattern for AI assistants working with ACP:

```
1. RECEIVE task from user
2. QUERY relevant context:
   - acp_query(type="domain", name=<relevant_domain>)
   - acp_query(type="search", pattern=<keywords>)
3. CHECK constraints before any modification:
   - acp_constraints(file=<target_file>)
4. RESPECT constraint response:
   - If frozen: refuse modification
   - If restricted: explain changes, request approval
   - If tests-required: include tests
5. PROCEED with task respecting constraints
```

### 6.2 Navigation Pattern

Finding related code:

```bash
# Start with a symbol
acp query symbol validateSession

# Find what it calls
acp query callees "src/auth/session.ts:SessionService.validateSession"

# Find what calls it
acp query callers "src/auth/session.ts:SessionService.validateSession"

# Explore the domain
acp query domain authentication
```

### 6.3 Audit Pattern

Checking codebase security posture:

```bash
# Find all frozen files
jq '.constraints.by_lock_level.frozen' .acp.cache.json

# Find restricted files without lock reasons
jq '.constraints.by_file | to_entries | map(select(.value.lock_level == "restricted" and .value.lock_reason == null)) | .[].key' .acp.cache.json

# Find files in auth domain without constraints
jq --argjson auth_files '["src/auth/session.ts", "src/auth/jwt.ts"]' '
  .constraints.by_file | keys as $constrained |
  $auth_files | map(select(. as $f | $constrained | index($f) | not))
' .acp.cache.json
```

### 6.4 Impact Analysis Pattern

Understanding change impact:

```bash
# Find all callers (direct)
jq '.graph.reverse["src/auth/jwt.ts:verifyToken"]' .acp.cache.json

# Find all callers (2 levels)
jq '
  .graph.reverse["src/auth/jwt.ts:verifyToken"] as $direct |
  $direct + [$direct[] as $d | .graph.reverse[$d][]] | unique
' .acp.cache.json
```

---

## 7. Performance Considerations

### 7.1 Cache Size Guidelines

| Codebase Size | Typical Cache Size | Load Time |
|---------------|-------------------|-----------|
| Small (<100 files) | <100 KB | <10ms |
| Medium (100-1000 files) | 100 KB - 1 MB | 10-100ms |
| Large (1000-10000 files) | 1 MB - 10 MB | 100ms-1s |
| Very Large (>10000 files) | >10 MB | >1s |

### 7.2 Query Optimization

**For frequent queries:**
- Load cache once, reuse in memory
- Index by frequently-queried fields
- Use streaming parsers for very large caches

**For jq:**
- Avoid repeated file reads
- Use `--slurp` for multiple queries
- Consider `jq` with `-c` for compact output in scripts

### 7.3 Staleness Handling

Queries return stale data if the cache is outdated. Implementations SHOULD:

1. Check `generated_at` against source file modification times
2. Compare `git_commit` with current HEAD
3. Warn users if cache appears stale
4. Provide `--force` flag to rebuild cache

---

## 8. Query vs Context

*(RFC-0015)*

RFC-0015 introduced the `acp context` command which complements but is distinct from query operations.

### 8.1 Distinction

| Aspect | `acp query *` | `acp context *` |
|--------|---------------|-----------------|
| **Purpose** | Data retrieval from cache | Operation-specific AI context |
| **Focus** | Specific cache fields | Holistic context for a task |
| **Output** | Raw data structures | Formatted guidance |
| **Use Case** | Scripting, tooling | AI agent bootstrap |

### 8.2 When to Use Each

**Use `acp query`** when:
- Retrieving specific file, symbol, or domain data
- Building scripts or automation
- Analyzing cache structure
- Custom tooling integration

**Use `acp context`** when:
- Preparing AI agents for file creation, modification, or debugging
- Need holistic context including constraints, dependencies, and conventions
- Want pre-formatted output optimized for AI consumption

### 8.3 Command Comparison

```bash
# Query: Get raw file data
acp query file src/auth/login.ts

# Context: Get modification context (includes constraints, importers, etc.)
acp context modify --file src/auth/login.ts
```

**Query output** returns the raw cache entry with all indexed fields.

**Context output** returns a curated view optimized for the specific operation, including:
- Lock level and reason
- Files that import this file (`imported_by`)
- Related symbols and dependencies
- Style and behavior constraints

See [Chapter 14: Bootstrap & AI Integration](14-bootstrap.md) for complete `acp context` documentation.

---

## Appendix A: jq Quick Reference

### Common Patterns

| Goal | Query |
|------|-------|
| List all files | `jq '.files \| keys'` |
| Get file info | `jq '.files["path/to/file.ts"]'` |
| List all symbols | `jq '.symbols \| keys'` |
| Get symbol info | `jq '.symbols["qualified:name"]'` |
| List domains | `jq '.domains \| keys'` |
| Get domain files | `jq '.domains["name"].files'` |
| Find callers | `jq '.graph.reverse["symbol"]'` |
| Find callees | `jq '.graph.forward["symbol"]'` |
| Get frozen files | `jq '.constraints.by_lock_level.frozen'` |
| Check lock level | `jq '.constraints.by_file["path"].lock_level'` |
| Get stats | `jq '.stats'` |

### Filtering Patterns

| Goal | Query |
|------|-------|
| Files by language | `jq '.files \| to_entries \| map(select(.value.language == "typescript"))'` |
| Symbols by type | `jq '.symbols \| to_entries \| map(select(.value.type == "function"))'` |
| Exported only | `jq '.symbols \| to_entries \| map(select(.value.exported))'` |
| Async functions | `jq '.symbols \| to_entries \| map(select(.value.async))'` |
| By name pattern | `jq '.symbols \| to_entries \| map(select(.key \| contains("validate")))'` |

---

## Appendix B: Related Documents

- [Cache Format](03-cache-format.md) — Cache file structure
- [Variables](07-variables.md) — Variable expansion
- [Constraints](06-constraints.md) — Constraint definitions
- [Introduction](01-introduction.md) — Overview and workflows
- [Bootstrap & AI Integration](14-bootstrap.md) — `acp context` command and AI workflows (RFC-0015)
- [Tool Integration](11-tool-integration.md) — Primer system and tiered output (RFC-0015)
- [Terminology](02-terminology.md) — Definitions

---

*End of Query Interface Specification*
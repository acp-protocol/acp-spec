# Minimal ACP Example

This document demonstrates the minimum viable ACP setup. Use this as a starting point for new projects.

---

## Overview

A minimal ACP setup requires:
1. At least one source file (annotations optional)
2. A generated cache file (`.acp.cache.json`)

That's it. No config file, no variables file, no annotations needed to start.

---

## Source File

A simple TypeScript file with basic annotations:

**`src/app.ts`**
```typescript
/**
 * @acp:module "Main Application"
 * @acp:summary Application entry point
 */

export function main(): void {
  console.log("Hello, ACP!");
}
```

---

## Generated Cache

Running `acp index` produces this minimal cache:

**`.acp.cache.json`**
```json
{
  "version": "1.0.0",
  "generated_at": "2024-12-18T12:00:00Z",
  "project": {
    "name": "minimal-example",
    "root": "/home/user/minimal-example"
  },
  "stats": {
    "files": 1,
    "symbols": 1,
    "lines": 8
  },
  "source_files": {
    "src/app.ts": "2024-12-18T11:55:00Z"
  },
  "files": {
    "src/app.ts": {
      "path": "src/app.ts",
      "module": "Main Application",
      "summary": "Application entry point",
      "lines": 8,
      "language": "typescript",
      "domains": [],
      "layer": null,
      "stability": null,
      "exports": ["src/app.ts:main"],
      "imports": []
    }
  },
  "symbols": {
    "src/app.ts:main": {
      "name": "main",
      "qualified_name": "src/app.ts:main",
      "type": "function",
      "file": "src/app.ts",
      "lines": [7, 9],
      "signature": "() => void",
      "summary": null,
      "async": false,
      "exported": true,
      "visibility": "public",
      "calls": [],
      "called_by": []
    }
  },
  "graph": {
    "forward": {},
    "reverse": {}
  },
  "domains": {},
  "constraints": {
    "by_file": {},
    "by_lock_level": {}
  }
}
```

---

## Even More Minimal

You can have a valid cache with **zero annotations**. The indexer extracts structure from code:

**`src/utils.ts`** (no annotations)
```typescript
export function add(a: number, b: number): number {
  return a + b;
}

export function multiply(a: number, b: number): number {
  return a * b;
}
```

**Generated cache entry:**
```json
{
  "files": {
    "src/utils.ts": {
      "path": "src/utils.ts",
      "module": null,
      "summary": null,
      "lines": 8,
      "language": "typescript",
      "domains": [],
      "layer": null,
      "stability": null,
      "exports": ["src/utils.ts:add", "src/utils.ts:multiply"],
      "imports": []
    }
  },
  "symbols": {
    "src/utils.ts:add": {
      "name": "add",
      "qualified_name": "src/utils.ts:add",
      "type": "function",
      "file": "src/utils.ts",
      "lines": [1, 3],
      "signature": "(a: number, b: number) => number",
      "summary": null,
      "async": false,
      "exported": true,
      "visibility": "public",
      "calls": [],
      "called_by": []
    },
    "src/utils.ts:multiply": {
      "name": "multiply",
      "qualified_name": "src/utils.ts:multiply",
      "type": "function",
      "file": "src/utils.ts",
      "lines": [5, 7],
      "signature": "(a: number, b: number) => number",
      "summary": null,
      "async": false,
      "exported": true,
      "visibility": "public",
      "calls": [],
      "called_by": []
    }
  }
}
```

---

## Basic Queries

Even with minimal setup, you can query:

```bash
# List all files
jq '.files | keys' .acp.cache.json

# Get file info
jq '.files["src/app.ts"]' .acp.cache.json

# List all symbols
jq '.symbols | keys' .acp.cache.json

# Get stats
jq '.stats' .acp.cache.json
```

---

## Adding First Constraint

Add your first constraint to protect critical code:

**`src/config.ts`**
```typescript
/**
 * @acp:module "Configuration"
 * @acp:lock frozen
 * @acp:lock-reason "Production configuration - do not modify"
 */

export const DATABASE_URL = process.env.DATABASE_URL;
export const API_KEY = process.env.API_KEY;
```

Now queries can check constraints:

```bash
# Check if file is frozen
jq '.constraints.by_file["src/config.ts"].lock_level' .acp.cache.json
# Output: "frozen"

# List all frozen files
jq '.constraints.by_lock_level.frozen' .acp.cache.json
# Output: ["src/config.ts"]
```

---

## Next Steps

From this minimal setup, you can:

1. **Add more annotations** — See [Complete Example](complete.md)
2. **Add configuration** — Create `.acp.config.json` for custom settings
3. **Add variables** — Run `acp vars` to generate `.acp.vars.json`
4. **Set up MCP** — Enable AI assistant integration

---

## File Structure

Minimal project structure:

```
my-project/
├── src/
│   └── app.ts              # Source with optional annotations
└── .acp.cache.json         # Generated (add to .gitignore)
```

With optional files:

```
my-project/
├── src/
│   ├── app.ts
│   └── config.ts
├── .acp.cache.json         # Generated
├── .acp.config.json        # Optional: configuration
└── .acp.vars.json          # Optional: variables (generated)
```

---

## Quick Start Commands

```bash
# Initialize (creates .acp.config.json with defaults)
acp init

# Index codebase (creates .acp.cache.json)
acp index

# Index with variables (also creates .acp.vars.json)
acp index --vars

# Validate cache
acp validate .acp.cache.json
```

---

*See [Complete Example](complete.md) for a full-featured ACP setup.*
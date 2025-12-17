# File Discovery Specification

**ACP Version**: 1.0.0-revised
**Document Version**: 1.0.0
**Last Updated**: 2024-12-17
**Status**: Revised Draft

---

## Table of Contents

1. [Overview](#1-overview)
2. [Discovery Algorithm](#2-discovery-algorithm)
3. [Exclusion Patterns](#3-exclusion-patterns)
4. [Cache Building Details](#4-cache-building-details)
5. [Language Detection](#5-language-detection)
6. [Implementation Limits](#6-implementation-limits)

---

## 1. Overview

### 1.1 Purpose

File discovery is the process of finding and indexing source files in a project to build the ACP cache. This specification defines:

- How files are discovered
- What files are included/excluded
- How the cache is built from source files
- How languages are detected
- What limits implementations should respect

**From main specification Section 8 (Lines 1054-1232):**

### 1.2 Design Principles

- **Configurable**: Include/exclude patterns can be customized
- **Efficient**: Smart exclusions avoid scanning unnecessary files
- **Deterministic**: Same project state produces same cache
- **Language-Aware**: Proper detection of programming languages

### 1.3 Conformance

The key words "MUST", "MUST NOT", "REQUIRED", "SHALL", "SHALL NOT", "SHOULD", "SHOULD NOT", "RECOMMENDED", "MAY", and "OPTIONAL" in this document are to be interpreted as described in [RFC 2119](https://datatracker.ietf.org/doc/html/rfc2119).

---

## 2. Discovery Algorithm

From main specification Section 8.1 (Lines 1056-1067):

### 2.1 Basic Algorithm

```
FUNCTION discoverFiles(projectRoot, config):
  1. Start from project root (containing .acp.config.json or first .acp.cache.json)
  2. Recursively scan directories
  3. For each file:
     a. Check if matches include patterns (default: all files)
     b. Check if matches exclude patterns
     c. If included and not excluded, add to processing list
  4. Parse annotations from each file
  5. Build cache structure

  RETURN list of files to process
```

### 2.2 Step-by-Step Process

#### Step 1: Find Project Root

**Project root** is determined by:
1. Directory containing `.acp.config.json`, OR
2. Directory containing `.acp.cache.json`, OR
3. Current working directory (if neither exists)

#### Step 2: Load Configuration

Load `.acp.config.json` if present:
- Read `include` patterns
- Read `exclude` patterns
- Read other configuration

If no config file, use defaults.

#### Step 3: Scan Directories

Starting from project root:
```
FOR each directory in tree:
  IF directory matches exclude pattern:
    Skip entire directory
  ELSE:
    FOR each file in directory:
      IF file matches include AND NOT matches exclude:
        Add to file list
```

#### Step 4: Process Files

For each discovered file:
1. Detect language (Section 5)
2. Parse annotations (see [Annotation Syntax](annotations.md))
3. Extract symbols
4. Build file entry
5. Build symbol entries

#### Step 5: Build Indexes

After processing all files:
1. Build domain index (Section 4.1)
2. Build call graph (Section 4.3)
3. Build constraint index
4. Calculate statistics

---

## 3. Exclusion Patterns

From main specification Section 8.2 (Lines 1068-1085):

### 3.1 Default Exclusions

```json
{
  "exclude": [
    "node_modules/**",
    ".git/**",
    "dist/**",
    "build/**",
    "coverage/**",
    "**/*.test.*",
    "**/*.spec.*"
  ]
}
```

### 3.2 Pattern Syntax

Uses glob syntax:
- `**` - Match any number of directories
- `*` - Match any characters except `/`
- `?` - Match single character
- `{a,b}` - Match either a or b

**Examples:**
- `node_modules/**` - Everything in node_modules
- `**/*.test.ts` - All .test.ts files anywhere
- `dist/**/*.js` - All .js files in dist/ and subdirs
- `src/**/*.{ts,tsx}` - TypeScript files in src/

### 3.3 Custom Exclusions

Add to `.acp.config.json`:

```json
{
  "exclude": [
    "**/*.test.ts",
    "**/*.spec.ts",
    "node_modules/**",
    "generated/**",
    "vendor/**",
    "third-party/**"
  ]
}
```

### 3.4 Precedence

- Exclude patterns take precedence over include patterns
- If a file matches both include and exclude, it is excluded

---

## 4. Cache Building Details

From main specification Section 8.3 (Lines 1086-1159):

### 4.1 Domain Detection

**From specification Lines 1090-1117:**

**Algorithm:**
1. Check for `@acp:domain` annotation (Priority 1)
2. Check directory patterns in config (Priority 2)
3. Analyze imports to infer domain (Priority 3)
4. Leave unclassified if inconclusive

**Directory Pattern Example** (in `.acp.config.json`):
```json
{
  "domains": {
    "authentication": {
      "patterns": ["src/auth/**", "lib/security/**"]
    },
    "database": {
      "patterns": ["src/db/**", "src/models/**"]
    }
  }
}
```

**Import Analysis:**
- If file imports primarily from one domain, classify in that domain
- Threshold: >60% of imports from single domain

**Example:**
```typescript
// src/services/user-auth.ts
import { validateToken } from '../auth/token';
import { createSession } from '../auth/session';
import { logAction } from '../utils/logger';

// 2 of 3 imports from auth/ -> classify as "authentication" domain
```

### 4.2 Layer Detection

**From specification Lines 1118-1135:**

**Algorithm:**
1. Check for `@acp:layer` annotation (Priority 1)
2. Check directory naming conventions (Priority 2)
3. Analyze dependencies to infer layer (Priority 3)
4. Default to null if inconclusive

**Directory Naming Conventions:**

| Pattern | Layer |
|---------|-------|
| `**/handlers/**`, `**/routes/**` | handler |
| `**/services/**`, `**/business/**` | service |
| `**/repositories/**`, `**/data/**` | repository |
| `**/models/**`, `**/entities/**` | model |
| `**/utils/**`, `**/helpers/**` | utility |

**Example:**
```
src/
  handlers/    → layer: handler
  services/    → layer: service
  repositories/→ layer: repository
  models/      → layer: model
  utils/       → layer: utility
```

### 4.3 Call Graph Construction

**From specification Lines 1136-1159:**

**Algorithm:**
1. Use static analysis to identify function calls
2. Exclude standard library calls (configurable)
3. Build forward map: caller → [callees]
4. Build reverse map: callee → [callers]
5. Handle indirect calls conservatively (include if detectable)

**Limitations:**
- Dynamic calls may not be detected
- Reflection/metaprogramming not tracked
- Cross-language calls require explicit annotation

**Configuration** (in `.acp.config.json`):
```json
{
  "call_graph": {
    "include_stdlib": false,
    "max_depth": null,
    "exclude_patterns": ["**/test/**"]
  }
}
```

**Example:**
```typescript
// Detected calls
function validateUser(user) {
  hashPassword(user.password);  // ✓ Detected
  db.query("SELECT...");        // ✓ Detected
}

// Not detected
function dynamicCall(fnName) {
  this[fnName]();  // ✗ Dynamic - not detected
}
```

---

## 5. Language Detection

From main specification Section 8.4 (Lines 1161-1194):

### 5.1 Extension Mapping

Files are classified by extension:

| Extension(s) | Language |
|--------------|----------|
| `.ts`, `.tsx`, `.mts`, `.cts` | typescript |
| `.js`, `.jsx`, `.mjs`, `.cjs` | javascript |
| `.py`, `.pyw`, `.pyi` | python |
| `.rs` | rust |
| `.go` | go |
| `.java` | java |
| `.cs` | c-sharp |
| `.rb` | ruby |
| `.php` | php |
| `.cpp`, `.cc`, `.cxx`, `.hpp` | cpp |
| `.c`, `.h` | c |
| `.swift` | swift |
| `.kt`, `.kts` | kotlin |

### 5.2 Ambiguous Extensions

**From specification Lines 1183-1189:**

| Extension | Check For | If Found | Else |
|-----------|-----------|----------|------|
| `.h` | `#include <iostream>` or C++ keywords | cpp | c |
| `.m` | `@interface`, `@implementation` | objective-c | (error: unknown) |

**Example `.h` file detection:**
```c
// If contains C++ keywords:
#include <iostream>
class MyClass { };  // Detected as cpp

// Otherwise:
#include <stdio.h>
struct Data { };    // Detected as c
```

### 5.3 Unknown Extensions

**From specification Lines 1190-1194:**

- Emit warning
- Skip file in permissive mode
- Error in strict mode

**Example:**
```
WARNING: Unknown file extension .xyz for file: src/custom.xyz
  Skipping file (use strictness: strict to error instead)
```

---

## 6. Implementation Limits

From main specification Section 8.5 (Lines 1196-1232):

### 6.1 Default Limits

Implementations SHOULD respect these limits:

| Limit | Default | Rationale |
|-------|---------|-----------|
| Max source file size | 10 MB | Prevent parser hang, memory issues |
| Max files in project | 100,000 | Performance, memory |
| Max annotations per file | 1,000 | Performance |
| Max symbols per file | 10,000 | Performance, cache size |
| Max cache file size | 100 MB | Memory, network transfer |
| Max variable expansion depth | 10 | Circular reference protection |
| Max inheritance depth | 4 | Complexity management |

### 6.2 Configuration

Limits SHOULD be configurable in `.acp.config.json`:

```json
{
  "limits": {
    "max_file_size_mb": 10,
    "max_files": 100000,
    "max_annotations_per_file": 1000,
    "max_cache_size_mb": 100
  }
}
```

### 6.3 Behavior When Exceeded

**Permissive mode** (default):
- Warn
- Skip offending item
- Continue processing

**Strict mode**:
- Error
- Abort processing

**Example:**
```
WARNING: File src/generated/huge.ts exceeds size limit (15MB > 10MB)
  Skipping file. To include, increase limits.max_file_size_mb in config.
```

### 6.4 Large Projects

For projects exceeding limits, consider:
- Exclude generated files
- Separate into multiple ACP projects
- Increase limits (with caution)

**Example for monorepo:**
```json
{
  "exclude": [
    "**/generated/**",
    "**/vendor/**",
    "**/__pycache__/**"
  ],
  "limits": {
    "max_files": 500000,
    "max_cache_size_mb": 500
  }
}
```

---

## Appendix A: Complete Discovery Example

```
1. Find project root:
   /home/user/my-project/ (contains .acp.config.json)

2. Load config:
   include: ["src/**/*.ts"]
   exclude: ["**/*.test.ts", "node_modules/**"]

3. Scan directories:
   /home/user/my-project/
   ├── src/
   │   ├── auth/
   │   │   ├── session.ts       ✓ Include
   │   │   └── session.test.ts  ✗ Exclude (matches exclude pattern)
   │   └── utils/
   │       └── helpers.ts       ✓ Include
   ├── node_modules/            ✗ Skip entire directory
   └── dist/                    ✗ Not in include pattern

4. Process files:
   - src/auth/session.ts
     - Detect language: typescript
     - Parse annotations
     - Extract symbols
     - Domain: authentication (from @acp:domain)

   - src/utils/helpers.ts
     - Detect language: typescript
     - Parse annotations
     - Extract symbols
     - Domain: utility (from directory pattern)

5. Build indexes:
   - Domains: { authentication: [...], utility: [...] }
   - Call graph: Forward and reverse maps
   - Constraints: Index by file and lock level
   - Stats: { files: 2, symbols: 15, lines: 450 }
```

---

## Appendix B: Related Documents

- [Configuration](config.md) - Include/exclude configuration
- [Annotation Syntax](annotations.md) - How annotations are parsed
- [Cache Format](cache.md) - Cache structure
- [Constraint System](constraints.md) - Constraint indexing

---

*End of File Discovery Specification*

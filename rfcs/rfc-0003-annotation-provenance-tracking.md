# RFC-0003: Annotation Provenance Tracking

- **RFC ID**: 0003
- **Title**: Annotation Provenance Tracking
- **Author**: David (ACP Protocol)
- **Status**: Implemented
- **Created**: 2025-12-20
- **Updated**: 2025-12-22
- **Implemented**: 2025-12-22
- **Implementation**: RFC-0005
- **Discussion**: [Pending GitHub Discussion]

---

## Summary

This RFC introduces a system for tracking the provenance (origin) of ACP annotations, enabling identification of auto-generated annotations that may need human review. It adds a grepable source marker (`@acp:source`) to annotated code, extends the cache schema to track annotation origins, and provides CLI commands for querying and refining machine-generated annotations.

## Motivation

### Problem Statement

The `acp annotate` command generates annotations from heuristics and converted documentation. Currently:

1. **No in-source identification** — Once applied, auto-generated annotations are indistinguishable from human-written ones
2. **No review workflow** — Users can't easily find annotations that need refinement
3. **No quality tracking** — Cache doesn't record which annotations are machine-generated vs. human-verified
4. **No confidence visibility** — Heuristic confidence scores are lost after generation

### Use Cases

**1. Code Review Workflow**
```bash
# Find all auto-generated annotations for review
grep -r "@acp:source heuristic" src/

# Or using ACP CLI
acp query --source heuristic --needs-review
```

**2. AI-Assisted Refinement**
```
User: "Go through all heuristic-generated summaries in src/auth/ and improve them"
AI: [Queries cache for heuristic annotations, refines each one, updates source to "refined"]
```

**3. Quality Dashboard**
```bash
acp stats --annotations
# Output:
# Total annotations: 847
#   - Explicit (human): 523 (62%)
#   - Converted (docs): 198 (23%)
#   - Heuristic (auto): 89 (11%)
#   - Refined (AI-improved): 37 (4%)
```

**4. Progressive Annotation Adoption**
- Start with `acp annotate` to bootstrap
- Gradually review and refine heuristic annotations
- Track progress toward fully human-verified codebase

### Goals

1. **Grepable source marker** — `@acp:source <origin>` annotation in source code
2. **Cache provenance tracking** — Store origin, confidence, and review status per annotation
3. **CLI query support** — Find annotations by source type
4. **Refinement workflow** — Track when annotations are reviewed/improved
5. **Backward compatible** — Annotations without `@acp:source` assumed human-written

### Non-Goals

1. **Automatic quality assessment** — Not judging if annotations are "good"
2. **Mandatory provenance** — Optional feature, not required
3. **Blame/audit trail** — Not tracking who wrote what (that's git's job)
4. **Version history** — Not tracking annotation changes over time

---

## Detailed Design

### Overview

```
┌─────────────────────────────────────────────────────────────────┐
│                     Source Code                                  │
│  ┌───────────────────────────────────────────────────────────┐  │
│  │ /**                                                       │  │
│  │  * @acp:summary "Validates user session tokens"          │  │
│  │  * @acp:source heuristic                                 │  │
│  │  * @acp:source-confidence 0.85                           │  │
│  │  */                                                       │  │
│  └───────────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│                     .acp.cache.json                              │
│  ┌───────────────────────────────────────────────────────────┐  │
│  │ "annotations": {                                          │  │
│  │   "src/auth.ts:validateSession": {                       │  │
│  │     "summary": {                                          │  │
│  │       "value": "Validates user session tokens",          │  │
│  │       "source": "heuristic",                             │  │
│  │       "confidence": 0.85,                                │  │
│  │       "needsReview": true,                               │  │
│  │       "generatedAt": "2025-12-20T10:30:00Z"             │  │
│  │     }                                                     │  │
│  │   }                                                       │  │
│  │ }                                                         │  │
│  └───────────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│                     CLI Queries                                  │
│  ┌───────────────────────────────────────────────────────────┐  │
│  │ $ acp query --source heuristic --confidence "<0.8"       │  │
│  │ $ acp annotate --refine --source heuristic               │  │
│  │ $ acp stats --provenance                                  │  │
│  └───────────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────────┘
```

---

### 1. Annotation Specifications

#### 1.1 `@acp:source` — Provenance Marker

**Scope:** Immediately follows the annotation it describes  
**Purpose:** Identify origin of an annotation

| Annotation | Parameters | Example | Description |
|------------|-----------|---------|-------------|
| `@acp:source` | `<origin>` | `@acp:source heuristic` | Origin of preceding annotation(s) |
| `@acp:source-confidence` | `<0.0-1.0>` | `@acp:source-confidence 0.85` | Confidence score |
| `@acp:source-reviewed` | `<boolean>` | `@acp:source-reviewed true` | Human review status |
| `@acp:source-id` | `<uuid>` | `@acp:source-id abc123` | Unique generation ID |

#### 1.2 Source Origin Values

| Origin | Description | Grepable Pattern |
|--------|-------------|------------------|
| `explicit` | Human-written (default if no `@acp:source`) | `@acp:source explicit` |
| `converted` | Converted from JSDoc/docstring/etc. | `@acp:source converted` |
| `heuristic` | Generated by naming/path/visibility heuristics | `@acp:source heuristic` |
| `refined` | AI-improved from previous auto-generation | `@acp:source refined` |
| `inferred` | Inferred from code analysis (future) | `@acp:source inferred` |

#### 1.3 Syntax Examples

**Single annotation with provenance:**
```javascript
/**
 * @acp:summary "Validates user authentication tokens"
 * @acp:source heuristic
 * @acp:source-confidence 0.85
 */
function validateToken(token) { ... }
```

**Multiple annotations, single provenance block:**
```javascript
/**
 * @acp:summary "Payment processing service"
 * @acp:domain billing
 * @acp:lock restricted
 * @acp:source heuristic
 * @acp:source-confidence 0.72
 */
```

**Mixed provenance (rare but supported):**
```javascript
/**
 * @acp:summary "User authentication handler"
 * @acp:source explicit
 * 
 * @acp:domain auth
 * @acp:source heuristic
 * @acp:source-confidence 0.90
 */
```

**Refined annotation:**
```javascript
/**
 * @acp:summary "Securely validates JWT tokens with RS256 signature verification"
 * @acp:source refined
 * @acp:source-reviewed true
 */
```

#### 1.4 Grammar (EBNF)

```ebnf
source_annotation    = "@acp:source" , whitespace , source_origin ;
source_origin        = "explicit" | "converted" | "heuristic" | "refined" | "inferred" ;

source_confidence    = "@acp:source-confidence" , whitespace , confidence_value ;
confidence_value     = digit , "." , digit , [ digit ] ;  (* 0.0 to 1.0 *)

source_reviewed      = "@acp:source-reviewed" , whitespace , boolean_value ;
boolean_value        = "true" | "false" ;

source_id            = "@acp:source-id" , whitespace , identifier ;
```

---

### 2. Cache Schema Additions

#### 2.1 Annotation Entry Structure

Add to `cache.schema.json` under `$defs`:

```json
{
  "$defs": {
    "annotation_provenance": {
      "type": "object",
      "properties": {
        "value": {
          "type": "string",
          "description": "The annotation value"
        },
        "source": {
          "type": "string",
          "enum": ["explicit", "converted", "heuristic", "refined", "inferred"],
          "default": "explicit",
          "description": "Origin of this annotation"
        },
        "confidence": {
          "type": "number",
          "minimum": 0.0,
          "maximum": 1.0,
          "description": "Confidence score (for auto-generated)"
        },
        "needsReview": {
          "type": "boolean",
          "default": false,
          "description": "Whether this annotation should be reviewed"
        },
        "reviewed": {
          "type": "boolean",
          "default": false,
          "description": "Whether a human has reviewed this"
        },
        "reviewedAt": {
          "type": "string",
          "format": "date-time",
          "description": "When the annotation was reviewed"
        },
        "generatedAt": {
          "type": "string",
          "format": "date-time",
          "description": "When the annotation was auto-generated"
        },
        "generationId": {
          "type": "string",
          "description": "Unique ID for this generation batch"
        },
        "previousSource": {
          "type": "string",
          "enum": ["explicit", "converted", "heuristic", "refined", "inferred"],
          "description": "Previous source before refinement"
        },
        "conversionOrigin": {
          "type": "string",
          "description": "For 'converted': the doc standard (jsdoc, docstring, etc.)"
        },
        "heuristicRule": {
          "type": "string",
          "description": "For 'heuristic': which rule generated this"
        }
      },
      "required": ["value"]
    }
  }
}
```

#### 2.2 File Entry Additions

Extend `file_entry` in cache schema:

```json
{
  "file_entry": {
    "properties": {
      "annotations": {
        "type": "object",
        "description": "Annotation provenance for this file",
        "properties": {
          "module": { "$ref": "#/$defs/annotation_provenance" },
          "summary": { "$ref": "#/$defs/annotation_provenance" },
          "domain": { "$ref": "#/$defs/annotation_provenance" },
          "layer": { "$ref": "#/$defs/annotation_provenance" },
          "lock": { "$ref": "#/$defs/annotation_provenance" },
          "style": { "$ref": "#/$defs/annotation_provenance" }
        },
        "additionalProperties": { "$ref": "#/$defs/annotation_provenance" }
      }
    }
  }
}
```

#### 2.3 Symbol Entry Additions

Extend `symbol_entry` similarly:

```json
{
  "symbol_entry": {
    "properties": {
      "annotations": {
        "type": "object",
        "description": "Annotation provenance for this symbol",
        "additionalProperties": { "$ref": "#/$defs/annotation_provenance" }
      }
    }
  }
}
```

#### 2.4 Top-Level Provenance Statistics

Add provenance summary to cache:

```json
{
  "provenance": {
    "type": "object",
    "description": "Annotation provenance statistics",
    "properties": {
      "summary": {
        "type": "object",
        "properties": {
          "total": { "type": "integer" },
          "bySource": {
            "type": "object",
            "properties": {
              "explicit": { "type": "integer" },
              "converted": { "type": "integer" },
              "heuristic": { "type": "integer" },
              "refined": { "type": "integer" },
              "inferred": { "type": "integer" }
            }
          },
          "needsReview": { "type": "integer" },
          "reviewed": { "type": "integer" },
          "averageConfidence": {
            "type": "object",
            "properties": {
              "converted": { "type": "number" },
              "heuristic": { "type": "number" }
            }
          }
        }
      },
      "lowConfidence": {
        "type": "array",
        "items": {
          "type": "object",
          "properties": {
            "target": { "type": "string" },
            "annotation": { "type": "string" },
            "confidence": { "type": "number" },
            "value": { "type": "string" }
          }
        },
        "description": "Annotations with confidence < threshold"
      },
      "lastGeneration": {
        "type": "object",
        "properties": {
          "id": { "type": "string" },
          "timestamp": { "type": "string", "format": "date-time" },
          "annotationsGenerated": { "type": "integer" },
          "filesAffected": { "type": "integer" }
        }
      }
    }
  }
}
```

---

### 3. CLI Commands

#### 3.1 `acp annotate` Enhancements

```bash
# Generate annotations WITH provenance markers (new default)
acp annotate

# Generate WITHOUT provenance markers (legacy behavior)
acp annotate --no-provenance

# Only generate for targets without existing annotations
acp annotate --no-overwrite

# Set confidence threshold for inclusion
acp annotate --min-confidence 0.7

# Generate but mark all as needing review
acp annotate --needs-review

# Dry run - show what would be generated
acp annotate --dry-run

# Refine existing heuristic annotations (AI-assisted)
acp annotate --refine --source heuristic
```

#### 3.2 `acp query` Provenance Queries

```bash
# Find all heuristic-generated annotations
acp query --source heuristic

# Find low-confidence annotations
acp query --confidence "<0.7"

# Find annotations needing review
acp query --needs-review

# Find annotations in a specific file by source
acp query --file src/auth.ts --source heuristic

# Combined: low-confidence heuristics in auth domain
acp query --source heuristic --confidence "<0.8" --domain auth

# Output as JSON for tooling
acp query --source heuristic --format json
```

#### 3.3 `acp stats` Provenance Statistics

```bash
acp stats --provenance

# Example output:
╭──────────────────────────────────────────────────────────╮
│              Annotation Provenance Statistics             │
├──────────────────────────────────────────────────────────┤
│ Total Annotations: 847                                    │
│                                                           │
│ By Source:                                                │
│   ├─ Explicit (human):     523  (61.7%)  ████████████░░  │
│   ├─ Converted (docs):     198  (23.4%)  █████░░░░░░░░░  │
│   ├─ Heuristic (auto):      89  (10.5%)  ██░░░░░░░░░░░░  │
│   └─ Refined (improved):    37  ( 4.4%)  █░░░░░░░░░░░░░  │
│                                                           │
│ Review Status:                                            │
│   ├─ Needs review:          52                            │
│   └─ Reviewed:              74                            │
│                                                           │
│ Confidence (auto-generated):                              │
│   ├─ Converted avg:        0.91                           │
│   ├─ Heuristic avg:        0.73                           │
│   └─ Below 0.7:            23 annotations                 │
╰──────────────────────────────────────────────────────────╯
```

#### 3.4 `acp review` New Command

```bash
# Interactive review of auto-generated annotations
acp review

# Review only heuristic annotations
acp review --source heuristic

# Review low-confidence annotations
acp review --confidence "<0.7"

# Review annotations in specific domain
acp review --domain auth

# Mark annotations as reviewed (bulk)
acp review --mark-reviewed --source heuristic --confidence ">0.9"
```

**Interactive Review Flow:**
```
$ acp review --source heuristic

Reviewing: src/auth/session.ts:validateSession
Current:   @acp:summary "Validates session"
Source:    heuristic (confidence: 0.72)

Actions:
  [a] Accept as-is (mark reviewed)
  [e] Edit annotation
  [r] Regenerate with AI
  [s] Skip for now
  [q] Quit review

Choice: e
New value: Validates user session tokens against Redis store
Updated and marked as reviewed.

[1/23] Next: src/auth/session.ts:refreshToken
...
```

---

### 4. Configuration Options

Add to `config.schema.json`:

```json
{
  "annotate": {
    "type": "object",
    "description": "Annotation generation settings",
    "properties": {
      "provenance": {
        "type": "object",
        "properties": {
          "enabled": {
            "type": "boolean",
            "default": true,
            "description": "Include @acp:source markers in generated annotations"
          },
          "includeConfidence": {
            "type": "boolean",
            "default": true,
            "description": "Include @acp:source-confidence markers"
          },
          "reviewThreshold": {
            "type": "number",
            "default": 0.8,
            "description": "Mark annotations below this confidence as needsReview"
          },
          "minConfidence": {
            "type": "number",
            "default": 0.5,
            "description": "Don't generate annotations below this confidence"
          }
        }
      },
      "defaults": {
        "type": "object",
        "properties": {
          "markNeedsReview": {
            "type": "boolean",
            "default": false,
            "description": "Mark all generated annotations as needing review"
          },
          "overwriteExisting": {
            "type": "boolean",
            "default": false,
            "description": "Overwrite existing annotations"
          }
        }
      }
    }
  }
}
```

---

### 5. Complete Examples

#### 5.1 Source Code Before `acp annotate`

```typescript
// src/auth/session.ts

export function validateSession(token: string): boolean {
  // ... validation logic
}

export function refreshToken(oldToken: string): string {
  // ... refresh logic  
}

export class SessionManager {
  private store: RedisClient;
  
  async getSession(id: string): Promise<Session | null> {
    // ... retrieval logic
  }
}
```

#### 5.2 After `acp annotate`

```typescript
// src/auth/session.ts

/**
 * @acp:module "Session Management"
 * @acp:domain auth
 * @acp:source heuristic
 * @acp:source-confidence 0.88
 */

/**
 * @acp:summary "Validates session tokens"
 * @acp:source heuristic
 * @acp:source-confidence 0.72
 */
export function validateSession(token: string): boolean {
  // ... validation logic
}

/**
 * @acp:summary "Refreshes authentication tokens"
 * @acp:source heuristic
 * @acp:source-confidence 0.81
 */
export function refreshToken(oldToken: string): string {
  // ... refresh logic  
}

/**
 * @acp:summary "Manages user sessions"
 * @acp:source heuristic
 * @acp:source-confidence 0.85
 */
export class SessionManager {
  private store: RedisClient;
  
  /**
   * @acp:summary "Retrieves session by ID"
   * @acp:source heuristic
   * @acp:source-confidence 0.79
   */
  async getSession(id: string): Promise<Session | null> {
    // ... retrieval logic
  }
}
```

#### 5.3 After Human Review/Refinement

```typescript
/**
 * @acp:module "Session Management"
 * @acp:summary "JWT session handling with Redis-backed storage"
 * @acp:domain auth
 * @acp:lock restricted
 * @acp:lock-reason "Security critical - requires review"
 * @acp:source explicit
 */

/**
 * @acp:summary "Validates JWT session tokens against Redis store with expiry check"
 * @acp:source refined
 * @acp:source-reviewed true
 */
export function validateSession(token: string): boolean {
  // ... validation logic
}
```

#### 5.4 Cache Output

```json
{
  "files": {
    "src/auth/session.ts": {
      "path": "src/auth/session.ts",
      "module": "Session Management",
      "summary": "JWT session handling with Redis-backed storage",
      "domain": ["auth"],
      "annotations": {
        "module": {
          "value": "Session Management",
          "source": "explicit"
        },
        "summary": {
          "value": "JWT session handling with Redis-backed storage",
          "source": "explicit"
        },
        "domain": {
          "value": "auth",
          "source": "heuristic",
          "confidence": 0.88,
          "reviewed": true,
          "reviewedAt": "2025-12-20T14:30:00Z"
        },
        "lock": {
          "value": "restricted",
          "source": "explicit"
        }
      }
    }
  },
  "symbols": {
    "src/auth/session.ts:validateSession": {
      "name": "validateSession",
      "summary": "Validates JWT session tokens against Redis store with expiry check",
      "annotations": {
        "summary": {
          "value": "Validates JWT session tokens against Redis store with expiry check",
          "source": "refined",
          "previousSource": "heuristic",
          "confidence": 0.72,
          "reviewed": true,
          "reviewedAt": "2025-12-20T14:35:00Z",
          "generatedAt": "2025-12-20T10:30:00Z"
        }
      }
    }
  },
  "provenance": {
    "summary": {
      "total": 847,
      "bySource": {
        "explicit": 523,
        "converted": 198,
        "heuristic": 89,
        "refined": 37,
        "inferred": 0
      },
      "needsReview": 52,
      "reviewed": 74,
      "averageConfidence": {
        "converted": 0.91,
        "heuristic": 0.73
      }
    },
    "lowConfidence": [
      {
        "target": "src/utils/helper.ts:processData",
        "annotation": "summary",
        "confidence": 0.52,
        "value": "Processes data"
      }
    ],
    "lastGeneration": {
      "id": "gen-20251220-103000",
      "timestamp": "2025-12-20T10:30:00Z",
      "annotationsGenerated": 126,
      "filesAffected": 34
    }
  }
}
```

#### 5.5 Grep Examples

```bash
# Find all heuristic-generated annotations
grep -rn "@acp:source heuristic" src/
# src/auth/session.ts:8:@acp:source heuristic
# src/auth/session.ts:14:@acp:source heuristic
# ...

# Find low-confidence annotations (requires parsing, but visible)
grep -rn "@acp:source-confidence 0\.[0-6]" src/
# src/utils/helper.ts:5:@acp:source-confidence 0.52

# Find reviewed annotations
grep -rn "@acp:source-reviewed true" src/

# Find all provenance markers
grep -rn "@acp:source" src/ | grep -v "source-confidence\|source-reviewed"
```

---

### 6. AI Integration Guidelines

#### 6.1 Reading Provenance

AI tools SHOULD:
- Check `@acp:source` to understand annotation origin
- Treat `heuristic` annotations as potentially needing improvement
- Preserve `@acp:source` markers when making changes
- Update source to `refined` when improving auto-generated annotations

#### 6.2 Refinement Workflow

When user requests annotation refinement:

```
1. Query cache for target annotations (by source, confidence, domain)
2. For each annotation:
   a. Read current value and context
   b. Generate improved annotation
   c. Update source: heuristic → refined
   d. Set reviewed: true
   e. Preserve generatedAt, add reviewedAt
3. Report changes made
```

#### 6.3 Primer Integration

Add to primers:

```markdown
## Annotation Provenance

Annotations may have `@acp:source` markers indicating origin:
- `explicit` — Human-written (authoritative)
- `converted` — From JSDoc/docstring (usually reliable)
- `heuristic` — Auto-generated (may need review)
- `refined` — AI-improved (reviewed)

When you see `@acp:source heuristic`, the annotation may be:
- Generic or imprecise
- Missing important context
- A good candidate for improvement

You can improve heuristic annotations by:
1. Reading the actual code
2. Writing a more specific, accurate annotation
3. Updating the source marker to `refined`
```

---

### 7. Error Handling

| Error Condition | Permissive Mode | Strict Mode |
|-----------------|-----------------|-------------|
| Invalid source value | Warn, default to `explicit` | Error, abort |
| Confidence out of range | Warn, clamp to 0.0-1.0 | Error, abort |
| Missing value for provenance | Warn, skip provenance | Error, abort |
| Conflicting provenance markers | Warn, use last | Error, abort |

---

## Drawbacks

1. **Annotation verbosity** — Provenance markers add lines to source files
2. **Grep noise** — More annotations to filter through
3. **Schema complexity** — Cache grows with provenance data
4. **Maintenance** — Provenance can become stale if code changes without annotation updates

## Alternatives

### Alternative A: Comment-Based Markers

Use regular comments instead of annotations:
```javascript
// @acp:summary "Validates session" [AUTO-GENERATED: heuristic, confidence=0.72]
```

**Rejected because:**
- Not parseable as standard ACP annotation
- Harder to query
- Inconsistent with annotation syntax

### Alternative B: Separate Provenance File

Store provenance in `.acp.provenance.json` instead of cache/source.

**Rejected because:**
- Another file to manage
- Not grepable in source
- Sync issues between files

### Alternative C: Git-Based Tracking

Use git blame to determine annotation origin.

**Rejected because:**
- Can't distinguish human vs. auto-generated commits
- Requires git history
- Doesn't track confidence

### Alternative D: No Source Markers

Only track provenance in cache, not in source.

**Rejected because:**
- Not grepable
- Lost if cache is regenerated
- Can't see provenance when reading code

---

## Compatibility

### Backward Compatibility

- **Existing annotations:** Treated as `source: explicit` (human-written)
- **Existing cache:** Annotations without provenance data assumed explicit
- **Existing configs:** No breaking changes

### Forward Compatibility

- **New source types:** Can add new origin values in minor versions
- **New provenance fields:** Additive, won't break existing tools

### Migration Path

1. Run `acp index` — existing annotations get `source: explicit` in cache
2. Run `acp annotate` — new annotations get appropriate provenance markers
3. Optionally run `acp review` to verify auto-generated annotations

---

## Implementation

### Specification Changes

| Document | Changes |
|----------|---------|
| `ACP-1.0.md` Appendix A | Add `@acp:source`, `@acp:source-confidence`, `@acp:source-reviewed`, `@acp:source-id` |
| `spec/chapters/05-annotations.md` | Add Section 6: Annotation Provenance |

### Schema Changes

| Schema | Changes |
|--------|---------|
| `cache.schema.json` | Add `annotation_provenance` def, `annotations` to file/symbol entries, `provenance` top-level |
| `config.schema.json` | Add `annotate.provenance` settings |

### CLI Changes

| Command | Changes |
|---------|---------|
| `acp annotate` | Add `--no-provenance`, `--min-confidence`, `--needs-review` flags |
| `acp query` | Add `--source`, `--confidence`, `--needs-review` filters |
| `acp stats` | Add `--provenance` flag |
| `acp review` (new) | Interactive review command |

### Code Changes

| File | Changes |
|------|---------|
| `cli/src/annotate/mod.rs` | Add provenance generation to output |
| `cli/src/annotate/writer.rs` | Write `@acp:source` markers |
| `cli/src/cache/types.rs` | Add `AnnotationProvenance` struct |
| `cli/src/commands/review.rs` (new) | Interactive review command |

---

## Rollout Plan

1. **Phase 1** (v1.0.x): Add to specification, update schemas
2. **Phase 2** (v1.1.0): CLI generates provenance markers, cache stores provenance
3. **Phase 3** (v1.1.x): Add `acp query` provenance filters
4. **Phase 4** (v1.2.0): Add `acp review` interactive command, stats

---

## Open Questions

1. **Should provenance be required or optional?**
    - Current proposal: Optional (off by default for generated output, always tracked in cache)
    - Alternative: Always include in source

2. **How to handle provenance during merge conflicts?**
    - Keep source marker from version with higher confidence?
    - Let user resolve manually?

3. **Should we track multiple generations?**
    - Current proposal: Only latest generation
    - Alternative: Full history (significant complexity)

4. **IDE integration for provenance?**
    - Show confidence as hover info?
    - Highlight low-confidence annotations?

---

## Resolved Questions

1. **Q**: Where should `@acp:source` appear relative to other annotations?
   **A**: Immediately after the annotations it describes, at the end of the annotation block.

2. **Q**: Can a single `@acp:source` apply to multiple annotations?
   **A**: Yes, it applies to all preceding annotations in the same comment block until another `@acp:source` is encountered.

3. **Q**: What's the default source for annotations without markers?
   **A**: `explicit` (assumed human-written).

---

## References

- Related: `cli/src/annotate/mod.rs` — Existing `SuggestionSource` enum
- Related: `cli/src/annotate/suggester.rs` — Suggestion generation with priority
- Prior art: ESLint `--fix` dry-run mode
- Prior art: TypeScript `// @ts-expect-error` comments

---

## Appendix

### A. Quick Reference Card

```
┌────────────────────────────────────────────────────────────┐
│              Annotation Provenance Quick Reference          │
├────────────────────────────────────────────────────────────┤
│                                                             │
│  ANNOTATIONS:                                               │
│    @acp:source <origin>           Origin of annotation      │
│    @acp:source-confidence <0-1>   Confidence score          │
│    @acp:source-reviewed <bool>    Review status             │
│    @acp:source-id <uuid>          Generation batch ID       │
│                                                             │
│  ORIGINS:                                                   │
│    explicit    Human-written (default)                      │
│    converted   From JSDoc/docstring                         │
│    heuristic   Auto-generated from patterns                 │
│    refined     AI-improved                                  │
│    inferred    Code analysis (future)                       │
│                                                             │
│  CLI:                                                       │
│    acp annotate                   Generate with provenance  │
│    acp query --source heuristic   Find auto-generated       │
│    acp query --confidence "<0.7"  Find low-confidence       │
│    acp query --needs-review       Find needing review       │
│    acp stats --provenance         Show statistics           │
│    acp review                     Interactive review        │
│                                                             │
│  GREP:                                                      │
│    grep -r "@acp:source heuristic" src/                     │
│    grep -r "@acp:source-confidence 0\.[0-6]" src/           │
│                                                             │
└────────────────────────────────────────────────────────────┘
```

### B. Integration with RFC-0010

This RFC complements RFC-0010 (Documentation References and Style Guides). When both are implemented:

- `@acp:style tailwindcss-v4` with `@acp:source heuristic` indicates the style was auto-detected
- Refinement can improve both style selection and documentation references
- Review workflow covers all annotation types including refs and styles

---

## Changelog

| Date | Change |
|------|--------|
| 2025-12-20 | Initial draft |

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

# ACP Edge Cases & Gotchas

**Updated for ACP 1.3.0 (RFC-0015)**

This document covers unusual scenarios, edge cases, and common pitfalls when working with ACP.

---

## Table of Contents

1. [Annotation Parsing Edge Cases](#1-annotation-parsing-edge-cases)
2. [Constraint Inheritance Edge Cases](#2-constraint-inheritance-edge-cases)
3. [Variable Edge Cases](#3-variable-edge-cases)
4. [Cache Edge Cases](#4-cache-edge-cases)
5. [Multi-Language Edge Cases](#5-multi-language-edge-cases)
6. [Common Mistakes](#6-common-mistakes)

---

## 1. Annotation Parsing Edge Cases

### 1.1 Annotations in String Literals

**Problem:** Annotations inside strings should be ignored.

```typescript
// This annotation IS processed
// @acp:lock frozen

const message = `
  This is a template literal.
  // @acp:lock frozen  <-- Should be IGNORED (inside string)
`;

const regex = /@acp:lock frozen/;  // Should be IGNORED
```

**Rule:** Parsers MUST use language-aware parsing to detect string contexts.

### 1.2 Annotation Inside Code Comments

**Problem:** Only documentation comments should be processed.

```typescript
// @acp:lock frozen          // ✗ Regular comment - should be IGNORED in some parsers

/**
 * @acp:lock frozen          // ✓ Documentation comment - processed
 */

/* @acp:lock frozen */       // ✗ Block comment - typically IGNORED
```

**Rule:** Only process annotations in documentation comment styles for each language.

### 1.3 Multiple Annotations on Same Line

**Problem:** Multiple annotations on one line.

```typescript
/**
 * @acp:lock frozen @acp:domain security
 */
```

**Resolution:** Each annotation SHOULD be on its own line. If on same line, parser MAY:
- Process only the first annotation
- Process all annotations (implementation-dependent)

**Recommended:** One annotation per line.

### 1.4 Annotations with Special Characters

**Problem:** Values containing special characters.

```typescript
/**
 * @acp:summary "Handles \"quoted\" strings and \n newlines"
 * @acp:lock-reason "Uses: colons, commas, and (parentheses)"
 */
```

**Rule:** Use quoted strings for values with special characters. Escape sequences:
- `\"` — Literal quote
- `\\` — Literal backslash
- `\n` — Newline (preserved in value)

### 1.5 Empty and Whitespace Values

```typescript
/**
 * @acp:summary ""                  // Empty string - valid
 * @acp:summary "   "               // Whitespace only - valid but unusual
 * @acp:lock                        // Missing value - uses namespace default
 * @acp:domain                      // Missing required value - ERROR
 */
```

**Rules:**
- Empty quoted strings are valid
- Missing values use defaults where applicable
- Required values without defaults cause errors

### 1.6 Very Long Annotations

```typescript
/**
 * @acp:summary "This is an extremely long summary that goes on and on
 * and continues across multiple lines because someone wrote a very
 * detailed description that probably should have been shorter but
 * here we are with a very long annotation value"
 */
```

**Rule:** Multi-line continuation is supported. Lines are joined with single space.

### 1.7 Unicode in Annotations

```typescript
/**
 * @acp:module "認証サービス"
 * @acp:summary "Handles émojis 🔐 and ünïcödé"
 */
```

**Rule:** UTF-8 is fully supported. All Unicode characters valid in values.

### 1.8 Case Sensitivity

```typescript
/**
 * @acp:lock FROZEN      // ✗ Invalid - values are lowercase
 * @acp:LOCK frozen      // ✗ Invalid - namespaces are lowercase
 * @ACP:lock frozen      // ✗ Invalid - prefix is lowercase
 * @acp:lock frozen      // ✓ Valid
 */
```

**Rule:** Prefix, namespaces, and standard values are case-sensitive lowercase.

---

## 2. Constraint Inheritance Edge Cases

### 2.1 Symbol Override of File Constraint

**Scenario:** Symbol has less restrictive lock than file.

```typescript
/**
 * @acp:lock frozen
 */

/**
 * @acp:lock normal    // Can a symbol override to LESS restrictive?
 */
function helper() {}
```

**Resolution:** For lock levels, **most restrictive wins**. The symbol inherits `frozen`.

### 2.2 Multiple Domains with Different Constraints

**Scenario:** File belongs to domains with conflicting defaults.

```typescript
/**
 * @acp:domain security      // Domain default: lock=restricted
 * @acp:domain experimental  // Domain default: lock=experimental
 */
```

**Resolution:** Explicit file annotation wins. Without explicit annotation, **most restrictive domain default** applies.

### 2.3 Directory Config vs File Annotation

**Scenario:** `.acp.dir.json` sets one constraint, file annotation sets another.

```
src/
├── .acp.dir.json         // { "constraints": { "lock": "restricted" } }
└── utils/
    └── helper.ts         // @acp:lock experimental
```

**Resolution:** File annotation (more specific) wins. `helper.ts` is `experimental`.

### 2.4 Nested Directory Configs

**Scenario:** Multiple `.acp.dir.json` in path.

```
src/
├── .acp.dir.json         // { "constraints": { "lock": "normal" } }
└── auth/
    ├── .acp.dir.json     // { "constraints": { "lock": "restricted" } }
    └── session.ts        // No annotation
```

**Resolution:** Nearest (most specific) directory config wins. `session.ts` is `restricted`.

### 2.5 Quality Constraints Accumulation

**Scenario:** Multiple levels define quality requirements.

```typescript
// Project config: quality: ["tests-required"]
// File annotation: @acp:quality security-review
// Symbol annotation: @acp:quality performance-test
```

**Resolution:** Quality constraints **accumulate**. The symbol has all three requirements:
- tests-required (from project)
- security-review (from file)
- performance-test (from symbol)

### 2.6 Style Constraint Inheritance

**Scenario:** File and symbol both specify style.

```typescript
/**
 * @acp:style google-typescript
 * @acp:style-rules max-line-length=100
 */

/**
 * @acp:style-rules no-any
 */
function strict() {}
```

**Resolution:** Style guide is inherited. Style rules **accumulate**:
- `strict()` has: `google-typescript` with rules `max-line-length=100` AND `no-any`

---

## 3. Variable Edge Cases

### 3.1 Undefined Variables

**Scenario:** Reference to non-existent variable.

```
Check the $SYM_NONEXISTENT function.
```

**Resolution by mode:**
- **Permissive:** Warn, leave as literal `$SYM_NONEXISTENT`
- **Strict:** Error, abort expansion

### 3.2 Circular Variable References

**Scenario:** Variables that reference each other.

```json
{
  "VAR_A": { "type": "symbol", "value": "$VAR_B" },
  "VAR_B": { "type": "symbol", "value": "$VAR_A" }
}
```

**Resolution:** Max expansion depth (default: 10). Returns `[CIRCULAR: $VAR_A]`.

### 3.3 Variable in Variable Value

**Scenario:** Variable value contains another variable reference.

```json
{
  "SYM_AUTH": { "type": "symbol", "value": "src/auth/session.ts:validateSession" },
  "CONTEXT_AUTH": { "type": "symbol", "value": "See $SYM_AUTH for details" }
}
```

**Resolution:** Variables in values are expanded recursively (up to max depth).

### 3.4 Invalid Modifier

**Scenario:** Using non-existent modifier.

```
Check $SYM_VALIDATE.invalid_modifier
```

**Resolution:**
- **Permissive:** Warn, return base expansion (no modifier)
- **Strict:** Error

### 3.5 Modifier on Wrong Type

**Scenario:** Using `.signature` on a file variable.

```
$FILE_SESSION.signature    // Files don't have signatures
```

**Resolution:** Warn, return base expansion. Modifier only applies to applicable types.

### 3.6 Variable Name Collisions

**Scenario:** User defines variable with reserved-looking name.

```json
{
  "SYM_": { "type": "symbol", "value": "..." },
  "SYM_123": { "type": "symbol", "value": "..." }
}
```

**Resolution:** Variable names MUST match pattern `[A-Z][A-Z0-9_]+`. These are invalid.

### 3.7 Partial Variable Match

**Scenario:** Text that looks like a variable but isn't.

```
The price is $50 and $SYM_VALIDATE is the function.
```

**Resolution:** Only `$PREFIX_NAME` patterns are expanded. `$50` is not touched.

---

## 4. Cache Edge Cases

### 4.1 Stale Cache Detection

**Scenario:** Source files modified after cache generation.

```bash
$ cat .acp.cache.json | jq '.generated_at'
"2024-12-17T10:00:00Z"

$ stat src/auth/session.ts
Modified: 2024-12-17T15:00:00Z  # Newer than cache!
```

**Detection methods:**
1. Compare `generated_at` with file modification times
2. Compare `git_commit` with current HEAD
3. Compare `source_files` timestamps with actual files

**Resolution:** Warn user, suggest `acp index --force`.

### 4.2 Missing Files in Cache

**Scenario:** File exists in cache but was deleted.

```json
{
  "files": {
    "src/deleted.ts": { ... }  // File no longer exists
  }
}
```

**Resolution:**
- Queries for missing files return `null`
- Validation reports missing file
- Rebuild cache to fix

### 4.3 Orphan Symbols

**Scenario:** Symbol references non-existent file.

```json
{
  "symbols": {
    "src/missing.ts:func": {
      "file": "src/missing.ts"  // File not in files map
    }
  }
}
```

**Resolution:** Validation error. Cache is corrupt, rebuild required.

### 4.4 Inconsistent Call Graph

**Scenario:** Forward and reverse graphs don't match.

```json
{
  "graph": {
    "forward": { "A": ["B"] },
    "reverse": { "B": [] }      // Should include "A"!
  }
}
```

**Resolution:** Validation error. Graphs MUST be consistent inverses.

### 4.5 Very Large Cache

**Scenario:** Cache exceeds size limits.

```json
{
  "stats": {
    "files": 50000,
    "symbols": 500000
  }
}
```

**Resolution:**
- Consider splitting into multiple ACP projects
- Exclude generated files
- Increase limits in config (with caution)
- Use compressed cache format (future feature)

### 4.6 Binary Files

**Scenario:** Non-text files in include patterns.

```
include: ["**/*"]  // Includes images, compiled files, etc.
```

**Resolution:** Binary files are skipped with warning. Only text/code files indexed.

### 4.7 Symlinks

**Scenario:** Symbolic links in source tree.

```
src/
├── auth/
│   └── session.ts
└── legacy/
    └── auth -> ../auth/      # Symlink
```

**Resolution:** Implementation-dependent:
- Follow symlinks (may cause duplicates)
- Skip symlinks (may miss files)
- Follow with cycle detection (recommended)

---

## 5. Multi-Language Edge Cases

### 5.1 Mixed Language Project

**Scenario:** Project with TypeScript, Python, and Rust.

```
src/
├── api/          # TypeScript
├── ml/           # Python
└── core/         # Rust
```

**Resolution:** All languages indexed. Annotation syntax same, comment style varies.

### 5.2 Cross-Language Calls

**Scenario:** TypeScript calls Python via API.

```typescript
// TypeScript
const result = await fetch('/api/ml/predict');

# Python
@app.route('/api/ml/predict')
def predict():
    pass
```

**Resolution:** Cross-language calls not detected in call graph. Use `@acp:ref` for documentation:

```typescript
/**
 * @acp:ref "src/ml/predict.py:predict"
 */
async function callPredict() { ... }
```

### 5.3 Ambiguous File Extensions

**Scenario:** File extension shared by multiple languages.

```
src/
├── module.h      # Could be C or C++
└── script.m      # Could be Objective-C or MATLAB
```

**Resolution:** Heuristics applied:
- `.h`: Check for C++ keywords, default to C
- `.m`: Check for `@interface`, default to MATLAB (or error)

Configure explicitly in `.acp.config.json` if needed:
```json
{
  "languages": {
    "*.h": "cpp",
    "*.m": "objective-c"
  }
}
```

### 5.4 Embedded Languages

**Scenario:** SQL in TypeScript, HTML in Python.

```typescript
const query = `
  SELECT * FROM users
  WHERE id = $1
`;  // Embedded SQL - not parsed
```

**Resolution:** Embedded languages not parsed. Only host language annotations processed.

### 5.5 Generated Code

**Scenario:** Code generated from templates or tools.

```typescript
// Generated by protobuf - DO NOT EDIT
/**
 * @acp:lock frozen
 * @acp:lock-reason "Generated code - regenerate from .proto"
 */
```

**Resolution:** Mark generated code as `frozen`. Consider excluding from index:
```json
{
  "exclude": ["**/generated/**", "**/*.pb.ts"]
}
```

---

## 6. Common Mistakes

### 6.1 Forgetting to Rebuild Cache

**Mistake:** Adding annotations but not rebuilding cache.

```typescript
// Added new annotation
/**
 * @acp:lock restricted
 */
function newFunction() {}
```

```bash
# Cache doesn't include new function!
jq '.symbols["src/file.ts:newFunction"]' .acp.cache.json
null
```

**Fix:** Run `acp index` after adding/changing annotations.

### 6.2 Wrong Comment Style

**Mistake:** Using wrong comment style for annotations.

```typescript
/* @acp:lock frozen */        // ✗ Regular block comment
// @acp:lock frozen           // ✗ Regular line comment (in some parsers)

/**
 * @acp:lock frozen           // ✓ JSDoc style
 */
```

**Fix:** Use documentation comment style for your language.

### 6.3 Typos in Namespaces

**Mistake:** Misspelling annotation namespaces.

```typescript
/**
 * @acp:lok frozen            // ✗ Typo: "lok" not "lock"
 * @acp:sumary "Description"  // ✗ Typo: "sumary" not "summary"
 */
```

**Fix:** Enable strict mode to catch unknown namespaces.

### 6.4 Inconsistent Lock Levels

**Mistake:** Setting symbol less restrictive than file.

```typescript
/**
 * @acp:lock frozen
 */

/**
 * @acp:lock normal           // Has no effect! File is frozen.
 */
function helper() {}
```

**Fix:** Understand inheritance rules. Most restrictive wins.

### 6.5 Forgetting Quotes for Spaces

**Mistake:** Values with spaces not quoted.

```typescript
/**
 * @acp:summary Handles user authentication   // Stops at "user"
 * @acp:summary "Handles user authentication" // ✓ Full string
 */
```

**Fix:** Quote values containing spaces or special characters.

### 6.6 Circular Domain References

**Mistake:** Files annotated with domains that reference each other.

```json
// .acp.config.json
{
  "domains": {
    "auth": { "depends_on": ["db"] },
    "db": { "depends_on": ["auth"] }  // Circular!
  }
}
```

**Fix:** Design domain hierarchy without cycles.

### 6.7 Over-Constraining Code

**Mistake:** Making everything frozen or restricted.

```typescript
/**
 * @acp:lock frozen    // Is this really necessary?
 */
function formatDate(d) { return d.toISOString(); }
```

**Fix:** Reserve high restriction levels for truly critical code. Most code should be `normal`.

### 6.8 Missing Lock Reasons

**Mistake:** Using restrictive lock without explanation.

```typescript
/**
 * @acp:lock restricted
 * // No lock-reason - AI doesn't know why!
 */
```

**Fix:** Always provide `@acp:lock-reason` for restricted/frozen code.

### 6.9 Committing Cache File

**Mistake:** Checking `.acp.cache.json` into version control.

```bash
git add .acp.cache.json   // ✗ Don't do this
```

**Problems:**
- Cache contains absolute paths
- Merge conflicts on every change
- Large file bloating repo

**Fix:** Add to `.gitignore`:
```
.acp.cache.json
.acp.vars.json
.acp.violations.log
```

### 6.10 Conflicting Annotations

**Mistake:** Contradictory annotations.

```typescript
/**
 * @acp:stability stable
 * @acp:stability experimental    // Which is it?
 */
```

**Resolution:** Last one wins, but this is confusing.

**Fix:** Use only one annotation of each type per scope.

---

## Summary

| Category | Key Rule |
|----------|----------|
| Parsing | Use documentation comments, one annotation per line |
| Inheritance | More specific wins; most restrictive for locks |
| Variables | Check for undefined, watch for circular refs |
| Cache | Rebuild after changes, validate regularly |
| Multi-language | Same annotation syntax, language-specific comments |
| Best Practices | Quote values, provide reasons, don't over-constrain |

---

*See [Minimal Example](minimal.md) and [Complete Example](complete.md) for working setups.*
---
description: Verify that the ACP CLI produces correct cache output and command behavior after RFC-001 remediation. Tests against sample projects and validates schema compliance.
handoffs:
  - label: Back to CLI Remediate
    agent: acp.cli-remediate
    prompt: Fix issues found during verification
  - label: Update Schema
    agent: acp.schema-update-cache
    prompt: Update schema if cache structure differs
  - label: Release Prep
    agent: acp.cli-release
    prompt: Prepare CLI for release after verification passes
    send: true
---

## User Input

```text
$ARGUMENTS
```

You **MUST** consider the user input before proceeding (if not empty). The user input may specify:
- `--sample <path>` to use specific sample project
- `--schema <path>` to validate against specific schema
- `--quick` to run only essential checks
- `--verbose` for detailed output
- `--json` for machine-readable output
- `--fix` to attempt automatic fixes

## Purpose

This command verifies that the remediated CLI:

1. **Produces valid cache** - Output matches updated schema
2. **Extracts directives** - Annotations include directive field
3. **Builds symbol index** - Symbol-level annotations captured
4. **Indexes inline markers** - Hacks, todos, etc. tracked
5. **Commands work correctly** - map, migrate, query, constraints

## Outline

1. **Run verification suite**:
   ```bash
   ./scripts/verify-cli-directives.sh --json
   ```

   Expected output:
   ```json
   {
     "status": "pass|fail|partial",
     "cli_version": "0.3.0",
     "tests_run": 25,
     "tests_passed": 23,
     "tests_failed": 2,
     "categories": {
       "cache_generation": "pass|fail",
       "directive_parsing": "pass|fail",
       "symbol_indexing": "pass|fail",
       "inline_tracking": "pass|fail",
       "command_output": "pass|fail",
       "schema_compliance": "pass|fail"
     },
     "failures": [...],
     "sample_cache": ".acp/test-cache.json"
   }
   ```

2. **Validate each category** (see detailed sections)

3. **Generate verification report**

## Test Categories

### 1. Cache Generation Tests

Verify `acp index` produces correct output structure:

| Test ID | Description | Expected |
|---------|-------------|----------|
| CG01 | Cache has `$schema` field | Present, valid URL |
| CG02 | Cache has `version` field | Matches CLI version |
| CG03 | Files have `purpose` field | Present when @acp:purpose exists |
| CG04 | Files have `symbols` array | Array of SymbolAnnotation |
| CG05 | Files have `inline` array | Array of InlineAnnotation |
| CG06 | Cache passes schema validation | jsonschema validates |

**Test Script**:
```bash
# Generate test cache
acp index test-project/ --output .acp/test-cache.json

# Validate schema
acp validate .acp/test-cache.json --schema schemas/v1/cache.schema.json
```

---

### 2. Directive Parsing Tests

Verify annotations include directive field:

| Test ID | Description | Expected |
|---------|-------------|----------|
| DP01 | Explicit directive extracted | `directive` contains text after ` - ` |
| DP02 | Missing directive auto-generated | `directive` has default, `auto_generated: true` |
| DP03 | Multi-line directive joined | Continuation lines appended |
| DP04 | Directive trimmed properly | No leading/trailing whitespace |
| DP05 | Empty directive handled | `directive: null` or omitted |

**Test Input** (`test-project/directive-test.ts`):
```typescript
// @acp:lock frozen - MUST NOT modify this file
// @acp:lock restricted
// @acp:ref https://docs.example.com - Consult before changes.
//   This is critical infrastructure.
//   Contact: team@example.com
export function test() {}
```

**Expected Cache**:
```json
{
  "annotations": {
    "test-project/directive-test.ts": [
      {
        "name": "lock",
        "value": "frozen",
        "directive": "MUST NOT modify this file",
        "auto_generated": false,
        "line": 1
      },
      {
        "name": "lock",
        "value": "restricted",
        "directive": "Explain proposed changes and wait for explicit approval",
        "auto_generated": true,
        "line": 2
      },
      {
        "name": "ref",
        "value": "https://docs.example.com",
        "directive": "Consult before changes. This is critical infrastructure. Contact: team@example.com",
        "auto_generated": false,
        "line": 3
      }
    ]
  }
}
```

---

### 3. Symbol Indexing Tests

Verify symbol-level annotations captured:

| Test ID | Description | Expected |
|---------|-------------|----------|
| SI01 | @acp:fn parsed | Symbol with purpose |
| SI02 | @acp:class parsed | Symbol with class info |
| SI03 | Symbol line range captured | `lines: {start, end}` |
| SI04 | Symbol signature captured | `signature` field present |
| SI05 | Symbol constraints captured | `constraints` with level/directive |
| SI06 | @acp:param parsed | `params` array populated |
| SI07 | @acp:returns parsed | `returns` field present |

**Test Input** (`test-project/symbol-test.ts`):
```typescript
// @acp:purpose User authentication module

// @acp:fn validateUser - Validates user credentials against database
// @acp:param userId - The unique user identifier
// @acp:param token - Authentication token
// @acp:returns Boolean indicating validation success
// @acp:lock frozen - MUST NOT modify validation logic
export function validateUser(userId: string, token: string): boolean {
    // Implementation
    return true;
}

// @acp:class UserStore - In-memory user cache with TTL
export class UserStore {
    // @acp:method get - Retrieve user by ID
    get(id: string): User | null { return null; }
}
```

**Expected Cache**:
```json
{
  "files": {
    "test-project/symbol-test.ts": {
      "purpose": "User authentication module",
      "symbols": [
        {
          "name": "validateUser",
          "type": "function",
          "lines": {"start": 8, "end": 12},
          "purpose": "Validates user credentials against database",
          "signature": "function validateUser(userId: string, token: string): boolean",
          "constraints": {
            "level": "frozen",
            "directive": "MUST NOT modify validation logic"
          },
          "params": [
            {"name": "userId", "description": "The unique user identifier"},
            {"name": "token", "description": "Authentication token"}
          ],
          "returns": "Boolean indicating validation success"
        },
        {
          "name": "UserStore",
          "type": "class",
          "lines": {"start": 15, "end": 18},
          "purpose": "In-memory user cache with TTL"
        }
      ]
    }
  }
}
```

---

### 4. Inline Tracking Tests

Verify inline annotations (hacks, todos, etc.) tracked:

| Test ID | Description | Expected |
|---------|-------------|----------|
| IT01 | @acp:hack captured | In `inline` array |
| IT02 | Hack expiry parsed | `expires` field |
| IT03 | Hack ticket parsed | `ticket` field |
| IT04 | @acp:todo captured | In `inline` array |
| IT05 | @acp:fixme captured | In `inline` array |
| IT06 | @acp:critical captured | In `inline` array |

**Test Input** (`test-project/inline-test.ts`):
```typescript
export function process() {
    // @acp:hack - Timezone workaround for server clock drift
    //   @acp:hack-expires 2024-06-01
    //   @acp:hack-ticket JIRA-1234
    const offset = getTimezoneOffset();
    
    // @acp:todo - Add retry logic for network failures
    await fetch(url);
    
    // @acp:critical - Security boundary, validate all inputs
    validateInput(data);
}
```

**Expected Cache**:
```json
{
  "files": {
    "test-project/inline-test.ts": {
      "inline": [
        {
          "line": 2,
          "type": "hack",
          "directive": "Timezone workaround for server clock drift",
          "expires": "2024-06-01",
          "ticket": "JIRA-1234"
        },
        {
          "line": 8,
          "type": "todo",
          "directive": "Add retry logic for network failures"
        },
        {
          "line": 11,
          "type": "critical",
          "directive": "Security boundary, validate all inputs"
        }
      ]
    }
  }
}
```

---

### 5. Command Output Tests

Verify commands produce correct output:

| Test ID | Description | Command | Expected |
|---------|-------------|---------|----------|
| CO01 | Map shows structure | `acp map src/` | Tree with symbols |
| CO02 | Map shows constraints | `acp map src/` | Lock levels shown |
| CO03 | Map shows inline | `acp map --inline` | Hacks/todos listed |
| CO04 | Constraints shows directive | `acp constraints file` | Directive displayed |
| CO05 | Query file shows purpose | `acp query file path` | Purpose in output |
| CO06 | Query symbol shows lines | `acp query symbol name` | Line range shown |
| CO07 | Migrate dry-run works | `acp migrate --dry-run` | Changes listed |

**Test: `acp constraints` Output**

```bash
$ acp constraints test-project/symbol-test.ts

File: test-project/symbol-test.ts
Level: normal
Purpose: User authentication module

Annotations:
  @acp:purpose
    → User authentication module
  
Frozen Symbols:
  validateUser (lines 8-12)
    → MUST NOT modify validation logic
```

**Test: `acp map` Output**

```bash
$ acp map test-project/

test-project/
├── symbol-test.ts (normal)
│   User authentication module
│   ├─ validateUser (fn:8-12) [frozen]
│   ├─ UserStore (class:15-18)
│   └─ UserStore.get (method:17)
├── inline-test.ts (normal)
│   Active Issues:
│     :2 @acp:hack expires 2024-06-01
│     :8 @acp:todo - Add retry logic
└── directive-test.ts (frozen)
```

---

### 6. Schema Compliance Tests

Verify cache output matches schema:

| Test ID | Description | Expected |
|---------|-------------|----------|
| SC01 | Cache validates against schema | No validation errors |
| SC02 | All required fields present | Per schema `required` |
| SC03 | Field types correct | Per schema `type` |
| SC04 | Enum values valid | Per schema `enum` |
| SC05 | Additional properties | Matches `additionalProperties` |

**Test Script**:
```bash
# Full schema validation
acp validate .acp/test-cache.json \
  --schema schemas/v1/cache.schema.json \
  --strict

# Expected: "Valid: 0 errors"
```

---

## Sample Test Project

Create test project with all annotation types:

```
test-project/
├── acp.config.json
├── directive-test.ts      # Directive parsing tests
├── symbol-test.ts         # Symbol annotation tests
├── inline-test.ts         # Inline marker tests
├── multiline-test.ts      # Multi-line directive tests
├── constraint-test.ts     # Constraint aggregation tests
└── README.md
```

**Setup Script** (`scripts/setup-test-project.sh`):
```bash
#!/usr/bin/env bash
mkdir -p test-project
# Create test files with known annotations
# ... (file creation)
```

---

## Verification Report Format

```markdown
# CLI Directive Verification Report

**Generated**: [TIMESTAMP]
**CLI Version**: [VERSION]
**Sample Project**: [PATH]

## Summary

| Category | Tests | Passed | Failed |
|----------|-------|--------|--------|
| Cache Generation | 6 | 6 | 0 |
| Directive Parsing | 5 | 5 | 0 |
| Symbol Indexing | 7 | 6 | 1 |
| Inline Tracking | 6 | 6 | 0 |
| Command Output | 7 | 7 | 0 |
| Schema Compliance | 5 | 5 | 0 |
| **Total** | **36** | **35** | **1** |

## Status: PARTIAL PASS

### Failures

| Test | Category | Expected | Actual | Fix |
|------|----------|----------|--------|-----|
| SI03 | Symbol Indexing | lines.end = 12 | lines.end = 11 | Check line counting |

### Warnings

[any non-blocking issues]

## Sample Cache Validation

```
Schema: schemas/v1/cache.schema.json
Errors: 0
Warnings: 0
```

## Command Output Samples

### acp constraints
[sample output]

### acp map
[sample output]

## Recommendation

[pass/fix issues before release]
```

---

## Completion Criteria

### Verification Complete When:
- [ ] All test categories run
- [ ] Cache generation tests pass
- [ ] Directive parsing tests pass
- [ ] Symbol indexing tests pass
- [ ] Inline tracking tests pass
- [ ] Command output tests pass
- [ ] Schema compliance tests pass
- [ ] Report generated
- [ ] Sample cache saved for reference

## Error Conditions

| Error | Cause | Resolution |
|-------|-------|------------|
| "CLI not built" | Missing binary | Run `cargo build --release` |
| "Test project missing" | Setup not run | Run setup-test-project.sh |
| "Schema not found" | Missing schema file | Check schemas/v1/ path |
| "Validation failed" | Cache doesn't match schema | Fix CLI output or update schema |

## Next Steps

After verification passes:
1. **Update CHANGELOG** with new features
2. **Tag release** `v0.3.0`
3. **Build distributions** for all platforms
4. **Update documentation** website
---
description: Verify that the acp.cache.json schema and format support the new self-documenting directive requirements from RFC-001. Compares current cache structure against updated spec requirements.
handoffs:
  - label: Back to Remediate
    agent: acp.spec-remediate
    prompt: Schema issues found, need to update spec remediation
  - label: Update Cache Schema
    agent: acp.schema-update-cache
    prompt: Apply required schema changes
    send: true
  - label: Generate Migration
    agent: acp.cache-migration
    prompt: Generate cache migration script for existing projects
  - label: Full Audit
    agent: acp.spec-audit-directives
    prompt: Re-run full audit to verify everything
---

## User Input

```text
$ARGUMENTS
```

You **MUST** consider the user input before proceeding (if not empty). The user input may specify:
- `--cache <path>` to verify a specific cache file (default: `.acp/acp.cache.json`)
- `--schema <path>` to use specific schema file (default: `schemas/v1/cache.schema.json`)
- `--sample` to verify against a sample/test cache
- `--strict` to treat warnings as errors
- `--fix` to suggest automatic fixes where possible
- `--json` for machine-readable output

## Purpose

This command validates that the ACP cache format supports all directive-related fields required by RFC-001. It ensures:

1. **Schema completeness** - All required fields are defined
2. **Field presence** - Directive fields exist in the schema
3. **Type correctness** - Field types match RFC-001 requirements
4. **Example validity** - Sample caches conform to updated schema
5. **Backward compatibility** - Existing caches remain valid

## Outline

1. **Load verification context**:
   ```bash
   ./scripts/verify-cache-schema.sh --json
   ```

   Expected output:
   ```json
   {
     "schema_path": "schemas/v1/cache.schema.json",
     "schema_version": "1.0.0",
     "spec_version": "1.0.0-revised",
     "rfc_reference": "RFC-001",
     "checks": {
       "directive_field": "missing|present|partial",
       "purpose_field": "missing|present|partial",
       "lines_field": "missing|present|partial",
       "inline_array": "missing|present|partial",
       "auto_generated_flag": "missing|present|partial",
       "symbol_annotations": "missing|present|partial"
     },
     "overall_status": "compatible|needs_update|incompatible",
     "required_changes": [...],
     "sample_cache_valid": true|false
   }
   ```

2. **Check RFC-001 required fields**:

   | Field | Location | Required | Status |
      |-------|----------|----------|--------|
   | `directive` | `annotations[].directive` | Yes | ○ |
   | `purpose` | `files[].purpose` | Yes | ○ |
   | `lines` | `files[].symbols[].lines` | Yes | ○ |
   | `inline` | `files[].inline` | Yes | ○ |
   | `auto_generated` | `annotations[].auto_generated` | Per Q08 | ○ |
   | `signature` | `files[].symbols[].signature` | Recommended | ○ |
   | `params` | `files[].symbols[].params` | Recommended | ○ |
   | `returns` | `files[].symbols[].returns` | Recommended | ○ |

3. **Validate against sample cache**

4. **Generate compatibility report**

## RFC-001 Cache Requirements

### Section 4.1: Annotation Storage

The cache MUST store the full directive text:

```json
{
  "annotations": {
    "src/payments/stripe.ts": [
      {
        "type": "lock",
        "value": "frozen",
        "directive": "MUST NOT modify this file under any circumstances",
        "line": 1
      },
      {
        "type": "ref",
        "value": "https://stripe.com/docs/api",
        "directive": "Consult https://stripe.com/docs/api before making changes",
        "line": 2
      }
    ]
  }
}
```

**Verification checks:**
- [ ] `directive` field exists in annotation schema
- [ ] `directive` is type `string`
- [ ] `directive` allows empty string (for legacy annotations)
- [ ] `directive` has description matching RFC-001

---

### Section 4.2: Constraint Aggregation

The `constraints.by_file` structure MUST include aggregated directives:

```json
{
  "constraints": {
    "by_file": {
      "src/payments/stripe.ts": {
        "level": "frozen",
        "directive": "MUST NOT modify this file under any circumstances",
        "annotations": [
          {
            "type": "ref",
            "value": "https://stripe.com/docs/api",
            "directive": "Consult before making changes"
          }
        ]
      }
    }
  }
}
```

**Verification checks:**
- [ ] `constraints.by_file[].directive` field exists
- [ ] `constraints.by_file[].annotations` includes directives
- [ ] Aggregation logic documented

---

### Section 5: Hierarchical Annotations

The cache MUST support file-level, symbol-level, and inline annotations:

```json
{
  "files": {
    "src/auth/session.ts": {
      "purpose": "Session management and JWT validation",
      "domain": "auth",
      "owner": "security-team",
      "lines": { "start": 1, "end": 245 },
      "language": "typescript",
      "constraints": {
        "level": "restricted",
        "directive": "Explain changes, wait for approval"
      },
      "symbols": [
        {
          "name": "validateSession",
          "type": "function",
          "lines": { "start": 45, "end": 67 },
          "purpose": "Validates JWT token and checks session store",
          "signature": "function validateSession(token: string): Session | null",
          "constraints": {
            "level": "frozen",
            "directive": "MUST NOT modify validation logic"
          },
          "params": [
            { "name": "token", "type": "string" }
          ],
          "returns": { "type": "Session | null" }
        }
      ],
      "inline": [
        {
          "line": 56,
          "type": "hack",
          "directive": "Timezone workaround",
          "expires": "2024-06-01",
          "ticket": "JIRA-1234"
        }
      ]
    }
  }
}
```

**Verification checks:**
- [ ] `files[].purpose` field exists
- [ ] `files[].symbols` array exists
- [ ] `files[].symbols[].lines` object exists with `start` and `end`
- [ ] `files[].symbols[].purpose` field exists
- [ ] `files[].symbols[].signature` field exists
- [ ] `files[].symbols[].constraints` object exists
- [ ] `files[].inline` array exists
- [ ] `files[].inline[].directive` field exists

---

### Decision-Dependent Fields (Q08)

If decision Q08 = C (mark with flag):

```json
{
  "annotations": [{
    "type": "lock",
    "value": "frozen",
    "directive": "MUST NOT modify this file under any circumstances",
    "auto_generated": true
  }]
}
```

**Verification checks:**
- [ ] `auto_generated` field exists if Q08=C
- [ ] `auto_generated` is type `boolean`
- [ ] `auto_generated` defaults to `false`

## Schema Diff Analysis

Compare current schema against required structure:

```bash
./scripts/schema-diff.sh \
  --current schemas/v1/cache.schema.json \
  --required .acp/required-cache-schema.json \
  --output diff
```

**Output format:**
```
Schema Diff: cache.schema.json

ADDITIONS REQUIRED:
  + $.properties.files.additionalProperties.properties.purpose
  + $.properties.files.additionalProperties.properties.symbols
  + $.properties.files.additionalProperties.properties.inline
  + $.$defs.annotation.properties.directive
  + $.$defs.annotation.properties.auto_generated
  + $.$defs.symbol (new definition)
  + $.$defs.inlineAnnotation (new definition)

MODIFICATIONS REQUIRED:
  ~ $.properties.constraints.additionalProperties
    - Add 'directive' to constraint object

NO REMOVALS (backward compatible)
```

## Sample Cache Validation

Validate a sample cache against the updated schema:

```bash
./scripts/validate-sample-cache.sh --cache .acp/acp.cache.json
```

**Validation results:**
```json
{
  "valid": false,
  "errors": [
    {
      "path": "$.annotations.src/auth.ts[0]",
      "message": "Missing required property: directive",
      "severity": "warning"
    }
  ],
  "warnings": [
    {
      "path": "$.files.src/auth.ts",
      "message": "Missing optional property: symbols",
      "severity": "info"
    }
  ],
  "migration_needed": true
}
```

## Compatibility Matrix

| Current Schema | RFC-001 Requirement | Status | Action |
|----------------|---------------------|--------|--------|
| No `directive` field | Required | ⚠️ | Add field |
| No `purpose` field | Required | ⚠️ | Add field |
| No `symbols` array | Required | ⚠️ | Add structure |
| No `inline` array | Required | ⚠️ | Add structure |
| No `lines` in annotations | Optional | ℹ️ | Consider |
| `constraints.by_file` exists | Extend | ⚠️ | Add directive |

## Migration Impact

**For existing projects:**
- Caches without `directive` field remain valid (field optional in schema)
- CLI generates directives for legacy annotations
- Migration tool available: `acp migrate --add-directives`

**Breaking changes:**
- None if directives remain optional (Q01=B)
- If directives required (Q01=A): existing caches need migration

## Output Report

Generate verification report:

```markdown
# Cache Schema Verification Report

**Generated**: [TIMESTAMP]
**Schema Version**: [VERSION]
**RFC Reference**: RFC-001

## Summary

| Check | Status | Notes |
|-------|--------|-------|
| Directive field | ⚠️ Missing | Add to annotation schema |
| Purpose field | ⚠️ Missing | Add to file schema |
| Symbol support | ⚠️ Missing | Add symbols array and definition |
| Inline support | ⚠️ Missing | Add inline array and definition |
| Auto-generated flag | ℹ️ Missing | Add if Q08=C decision |

## Required Schema Changes

[list of specific changes needed]

## Sample Cache Status

[validation results against sample]

## Recommended Actions

1. Update schemas/v1/cache.schema.json with required fields
2. Update cli/src/cache/ to generate new fields
3. Add migration tooling for existing caches
4. Update documentation with new schema

## Next Steps

- [ ] Apply schema updates
- [ ] Test with sample caches
- [ ] Update CLI cache generation
- [ ] Generate migration script
```

## Completion Criteria

### Verification Complete When:
- [ ] All RFC-001 required fields checked
- [ ] Schema diff generated
- [ ] Sample cache validated
- [ ] Compatibility matrix complete
- [ ] Migration impact assessed
- [ ] Report generated
- [ ] Required changes documented

## Error Conditions

| Error | Cause | Resolution |
|-------|-------|------------|
| "Schema not found" | Missing cache schema file | Check schemas/v1/ path |
| "Invalid schema" | Malformed JSON Schema | Fix schema syntax |
| "Sample cache error" | Cache file invalid JSON | Fix cache file |
| "Decision not found" | Q08 decision missing | Run clarify first |

## Next Steps

After verification:

1. **If compatible**: Proceed to generate migration guide
2. **If needs updates**: Run `/acp.schema-update-cache` to apply changes
3. **If incompatible**: Review required changes and update remediation plan
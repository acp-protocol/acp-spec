---
description: Remediate the ACP specification to include self-documenting annotation directive requirements per RFC-001. Updates spec/ACP-1.0.md and spec/chapters/* based on audit findings and user decisions.
handoffs:
  - label: Back to Clarify
    agent: acp.spec-clarify
    prompt: Need to revisit or change decisions
  - label: Verify Cache
    agent: acp.spec-verify-cache
    prompt: Verify cache schema supports new directive requirements
    send: true
  - label: Re-run Audit
    agent: acp.spec-audit-directives
    prompt: Verify remediation addressed all findings
  - label: Generate Changelog
    agent: acp.spec-changelog
    prompt: Generate changelog entry for spec updates
---

## User Input

```text
$ARGUMENTS
```

You **MUST** consider the user input before proceeding (if not empty). The user input may specify:
- `--decisions <path>` to load decisions file (default: `.acp/spec-decisions.json`)
- `--dry-run` to show proposed changes without applying
- `--chapter <name>` to remediate only a specific chapter
- `--backup` to create backups before modifying (default: true)
- `--no-backup` to skip backups
- `--verbose` for detailed progress output

## Purpose

This command updates the ACP specification to comply with RFC-001 requirements for self-documenting annotations. It modifies:

1. **spec/ACP-1.0.md** - Main specification overview
2. **spec/chapters/02-annotation-syntax.md** - Annotation format and directives
3. **spec/chapters/03-cache-format.md** - Cache schema with directive fields
4. **spec/chapters/05-annotations.md** - Full annotation reference
5. **spec/chapters/06-constraints.md** - Constraint system with directives
6. **Other chapters** - Examples and cross-references

## Prerequisites

Before running this command:

1. **Audit completed**: Run `/acp.spec-audit-directives` first
2. **Decisions captured**: Run `/acp.spec-clarify` to resolve open questions
3. **Decisions file exists**: `.acp/spec-decisions.json` must be present

## Outline

1. **Load prerequisites**:
   ```bash
   ./scripts/load-remediation-context.sh --json
   ```

   Expected output:
   ```json
   {
     "decisions_file": ".acp/spec-decisions.json",
     "decisions_valid": true,
     "audit_findings": ".acp/spec-audit-findings.json",
     "spec_version": "1.0.0",
     "target_version": "1.0.0-revised",
     "files_to_modify": [
       "spec/ACP-1.0.md",
       "spec/chapters/02-annotation-syntax.md",
       ...
     ],
     "backup_dir": ".acp/backups/spec-[TIMESTAMP]"
   }
   ```

2. **Create backups** (unless `--no-backup`):
   ```bash
   ./scripts/backup-spec.sh
   ```

3. **Apply remediations** by chapter (see detailed sections below)

4. **Update version markers** in all modified files

5. **Validate changes** against RFC-001 checklist

6. **Generate summary** of all modifications

## Remediation Tasks

### Chapter 2: Annotation Syntax

**File**: `spec/chapters/02-annotation-syntax.md`

| Task ID | Description | Status |
|---------|-------------|--------|
| R-02-01 | Add directive suffix syntax definition | ○ |
| R-02-02 | Add directive separator grammar (`- `) | ○ |
| R-02-03 | Add multi-line directive format | ○ |
| R-02-04 | Add directive MUST/SHOULD/MUST NOT rules | ○ |
| R-02-05 | Add file-level annotations table with directives | ○ |
| R-02-06 | Add symbol-level annotations table with directives | ○ |
| R-02-07 | Add inline annotations table with directives | ○ |
| R-02-08 | Update all examples to include directives | ○ |
| R-02-09 | Add hierarchical annotation level section | ○ |
| R-02-10 | Add directive writing guidelines | ○ |

**Key Content to Add:**

```markdown
### Directive Suffix

All `@acp:*` annotations MUST include a directive suffix that provides
self-documenting instructions for AI agents. The directive:

- Follows the tag and any parameters
- Is separated by ` - ` (space-dash-space)
- Contains actionable instructions in imperative mood
- Uses RFC 2119 keywords (MUST, SHOULD, MAY) for clarity

**Syntax:**
\`\`\`
@acp:<tag> [parameters] - <directive>
\`\`\`

**Example:**
\`\`\`typescript
// @acp:lock frozen - MUST NOT modify this file under any circumstances
\`\`\`
```

---

### Chapter 3: Cache Format

**File**: `spec/chapters/03-cache-format.md`

| Task ID | Description | Status |
|---------|-------------|--------|
| R-03-01 | Add `directive` field to annotation schema | ○ |
| R-03-02 | Add `purpose` field to file entries | ○ |
| R-03-03 | Add `lines` field to symbol entries | ○ |
| R-03-04 | Add `inline` array for inline annotations | ○ |
| R-03-05 | Add `auto_generated` flag for generated directives | ○ |
| R-03-06 | Update cache examples with directive fields | ○ |
| R-03-07 | Add symbol-level annotation caching | ○ |

**Key Schema Additions:**

```json
{
  "annotations": {
    "type": "array",
    "items": {
      "type": "object",
      "properties": {
        "type": { "type": "string" },
        "value": { "type": "string" },
        "directive": {
          "type": "string",
          "description": "Self-documenting instruction for AI agents"
        },
        "auto_generated": {
          "type": "boolean",
          "description": "True if directive was auto-generated from defaults"
        },
        "line": { "type": "integer" }
      }
    }
  }
}
```

---

### Chapter 5: Full Annotation Reference

**File**: `spec/chapters/05-annotations.md`

| Task ID | Description | Status |
|---------|-------------|--------|
| R-05-01 | Add all new annotation types from RFC-001 | ○ |
| R-05-02 | Add `@acp:purpose` annotation | ○ |
| R-05-03 | Add symbol annotations: `@acp:fn`, `@acp:class`, `@acp:method` | ○ |
| R-05-04 | Add `@acp:param`, `@acp:returns`, `@acp:throws` | ○ |
| R-05-05 | Add inline markers: `@acp:critical`, `@acp:todo`, `@acp:fixme`, `@acp:perf` | ○ |
| R-05-06 | Add recommended directive for each annotation | ○ |
| R-05-07 | Add directive customization guidance | ○ |
| R-05-08 | Update all examples throughout | ○ |

**New Annotations to Document:**

| Annotation | Level | Category | Description |
|------------|-------|----------|-------------|
| `@acp:purpose` | File | Metadata | File/module purpose |
| `@acp:fn` | Symbol | Documentation | Function description |
| `@acp:class` | Symbol | Documentation | Class description |
| `@acp:method` | Symbol | Documentation | Method description |
| `@acp:param` | Symbol | Documentation | Parameter description |
| `@acp:returns` | Symbol | Documentation | Return value description |
| `@acp:throws` | Symbol | Documentation | Exception description |
| `@acp:example` | Symbol | Documentation | Usage example |
| `@acp:critical` | Inline | Safety | Critical code marker |
| `@acp:todo` | Inline | Tracking | Pending work |
| `@acp:fixme` | Inline | Tracking | Known issue |
| `@acp:perf` | Inline | Optimization | Performance note |

---

### Chapter 6: Constraints

**File**: `spec/chapters/06-constraints.md`

| Task ID | Description | Status |
|---------|-------------|--------|
| R-06-01 | Update constraint output to include directives | ○ |
| R-06-02 | Add directive display in `acp constraints` output | ○ |
| R-06-03 | Document directive aggregation from annotations | ○ |
| R-06-04 | Add conflict resolution rules (per Q03 decision) | ○ |

---

### Chapter 10/13: Tool Integration

**File**: `spec/chapters/10-bootstrap.md` or `spec/chapters/13-tool-integration.md`

| Task ID | Description | Status |
|---------|-------------|--------|
| R-10-01 | Add minimal bootstrap prompt section | ○ |
| R-10-02 | Add extended bootstrap (optional) | ○ |
| R-10-03 | Add bootstrap components table | ○ |
| R-10-04 | Update primer sections for directive awareness | ○ |

**Minimal Bootstrap Content:**

```markdown
### Minimal Bootstrap Prompt

The following minimal bootstrap is sufficient when all annotations
include self-documenting directives:

\`\`\`
This project uses ACP. @acp:* comments in code are directives for you.
BEFORE editing: acp constraints <path>
Explore: acp query symbol|file|domain <n>
Map: acp map <path>
Help: acp knowledge "question"
\`\`\`

**Token count:** ~40 tokens
```

---

### Examples Update

All example code throughout the spec must be updated:

**Before:**
```typescript
// @acp:lock frozen
export function criticalFunction() { }
```

**After:**
```typescript
// @acp:lock frozen - MUST NOT modify this file under any circumstances
export function criticalFunction() { }
```

## Remediation Script

Run the full remediation:

```bash
./scripts/remediate-spec-directives.sh \
  --decisions .acp/spec-decisions.json \
  --backup \
  --verbose
```

**Script behavior:**
1. Validates decisions file
2. Creates timestamped backup
3. Applies changes chapter by chapter
4. Updates version markers
5. Validates result against RFC-001
6. Outputs summary

## Dry Run Mode

When using `--dry-run`, output shows proposed changes:

```
=== DRY RUN: Spec Directive Remediation ===

File: spec/chapters/02-annotation-syntax.md
  + Section 2.3: Directive Suffix (new section, ~45 lines)
  + Section 2.4: Multi-line Directives (new section, ~20 lines)
  ~ Section 2.5: Examples (updated, 15 changes)
  
File: spec/chapters/03-cache-format.md
  ~ Section 3.2: Annotation Schema (modified, added `directive` field)
  + Section 3.5: Symbol Caching (new section, ~35 lines)

[... more files ...]

Summary:
  - 6 files would be modified
  - 4 new sections would be added
  - 23 examples would be updated
  - 0 breaking changes

Run without --dry-run to apply these changes.
```

## Validation

After remediation, validate changes:

```bash
./scripts/validate-remediation.sh --json
```

**Validation checks:**
- [ ] All RFC-001 checklist items addressed
- [ ] No syntax errors in modified files
- [ ] All examples compile/parse correctly
- [ ] Schema changes are valid JSON Schema
- [ ] Cross-references are intact
- [ ] Version markers updated consistently

## Output Files

| File | Purpose |
|------|---------|
| `.acp/spec-remediation-log.json` | Detailed log of all changes |
| `.acp/spec-remediation-summary.md` | Human-readable summary |
| `.acp/backups/spec-[TIMESTAMP]/` | Original files before changes |

## Completion Criteria

### Remediation Complete When:
- [ ] All tasks marked complete (R-XX-XX)
- [ ] All chapter files updated
- [ ] All examples include directives
- [ ] Schema changes applied
- [ ] Version markers updated
- [ ] Validation passes
- [ ] Summary generated
- [ ] Backups preserved

## Rollback

If remediation fails or results are unsatisfactory:

```bash
./scripts/rollback-spec.sh --backup .acp/backups/spec-[TIMESTAMP]
```

## Next Steps

After successful remediation:

1. **Run verification**: `/acp.spec-verify-cache` to ensure cache schema is compatible
2. **Re-run audit**: `/acp.spec-audit-directives` to confirm all findings addressed
3. **Update schemas**: Update JSON schema files in `schemas/v1/`
4. **Generate changelog**: Document spec changes for release

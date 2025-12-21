---
description: Audit the ACP specification for self-documenting annotation directive compliance. Analyzes spec/ACP-1.0.md and spec/chapters/* against RFC-001 requirements.
handoffs:
  - label: Clarify Decisions
    agent: acp.spec-clarify
    prompt: Review findings and get user decisions on open questions
    send: true
  - label: Remediate Spec
    agent: acp.spec-remediate
    prompt: Apply remediation based on audit findings
  - label: Verify Cache Schema
    agent: acp.spec-verify-cache
    prompt: Verify cache schema supports new directive requirements
  - label: Generate Migration Guide
    agent: acp.spec-migration-guide
    prompt: Generate migration guide for existing ACP users
---

## User Input

```text
$ARGUMENTS
```

You **MUST** consider the user input before proceeding (if not empty). The user input may specify:
- `--chapters-only` to audit only chapter files
- `--main-only` to audit only ACP-1.0.md
- `--focus <section>` to focus on specific sections (e.g., "annotations", "cache", "constraints")
- `--json` to output findings in JSON format for downstream processing
- `--strict` to treat SHOULD violations as errors

## Purpose

This command audits the ACP specification against RFC-001 (Self-Documenting Annotations) requirements. The audit ensures that:

1. **All annotation types include directive requirements** - Every `@acp:*` annotation MUST have documented directive syntax
2. **Recommended directive text is provided** - Standard directive language for consistency
3. **Hierarchical annotation levels are documented** - File, symbol, and inline levels
4. **Cache format supports directives** - The `directive` field is properly specified
5. **Examples demonstrate directives** - All spec examples show directive suffixes

## Audit Categories

| Category | Code | Description |
|----------|------|-------------|
| Blocker | B | MUST requirement from RFC-001 not met |
| Violation | V | SHOULD requirement not met |
| Warning | W | MAY recommendation not followed |
| Info | I | Observation or suggestion |

## Outline

1. **Initialize audit** by running the analysis script:
   ```bash
   ./scripts/audit-spec-directives.sh --json
   ```

   Parse JSON response:
   ```json
   {
     "status": "findings|clean",
     "spec_version": "1.0.0",
     "rfc_version": "001",
     "files_audited": [
       "spec/ACP-1.0.md",
       "spec/chapters/02-annotation-syntax.md",
       ...
     ],
     "findings": {
       "blockers": [...],
       "violations": [...],
       "warnings": [...],
       "info": [...]
     },
     "coverage": {
       "annotation_types_total": 25,
       "annotation_types_with_directives": 12,
       "examples_total": 45,
       "examples_with_directives": 18
     },
     "chapters_status": {
       "02-annotation-syntax.md": "needs_update",
       "03-cache-format.md": "needs_update",
       ...
     }
   }
   ```

2. **Analyze findings** by category:

   **Blockers (MUST fix before release)**:
    - Missing directive syntax definition
    - Missing directive field in cache schema
    - Annotation types without recommended directives

   **Violations (SHOULD fix)**:
    - Examples without directive suffixes
    - Inconsistent directive language
    - Missing hierarchical annotation documentation

   **Warnings (MAY fix)**:
    - Verbose directives that could be simplified
    - Non-standard directive wording

3. **Generate audit report** using the template format

4. **Identify open questions** that require user decisions (hand off to clarify)

## RFC-001 Compliance Checklist

The audit verifies these RFC-001 requirements:

### Annotation Format (Section 1)

| ID | Requirement | Spec Location | Status |
|----|-------------|---------------|--------|
| A01 | Directive suffix syntax defined | Chapter 2 | ○ |
| A02 | Multi-line directive format | Chapter 2 | ○ |
| A03 | Standard directives for file-level annotations | Chapter 2 | ○ |
| A04 | Standard directives for symbol-level annotations | Chapter 2 | ○ |
| A05 | Standard directives for inline annotations | Chapter 2 | ○ |
| A06 | Directive MUST/SHOULD/MUST NOT rules | Chapter 2 | ○ |

### Cache Format (Section 4)

| ID | Requirement | Spec Location | Status |
|----|-------------|---------------|--------|
| C01 | `directive` field in annotation objects | Chapter 3 | ○ |
| C02 | `purpose` field for files and symbols | Chapter 3 | ○ |
| C03 | `lines` field for symbols | Chapter 3 | ○ |
| C04 | `inline` array for inline annotations | Chapter 3 | ○ |
| C05 | Constraint aggregation includes directives | Chapter 5 | ○ |

### Hierarchical Annotations (Section 5)

| ID | Requirement | Spec Location | Status |
|----|-------------|---------------|--------|
| H01 | File-level annotation documentation | Chapter 2 | ○ |
| H02 | Symbol-level annotation documentation | Chapter 2 | ○ |
| H03 | Inline annotation documentation | Chapter 2 | ○ |
| H04 | Annotation level precedence rules | Chapter 5 | ○ |

### Bootstrap Prompt (Section 2)

| ID | Requirement | Spec Location | Status |
|----|-------------|---------------|--------|
| B01 | Minimal bootstrap defined | Chapter 10/13 | ○ |
| B02 | Extended bootstrap (optional) | Chapter 10/13 | ○ |
| B03 | Bootstrap components table | Chapter 10/13 | ○ |

### New Annotation Types (RFC-001 Implementation Checklist)

| Annotation | Documented | Has Directive | In Examples |
|------------|------------|---------------|-------------|
| `@acp:purpose` | ○ | ○ | ○ |
| `@acp:fn` | ○ | ○ | ○ |
| `@acp:class` | ○ | ○ | ○ |
| `@acp:method` | ○ | ○ | ○ |
| `@acp:param` | ○ | ○ | ○ |
| `@acp:returns` | ○ | ○ | ○ |
| `@acp:throws` | ○ | ○ | ○ |
| `@acp:critical` | ○ | ○ | ○ |
| `@acp:todo` | ○ | ○ | ○ |
| `@acp:fixme` | ○ | ○ | ○ |
| `@acp:perf` | ○ | ○ | ○ |

**Legend**: ○ Pending | ● Complete | ◐ Partial | ✗ Missing

## Script Reference

| Script | Purpose | When Used |
|--------|---------|-----------|
| `audit-spec-directives.sh` | Scan spec files for RFC-001 compliance | Always |
| `extract-annotations.sh` | Extract all annotation definitions | Analysis |
| `check-examples.sh` | Verify examples include directives | Analysis |

All scripts support `--json` for machine-readable output and `--help` for usage.

## Output Format

Generate the audit report in the following format:

```markdown
# ACP Spec Directive Audit Report

**Generated**: [TIMESTAMP]
**Spec Version**: [VERSION]
**RFC Reference**: RFC-001 (Self-Documenting Annotations)
**Status**: [PASS|FINDINGS|BLOCKERS]

## Executive Summary

| Category | Count | Status |
|----------|-------|--------|
| Blockers | X | [description] |
| Violations | X | [description] |
| Warnings | X | [description] |

## Coverage Analysis

[directive coverage statistics]

## Detailed Findings

### Blockers

[list with IDs, locations, descriptions, and remediation]

### Violations

[list with IDs, locations, descriptions, and remediation]

### Warnings

[list with IDs, locations, descriptions, and remediation]

## Open Questions

[questions requiring user decision]

## Recommended Actions

[prioritized list of next steps]
```

## Error Conditions

| Error | Cause | Resolution |
|-------|-------|------------|
| "Spec files not found" | Missing spec/ directory | Ensure running from repo root |
| "Invalid spec version" | Cannot parse version from ACP-1.0.md | Check spec header format |
| "RFC-001 not found" | Missing rfc-001 reference file | Provide RFC-001 path |

## Completion Criteria

### Audit Complete When:
- [ ] All spec files scanned
- [ ] All annotation types catalogued
- [ ] All examples checked for directive format
- [ ] Cache schema analyzed for directive support
- [ ] Findings categorized by severity
- [ ] Open questions identified
- [ ] Report generated with recommendations
- [ ] Coverage statistics calculated

## Integration Notes

This audit is the **first step** in the spec remediation workflow:

```
┌─────────────────────┐
│ acp.spec-audit      │ ← You are here
│ (analyze gaps)      │
└─────────┬───────────┘
          │
          ▼
┌─────────────────────┐
│ acp.spec-clarify    │
│ (get decisions)     │
└─────────┬───────────┘
          │
          ▼
┌─────────────────────┐
│ acp.spec-remediate  │
│ (update spec)       │
└─────────┬───────────┘
          │
          ▼
┌─────────────────────┐
│ acp.spec-verify     │
│ (validate cache)    │
└─────────────────────┘
```

After audit, hand off to `/acp.spec-clarify` with the findings to resolve open questions before remediation.
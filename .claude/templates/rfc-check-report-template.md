# RFC Consistency Check Report

<!-- 
TEMPLATE: RFC Consistency Check Report
COMMAND: /rfc.check
PURPOSE: Document results of consistency verification between implementation and spec/schemas

INSTRUCTIONS:
- This report is generated after /rfc.implement completes
- Five check types are performed: spec-code, schema, cross-refs, versions, conventions
- Each check produces: PASS, WARNINGS, or FAIL
- Issues marked [AUTO-FIX] can be resolved with --fix flag
-->

## Check Metadata

| Field | Value |
|-------|-------|
| RFC ID | {{RFC_ID}} |
| Check Run ID | {{CHECK_RUN_ID}} |
| Timestamp | {{CHECK_TIMESTAMP}} |
| Duration | {{CHECK_DURATION}} |
| Mode | {{CHECK_MODE}} |

<!-- Modes: full, quick, targeted -->

---

## Summary

**Overall Result: {{OVERALL_RESULT}}**

<!-- PASS / WARNINGS / FAIL -->

| Check Type | Result | Issues | Auto-Fixable |
|------------|--------|--------|--------------|
| Spec-Code Consistency | {{SPEC_CODE_RESULT}} | {{SPEC_CODE_ISSUES}} | {{SPEC_CODE_FIXABLE}} |
| Schema Validation | {{SCHEMA_RESULT}} | {{SCHEMA_ISSUES}} | {{SCHEMA_FIXABLE}} |
| Cross-References | {{XREF_RESULT}} | {{XREF_ISSUES}} | {{XREF_FIXABLE}} |
| Version Alignment | {{VERSION_RESULT}} | {{VERSION_ISSUES}} | {{VERSION_FIXABLE}} |
| Conventions | {{CONVENTIONS_RESULT}} | {{CONVENTIONS_ISSUES}} | {{CONVENTIONS_FIXABLE}} |

**Total Issues: {{TOTAL_ISSUES}}** ({{TOTAL_ERRORS}} errors, {{TOTAL_WARNINGS}} warnings)
**Auto-Fixable: {{TOTAL_FIXABLE}}**

---

## Check 1: Spec-Code Consistency

**Result: {{SPEC_CODE_RESULT}}**

### Purpose

Verify that code implementation matches specification requirements.

### Files Checked

| Spec File | Implementation File | Status |
|-----------|---------------------|--------|
{{SPEC_CODE_FILES}}

### Issues Found

{{#if SPEC_CODE_HAS_ISSUES}}
| ID | Severity | Location | Description | Auto-Fix |
|----|----------|----------|-------------|----------|
{{SPEC_CODE_ISSUE_TABLE}}

#### Issue Details

{{SPEC_CODE_ISSUE_DETAILS}}

{{else}}
✅ No issues found. Implementation matches specification.
{{/if}}

### Recommendations

{{SPEC_CODE_RECOMMENDATIONS}}

---

## Check 2: Schema Validation

**Result: {{SCHEMA_RESULT}}**

### Purpose

Validate JSON Schema syntax, structure, and cross-references.

### Schemas Checked

| Schema | Version | Valid | Errors |
|--------|---------|-------|--------|
{{SCHEMA_FILES}}

### Issues Found

{{#if SCHEMA_HAS_ISSUES}}
| ID | Severity | Schema | Line | Description | Auto-Fix |
|----|----------|--------|------|-------------|----------|
{{SCHEMA_ISSUE_TABLE}}

#### Issue Details

{{SCHEMA_ISSUE_DETAILS}}

{{else}}
✅ All schemas are valid.
{{/if}}

### Schema Cross-References

| Source Schema | References | Target Schema | Valid |
|---------------|------------|---------------|-------|
{{SCHEMA_XREFS}}

---

## Check 3: Cross-References

**Result: {{XREF_RESULT}}**

### Purpose

Validate all internal links and anchors in documentation and specs.

### Files Scanned

{{XREF_FILES_SCANNED}} files scanned, {{XREF_LINKS_CHECKED}} links checked

### Issues Found

{{#if XREF_HAS_ISSUES}}
| ID | Severity | Source File | Line | Link | Problem | Auto-Fix |
|----|----------|-------------|------|------|---------|----------|
{{XREF_ISSUE_TABLE}}

#### Broken Links

{{XREF_BROKEN_LINKS}}

#### Missing Anchors

{{XREF_MISSING_ANCHORS}}

{{else}}
✅ All cross-references are valid.
{{/if}}

---

## Check 4: Version Alignment

**Result: {{VERSION_RESULT}}**

### Purpose

Ensure version numbers are consistent across all project files.

### Version Files

| File | Current Version | Expected | Match |
|------|-----------------|----------|-------|
{{VERSION_FILES}}

### Issues Found

{{#if VERSION_HAS_ISSUES}}
| ID | Severity | File | Current | Expected | Auto-Fix |
|----|----------|------|---------|----------|----------|
{{VERSION_ISSUE_TABLE}}

{{else}}
✅ All versions are aligned.
{{/if}}

### Version History Check

| Version | Changelog Entry | Release Tag | Consistent |
|---------|-----------------|-------------|------------|
{{VERSION_HISTORY}}

---

## Check 5: Conventions

**Result: {{CONVENTIONS_RESULT}}**

### Purpose

Verify adherence to project conventions including RFC 2119 keywords and code style.

### RFC 2119 Keyword Usage

| Keyword | Count | Correct Usage | Issues |
|---------|-------|---------------|--------|
| MUST | {{MUST_COUNT}} | {{MUST_CORRECT}} | {{MUST_ISSUES}} |
| MUST NOT | {{MUST_NOT_COUNT}} | {{MUST_NOT_CORRECT}} | {{MUST_NOT_ISSUES}} |
| SHOULD | {{SHOULD_COUNT}} | {{SHOULD_CORRECT}} | {{SHOULD_ISSUES}} |
| SHOULD NOT | {{SHOULD_NOT_COUNT}} | {{SHOULD_NOT_CORRECT}} | {{SHOULD_NOT_ISSUES}} |
| MAY | {{MAY_COUNT}} | {{MAY_CORRECT}} | {{MAY_ISSUES}} |

### Code Style

| Rule | Files Checked | Violations |
|------|---------------|------------|
{{CODE_STYLE_RULES}}

### Issues Found

{{#if CONVENTIONS_HAS_ISSUES}}
| ID | Severity | File | Line | Rule | Description | Auto-Fix |
|----|----------|------|------|------|-------------|----------|
{{CONVENTIONS_ISSUE_TABLE}}

{{else}}
✅ All conventions are followed.
{{/if}}

---

## Detailed Issue List

### Errors (Must Fix)

{{#if HAS_ERRORS}}
{{ERROR_LIST}}
{{else}}
No errors found.
{{/if}}

### Warnings (Should Fix)

{{#if HAS_WARNINGS}}
{{WARNING_LIST}}
{{else}}
No warnings found.
{{/if}}

---

## Auto-Fix Summary

{{#if HAS_FIXABLE}}
The following issues can be automatically fixed with `--fix`:

| Issue ID | Type | Fix Description |
|----------|------|-----------------|
{{FIXABLE_ISSUES}}

**Command:** `./rfc-check.sh --fix {{RFC_ID}}`
{{else}}
No auto-fixable issues found.
{{/if}}

---

## Manual Fixes Required

{{#if HAS_MANUAL_FIXES}}
The following issues require manual intervention:

{{MANUAL_FIX_LIST}}
{{else}}
No manual fixes required.
{{/if}}

---

## Recommendations

### High Priority

{{HIGH_PRIORITY_RECOMMENDATIONS}}

### Medium Priority

{{MEDIUM_PRIORITY_RECOMMENDATIONS}}

### Low Priority

{{LOW_PRIORITY_RECOMMENDATIONS}}

---

## Handoff Information

<!-- For use by /rfc.test -->

```yaml
check_run_id: {{CHECK_RUN_ID}}
rfc_id: {{RFC_ID}}
overall_result: {{OVERALL_RESULT}}
total_errors: {{TOTAL_ERRORS}}
total_warnings: {{TOTAL_WARNINGS}}
blocking_issues: {{BLOCKING_ISSUE_COUNT}}
ready_for_testing: {{READY_FOR_TESTING}}
issues_by_type:
  spec_code: {{SPEC_CODE_ISSUES}}
  schema: {{SCHEMA_ISSUES}}
  cross_refs: {{XREF_ISSUES}}
  versions: {{VERSION_ISSUES}}
  conventions: {{CONVENTIONS_ISSUES}}
next_command: {{NEXT_COMMAND}}
```

---

*Generated by `/rfc.check` on {{CHECK_TIMESTAMP}}*

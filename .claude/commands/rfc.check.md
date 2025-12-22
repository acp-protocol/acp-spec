---
description: Verify implementation consistency with specifications and schemas. Ensures code matches documented behavior, schemas validate correctly, and cross-references are accurate.
handoffs:
  - label: Run Tests
    agent: rfc.test
    prompt: Run tests on the verified implementation
    send: true
  - label: Fix Issues
    agent: rfc.implement
    prompt: Fix consistency issues found during check
  - label: Update Spec
    agent: rfc.refine
    prompt: Update spec to match implementation changes
---

## User Input

```text
$ARGUMENTS
```

You **MUST** consider the user input before proceeding (if not empty). The user input may specify:
- RFC ID to check
- Specific check to run (e.g., `--check schemas`)
- `--strict` to fail on warnings
- `--fix` to attempt automatic fixes
- `--report` to generate detailed report only
- `--diff` to show spec vs implementation differences

## Purpose

Consistency checking verifies implementation integrity:

| Check Type | Verifies | Catches |
|------------|----------|---------|
| Spec-Code | Code matches spec | Implementation drift |
| Schema | Schemas validate | Invalid data structures |
| Cross-Ref | References resolve | Broken links |
| Version | Versions aligned | Mismatch errors |
| Convention | Follows standards | Style violations |

**Goal**: Catch discrepancies before testing and release.

## Outline

1. **Load Implementation Context**:
   - Load RFC from ID/path
   - Load implementation status from `.claude/memory/rfc-status-{id}.md`
   - Load change list from `.claude/memory/rfc-changes-{id}.md`
   - Identify all affected files

2. **Run Consistency Checks**:
   ```bash
   .claude/scripts/bash/rfc-check.sh --json <rfc-id>
   ```
   
   Parse JSON response:
   ```json
   {
     "rfc_id": "RFC-001",
     "check_timestamp": "2024-12-21T16:30:00Z",
     "overall_status": "WARNINGS",
     "checks": {
       "spec_code": {
         "status": "PASS",
         "items_checked": 12,
         "issues": []
       },
       "schema_validation": {
         "status": "PASS",
         "schemas_checked": ["cache.schema.json", "annotation.schema.json"],
         "issues": []
       },
       "cross_references": {
         "status": "WARNING",
         "refs_checked": 45,
         "issues": [
           {"type": "broken_link", "location": "spec/chapters/02.md:234", "ref": "#nonexistent-section"}
         ]
       },
       "version_alignment": {
         "status": "PASS",
         "versions": {"spec": "1.1.0", "schema": "1.1.0", "cli": "0.2.0"},
         "issues": []
       },
       "conventions": {
         "status": "WARNING",
         "issues": [
           {"type": "rfc2119", "location": "spec/chapters/02.md:156", "msg": "Use MUST instead of 'must'"}
         ]
       }
     },
     "summary": {
       "total_issues": 2,
       "errors": 0,
       "warnings": 2,
       "info": 0
     }
   }
   ```

3. **Execute Individual Checks**:

   **a. Spec-Code Consistency**:
   - Compare documented behavior to implementation
   - Verify all spec requirements have code
   - Check code comments match spec
   
   **b. Schema Validation**:
   - Validate all modified schemas
   - Test schemas against example data
   - Check schema cross-references
   
   **c. Cross-Reference Integrity**:
   - Verify all internal links resolve
   - Check external URL validity (optional)
   - Validate anchor references
   
   **d. Version Alignment**:
   - Spec version matches expectations
   - Schema versions consistent
   - Breaking changes properly versioned
   
   **e. Convention Compliance**:
   - RFC 2119 keyword usage
   - Code style guidelines
   - Documentation formatting

4. **Generate Consistency Report**:
   
   Create `.claude/memory/rfc-check-{id}.md`:
   
   ```markdown
   # Consistency Check Report: RFC-001
   
   **Checked**: 2024-12-21 16:30:00
   **Status**: ⚠️ WARNINGS (2 issues)
   
   ## Summary
   | Check | Status | Issues |
   |-------|--------|--------|
   | Spec-Code | ✓ Pass | 0 |
   | Schema | ✓ Pass | 0 |
   | Cross-Refs | ⚠ Warning | 1 |
   | Versions | ✓ Pass | 0 |
   | Conventions | ⚠ Warning | 1 |
   
   ## Issues
   
   ### W001: Broken Cross-Reference
   - **Location**: spec/chapters/02.md:234
   - **Issue**: Link to #nonexistent-section does not resolve
   - **Fix**: Update link or create target section
   
   ### W002: RFC 2119 Convention
   - **Location**: spec/chapters/02.md:156
   - **Issue**: Use uppercase MUST instead of lowercase 'must'
   - **Fix**: Replace 'must' with 'MUST'
   
   ## Files Checked
   - spec/chapters/02-annotation-syntax.md
   - spec/chapters/03-cache-format.md
   - schemas/v1/cache.schema.json
   - cli/src/parse/annotations.rs
   ```

5. **Apply Automatic Fixes** (if `--fix`):
   
   For fixable issues:
   - Apply corrections
   - Log changes
   - Re-run affected checks
   - Report results

## Check Details

### Spec-Code Consistency Check

Verifies implementation matches specification:

```
For each documented behavior in spec:
  1. Identify corresponding code location
  2. Verify code implements documented behavior
  3. Check edge cases are handled
  4. Verify error conditions match
```

**Signals to check**:
- Function signatures match spec
- Parameter constraints enforced
- Return types as documented
- Error messages consistent

**Example Issue**:
```markdown
### E001: Spec-Code Mismatch
- **Spec**: "Directive suffix MUST be separated by ` - ` (space-dash-space)"
- **Code**: Regex accepts `-` without spaces: `/^@acp:\w+\s+-/`
- **Resolution**: Update regex to require spaces
```

### Schema Validation Check

Ensures schemas are valid and consistent:

```
For each modified schema:
  1. Validate JSON Schema syntax
  2. Check against meta-schema
  3. Test with valid examples (should pass)
  4. Test with invalid examples (should fail)
  5. Verify cross-schema references
```

**Schema Test Matrix**:
| Schema | Valid Examples | Invalid Examples | Cross-Refs |
|--------|----------------|------------------|------------|
| cache.schema.json | ✓ | ✓ | ✓ |
| annotation.schema.json | ✓ | ✓ | ✓ |

### Cross-Reference Check

Validates all internal and external references:

```
For each reference in modified files:
  1. Extract reference type (anchor, file, URL)
  2. Resolve reference target
  3. Verify target exists
  4. Check for circular references
```

**Reference Types**:
| Type | Example | Validation |
|------|---------|------------|
| Anchor | `#section-name` | Section exists in document |
| File | `./other-doc.md` | File exists |
| Chapter | `Chapter 02` | Chapter exists in spec |
| Schema | `$ref: "annotation.schema.json"` | Schema file exists |
| URL | `https://example.com` | Optional URL check |

### Version Alignment Check

Ensures version numbers are consistent:

```
Extract versions from:
  - spec/ACP-*.md (spec version)
  - schemas/v*/schema.json ($version field)
  - Cargo.toml / package.json (implementation version)
  - CHANGELOG.md (documented version)

Verify:
  - Breaking changes bump major version
  - New features bump minor version
  - Patches bump patch version
  - All versions follow semver
```

### Convention Compliance Check

Validates adherence to project conventions:

**RFC 2119 Keywords**:
```
Find: /\b(must|shall|should|may)\b/gi
Flag: Lowercase usage in normative text
Fix: Uppercase in normative context
```

**Code Style**:
- Rust: rustfmt compliance
- TypeScript: eslint/prettier compliance
- Markdown: markdownlint compliance

**Documentation Style**:
- Consistent heading levels
- Table formatting
- Code block languages specified

## Fix Strategies

### Automatic Fixes

| Issue Type | Auto-Fix Strategy |
|------------|-------------------|
| RFC 2119 Case | Uppercase in normative context |
| Broken Anchor | Suggest closest match |
| Missing Schema Field | Add with default |
| Format Violations | Run formatter |

### Manual Fix Required

| Issue Type | Why Manual |
|------------|------------|
| Spec-Code Mismatch | Requires understanding intent |
| Missing Implementation | Needs code changes |
| Version Decisions | Requires human judgment |
| Breaking Changes | Need migration strategy |

## Scripts Reference

| Script | Purpose | When Used |
|--------|---------|-----------|
| `rfc-check.sh` | Run all consistency checks | Main check command |
| `rfc-check-spec.sh` | Spec-code consistency only | Targeted checking |
| `rfc-check-schema.sh` | Schema validation only | After schema changes |
| `rfc-check-refs.sh` | Cross-reference validation | After doc changes |
| `rfc-fix.sh` | Apply automatic fixes | When --fix specified |

## Error Conditions

| Error | Cause | Resolution |
|-------|-------|------------|
| "RFC not implemented" | Missing status file | Run /rfc.implement first |
| "Schema invalid" | Malformed JSON Schema | Fix schema syntax |
| "Spec not found" | Missing spec chapter | Check file paths |
| "Circular reference" | A references B references A | Break the cycle |

## Output Files

| File | Purpose | Location |
|------|---------|----------|
| Check Report | Full consistency report | `.claude/memory/rfc-check-{id}.md` |
| Fix Log | Applied fixes | `.claude/memory/rfc-fixes-{id}.log` |
| Issue List | Machine-readable issues | `.claude/memory/rfc-issues-{id}.json` |

## Completion Criteria

### Check Complete When:
- [ ] All check types executed
- [ ] All issues documented
- [ ] Report generated
- [ ] Auto-fixes applied (if requested)
- [ ] No blocking errors remain (for handoff)

### Pass Criteria:
- **PASS**: No errors, no warnings
- **WARNINGS**: No errors, some warnings (acceptable)
- **FAIL**: Errors present (blocks testing)

## Handoff Information

When handing off to `/rfc.test`:
- Provide the RFC ID
- Provide check report path
- Note any warnings that may affect tests
- Confirm no blocking errors

Example handoff (passing):
```
RFC-001 consistency check complete.
Report: .claude/memory/rfc-check-001.md
Status: ✓ PASS

All checks passed:
- Spec-Code: ✓
- Schema: ✓
- Cross-Refs: ✓
- Versions: ✓
- Conventions: ✓

Ready for /rfc.test
```

Example handoff (with warnings):
```
RFC-001 consistency check complete.
Report: .claude/memory/rfc-check-001.md
Status: ⚠️ WARNINGS (2 issues)

Warnings (non-blocking):
- W001: Broken link in spec/chapters/02.md:234
- W002: RFC 2119 convention in spec/chapters/02.md:156

These warnings should be fixed but don't block testing.
Ready for /rfc.test
```

Example handoff (failing):
```
RFC-001 consistency check found errors.
Report: .claude/memory/rfc-check-001.md
Status: ✗ FAIL (1 error, 2 warnings)

Blocking errors:
- E001: Spec-code mismatch in directive parser

Must resolve errors before testing.
Use /rfc.implement to fix, then re-run /rfc.check
```

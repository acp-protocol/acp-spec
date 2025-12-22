---
description: Run comprehensive tests on RFC implementation including unit tests, integration tests, and acceptance criteria validation.
handoffs:
  - label: Finalize RFC
    agent: rfc.finalize
    prompt: Finalize the RFC - update docs and version
    send: true
  - label: Fix Failures
    agent: rfc.implement
    prompt: Fix test failures
  - label: Update Tests
    agent: rfc.refine
    prompt: Update test plan based on findings
---

## User Input

```text
$ARGUMENTS
```

You **MUST** consider the user input before proceeding (if not empty). The user input may specify:
- RFC ID to test
- Specific test suite (e.g., `--suite unit`)
- `--coverage` to include coverage report
- `--verbose` for detailed output
- `--quick` to run smoke tests only
- `--acceptance` to run acceptance criteria only
- `--regression` to include regression tests

## Purpose

RFC testing validates the implementation works correctly:

| Test Type | Purpose | Scope |
|-----------|---------|-------|
| Unit Tests | Individual function correctness | New/modified functions |
| Integration Tests | Components work together | Affected modules |
| Acceptance Tests | RFC requirements met | Acceptance criteria |
| Regression Tests | Nothing broke | Related functionality |
| Edge Case Tests | Boundary conditions | Identified edge cases |

**Goal**: Confidence that the implementation is correct, complete, and doesn't break existing functionality.

## Outline

1. **Load Test Context**:
   - Load RFC and implementation status
   - Load check report to verify no blocking issues
   - Identify test targets from modified files
   - Build test execution plan

2. **Run Test Suite**:
   ```bash
   .claude/scripts/bash/rfc-test.sh --json <rfc-id>
   ```
   
   Parse JSON response:
   ```json
   {
     "rfc_id": "RFC-001",
     "test_run_id": "test-001-20241221-170000",
     "timestamp": "2024-12-21T17:00:00Z",
     "overall_status": "PASS",
     "suites": {
       "unit": {
         "status": "PASS",
         "total": 24,
         "passed": 24,
         "failed": 0,
         "skipped": 0,
         "duration_ms": 1234
       },
       "integration": {
         "status": "PASS",
         "total": 8,
         "passed": 8,
         "failed": 0,
         "skipped": 0,
         "duration_ms": 5678
       },
       "acceptance": {
         "status": "PASS",
         "total": 6,
         "passed": 6,
         "failed": 0,
         "skipped": 0,
         "duration_ms": 2345
       }
     },
     "coverage": {
       "overall": 87.5,
       "by_file": {
         "cli/src/parse/annotations.rs": 92.3,
         "cli/src/commands/index.rs": 85.2
       },
       "threshold": 80,
       "meets_threshold": true
     },
     "acceptance_criteria": [
       {"id": "AC1", "description": "Directive suffix parsed correctly", "status": "PASS"},
       {"id": "AC2", "description": "Missing directive generates warning", "status": "PASS"},
       {"id": "AC3", "description": "Multi-line directives supported", "status": "PASS"}
     ],
     "new_tests_added": 12,
     "modified_tests": 3
   }
   ```

3. **Execute Test Types**:

   **a. Unit Tests**:
   - Run tests for new/modified functions
   - Check assertions match spec
   - Verify error handling
   
   **b. Integration Tests**:
   - Test component interactions
   - Verify end-to-end workflows
   - Check data flow
   
   **c. Acceptance Tests**:
   - Map to RFC acceptance criteria
   - Validate requirements met
   - Document evidence
   
   **d. Regression Tests**:
   - Run related existing tests
   - Verify no breakage
   - Check backward compatibility

4. **Collect Coverage Data**:
   
   If `--coverage` specified:
   ```bash
   .claude/scripts/bash/rfc-coverage.sh --json <rfc-id>
   ```
   
   Coverage requirements:
   - New code: ≥80% line coverage
   - Modified code: ≥existing coverage
   - Critical paths: ≥90% branch coverage

5. **Validate Acceptance Criteria**:
   
   Map RFC acceptance criteria to test results:
   
   | Criteria | Test(s) | Status | Evidence |
   |----------|---------|--------|----------|
   | AC1: Directive parsed | test_parse_directive_* | ✓ | 5 tests pass |
   | AC2: Warning on missing | test_missing_directive_* | ✓ | 2 tests pass |
   | AC3: Multi-line support | test_multiline_* | ✓ | 3 tests pass |

6. **Generate Test Report**:
   
   Create `.claude/memory/rfc-test-{id}.md`:
   
   ```markdown
   # Test Report: RFC-001
   
   **Run**: test-001-20241221-170000
   **Date**: 2024-12-21 17:00:00
   **Status**: ✓ PASS
   
   ## Summary
   | Suite | Total | Passed | Failed | Skipped | Duration |
   |-------|-------|--------|--------|---------|----------|
   | Unit | 24 | 24 | 0 | 0 | 1.2s |
   | Integration | 8 | 8 | 0 | 0 | 5.7s |
   | Acceptance | 6 | 6 | 0 | 0 | 2.3s |
   | **Total** | **38** | **38** | **0** | **0** | **9.2s** |
   
   ## Coverage
   | Metric | Value | Threshold | Status |
   |--------|-------|-----------|--------|
   | Overall | 87.5% | 80% | ✓ |
   | New Code | 92.3% | 80% | ✓ |
   | Branch | 81.2% | 75% | ✓ |
   
   ## Acceptance Criteria
   | ID | Criteria | Status | Tests |
   |----|----------|--------|-------|
   | AC1 | Directive parsed correctly | ✓ | 5 |
   | AC2 | Missing directive warning | ✓ | 2 |
   | AC3 | Multi-line directives | ✓ | 3 |
   
   ## Test Details
   [Detailed test results...]
   ```

## Test Execution Details

### Unit Test Strategy

For each modified function:

```rust
// Example: Testing directive parser
#[test]
fn test_parse_directive_basic() {
    let input = "// @acp:lock frozen - MUST NOT modify";
    let result = parse_directive(input);
    assert_eq!(result.directive, "MUST NOT modify");
}

#[test]
fn test_parse_directive_multiline() {
    let input = r#"// @acp:lock restricted - Explain changes.
//   Additional context here.
//   Contact: team@example.com"#;
    let result = parse_directive(input);
    assert!(result.directive.contains("Additional context"));
}
```

### Integration Test Strategy

Test workflows end-to-end:

```rust
#[test]
fn test_index_with_directives() {
    // Setup: Create test files with directives
    // Execute: Run indexer
    // Verify: Cache contains directive data
    // Cleanup: Remove test files
}
```

### Acceptance Test Mapping

Map each RFC acceptance criterion to tests:

```markdown
## AC1: "Directive suffix MUST be parsed correctly"

Tests:
- test_parse_directive_basic
- test_parse_directive_with_params
- test_parse_directive_special_chars
- test_parse_directive_unicode
- test_parse_directive_edge_cases

Evidence:
- All 5 tests pass
- 100% of documented directive formats parsed
- No false positives/negatives
```

### Regression Test Selection

Select tests based on impact analysis:

| Component | Affected Tests | Reason |
|-----------|----------------|--------|
| Parser | test_parse_* | Direct changes |
| Indexer | test_index_* | Uses parser |
| Cache | test_cache_format_* | Schema changes |
| CLI | test_cmd_index | Integration |

## Coverage Analysis

### Coverage Targets

| Category | Minimum | Target | Critical |
|----------|---------|--------|----------|
| Line Coverage | 70% | 80% | 90% |
| Branch Coverage | 60% | 75% | 85% |
| Function Coverage | 80% | 90% | 100% |

### Coverage by Component

```
cli/src/parse/annotations.rs
├── parse_annotation    92.3% ██████████▓░░░░ 
├── parse_directive     94.1% ██████████▓░░░░
├── validate_directive  88.7% █████████▓░░░░░
└── extract_metadata    85.2% █████████░░░░░░

Overall: 87.5% ████████▓░░░░░░ (threshold: 80%)
```

### Uncovered Code Analysis

Document why certain code isn't covered:

| File:Line | Reason | Action |
|-----------|--------|--------|
| annotations.rs:234-240 | Error path for malformed input | Add negative test |
| annotations.rs:312 | Platform-specific branch | Skip (different platform) |

## Test Failure Handling

### On Unit Test Failure

1. Identify failing test
2. Determine if:
   - Implementation bug → Fix code
   - Test bug → Fix test
   - Spec ambiguity → Clarify spec
3. Fix and re-run

### On Integration Test Failure

1. Identify failing workflow
2. Isolate failing component
3. Check component interactions
4. Fix integration point

### On Acceptance Test Failure

1. Map to acceptance criterion
2. Determine if:
   - Implementation doesn't meet criterion → Fix implementation
   - Criterion unclear → Clarify RFC
   - Test incorrectly validates → Fix test
3. Ensure criterion is truly satisfied

## Scripts Reference

| Script | Purpose | When Used |
|--------|---------|-----------|
| `rfc-test.sh` | Run all test suites | Main test command |
| `rfc-test-unit.sh` | Run unit tests only | Targeted testing |
| `rfc-test-integration.sh` | Run integration tests | After unit pass |
| `rfc-test-acceptance.sh` | Run acceptance tests | Criteria validation |
| `rfc-coverage.sh` | Generate coverage report | When requested |

## Error Conditions

| Error | Cause | Resolution |
|-------|-------|------------|
| "Consistency check failed" | Blocking errors exist | Run /rfc.check first |
| "No tests found" | Tests not written | Create tests for changes |
| "Coverage below threshold" | Insufficient tests | Add more test cases |
| "Flaky test" | Non-deterministic test | Fix test reliability |

## Output Files

| File | Purpose | Location |
|------|---------|----------|
| Test Report | Full test results | `.claude/memory/rfc-test-{id}.md` |
| Coverage Report | Coverage data | `.claude/memory/rfc-coverage-{id}.html` |
| Test Log | Raw test output | `.claude/memory/rfc-test-{id}.log` |
| Failure Analysis | Failed test details | `.claude/memory/rfc-failures-{id}.md` |

## Completion Criteria

### Test Run Complete When:
- [ ] All test suites executed
- [ ] Results documented
- [ ] Coverage analyzed
- [ ] Acceptance criteria mapped

### Pass Criteria:
- **PASS**: All tests pass, coverage meets threshold
- **PARTIAL**: Some tests fail, but non-critical
- **FAIL**: Critical tests fail or coverage below threshold

### Ready for Finalization When:
- [ ] All test suites pass
- [ ] Coverage meets threshold
- [ ] Acceptance criteria validated
- [ ] No flaky tests
- [ ] Regression tests pass

## Handoff Information

When handing off to `/rfc.finalize`:
- Provide the RFC ID
- Provide test report path
- Confirm all tests pass
- Note coverage percentage

Example handoff (passing):
```
RFC-001 test suite complete.
Report: .claude/memory/rfc-test-001.md

Results:
- Unit Tests: 24/24 passed ✓
- Integration Tests: 8/8 passed ✓
- Acceptance Tests: 6/6 passed ✓
- Coverage: 87.5% (threshold: 80%) ✓

All acceptance criteria validated:
- AC1: Directive parsing ✓
- AC2: Warning generation ✓
- AC3: Multi-line support ✓

Ready for /rfc.finalize
```

Example handoff (failing):
```
RFC-001 test suite has failures.
Report: .claude/memory/rfc-test-001.md

Results:
- Unit Tests: 22/24 passed ✗
- Integration Tests: 8/8 passed ✓
- Acceptance Tests: 5/6 passed ✗

Failures:
1. test_parse_directive_unicode - assertion failed
2. test_acceptance_ac3_multiline - timeout

Must fix failures before finalizing.
Use /rfc.implement to address issues.
```

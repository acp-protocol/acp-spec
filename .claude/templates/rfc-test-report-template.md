# RFC Test Report

<!-- 
TEMPLATE: RFC Test Report
COMMAND: /rfc.test
PURPOSE: Document comprehensive test results including unit, integration, and acceptance tests

INSTRUCTIONS:
- This report is generated after /rfc.check passes
- Three test suites: unit, integration, acceptance
- Coverage thresholds: line 80%, branch 70%, function 90%
- Acceptance criteria must map to RFC requirements
-->

## Test Metadata

| Field | Value |
|-------|-------|
| RFC ID | {{RFC_ID}} |
| Test Run ID | {{TEST_RUN_ID}} |
| Timestamp | {{TEST_TIMESTAMP}} |
| Duration | {{TOTAL_DURATION}} |
| Mode | {{TEST_MODE}} |
| Environment | {{TEST_ENVIRONMENT}} |

<!-- Modes: full, quick, regression, acceptance-only -->

---

## Summary

**Overall Result: {{OVERALL_RESULT}}**

<!-- PASS / FAIL -->

| Suite | Tests | Passed | Failed | Skipped | Duration |
|-------|-------|--------|--------|---------|----------|
| Unit Tests | {{UNIT_TOTAL}} | {{UNIT_PASSED}} | {{UNIT_FAILED}} | {{UNIT_SKIPPED}} | {{UNIT_DURATION}} |
| Integration Tests | {{INTEG_TOTAL}} | {{INTEG_PASSED}} | {{INTEG_FAILED}} | {{INTEG_SKIPPED}} | {{INTEG_DURATION}} |
| Acceptance Tests | {{ACCEPT_TOTAL}} | {{ACCEPT_PASSED}} | {{ACCEPT_FAILED}} | {{ACCEPT_SKIPPED}} | {{ACCEPT_DURATION}} |
| **Total** | **{{TOTAL_TESTS}}** | **{{TOTAL_PASSED}}** | **{{TOTAL_FAILED}}** | **{{TOTAL_SKIPPED}}** | **{{TOTAL_DURATION}}** |

**Pass Rate: {{PASS_RATE}}%**

---

## Coverage Analysis

### Overall Coverage

| Metric | Current | Threshold | Status |
|--------|---------|-----------|--------|
| Line Coverage | {{LINE_COVERAGE}}% | 80% | {{LINE_STATUS}} |
| Branch Coverage | {{BRANCH_COVERAGE}}% | 70% | {{BRANCH_STATUS}} |
| Function Coverage | {{FUNC_COVERAGE}}% | 90% | {{FUNC_STATUS}} |

**Coverage Status: {{COVERAGE_STATUS}}**

<!-- Status: ✅ PASSING / ⚠️ BELOW THRESHOLD / ❌ FAILING -->

### Coverage by Component

| Component | Lines | Branches | Functions | Status |
|-----------|-------|----------|-----------|--------|
{{COVERAGE_BY_COMPONENT}}

### Uncovered Code

{{#if HAS_UNCOVERED}}
#### Critical Uncovered Sections

| File | Lines | Reason | Priority |
|------|-------|--------|----------|
{{UNCOVERED_CRITICAL}}

#### Other Uncovered

{{UNCOVERED_OTHER}}
{{else}}
All critical paths are covered.
{{/if}}

---

## Unit Tests

**Result: {{UNIT_RESULT}}**

### Test Files

| File | Tests | Passed | Failed | Duration |
|------|-------|--------|--------|----------|
{{UNIT_TEST_FILES}}

### Failed Tests

{{#if UNIT_HAS_FAILURES}}
| Test | File | Error | Duration |
|------|------|-------|----------|
{{UNIT_FAILURES}}

#### Failure Details

{{UNIT_FAILURE_DETAILS}}
{{else}}
✅ All unit tests passed.
{{/if}}

### Slow Tests (>1s)

{{#if HAS_SLOW_UNIT_TESTS}}
| Test | Duration | Recommendation |
|------|----------|----------------|
{{SLOW_UNIT_TESTS}}
{{else}}
No slow tests detected.
{{/if}}

---

## Integration Tests

**Result: {{INTEG_RESULT}}**

### Test Scenarios

| Scenario | Description | Status | Duration |
|----------|-------------|--------|----------|
{{INTEG_SCENARIOS}}

### Failed Tests

{{#if INTEG_HAS_FAILURES}}
| Test | Scenario | Error | Duration |
|------|----------|-------|----------|
{{INTEG_FAILURES}}

#### Failure Details

{{INTEG_FAILURE_DETAILS}}
{{else}}
✅ All integration tests passed.
{{/if}}

### Workflow Coverage

| Workflow | Steps Tested | Coverage | Status |
|----------|--------------|----------|--------|
{{WORKFLOW_COVERAGE}}

---

## Acceptance Tests

**Result: {{ACCEPT_RESULT}}**

### RFC Acceptance Criteria Mapping

| Criterion ID | Description | Test(s) | Status |
|--------------|-------------|---------|--------|
{{ACCEPTANCE_CRITERIA_MAP}}

### Criteria Coverage

**Criteria Tested: {{CRITERIA_TESTED}}/{{CRITERIA_TOTAL}}**
**Criteria Passing: {{CRITERIA_PASSING}}/{{CRITERIA_TOTAL}}**

{{#if CRITERIA_MISSING}}
#### Missing Coverage

The following acceptance criteria lack test coverage:

{{CRITERIA_MISSING_LIST}}
{{/if}}

### Failed Acceptance Tests

{{#if ACCEPT_HAS_FAILURES}}
| Criterion | Test | Expected | Actual | Status |
|-----------|------|----------|--------|--------|
{{ACCEPT_FAILURES}}

#### Failure Analysis

{{ACCEPT_FAILURE_ANALYSIS}}
{{else}}
✅ All acceptance criteria validated.
{{/if}}

---

## Regression Analysis

### Previous Test Results

| Run | Date | Tests | Pass Rate | Coverage |
|-----|------|-------|-----------|----------|
{{PREVIOUS_RUNS}}

### Regressions Detected

{{#if HAS_REGRESSIONS}}
| Test | Previous | Current | Regression Type |
|------|----------|---------|-----------------|
{{REGRESSIONS}}

#### Regression Details

{{REGRESSION_DETAILS}}
{{else}}
✅ No regressions detected.
{{/if}}

### New Tests Added

{{#if HAS_NEW_TESTS}}
| Test | File | Type | Coverage |
|------|------|------|----------|
{{NEW_TESTS}}
{{else}}
No new tests in this run.
{{/if}}

---

## Performance Metrics

### Test Execution

| Metric | Value | Threshold | Status |
|--------|-------|-----------|--------|
| Total Duration | {{TOTAL_DURATION}} | {{DURATION_THRESHOLD}} | {{DURATION_STATUS}} |
| Avg Test Duration | {{AVG_TEST_DURATION}} | {{AVG_THRESHOLD}} | {{AVG_STATUS}} |
| Slowest Test | {{SLOWEST_TEST_DURATION}} | {{SLOW_THRESHOLD}} | {{SLOW_STATUS}} |
| Parallel Efficiency | {{PARALLEL_EFFICIENCY}}% | 80% | {{PARALLEL_STATUS}} |

### Resource Usage

| Resource | Peak | Average | Status |
|----------|------|---------|--------|
| Memory | {{PEAK_MEMORY}} | {{AVG_MEMORY}} | {{MEMORY_STATUS}} |
| CPU | {{PEAK_CPU}} | {{AVG_CPU}} | {{CPU_STATUS}} |
| Disk I/O | {{PEAK_DISK}} | {{AVG_DISK}} | {{DISK_STATUS}} |

---

## Test Environment

### Configuration

| Setting | Value |
|---------|-------|
| OS | {{TEST_OS}} |
| Runtime | {{TEST_RUNTIME}} |
| Test Framework | {{TEST_FRAMEWORK}} |
| Parallelism | {{TEST_PARALLELISM}} |
| Timeout | {{TEST_TIMEOUT}} |

### Dependencies

| Dependency | Version | Status |
|------------|---------|--------|
{{TEST_DEPENDENCIES}}

---

## Recommendations

### Critical (Block Release)

{{#if HAS_CRITICAL}}
{{CRITICAL_RECOMMENDATIONS}}
{{else}}
No critical issues blocking release.
{{/if}}

### High Priority

{{HIGH_PRIORITY_RECOMMENDATIONS}}

### Medium Priority

{{MEDIUM_PRIORITY_RECOMMENDATIONS}}

### Test Improvements

{{TEST_IMPROVEMENTS}}

---

## Next Steps

{{#if ALL_PASSING}}
✅ **Ready for Finalization**

All tests passing, coverage thresholds met. Proceed to `/rfc.finalize`.
{{else}}
❌ **Not Ready for Finalization**

### Required Actions

{{REQUIRED_ACTIONS}}

### Recommended Actions

{{RECOMMENDED_ACTIONS}}
{{/if}}

---

## Handoff Information

<!-- For use by /rfc.finalize -->

```yaml
test_run_id: {{TEST_RUN_ID}}
rfc_id: {{RFC_ID}}
overall_result: {{OVERALL_RESULT}}
pass_rate: {{PASS_RATE}}
coverage:
  line: {{LINE_COVERAGE}}
  branch: {{BRANCH_COVERAGE}}
  function: {{FUNC_COVERAGE}}
  status: {{COVERAGE_STATUS}}
tests:
  total: {{TOTAL_TESTS}}
  passed: {{TOTAL_PASSED}}
  failed: {{TOTAL_FAILED}}
  skipped: {{TOTAL_SKIPPED}}
acceptance_criteria:
  total: {{CRITERIA_TOTAL}}
  tested: {{CRITERIA_TESTED}}
  passing: {{CRITERIA_PASSING}}
regressions: {{HAS_REGRESSIONS}}
ready_for_release: {{READY_FOR_RELEASE}}
blocking_issues: {{BLOCKING_ISSUE_COUNT}}
next_command: {{NEXT_COMMAND}}
```

---

*Generated by `/rfc.test` on {{TEST_TIMESTAMP}}*

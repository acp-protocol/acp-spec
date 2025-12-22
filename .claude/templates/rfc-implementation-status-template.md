# RFC Implementation Status

<!-- 
TEMPLATE: RFC Implementation Status
COMMAND: /rfc.implement
PURPOSE: Track real-time progress of RFC implementation

INSTRUCTIONS:
- This file is created when implementation begins and updated continuously
- Status values: pending, in-progress, blocked, complete, skipped
- Checkpoints capture resumable state for session continuity
- File changes are tracked for rollback capability
-->

## Session Information

| Field | Value |
|-------|-------|
| RFC ID | {{RFC_ID}} |
| Plan ID | {{PLAN_ID}} |
| Session ID | {{SESSION_ID}} |
| Started | {{SESSION_START}} |
| Last Updated | {{LAST_UPDATED}} |
| Session Mode | {{SESSION_MODE}} |

<!-- Session Modes: init, continue, resume, rollback -->

---

## Overall Progress

**Status: {{OVERALL_STATUS}}**

<!-- Status: not-started, in-progress, blocked, paused, complete -->

```
Phase 1 [{{PHASE1_PROGRESS}}] {{PHASE1_STATUS}}
Phase 2 [{{PHASE2_PROGRESS}}] {{PHASE2_STATUS}}
Phase 3 [{{PHASE3_PROGRESS}}] {{PHASE3_STATUS}}
Phase 4 [{{PHASE4_PROGRESS}}] {{PHASE4_STATUS}}
Phase 5 [{{PHASE5_PROGRESS}}] {{PHASE5_STATUS}}

Overall: {{OVERALL_PROGRESS}}% complete
```

<!-- Progress bars use: ████████░░ format (10 chars = 100%) -->

---

## Current State

### Active Phase

**Phase {{CURRENT_PHASE}}: {{CURRENT_PHASE_NAME}}**

### Current Task

| Field | Value |
|-------|-------|
| Task ID | {{CURRENT_TASK_ID}} |
| Description | {{CURRENT_TASK_DESC}} |
| Component | {{CURRENT_TASK_COMPONENT}} |
| Started | {{CURRENT_TASK_START}} |
| Status | {{CURRENT_TASK_STATUS}} |

### Next Tasks in Queue

| Priority | Task ID | Description | Dependencies |
|----------|---------|-------------|--------------|
{{NEXT_TASKS_QUEUE}}

---

## Phase Details

### Phase 1: Foundation

| Task ID | Task | Status | Started | Completed | Notes |
|---------|------|--------|---------|-----------|-------|
{{PHASE1_TASK_STATUS}}

**Phase 1 Exit Criteria:**
- [{{PHASE1_EXIT_1_CHECK}}] {{PHASE1_EXIT_1}}
- [{{PHASE1_EXIT_2_CHECK}}] {{PHASE1_EXIT_2}}
- [{{PHASE1_EXIT_3_CHECK}}] {{PHASE1_EXIT_3}}

---

### Phase 2: Core Implementation

| Task ID | Task | Status | Started | Completed | Notes |
|---------|------|--------|---------|-----------|-------|
{{PHASE2_TASK_STATUS}}

**Phase 2 Exit Criteria:**
- [{{PHASE2_EXIT_1_CHECK}}] {{PHASE2_EXIT_1}}
- [{{PHASE2_EXIT_2_CHECK}}] {{PHASE2_EXIT_2}}
- [{{PHASE2_EXIT_3_CHECK}}] {{PHASE2_EXIT_3}}

---

### Phase 3: Integration & Validation

| Task ID | Task | Status | Started | Completed | Notes |
|---------|------|--------|---------|-----------|-------|
{{PHASE3_TASK_STATUS}}

**Phase 3 Exit Criteria:**
- [{{PHASE3_EXIT_1_CHECK}}] {{PHASE3_EXIT_1}}
- [{{PHASE3_EXIT_2_CHECK}}] {{PHASE3_EXIT_2}}
- [{{PHASE3_EXIT_3_CHECK}}] {{PHASE3_EXIT_3}}

---

### Phase 4: Documentation

| Task ID | Task | Status | Started | Completed | Notes |
|---------|------|--------|---------|-----------|-------|
{{PHASE4_TASK_STATUS}}

**Phase 4 Exit Criteria:**
- [{{PHASE4_EXIT_1_CHECK}}] {{PHASE4_EXIT_1}}
- [{{PHASE4_EXIT_2_CHECK}}] {{PHASE4_EXIT_2}}
- [{{PHASE4_EXIT_3_CHECK}}] {{PHASE4_EXIT_3}}

---

### Phase 5: Release Preparation

| Task ID | Task | Status | Started | Completed | Notes |
|---------|------|--------|---------|-----------|-------|
{{PHASE5_TASK_STATUS}}

**Phase 5 Exit Criteria:**
- [{{PHASE5_EXIT_1_CHECK}}] {{PHASE5_EXIT_1}}
- [{{PHASE5_EXIT_2_CHECK}}] {{PHASE5_EXIT_2}}
- [{{PHASE5_EXIT_3_CHECK}}] {{PHASE5_EXIT_3}}

---

## File Changes

### Files Created

| File | Task | Timestamp |
|------|------|-----------|
{{FILES_CREATED}}

### Files Modified

| File | Task | Original Hash | Current Hash |
|------|------|---------------|--------------|
{{FILES_MODIFIED}}

### Files Deleted

| File | Task | Backup Location |
|------|------|-----------------|
{{FILES_DELETED}}

---

## Checkpoints

<!-- Checkpoints enable session resume and rollback -->

### Latest Checkpoint

| Field | Value |
|-------|-------|
| Checkpoint ID | {{LATEST_CP_ID}} |
| Created | {{LATEST_CP_TIME}} |
| Phase | {{LATEST_CP_PHASE}} |
| Task | {{LATEST_CP_TASK}} |
| Description | {{LATEST_CP_DESC}} |

### Checkpoint History

| ID | Timestamp | Phase | Task | Type | Description |
|----|-----------|-------|------|------|-------------|
{{CHECKPOINT_HISTORY}}

<!-- Types: auto, manual, phase-gate, error-recovery -->

---

## Blockers & Issues

### Active Blockers

| ID | Description | Blocking | Severity | Opened |
|----|-------------|----------|----------|--------|
{{ACTIVE_BLOCKERS}}

<!-- Severity: low, medium, high, critical -->

### Resolved Issues

| ID | Description | Resolution | Resolved |
|----|-------------|------------|----------|
{{RESOLVED_ISSUES}}

---

## Session Log

<!-- Recent activity log, most recent first -->

```
{{SESSION_LOG}}
```

<!-- Format: [TIMESTAMP] [LEVEL] Message -->
<!-- Levels: INFO, WARN, ERROR, DEBUG -->

---

## Metrics

| Metric | Value |
|--------|-------|
| Tasks Completed | {{TASKS_COMPLETED}}/{{TOTAL_TASKS}} |
| Time Elapsed | {{TIME_ELAPSED}} |
| Files Changed | {{FILES_CHANGED_COUNT}} |
| Lines Added | {{LINES_ADDED}} |
| Lines Removed | {{LINES_REMOVED}} |
| Checkpoints Created | {{CHECKPOINT_COUNT}} |
| Blockers Encountered | {{BLOCKERS_ENCOUNTERED}} |
| Blockers Resolved | {{BLOCKERS_RESOLVED}} |

---

## Resume Information

<!-- Used by /rfc.implement --continue and --resume -->

```yaml
session_id: {{SESSION_ID}}
rfc_id: {{RFC_ID}}
plan_id: {{PLAN_ID}}
status: {{OVERALL_STATUS}}
current_phase: {{CURRENT_PHASE}}
current_task: {{CURRENT_TASK_ID}}
last_checkpoint: {{LATEST_CP_ID}}
tasks_completed: {{TASKS_COMPLETED}}
tasks_remaining: {{TASKS_REMAINING}}
blockers_active: {{ACTIVE_BLOCKER_COUNT}}
can_continue: {{CAN_CONTINUE}}
next_action: {{NEXT_ACTION}}
rollback_available: {{ROLLBACK_AVAILABLE}}
```

---

## Handoff Information

<!-- For use by /rfc.check after implementation -->

```yaml
implementation_complete: {{IMPLEMENTATION_COMPLETE}}
files_to_check:
{{FILES_TO_CHECK_YAML}}
components_modified:
{{COMPONENTS_MODIFIED_YAML}}
tests_to_run:
{{TESTS_TO_RUN_YAML}}
next_command: {{NEXT_COMMAND}}
```

---

*Last updated: {{LAST_UPDATED}}*
*Session: {{SESSION_ID}}*

---
description: Execute an RFC implementation plan, working through tasks systematically while tracking progress and maintaining consistency with specifications.
handoffs:
  - label: Check Consistency
    agent: rfc.check
    prompt: Check implementation consistency with spec and schemas
    send: true
  - label: Update Plan
    agent: rfc.refine
    prompt: Update the plan based on implementation discoveries
  - label: View Progress
    agent: rfc.status
    prompt: Show implementation progress for this RFC
  - label: Skip to Testing
    agent: rfc.test
    prompt: Run tests on current implementation
---

## User Input

```text
$ARGUMENTS
```

You **MUST** consider the user input before proceeding (if not empty). The user input may specify:
- RFC ID or path to implement
- Specific task to execute (e.g., `T2.3`)
- Phase to focus on (e.g., `--phase 2`)
- `--continue` to resume from last checkpoint
- `--skip <task>` to skip a blocked task
- `--dry-run` to preview without changes
- `--interactive` for step-by-step confirmation

## Purpose

RFC implementation executes the refined plan systematically:

| Input | Process | Output |
|-------|---------|--------|
| Implementation Plan | Task Execution | Code/Spec Changes |
| Task List | Progress Tracking | Updated Status |
| Dependencies | Order Resolution | Proper Sequencing |
| Acceptance Criteria | Validation | Verified Completion |

**Goal**: Transform the plan into working code and updated specifications while maintaining traceability.

## Outline

1. **Load Plan and Task List**:
   - Load plan from `.claude/memory/rfc-plan-{id}.md`
   - Load tasks from `.claude/memory/rfc-tasks-{id}.md`
   - Load or create status from `.claude/memory/rfc-status-{id}.md`
   - Determine current position (phase, task)

2. **Initialize Implementation Session**:
   ```bash
   .claude/scripts/bash/rfc-implement.sh --init --json <rfc-id>
   ```
   
   Parse JSON response:
   ```json
   {
     "rfc_id": "RFC-001",
     "session_id": "impl-001-20241221-143052",
     "status_file": ".claude/memory/rfc-status-001.md",
     "current_phase": 1,
     "current_task": "T1.1",
     "completed_tasks": [],
     "pending_tasks": ["T1.1", "T1.2", "T2.1", "T2.2"],
     "blocked_tasks": [],
     "total_tasks": 15,
     "progress_percent": 0,
     "resume_point": null,
     "last_checkpoint": null
   }
   ```

3. **Execute Tasks in Dependency Order**:
   
   For each pending task:
   
   **a. Pre-Task Check**:
   - Verify dependencies are complete
   - Check for blockers
   - Confirm task is ready
   
   **b. Task Execution**:
   - Display task details and acceptance criteria
   - Perform the work (modify files, update specs, write code)
   - Track all file changes
   
   **c. Post-Task Validation**:
   - Verify acceptance criteria met
   - Run relevant checks (lint, type-check, tests)
   - Update task status
   
   **d. Checkpoint**:
   - Save progress to status file
   - Commit checkpoint if git available
   - Log activity

4. **Track Progress**:
   
   Update `.claude/memory/rfc-status-{id}.md` after each task:
   
   ```markdown
   # RFC-001 Implementation Status
   
   **Session**: impl-001-20241221-143052
   **Started**: 2024-12-21 14:30:52
   **Last Updated**: 2024-12-21 15:45:23
   **Progress**: 40% (6/15 tasks)
   
   ## Current Phase: 2 - Implementation
   
   ## Completed Tasks
   | Task | Completed | Duration | Notes |
   |------|-----------|----------|-------|
   | T1.1 | 14:35:12 | 4m 20s | Spec updated |
   | T1.2 | 14:42:33 | 7m 21s | Schema v1.1 |
   
   ## In Progress
   | Task | Started | Status |
   |------|---------|--------|
   | T2.1 | 15:30:00 | Implementing parser |
   
   ## Files Modified
   | File | Task | Change Type |
   |------|------|-------------|
   | spec/chapters/02-annotation-syntax.md | T1.1 | Modified |
   | schemas/v1/cache.schema.json | T1.2 | Modified |
   ```

5. **Handle Phase Transitions**:
   
   At phase boundaries:
   - Verify all phase tasks complete
   - Run phase gate checks
   - Generate phase summary
   - Prepare for next phase

## Task Execution Workflow

### For Specification Tasks

```
1. Identify target spec file
2. Review current content
3. Identify insertion/modification points
4. Make changes following spec conventions:
   - RFC 2119 keywords
   - Consistent formatting
   - Cross-references
5. Update table of contents if needed
6. Validate spec structure
```

### For Schema Tasks

```
1. Load current schema
2. Identify changes (additive vs breaking)
3. Make modifications:
   - New properties
   - Updated constraints
   - Description updates
4. Validate schema syntax
5. Update version if required
6. Test schema against examples
```

### For Code Tasks

```
1. Identify target modules
2. Review existing code structure
3. Implement changes:
   - Follow existing patterns
   - Add appropriate comments
   - Include error handling
4. Run lint/format
5. Run type checks
6. Write/update tests
```

### For Documentation Tasks

```
1. Identify target docs
2. Update content:
   - User-facing explanations
   - Examples
   - API references
3. Check links and references
4. Validate formatting
```

## Progress Tracking

### Task States

| State | Symbol | Meaning |
|-------|--------|---------|
| Pending | ○ | Not started, dependencies not met |
| Ready | ◐ | Dependencies met, ready to start |
| In Progress | ● | Currently being worked on |
| Complete | ✓ | Finished and validated |
| Blocked | ✗ | Cannot proceed, needs intervention |
| Skipped | ⊘ | Intentionally skipped |

### Phase Gates

Before transitioning phases, verify:

**Phase 1 → Phase 2** (Foundation → Implementation):
- [ ] All spec changes complete
- [ ] All schema changes complete
- [ ] Changes reviewed for consistency
- [ ] No blocking issues

**Phase 2 → Phase 3** (Implementation → Validation):
- [ ] All code changes complete
- [ ] Code compiles/lints clean
- [ ] Basic functionality works
- [ ] Ready for testing

**Phase 3 → Phase 4** (Validation → Documentation):
- [ ] All tests pass
- [ ] Coverage meets threshold
- [ ] No critical bugs
- [ ] Edge cases handled

**Phase 4 → Phase 5** (Documentation → Release):
- [ ] All docs updated
- [ ] Migration guide complete
- [ ] Changelog updated
- [ ] Ready for release

## Checkpoint System

Checkpoints allow resuming interrupted implementations:

```json
{
  "checkpoint_id": "cp-001-20241221-154523",
  "rfc_id": "RFC-001",
  "timestamp": "2024-12-21T15:45:23Z",
  "phase": 2,
  "last_completed_task": "T2.1",
  "files_modified": [
    {"path": "cli/src/parse/annotations.rs", "action": "modified"},
    {"path": "cli/src/parse/mod.rs", "action": "modified"}
  ],
  "git_commit": "abc123",
  "notes": "Parser complete, starting cache integration"
}
```

### Resume from Checkpoint

```bash
.claude/scripts/bash/rfc-implement.sh --resume <checkpoint-id> --json
```

## Error Handling

### Task Failure

When a task fails:
1. Log the error with context
2. Mark task as blocked
3. Analyze cause:
   - Dependency issue → Check dependencies
   - Technical blocker → Document and flag
   - Scope issue → May need plan revision
4. Offer options:
   - Retry task
   - Skip and continue
   - Stop and report

### Rollback

If implementation needs rollback:
```bash
.claude/scripts/bash/rfc-implement.sh --rollback <checkpoint-id>
```

## Scripts Reference

| Script | Purpose | When Used |
|--------|---------|-----------|
| `rfc-implement.sh` | Session management | Start/resume/checkpoint |
| `rfc-validate-task.sh` | Verify task completion | After each task |
| `rfc-checkpoint.sh` | Create checkpoints | At milestones |
| `rfc-rollback.sh` | Revert to checkpoint | On failure |

## Output Files

| File | Purpose | Location |
|------|---------|----------|
| Status File | Progress tracking | `.claude/memory/rfc-status-{id}.md` |
| Activity Log | Detailed activity | `.claude/memory/rfc-activity-{id}.log` |
| Checkpoint Data | Resume information | `.claude/memory/rfc-checkpoints-{id}.json` |
| File Changes | Modified files list | `.claude/memory/rfc-changes-{id}.md` |

## Completion Criteria

### Task Complete When:
- [ ] All acceptance criteria met
- [ ] Relevant checks pass (lint, type, test)
- [ ] Changes committed (if using git)
- [ ] Status updated

### Phase Complete When:
- [ ] All phase tasks complete
- [ ] Phase gate checks pass
- [ ] No blocking issues
- [ ] Ready for next phase

### Implementation Complete When:
- [ ] All phases complete
- [ ] All tasks verified
- [ ] No pending blockers
- [ ] Ready for consistency check

## Handoff Information

When handing off to `/rfc.check`:
- Provide the RFC ID
- Provide status file path
- List all modified files
- Note any concerns

Example handoff:
```
RFC-001 implementation complete.
Status: .claude/memory/rfc-status-001.md
Changes: .claude/memory/rfc-changes-001.md

Modified files:
- spec/chapters/02-annotation-syntax.md
- spec/chapters/03-cache-format.md  
- schemas/v1/cache.schema.json
- cli/src/parse/annotations.rs
- cli/src/commands/index.rs

All tasks complete. Ready for consistency check.
Run /rfc.check to verify implementation matches spec.
```

## Interactive Mode

When `--interactive` is specified:

1. Display task details before execution
2. Ask for confirmation: "Execute task T2.1? [Y/n/skip/details]"
3. Show progress after each task
4. Confirm phase transitions
5. Allow pausing at any point

This is recommended for complex or sensitive changes.

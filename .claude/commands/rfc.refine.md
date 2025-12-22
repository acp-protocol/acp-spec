---
description: Refine an accepted RFC into a detailed, actionable implementation plan with phased tasks, dependencies, and success criteria.
handoffs:
  - label: Implement Plan
    agent: rfc.implement
    prompt: Implement the refined plan
    send: true
  - label: Re-analyze RFC
    agent: rfc.analyze
    prompt: Re-analyze this RFC with new information
  - label: View Implementation Status
    agent: rfc.status
    prompt: Show current implementation progress
---

## User Input

```text
$ARGUMENTS
```

You **MUST** consider the user input before proceeding (if not empty). The user input may specify:
- RFC ID or path to refine
- Focus areas (spec-first, code-first, parallel)
- Time constraints or deadlines
- Resource considerations
- `--detailed` for comprehensive task breakdown
- `--minimal` for high-level phases only

## Purpose

RFC refinement transforms an accepted proposal into an executable plan:

| Input | Output | Deliverable |
|-------|--------|-------------|
| Accepted RFC | Implementation Plan | `rfc-plan-{id}.md` |
| Analysis Report | Task Breakdown | Phased task list |
| Open Questions | Decision Log | Resolved decisions |
| Spec Changes | Spec Tasks | Chapter-by-chapter updates |
| Schema Changes | Schema Tasks | Versioned schema updates |

**Goal**: Create a plan that any developer can follow to implement the RFC correctly and completely.

## Outline

1. **Load RFC and Analysis**:
   - Load the RFC from path or ID
   - Load analysis report from `.claude/memory/rfc-analysis-{id}.md`
   - Verify RFC status is "Accepted"
   - Extract key decisions from analysis

2. **Run Planning Script**:
   ```bash
   .claude/scripts/bash/rfc-plan.sh --json <rfc-path> [--analysis <analysis-path>]
   ```
   
   Parse JSON response:
   ```json
   {
     "rfc_id": "RFC-001",
     "title": "Self-Documenting Annotations",
     "components_affected": [
       {"component": "spec", "chapters": ["02", "03", "05"]},
       {"component": "schemas", "files": ["cache.schema.json", "annotation.schema.json"]},
       {"component": "cli", "modules": ["parse/annotations.rs", "commands/index.rs"]},
       {"component": "docs", "files": ["annotation-reference.md"]}
     ],
     "dependencies": {
       "external": [],
       "internal": ["cache format changes before CLI updates"],
       "blocking": []
     },
     "estimated_effort": {
       "spec": "2-3 hours",
       "schemas": "1-2 hours",
       "cli": "8-12 hours",
       "tests": "4-6 hours",
       "docs": "2-3 hours",
       "total": "17-26 hours"
     },
     "suggested_phases": [
       {"phase": 1, "name": "Foundation", "tasks": ["Update spec", "Update schemas"]},
       {"phase": 2, "name": "Implementation", "tasks": ["CLI parser", "Cache generation"]},
       {"phase": 3, "name": "Validation", "tasks": ["Unit tests", "Integration tests"]},
       {"phase": 4, "name": "Documentation", "tasks": ["Update docs", "Migration guide"]}
     ],
     "breaking_changes": {
       "has_breaking": true,
       "migration_required": true,
       "migration_strategy": "Auto-generate directives for existing annotations"
     }
   }
   ```

3. **Conduct Planning Session**:
   
   Walk through each component systematically:
   
   **a. Specification Changes**:
   - Which chapters need updates?
   - What new sections are needed?
   - What existing content needs revision?
   
   **b. Schema Changes**:
   - Which schemas are affected?
   - Is this additive or breaking?
   - Version bump requirements?
   
   **c. Implementation Tasks**:
   - What code modules need changes?
   - What new code needs to be written?
   - What's the dependency order?
   
   **d. Testing Requirements**:
   - What unit tests are needed?
   - What integration tests?
   - What edge cases to cover?
   
   **e. Documentation**:
   - User-facing docs to update?
   - Developer docs needed?
   - Migration guides?

4. **Generate Implementation Plan**:
   
   Create `.claude/memory/rfc-plan-{RFC_ID}.md` using template:
   `.claude/templates/rfc-plan-template.md`
   
   Structure:
   ```markdown
   # Implementation Plan: RFC-XXX
   
   ## Overview
   [Brief summary and goals]
   
   ## Phase 1: Foundation
   ### Tasks
   - [ ] T1.1: Update Chapter 02 - Annotation Syntax
   - [ ] T1.2: Update cache.schema.json
   
   ## Phase 2: Implementation
   ### Tasks
   - [ ] T2.1: Implement directive parser
   
   ## Dependencies
   [Task dependency graph]
   
   ## Risk Mitigation
   [Strategies for identified risks]
   
   ## Success Criteria
   [How we know it's done]
   ```

5. **Generate Task File**:
   
   Create `.claude/memory/rfc-tasks-{RFC_ID}.md` with detailed task breakdown:
   
   | Task ID | Phase | Description | Depends On | Est. Time | Status |
   |---------|-------|-------------|------------|-----------|--------|
   | T1.1 | 1 | Update spec chapter 02 | - | 1h | Pending |
   | T1.2 | 1 | Update cache schema | T1.1 | 30m | Pending |
   | T2.1 | 2 | Implement parser | T1.2 | 3h | Pending |

## Phase Structure

### Phase 1: Foundation (Spec & Schema)
**Purpose**: Establish the contract before implementation

Tasks typically include:
- Update specification chapters
- Modify JSON schemas
- Define error codes
- Document breaking changes

**Gate**: Spec and schemas reviewed and stable

### Phase 2: Implementation (Code)
**Purpose**: Build the functionality

Tasks typically include:
- Parser changes
- Data structure updates
- Command modifications
- Core logic implementation

**Gate**: Implementation complete, compiles clean

### Phase 3: Validation (Testing)
**Purpose**: Ensure correctness

Tasks typically include:
- Unit tests for new code
- Integration tests
- Backward compatibility tests
- Edge case coverage

**Gate**: All tests pass, coverage meets threshold

### Phase 4: Documentation (Docs)
**Purpose**: Enable users

Tasks typically include:
- Update user documentation
- Write migration guides
- Update CLI help text
- Add examples

**Gate**: Documentation complete and reviewed

### Phase 5: Release (Integration)
**Purpose**: Ship it

Tasks typically include:
- Version bump
- Changelog update
- Release notes
- Announcement preparation

**Gate**: Ready for release

## Task Specification Format

Each task in the plan should include:

```markdown
### T{phase}.{number}: {Title}

**Phase**: {phase_number}
**Component**: {component_name}
**Files**: {list of files to modify/create}
**Depends On**: {task_ids}
**Estimated Time**: {duration}

**Description**:
{What needs to be done}

**Acceptance Criteria**:
- [ ] {criterion_1}
- [ ] {criterion_2}

**Notes**:
{Any additional context}
```

## Dependency Resolution

When planning, resolve dependencies using:

1. **Hard Dependencies**: Must complete A before B
   - Schema changes before code
   - Parser before commands
   - Core logic before features

2. **Soft Dependencies**: Should complete A before B
   - Docs can parallel implementation
   - Tests can start early

3. **Parallel Tracks**: Can do simultaneously
   - Different components
   - Independent features

## Effort Estimation Guidelines

| Task Type | Small | Medium | Large |
|-----------|-------|--------|-------|
| Spec Update | 30m | 1-2h | 3-4h |
| Schema Change | 15m | 30m-1h | 1-2h |
| Parser Code | 1h | 2-4h | 6-8h |
| Command Code | 30m | 1-2h | 3-4h |
| Unit Tests | 30m | 1-2h | 3-4h |
| Integration Tests | 1h | 2-3h | 4-6h |
| Documentation | 30m | 1-2h | 3-4h |

## Risk Mitigation Planning

For each identified risk, document:

| Risk | Likelihood | Impact | Mitigation |
|------|------------|--------|------------|
| Schema migration fails | Medium | High | Auto-migration script + manual fallback |
| Parser performance | Low | Medium | Benchmark before/after |
| Breaking user workflows | Medium | High | Feature flag + opt-in period |

## Scripts Reference

| Script | Purpose | When Used |
|--------|---------|-----------|
| `rfc-plan.sh` | Generate initial plan structure | Start of refinement |
| `rfc-deps.sh` | Analyze component dependencies | During planning |
| `rfc-estimate.sh` | Calculate effort estimates | After task breakdown |

## Error Conditions

| Error | Cause | Resolution |
|-------|-------|------------|
| "RFC not accepted" | Wrong status | Run /rfc.analyze first |
| "Analysis not found" | Missing analysis report | Re-run analysis |
| "Circular dependency" | Tasks depend on each other | Review task order |
| "Missing component" | Unknown component referenced | Update component list |

## Output Files

| File | Purpose | Location |
|------|---------|----------|
| Implementation Plan | Full plan document | `.claude/memory/rfc-plan-{id}.md` |
| Task List | Detailed task breakdown | `.claude/memory/rfc-tasks-{id}.md` |
| Dependency Graph | Visual task dependencies | `.claude/memory/rfc-deps-{id}.mermaid` |

## Completion Criteria

### Refinement Complete When:
- [ ] All affected components identified
- [ ] All tasks defined with acceptance criteria
- [ ] Dependencies resolved and documented
- [ ] Effort estimates provided
- [ ] Risks identified with mitigations
- [ ] Success criteria defined
- [ ] Plan document generated
- [ ] Task list generated
- [ ] Ready for implementation handoff

## Handoff Information

When handing off to `/rfc.implement`:
- Provide the plan path
- Provide the task list path
- Note current phase (usually Phase 1)
- Highlight any blocking issues

Example handoff:
```
RFC-001 plan is ready for implementation.
Plan: .claude/memory/rfc-plan-001.md
Tasks: .claude/memory/rfc-tasks-001.md
Start with Phase 1: Foundation
- T1.1: Update Chapter 02 (spec/chapters/02-annotation-syntax.md)
- T1.2: Update cache.schema.json (schemas/v1/cache.schema.json)
No blockers. Estimated total: 20 hours.
Ready for /rfc.implement
```

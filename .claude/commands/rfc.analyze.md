---
description: Analyze an RFC for technical viability, completeness, and alignment with project goals. Determines accept/reject/clarify status through structured review.
handoffs:
  - label: Refine to Plan
    agent: rfc.refine
    prompt: Refine this accepted RFC into an actionable implementation plan
    send: true
  - label: Check Existing RFCs
    agent: rfc.status
    prompt: Show status of all RFCs in the project
  - label: View RFC Template
    agent: project.knowledge
    prompt: Show me the RFC template structure
---

## User Input

```text
$ARGUMENTS
```

You **MUST** consider the user input before proceeding (if not empty). The user input may specify:
- Path to the RFC file to analyze
- Specific concerns to evaluate
- Priority areas (security, performance, compatibility)
- `--quick` for abbreviated analysis
- `--strict` for rigorous standards compliance check

## Purpose

RFC analysis evaluates proposed protocol changes through a structured review process:

| Phase | Focus | Outcome |
|-------|-------|---------|
| Completeness | Required sections present | Pass/Fail |
| Technical Viability | Can this be implemented? | Viable/Concerns/Blocked |
| Alignment | Fits project goals and constitution | Aligned/Misaligned |
| Risk Assessment | Breaking changes, security, scope | Low/Medium/High/Critical |
| Decision | Overall recommendation | Accept/Reject/Clarify |

**Goal**: Make informed decisions quickly while capturing concerns for resolution.

## Outline

1. **Locate RFC** from user input ($ARGUMENTS):
   - If path provided: Load that file
   - If RFC number provided: Search `rfcs/proposed/` and `rfcs/accepted/`
   - If nothing provided: List available RFCs for selection

2. **Run Analysis Script**:
   ```bash
   .claude/scripts/bash/rfc-analyze.sh --json <rfc-path>
   ```
   
   Parse JSON response:
   ```json
   {
     "rfc": {
       "id": "RFC-XXXX",
       "title": "RFC Title",
       "path": "/path/to/rfc.md",
       "status": "Draft|Proposed|FCP",
       "author": "Author Name",
       "created": "YYYY-MM-DD"
     },
     "completeness": {
       "score": 85,
       "missing_sections": ["Benchmarks"],
       "incomplete_sections": ["Open Questions"],
       "warnings": ["No changelog entries"]
     },
     "technical": {
       "viable": true,
       "concerns": ["Requires tree-sitter changes"],
       "blockers": [],
       "affected_components": ["CLI", "Schemas"],
       "complexity": "Medium"
     },
     "alignment": {
       "constitution_compliant": true,
       "spec_chapters_affected": ["02", "03", "05"],
       "schema_changes_required": ["cache.schema.json"],
       "breaking_changes": false
     },
     "risk": {
       "level": "Medium",
       "factors": ["New annotation syntax", "Cache format changes"],
       "mitigations": ["Feature flag rollout", "Migration script"]
     },
     "questions": [
       "Should directive suffix be required or recommended?",
       "What is the maximum directive length?"
     ],
     "recommendation": "CLARIFY",
     "reasons": ["3 open questions need resolution", "Missing benchmark data"]
   }
   ```

3. **Conduct Interactive Review** (unless `--quick`):
   
   For each concern or open question:
   - Present the issue
   - Ask clarifying questions if needed
   - Record the resolution or flag for follow-up

4. **Generate Analysis Report**:
   
   Create/update `.claude/memory/rfc-analysis-{RFC_ID}.md` using template:
   `.claude/templates/rfc-analysis-template.md`
   
   Report includes:
   - Executive summary with recommendation
   - Completeness checklist
   - Technical viability assessment
   - Risk matrix
   - Open questions with answers/status
   - Decision record

5. **Update RFC Status** based on analysis:
   
   | Recommendation | Action |
   |----------------|--------|
   | ACCEPT | Move to `rfcs/accepted/`, set status to "Accepted" |
   | REJECT | Move to `rfcs/rejected/`, document reasons |
   | CLARIFY | Keep in `rfcs/proposed/`, list required clarifications |

## Analysis Criteria

### Completeness Check

| Section | Required | Weight |
|---------|----------|--------|
| Summary | Yes | High |
| Motivation | Yes | High |
| Detailed Design | Yes | Critical |
| Schema Changes | If applicable | Medium |
| Examples | Yes | Medium |
| Drawbacks | Yes | Medium |
| Alternatives | Recommended | Low |
| Open Questions | If any | High |
| Implementation Checklist | Yes | Medium |

### Technical Viability Signals

**Green Flags**:
- Clear implementation path
- Bounded scope
- Existing patterns to follow
- Author has implementation experience

**Yellow Flags**:
- Requires new dependencies
- Affects multiple components
- Novel approach (no prior art)
- Performance implications unclear

**Red Flags**:
- Fundamental architecture changes
- Unbounded scope
- No clear rollback path
- Security implications not addressed

### Alignment Verification

Check against:
- Project constitution (if exists)
- Existing specification chapters
- Schema versioning policy
- Backward compatibility requirements

### Risk Assessment Matrix

| Factor | Low | Medium | High | Critical |
|--------|-----|--------|------|----------|
| Breaking Changes | None | Deprecations | Migration needed | Fundamental change |
| Scope | Single file | Single component | Multiple components | Architecture |
| Security | N/A | Minor exposure | New attack surface | Critical path |
| Reversibility | Easy rollback | Some effort | Difficult | Irreversible |

## Interactive Questions

When analyzing, gather answers to:

1. **Scope Clarity**: "Is the scope well-defined? What's explicitly out of scope?"

2. **Implementation Feasibility**: "Can this be implemented incrementally? What's the MVP?"

3. **Compatibility**: "What breaks? What migration path exists?"

4. **Testing Strategy**: "How will this be validated? What are the edge cases?"

5. **Documentation Impact**: "What docs need updating? Any new concepts to explain?"

## Decision Framework

### Accept When:
- [ ] All required sections complete
- [ ] Technical approach is viable
- [ ] Aligns with project direction
- [ ] Risk level acceptable (Low-Medium)
- [ ] No unresolved blockers
- [ ] Open questions have paths to resolution

### Reject When:
- [ ] Fundamentally misaligned with project goals
- [ ] Technical approach has fatal flaws
- [ ] Risk outweighs benefit
- [ ] Scope creep beyond reasonable bounds
- [ ] Author unable/unwilling to address feedback

### Clarify When:
- [ ] Missing critical information
- [ ] Open questions affect implementation
- [ ] Alternatives not adequately explored
- [ ] Stakeholder input needed
- [ ] Risk assessment incomplete

## Scripts Reference

| Script | Purpose | When Used |
|--------|---------|-----------|
| `rfc-analyze.sh` | Parse RFC structure and metadata | Always |
| `rfc-check-refs.sh` | Validate internal references | During analysis |
| `rfc-move.sh` | Move RFC between directories | After decision |

## Error Conditions

| Error | Cause | Resolution |
|-------|-------|------------|
| "RFC not found" | Invalid path or ID | Check rfcs/ directory |
| "Invalid RFC format" | Missing required sections | Compare against template |
| "Cannot determine status" | No status field | Add status to RFC header |
| "Circular reference" | RFC references itself | Fix reference chain |

## Output Files

| File | Purpose | Location |
|------|---------|----------|
| Analysis report | Full analysis results | `.claude/memory/rfc-analysis-{id}.md` |
| Decision log | Accept/Reject/Clarify record | `.claude/memory/rfc-decisions.md` |
| Question log | Q&A during analysis | Appended to analysis report |

## Completion Criteria

### Analysis Complete When:
- [ ] All RFC sections reviewed
- [ ] Technical viability assessed
- [ ] Risk level determined
- [ ] All questions documented (with answers or follow-up status)
- [ ] Recommendation made with clear reasoning
- [ ] Analysis report generated
- [ ] RFC status updated (if decision is Accept/Reject)

## Handoff Information

When handing off to `/rfc.refine`:
- Provide the RFC path
- Include analysis report path
- Note any specific concerns to address in planning
- List stakeholder decisions that were made

Example handoff:
```
RFC-001 has been analyzed and ACCEPTED.
Analysis: .claude/memory/rfc-analysis-001.md
Key decisions:
- Directive suffix will be recommended, not required
- Maximum directive length: 500 characters
Ready for /rfc.refine to create implementation plan.
```

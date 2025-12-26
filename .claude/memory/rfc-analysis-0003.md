# Analysis Report: RFC-0003

**RFC**: RFC-0003 Annotation Provenance Tracking
**Analyzed**: 2025-12-22
**Status**: Draft
**Recommendation**: **ACCEPT**

---

## Executive Summary

RFC-0003 proposes a comprehensive system for tracking the origin (provenance) of ACP annotations, enabling identification and review of auto-generated annotations. The RFC is well-structured, technically viable, and addresses a real workflow need for projects using `acp annotate`.

**Verdict**: Accept with minor clarifications needed for open questions.

---

## Completeness Check

| Section | Present | Quality | Notes |
|---------|---------|---------|-------|
| Summary | ✓ | Good | Clear one-paragraph overview |
| Motivation | ✓ | Excellent | Strong problem statement with use cases |
| Detailed Design | ✓ | Excellent | Comprehensive with diagrams, examples |
| Schema Changes | ✓ | Complete | Both config and cache schemas detailed |
| Examples | ✓ | Excellent | 5+ complete examples with before/after |
| Drawbacks | ✓ | Good | 4 drawbacks identified |
| Alternatives | ✓ | Good | 4 alternatives considered |
| Open Questions | ✓ | Good | 4 questions listed |
| Implementation | ✓ | Good | Clear spec/schema/CLI breakdown |
| Rollout Plan | ✓ | Good | 4-phase plan |

**Completeness Score**: 95/100

---

## Technical Viability

### Assessment: ✓ VIABLE

**Green Flags**:
- Follows existing ACP annotation patterns
- Schema additions are additive (non-breaking)
- CLI changes extend existing commands
- Clear backward compatibility story

**Implementation Path**:
1. Add `@acp:source*` annotations to parser
2. Add `annotation_provenance` to cache schema
3. Add provenance config to config schema
4. Update `acp annotate` to emit provenance markers
5. Add `acp query --source` filters
6. Add `acp review` command (new)

**Complexity**: Medium
- Most changes are additive schema fields
- New `acp review` command is self-contained
- No architectural changes required

**Concerns**:
- Cache size growth with provenance data (minor)
- Annotation verbosity in source files (addressed in drawbacks)

---

## Alignment

### Constitution Compliance: ✓ ALIGNED

- Extends existing annotation system coherently
- Maintains backward compatibility
- Provides clear AI behavior guidelines
- Optional feature (non-mandatory)

### Affected Components

| Component | Impact |
|-----------|--------|
| `cache.schema.json` | Add `annotation_provenance` def, `provenance` top-level |
| `config.schema.json` | Add `annotate.provenance` settings |
| `spec/chapters/05-annotations.md` | Add provenance section |
| `ACP-1.0.md` Appendix A | Add 4 new annotations |
| CLI | New flags, new `review` command |

### Breaking Changes: None

All changes are additive. Existing annotations without `@acp:source` are treated as `explicit` (human-written).

---

## Risk Assessment

| Factor | Level | Notes |
|--------|-------|-------|
| Breaking Changes | Low | None identified |
| Scope | Medium | Multiple components but bounded |
| Security | Low | No security implications |
| Reversibility | High | Can remove provenance without breaking |

**Overall Risk**: LOW-MEDIUM

### Mitigations

1. **Annotation verbosity**: `--no-provenance` flag for legacy behavior
2. **Cache growth**: Provenance is optional, defaults not stored
3. **Merge conflicts**: Left to user resolution (acceptable)

---

## Open Questions Analysis

### Q1: Should provenance be required or optional?
**Recommendation**: Optional (current proposal is correct)
- Required would break existing workflows
- Optional allows gradual adoption

### Q2: How to handle provenance during merge conflicts?
**Recommendation**: Let user resolve manually
- Git already handles annotation conflicts
- No special handling needed

### Q3: Should we track multiple generations?
**Recommendation**: No (current proposal is correct)
- Significant complexity for marginal benefit
- Git history serves this purpose

### Q4: IDE integration for provenance?
**Recommendation**: Defer to separate RFC
- Out of scope for this RFC
- Can be added later without schema changes

---

## Dependencies

- **RFC-0001** (Implemented): Provides self-documenting directive format
- **RFC-0002** (Implemented): Compatible; provenance can track ref/style origins

Note: RFC mentions "RFC-0010" in Appendix B but should reference RFC-0002.

---

## Strengths

1. **Grepability**: `@acp:source heuristic` is easy to find in source
2. **Complete workflow**: From generation through review to refinement
3. **Statistics**: Clear visibility into annotation quality
4. **AI integration**: Good primer text and behavior guidelines
5. **Backward compatible**: No migration required

---

## Weaknesses

1. **Annotation overhead**: 2-3 extra lines per annotation block
2. **RFC reference error**: Mentions RFC-0010 instead of RFC-0002
3. **Open questions**: 4 questions could be resolved before acceptance

---

## Decision

**ACCEPT** - RFC-0003 is technically sound and addresses a real workflow need.

### Before Implementation

1. Fix RFC-0010 reference → RFC-0002
2. Resolve open questions (all have clear answers)
3. Update status to "Accepted"

### Implementation Priority

Medium - Can be implemented after higher-priority RFCs, but provides valuable workflow improvements.

---

## Handoff Notes

When proceeding to `/rfc.refine`:
- RFC path: `rfcs/rfc-0003-annotation-provenance-tracking.md`
- Analysis: `.claude/memory/rfc-analysis-0003.md`
- Key decisions:
  - Provenance is optional (off by default for source markers)
  - Single generation tracking (no history)
  - Merge conflict resolution is manual
- Focus areas: Schema additions, `acp review` command

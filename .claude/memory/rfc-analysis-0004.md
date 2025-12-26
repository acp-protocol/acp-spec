# Analysis Report: RFC-0004

**RFC**: RFC-0004 Tiered Interface Primers
**Analyzed**: 2025-12-22
**Status**: Draft
**Recommendation**: **ACCEPT**

---

## Executive Summary

RFC-0004 proposes a fundamental simplification of the ACP primer system, building on RFC-0001's self-documenting annotations insight. It eliminates teaching sections and replaces them with tiered (minimal/standard/full) interface documentation. The RFC is well-motivated, technically sound, and provides a cleaner architecture than the current v1 system.

**Verdict**: Accept - the simplification is well-justified and reduces unnecessary complexity.

---

## Completeness Check

| Section | Present | Quality | Notes |
|---------|---------|---------|-------|
| Summary | ✓ | Excellent | Clear motivation and scope |
| Motivation | ✓ | Excellent | Strong justification linked to RFC-0001 |
| Detailed Design | ✓ | Excellent | Complete schema, algorithm, examples |
| Schema Changes | ✓ | Complete | Full v2 schema provided |
| Examples | ✓ | Good | 3 budget examples with output |
| Drawbacks | ✓ | Good | 4 drawbacks identified |
| Alternatives | ✓ | Good | 3 alternatives considered |
| Open Questions | ✓ | Good | 4 questions listed |
| Rollout Plan | ✓ | Good | 4-phase plan with backward compatibility |
| Migration Guide | ✓ | Good | Appendix C covers v1→v2 migration |

**Completeness Score**: 92/100

---

## Technical Viability

### Assessment: ✓ VIABLE

**Green Flags**:
- Simpler than current system (reduces complexity)
- Clear selection algorithm (Python pseudocode provided)
- Backward compatible with v1 (version detection)
- No external dependencies

**Implementation Path**:
1. Create `primer.v2.schema.json`
2. Create `primer.defaults.v2.json` with tiered commands
3. Update `acp primer` to support v2 schema
4. Update `acp sync` to prefer v2
5. Deprecate v1 after feedback period

**Complexity**: Medium
- Schema is simpler than v1
- Algorithm is dramatically simpler (~30 lines vs hundreds)
- Migration requires new default file

**Concerns**:
- Breaking change for custom primer sections (addressed in migration guide)
- MCP parameter documentation may need richer support (deferred)

---

## Alignment

### Constitution Compliance: ✓ ALIGNED

- Builds directly on accepted RFC-0001
- Reduces unnecessary complexity
- Maintains core primer purpose
- Provides clear upgrade path

### Affected Components

| Component | Impact |
|-----------|--------|
| `primer.schema.json` | New v2 schema (breaking) |
| `primer.defaults.json` | New v2 format (breaking) |
| CLI `acp primer` | New selection algorithm |
| CLI `acp sync` | Schema version detection |
| Chapter 11 | Primer documentation updates |
| Chapter 14 | Bootstrap documentation updates |

### Breaking Changes: YES (Schema v2)

- v1 primer sections are incompatible with v2
- Projects with custom primers need migration
- Backward compatibility period provided

---

## Risk Assessment

| Factor | Level | Notes |
|--------|-------|-------|
| Breaking Changes | Medium | Schema change, but migration path provided |
| Scope | Medium | Primer system only |
| Security | Low | No security implications |
| Reversibility | Medium | Can revert to v1 if needed |

**Overall Risk**: MEDIUM

### Mitigations

1. **Schema migration**: v1 supported for one major version
2. **Custom sections**: `project.customRules` provides alternative
3. **Auto-detection**: Schema version detected from `version` field
4. **Phased rollout**: Flag-gated in v0.4.0, default in v0.5.0

---

## Open Questions Analysis

### Q1: Should daemon API be a separate interface type?
**Recommendation**: Yes, add as third interface type
- Daemon endpoints have different documentation needs
- Can group with CLI/MCP for now, expand later

### Q2: How should custom project commands be added?
**Recommendation**: Use `project.customRules` for now
- Full custom command support can be v2.1
- Most projects won't need custom commands

### Q3: Should tier selection be configurable?
**Recommendation**: No (hardcoded thresholds are fine)
- Adds complexity for marginal benefit
- Users can override via budget adjustment

### Q4: MCP parameter documentation richer than templates?
**Recommendation**: Defer to v2.1
- Templates work for most cases
- Can add `parameters` array to command schema later

---

## Dependencies

- **RFC-0001** (Implemented): Foundation - self-documenting directives eliminate teaching sections
- **RFC-0002** (Implemented): Independent but compatible

Note: RFC contains outdated reference path `./accepted/rfc-0001-self-documenting-annotations.md` - should be updated to flat structure.

---

## Strengths

1. **Dramatic simplification**: Removes multi-dimensional scoring entirely
2. **Clear justification**: Linked to RFC-0001 insight
3. **Practical budgets**: 40-1000 token range well-analyzed
4. **Complete migration**: Guide and examples provided
5. **Backward compatible**: v1 supported during transition

---

## Weaknesses

1. **Breaking schema change**: Requires migration for custom primers
2. **Reduced flexibility**: Priority-only vs multi-dimensional scoring
3. **Path references outdated**: Points to old directory structure
4. **Token estimates**: May need calibration in practice

---

## Decision

**ACCEPT** - RFC-0004 provides a well-justified simplification that aligns with RFC-0001.

### Before Implementation

1. Fix file path references (update to flat RFC structure)
2. Resolve open questions (recommendations above)
3. Update status to "Accepted"

### Implementation Priority

Medium-High - Primer simplification benefits all users and reduces maintenance burden.

---

## Comparison: v1 vs v2

| Aspect | v1 | v2 |
|--------|----|----|
| Selection Algorithm | Multi-dimensional scoring | Simple priority |
| Section Types | 35+ teaching sections | Interface tiers only |
| Schema Complexity | High | Low |
| Token Efficiency | Variable | Predictable |
| Customization | Weight presets | Custom rules string |
| Maintenance | High | Low |

---

## Handoff Notes

When proceeding to `/rfc.refine`:
- RFC path: `rfcs/rfc-0004-tiered-interface-primers.md`
- Analysis: `.claude/memory/rfc-analysis-0004.md`
- Key decisions:
  - Daemon API grouped with CLI/MCP initially
  - Custom commands via `project.customRules`
  - Hardcoded tier thresholds
  - MCP parameter richness deferred to v2.1
- Focus areas: New schema, selection algorithm, default commands file
- Breaking change: Plan v1→v2 migration carefully

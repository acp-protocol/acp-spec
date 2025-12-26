# Implementation Plan: RFC-0004

**RFC**: RFC-0004 Tiered Interface Primers
**Created**: 2025-12-22
**Status**: Ready for Implementation

---

## Overview

RFC-0004 simplifies the ACP primer system by eliminating teaching sections (now redundant per RFC-0001) and replacing them with tiered interface documentation. This is a **breaking schema change** requiring migration support.

### Goals

1. Create v2 primer schema with tiered command documentation
2. Implement simplified selection algorithm
3. Provide backward compatibility with v1
4. Create default primer v2 with all commands

### Non-Goals

- Changing annotation syntax (RFC-0001 scope)
- Modifying `acp sync` core behavior
- Altering cache format

---

## Phase 1: Foundation (Schema)

### T1.1: Create primer.v2.schema.json

**Component**: Schema
**Files**: `schemas/v1/primer.schema.json` (update)
**Depends On**: None
**Estimated Time**: 1 hour

**Description**:
Create v2 primer schema with simplified structure.

**Changes**:
- Add `version` pattern for `^2\\.\\d+\\.\\d+`
- Add `bootstrap` block definition
- Add `interface` with `cli`, `mcp`, `daemon` command sets
- Add `command` definition with `tiers` (minimal/standard/full)
- Add `tierContent` with `tokens` and `template`
- Add `projectConfig` for dynamic content
- Remove deprecated v1 elements (categories, value scoring, weights)

**Schema Structure**:
```json
{
  "version": "2.0.0",
  "bootstrap": {
    "awareness": "...",
    "workflow": "...",
    "expansion": "..."
  },
  "interface": {
    "cli": { "commands": [...] },
    "mcp": { "commands": [...] }
  },
  "project": {
    "showProtectedFiles": true,
    "customRules": "..."
  }
}
```

**Acceptance Criteria**:
- [ ] Schema validates correctly
- [ ] Draft-07 compliant
- [ ] All RFC fields included

---

### T1.2: Create primer.defaults.v2.json

**Component**: Primers
**Files**: `primers/primer.defaults.v2.json` (new)
**Depends On**: T1.1
**Estimated Time**: 2 hours

**Description**:
Create default primer with tiered command documentation.

**Commands to Document** (CLI):
| Command | Priority | Critical | Tiers |
|---------|----------|----------|-------|
| `constraints` | 1 | Yes | minimal/standard/full |
| `query file` | 2 | No | minimal/standard/full |
| `query symbol` | 3 | No | minimal/standard/full |
| `query domain` | 4 | No | minimal/standard/full |
| `map` | 5 | No | minimal/standard/full |
| `knowledge` | 6 | No | minimal/standard/full |
| `attempt` | 7 | No | minimal/standard/full |
| `primer` | 8 | No | minimal/standard/full |

**Commands to Document** (MCP):
| Tool | Priority | Critical | Tiers |
|------|----------|----------|-------|
| `acp_constraints` | 1 | Yes | minimal/standard/full |
| `acp_query` | 2 | No | minimal/standard/full |
| `acp_knowledge` | 3 | No | minimal/standard/full |
| `acp_map` | 4 | No | minimal/standard/full |
| `acp_primer` | 5 | No | minimal/standard/full |

**Acceptance Criteria**:
- [ ] All commands documented at all tiers
- [ ] Token counts accurate
- [ ] Templates clear and useful
- [ ] Validates against v2 schema

---

### T1.3: Update Chapter 11 - Tool Integration

**Component**: Specification
**Files**: `spec/chapters/11-tool-integration.md`
**Depends On**: T1.1
**Estimated Time**: 1.5 hours

**Description**:
Update primer system documentation for v2.

**Changes**:
- Update Section 7 (Primer System) for v2
- Document tier definitions (minimal/standard/full)
- Document bootstrap block
- Document command priority and critical flags
- Document selection algorithm
- Add migration notes for v1→v2
- Mark v1 elements as deprecated

**Acceptance Criteria**:
- [ ] v2 system fully documented
- [ ] v1 deprecation noted
- [ ] Migration path clear

---

### T1.4: Update Chapter 14 - Bootstrap

**Component**: Specification
**Files**: `spec/chapters/14-bootstrap.md`
**Depends On**: T1.1
**Estimated Time**: 45 minutes

**Description**:
Update bootstrap documentation for simplified primer.

**Changes**:
- Update bootstrap block documentation
- Reference RFC-0001 self-documenting directives
- Update token budget examples
- Simplify workflow section

**Acceptance Criteria**:
- [ ] Bootstrap block documented
- [ ] Token budgets updated
- [ ] Examples accurate

---

## Phase 2: Implementation (CLI)

### T2.1: Add v2 schema support to primer loader

**Component**: CLI
**Files**: `cli/src/primer/loader.rs` (or similar)
**Depends On**: T1.1
**Estimated Time**: 1.5 hours

**Description**:
Detect and load v2 primer schema.

**Changes**:
- Detect schema version from `version` field
- Load v2 structure into appropriate types
- Fall back to v1 for version `1.x.x`
- Emit deprecation warning for v1

**Acceptance Criteria**:
- [ ] v2 schema loads correctly
- [ ] v1 still works with warning
- [ ] Auto-detection works

---

### T2.2: Implement v2 selection algorithm

**Component**: CLI
**Files**: `cli/src/primer/selector.rs` (or similar)
**Depends On**: T2.1
**Estimated Time**: 2 hours

**Description**:
Implement simplified tier-based selection.

**Algorithm**:
```
1. Always include bootstrap (~20 tokens)
2. Determine tier from remaining budget:
   - <80: minimal
   - 80-299: standard
   - >=300: full
3. Sort commands by (not critical, priority)
4. Add commands until budget exhausted
5. Add project warnings if budget allows
```

**Changes**:
- Implement `select_commands_v2(budget, capabilities, commands)`
- Implement tier determination
- Implement critical command fallback
- Remove v1 multi-dimensional scoring (dead code)

**Acceptance Criteria**:
- [ ] Algorithm matches RFC specification
- [ ] Critical commands always included
- [ ] Tier selection correct
- [ ] Budget respected

---

### T2.3: Update `acp primer` command

**Component**: CLI
**Files**: `cli/src/commands/primer.rs`
**Depends On**: T2.2
**Estimated Time**: 1 hour

**Description**:
Update primer command for v2 output.

**Changes**:
- Add `--schema-version 2` flag (initially)
- Use v2 selector when v2 primer loaded
- Format output from tiered templates
- Add project warnings to output

**Acceptance Criteria**:
- [ ] v2 output correct
- [ ] Budget respected
- [ ] Capabilities filter works

---

### T2.4: Update `acp sync` for v2 preference

**Component**: CLI
**Files**: `cli/src/commands/sync.rs`
**Depends On**: T2.3
**Estimated Time**: 30 minutes

**Description**:
Update sync to prefer v2 primers.

**Changes**:
- Check for `primer.defaults.v2.json` first
- Fall back to `primer.defaults.json` (v1)
- Pass through to primer command

**Acceptance Criteria**:
- [ ] v2 preferred when available
- [ ] v1 fallback works

---

## Phase 3: Validation (Testing)

### T3.1: Unit tests for v2 schema

**Component**: CLI Tests
**Files**: `cli/tests/primer_v2.rs` (new)
**Depends On**: T1.1, T2.1
**Estimated Time**: 1 hour

**Description**:
Test v2 schema loading and validation.

**Test Cases**:
- Load valid v2 primer
- Reject invalid v2 primer
- Version detection
- v1 backward compatibility

**Acceptance Criteria**:
- [ ] All test cases pass

---

### T3.2: Unit tests for selection algorithm

**Component**: CLI Tests
**Files**: `cli/tests/primer_selection.rs` (new)
**Depends On**: T2.2
**Estimated Time**: 1.5 hours

**Description**:
Test tier-based selection algorithm.

**Test Cases**:
- Budget 40: error (too small)
- Budget 60: minimal tier, ~4 commands
- Budget 200: standard tier, ~6 commands
- Budget 500: full tier, all commands + warnings
- Critical commands always included
- Capability filtering

**Acceptance Criteria**:
- [ ] All budget scenarios correct
- [ ] Tier selection verified

---

### T3.3: Integration tests for primer output

**Component**: CLI Tests
**Files**: `cli/tests/primer_integration.rs` (new)
**Depends On**: T2.3
**Estimated Time**: 1 hour

**Description**:
Test end-to-end primer generation.

**Test Cases**:
- `acp primer --budget 60 --capabilities shell`
- `acp primer --budget 200 --capabilities mcp`
- `acp primer --budget 500` (all capabilities)
- Output contains bootstrap
- Output token count within budget

**Acceptance Criteria**:
- [ ] Output correct for all scenarios
- [ ] Token counts respected

---

## Phase 4: Documentation

### T4.1: Update schemas/README.md

**Component**: Documentation
**Files**: `schemas/README.md`
**Depends On**: T1.1
**Estimated Time**: 15 minutes

**Description**:
Document primer schema v2.

**Changes**:
- Add v2 schema entry
- Note v1 deprecation

**Acceptance Criteria**:
- [ ] v2 documented
- [ ] Deprecation noted

---

### T4.2: Add migration guide to RFC

**Component**: RFC
**Files**: `rfcs/rfc-0004-tiered-interface-primers.md`
**Depends On**: T1.2
**Estimated Time**: 30 minutes

**Description**:
Enhance migration guide in RFC.

**Changes**:
- Add complete migration examples
- Document breaking changes clearly
- Provide troubleshooting tips

**Acceptance Criteria**:
- [ ] Migration guide complete
- [ ] Examples working

---

### T4.3: Update CLI README

**Component**: Documentation
**Files**: `cli/README.md`
**Depends On**: T2.3
**Estimated Time**: 30 minutes

**Description**:
Update CLI documentation for v2 primer.

**Changes**:
- Document new primer behavior
- Note v1 deprecation timeline
- Add budget examples

**Acceptance Criteria**:
- [ ] v2 behavior documented
- [ ] Examples accurate

---

## Phase 5: Release

### T5.1: Update CHANGELOG

**Component**: Project
**Files**: `CHANGELOG.md`
**Depends On**: All previous tasks
**Estimated Time**: 15 minutes

**Description**:
Add changelog entry for RFC-0004 implementation.

**Changes**:
- Add version entry
- Note breaking change (primer schema v2)
- Document migration path

**Acceptance Criteria**:
- [ ] Version entry added
- [ ] Breaking change noted

---

### T5.2: Update RFC status

**Component**: RFC
**Files**: `rfcs/rfc-0004-tiered-interface-primers.md`
**Depends On**: All previous tasks
**Estimated Time**: 10 minutes

**Description**:
Update RFC status to Implemented.

**Changes**:
- Status: Accepted → Implemented
- Add Implemented date
- Add Release version

**Acceptance Criteria**:
- [ ] Status updated
- [ ] Dates correct

---

## Dependencies

```
T1.1 ──┬── T1.2
       │
       ├── T1.3
       │
       ├── T1.4
       │
       └── T2.1 ── T2.2 ── T2.3 ── T2.4
                    │
                    └── T3.1, T3.2, T3.3

All implementation → T4.x → T5.x
```

---

## Risk Mitigation

| Risk | Likelihood | Impact | Mitigation |
|------|------------|--------|------------|
| Breaking existing primers | High | High | v1 support for one major version |
| Token estimates inaccurate | Medium | Low | Calibrate with real usage |
| Missing v1 features | Medium | Medium | `project.customRules` for flexibility |
| Complex migration | Medium | Medium | Auto-detection, clear docs |

---

## Breaking Change Management

### v1 Deprecation Timeline

| Version | v1 Status | v2 Status |
|---------|-----------|-----------|
| 0.5.0 | Supported (warning) | Default |
| 0.6.0 | Deprecated (warning) | Required |
| 1.0.0 | Removed | Only option |

### Migration Checklist

For projects with custom primers:
- [ ] Check for custom `sections` → convert to `customRules`
- [ ] Check for `value` scoring → use `priority` instead
- [ ] Check for `selectionStrategy.weights` → remove (not needed)
- [ ] Update `version` to `2.0.0`

---

## Success Criteria

RFC-0004 implementation is complete when:
- [ ] v2 schema created and validates
- [ ] Default primer v2 created with all commands
- [ ] Selection algorithm simplified
- [ ] v1 backward compatibility works
- [ ] All tests pass
- [ ] Spec chapters updated
- [ ] Migration guide complete
- [ ] RFC status is Implemented

---

## Estimated Effort

| Phase | Tasks | Estimated Time |
|-------|-------|----------------|
| Phase 1: Foundation | 4 tasks | 5.25 hours |
| Phase 2: Implementation | 4 tasks | 5 hours |
| Phase 3: Validation | 3 tasks | 3.5 hours |
| Phase 4: Documentation | 3 tasks | 1.25 hours |
| Phase 5: Release | 2 tasks | 0.5 hours |
| **Total** | **16 tasks** | **~15.5 hours** |

---

## Next Steps

Ready for `/rfc.implement` to begin Phase 1.

Start with:
- T1.1: Create primer.v2.schema.json
- T1.2: Create primer.defaults.v2.json (after T1.1)

**Note**: This RFC has breaking changes. Recommend implementing behind feature flag first.

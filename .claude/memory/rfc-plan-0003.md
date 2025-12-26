# Implementation Plan: RFC-0003

**RFC**: RFC-0003 Annotation Provenance Tracking
**Created**: 2025-12-22
**Status**: Ready for Implementation

---

## Overview

RFC-0003 introduces annotation provenance tracking to identify auto-generated annotations that may need human review. This plan covers:
- New `@acp:source*` annotations
- Cache schema additions for provenance data
- Config schema additions for annotate settings
- CLI enhancements for query and review workflows

### Goals

1. Add `@acp:source`, `@acp:source-confidence`, `@acp:source-reviewed`, `@acp:source-id` annotations
2. Track provenance in cache with statistics
3. Enable CLI queries by source type and confidence
4. Provide interactive review workflow

### Non-Goals

- Automatic quality assessment
- Git blame integration
- Version history tracking

---

## Phase 1: Foundation (Spec & Schema)

### T1.1: Update ACP-1.0.md Appendix A

**Component**: Specification
**Files**: `spec/ACP-1.0.md`
**Depends On**: None
**Estimated Time**: 30 minutes

**Description**:
Add the four new provenance annotations to the reserved annotations table in Appendix A.

**Changes**:
- Add `@acp:source` - Annotation origin marker
- Add `@acp:source-confidence` - Confidence score (0.0-1.0)
- Add `@acp:source-reviewed` - Human review status
- Add `@acp:source-id` - Generation batch identifier

**Acceptance Criteria**:
- [ ] All 4 annotations in Appendix A table
- [ ] Examples provided for each
- [ ] RFC-0003 referenced

---

### T1.2: Update Chapter 05 - Annotations

**Component**: Specification
**Files**: `spec/chapters/05-annotations.md`
**Depends On**: T1.1
**Estimated Time**: 1 hour

**Description**:
Add new section documenting annotation provenance system.

**Changes**:
- Add Section X: Annotation Provenance (RFC-0003)
- Document source origin values: explicit, converted, heuristic, refined, inferred
- Document confidence scoring
- Document review workflow
- Add syntax examples

**Acceptance Criteria**:
- [ ] Section added with clear structure
- [ ] All origin types documented
- [ ] Syntax examples included
- [ ] Grammar extension documented

---

### T1.3: Update cache.schema.json

**Component**: Schema
**Files**: `schemas/v1/cache.schema.json`
**Depends On**: T1.1
**Estimated Time**: 45 minutes

**Description**:
Add provenance tracking to cache schema.

**Changes**:
- Add `$defs/annotation_provenance` definition
- Add `annotations` property to `file_entry`
- Add `annotations` property to `symbol_entry`
- Add top-level `provenance` statistics object

**Schema Additions**:
```json
{
  "$defs": {
    "annotation_provenance": {
      "type": "object",
      "properties": {
        "value": { "type": "string" },
        "source": { "enum": ["explicit", "converted", "heuristic", "refined", "inferred"] },
        "confidence": { "type": "number", "minimum": 0, "maximum": 1 },
        "needsReview": { "type": "boolean" },
        "reviewed": { "type": "boolean" },
        "reviewedAt": { "type": "string", "format": "date-time" },
        "generatedAt": { "type": "string", "format": "date-time" },
        "generationId": { "type": "string" }
      },
      "required": ["value"]
    }
  }
}
```

**Acceptance Criteria**:
- [ ] Schema validates correctly
- [ ] All fields from RFC included
- [ ] Backward compatible (annotations optional)

---

### T1.4: Update config.schema.json

**Component**: Schema
**Files**: `schemas/v1/config.schema.json`
**Depends On**: None
**Estimated Time**: 30 minutes

**Description**:
Add annotate configuration section.

**Changes**:
- Add `annotate` object to config schema
- Add `annotate.provenance` settings
- Add `annotate.defaults` settings

**Schema Additions**:
```json
{
  "annotate": {
    "type": "object",
    "properties": {
      "provenance": {
        "type": "object",
        "properties": {
          "enabled": { "type": "boolean", "default": true },
          "includeConfidence": { "type": "boolean", "default": true },
          "reviewThreshold": { "type": "number", "default": 0.8 },
          "minConfidence": { "type": "number", "default": 0.5 }
        }
      },
      "defaults": {
        "type": "object",
        "properties": {
          "markNeedsReview": { "type": "boolean", "default": false },
          "overwriteExisting": { "type": "boolean", "default": false }
        }
      }
    }
  }
}
```

**Acceptance Criteria**:
- [ ] Schema validates correctly
- [ ] Defaults documented
- [ ] Config section documented in spec

---

## Phase 2: Implementation (CLI)

### T2.1: Add provenance annotations to parser

**Component**: CLI
**Files**: `cli/src/parse/mod.rs`, `cli/src/parse/annotations.rs`
**Depends On**: T1.1, T1.2
**Estimated Time**: 2 hours

**Description**:
Extend annotation parser to recognize `@acp:source*` annotations.

**Changes**:
- Add `SourceAnnotation` enum variant
- Parse `@acp:source <origin>`
- Parse `@acp:source-confidence <value>`
- Parse `@acp:source-reviewed <bool>`
- Parse `@acp:source-id <uuid>`
- Associate with preceding annotations

**Acceptance Criteria**:
- [ ] All 4 annotations parsed correctly
- [ ] Origin values validated
- [ ] Confidence range validated (0.0-1.0)
- [ ] Unit tests pass

---

### T2.2: Update cache types for provenance

**Component**: CLI
**Files**: `cli/src/cache/types.rs`
**Depends On**: T1.3
**Estimated Time**: 1 hour

**Description**:
Add Rust types for annotation provenance.

**Changes**:
- Add `AnnotationProvenance` struct
- Add `Source` enum (explicit, converted, heuristic, refined, inferred)
- Add `ProvenanceStats` struct for top-level statistics
- Update `FileEntry` and `SymbolEntry`

**Acceptance Criteria**:
- [ ] Types match schema
- [ ] Serialization/deserialization works
- [ ] Backward compatible (optional fields)

---

### T2.3: Update indexer for provenance tracking

**Component**: CLI
**Files**: `cli/src/index/indexer.rs`
**Depends On**: T2.1, T2.2
**Estimated Time**: 2 hours

**Description**:
Track annotation provenance during indexing.

**Changes**:
- Extract `@acp:source*` during file parsing
- Store provenance in cache entries
- Calculate provenance statistics
- Populate `lowConfidence` list

**Acceptance Criteria**:
- [ ] Provenance captured for all annotations
- [ ] Statistics calculated correctly
- [ ] Low confidence threshold applied

---

### T2.4: Update `acp annotate` command

**Component**: CLI
**Files**: `cli/src/commands/annotate.rs`, `cli/src/annotate/writer.rs`
**Depends On**: T2.2
**Estimated Time**: 2 hours

**Description**:
Enhance annotate command to emit provenance markers.

**Changes**:
- Add `--no-provenance` flag
- Add `--min-confidence` flag
- Add `--needs-review` flag
- Write `@acp:source` after generated annotations
- Write `@acp:source-confidence` if enabled
- Track generation batch ID

**Acceptance Criteria**:
- [ ] Provenance markers written correctly
- [ ] Flags work as documented
- [ ] Existing behavior preserved with `--no-provenance`

---

### T2.5: Add `acp query` provenance filters

**Component**: CLI
**Files**: `cli/src/commands/query.rs`
**Depends On**: T2.3
**Estimated Time**: 1.5 hours

**Description**:
Add provenance filtering to query command.

**Changes**:
- Add `--source <origin>` filter
- Add `--confidence <expr>` filter (e.g., "<0.7")
- Add `--needs-review` filter
- Update output to show provenance info

**Acceptance Criteria**:
- [ ] Filters work correctly
- [ ] Combinable with existing filters
- [ ] JSON output includes provenance

---

### T2.6: Add `acp stats --provenance`

**Component**: CLI
**Files**: `cli/src/commands/stats.rs`
**Depends On**: T2.3
**Estimated Time**: 1 hour

**Description**:
Add provenance statistics display.

**Changes**:
- Add `--provenance` flag
- Display by-source breakdown
- Display review status
- Display average confidence
- Display low-confidence count

**Acceptance Criteria**:
- [ ] Statistics display correctly
- [ ] Percentages calculated
- [ ] Formatted output readable

---

### T2.7: Add `acp review` command (new)

**Component**: CLI
**Files**: `cli/src/commands/review.rs` (new)
**Depends On**: T2.3, T2.5
**Estimated Time**: 3 hours

**Description**:
Implement interactive review command.

**Changes**:
- Create new `review` subcommand
- Add `--source` filter
- Add `--confidence` filter
- Add `--domain` filter
- Implement interactive review flow
- Add `--mark-reviewed` bulk option

**Acceptance Criteria**:
- [ ] Interactive mode works
- [ ] Accept/Edit/Regenerate/Skip actions
- [ ] Bulk mark-reviewed works
- [ ] Updates source to reviewed

---

## Phase 3: Validation (Testing)

### T3.1: Unit tests for parser

**Component**: CLI Tests
**Files**: `cli/tests/parse_provenance.rs` (new)
**Depends On**: T2.1
**Estimated Time**: 1 hour

**Description**:
Add unit tests for provenance annotation parsing.

**Test Cases**:
- Parse single `@acp:source`
- Parse with confidence
- Parse with reviewed flag
- Parse with generation ID
- Parse mixed provenance blocks
- Invalid origin values
- Invalid confidence values

**Acceptance Criteria**:
- [ ] All test cases pass
- [ ] Edge cases covered

---

### T3.2: Integration tests for indexing

**Component**: CLI Tests
**Files**: `cli/tests/index_provenance.rs` (new)
**Depends On**: T2.3
**Estimated Time**: 1.5 hours

**Description**:
Test provenance tracking through full indexing.

**Test Cases**:
- Index file with provenance annotations
- Verify cache provenance data
- Verify statistics calculation
- Verify low-confidence detection

**Acceptance Criteria**:
- [ ] Cache output matches expected
- [ ] Statistics accurate

---

### T3.3: Integration tests for CLI commands

**Component**: CLI Tests
**Files**: `cli/tests/commands_provenance.rs` (new)
**Depends On**: T2.4, T2.5, T2.6, T2.7
**Estimated Time**: 2 hours

**Description**:
Test all provenance CLI commands.

**Test Cases**:
- `acp annotate` with/without provenance
- `acp query --source` filtering
- `acp query --confidence` filtering
- `acp stats --provenance` output
- `acp review --mark-reviewed`

**Acceptance Criteria**:
- [ ] All commands work correctly
- [ ] Output formats verified

---

## Phase 4: Documentation

### T4.1: Update Chapter 04 - Config Format

**Component**: Specification
**Files**: `spec/chapters/04-config-format.md`
**Depends On**: T1.4
**Estimated Time**: 30 minutes

**Description**:
Document the `annotate` configuration section.

**Changes**:
- Add Section X: Annotate Configuration (RFC-0003)
- Document all settings with examples

**Acceptance Criteria**:
- [ ] Section added
- [ ] All settings documented

---

### T4.2: Update Chapter 03 - Cache Format

**Component**: Specification
**Files**: `spec/chapters/03-cache-format.md`
**Depends On**: T1.3
**Estimated Time**: 30 minutes

**Description**:
Document the provenance fields in cache format.

**Changes**:
- Add Section X: Provenance Tracking (RFC-0003)
- Document `annotations` in file/symbol entries
- Document top-level `provenance` statistics

**Acceptance Criteria**:
- [ ] Section added
- [ ] All fields documented

---

### T4.3: Update CLI README

**Component**: Documentation
**Files**: `cli/README.md`
**Depends On**: T2.4, T2.5, T2.6, T2.7
**Estimated Time**: 45 minutes

**Description**:
Update CLI documentation with provenance features.

**Changes**:
- Add `acp review` command documentation
- Document new flags for `acp annotate`
- Document new flags for `acp query`
- Document `acp stats --provenance`

**Acceptance Criteria**:
- [ ] All new commands/flags documented
- [ ] Examples provided

---

## Phase 5: Release

### T5.1: Update CHANGELOG

**Component**: Project
**Files**: `CHANGELOG.md`
**Depends On**: All previous tasks
**Estimated Time**: 15 minutes

**Description**:
Add changelog entry for RFC-0003 implementation.

**Acceptance Criteria**:
- [ ] Version entry added
- [ ] All features listed
- [ ] Breaking changes noted (if any)

---

### T5.2: Update RFC status

**Component**: RFC
**Files**: `rfcs/rfc-0003-annotation-provenance-tracking.md`
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
T1.1 ──┬── T1.2 ──┐
       │          │
       └── T1.3 ──┼── T2.1 ── T2.2 ──┬── T2.3 ──┬── T2.5 ── T2.7
                  │                  │          │
T1.4 ─────────────┴──────────────────┴── T2.4   └── T2.6
                                          │
                                          └── T3.1, T3.2, T3.3

All implementation → T4.x → T5.x
```

---

## Risk Mitigation

| Risk | Likelihood | Impact | Mitigation |
|------|------------|--------|------------|
| Cache size growth | Medium | Low | Provenance optional, defaults not stored |
| Annotation verbosity | Medium | Low | `--no-provenance` flag |
| Parser complexity | Low | Medium | Follow existing annotation patterns |

---

## Success Criteria

RFC-0003 implementation is complete when:
- [ ] All 4 annotations documented in spec
- [ ] Schemas updated and validate
- [ ] `acp annotate` emits provenance markers
- [ ] `acp query` supports provenance filters
- [ ] `acp stats --provenance` works
- [ ] `acp review` command functional
- [ ] All tests pass
- [ ] Documentation updated
- [ ] RFC status is Implemented

---

## Estimated Effort

| Phase | Tasks | Estimated Time |
|-------|-------|----------------|
| Phase 1: Foundation | 4 tasks | 2.75 hours |
| Phase 2: Implementation | 7 tasks | 12.5 hours |
| Phase 3: Validation | 3 tasks | 4.5 hours |
| Phase 4: Documentation | 3 tasks | 1.75 hours |
| Phase 5: Release | 2 tasks | 0.5 hours |
| **Total** | **19 tasks** | **~22 hours** |

---

## Next Steps

Ready for `/rfc.implement` to begin Phase 1.

Start with:
- T1.1: Update ACP-1.0.md Appendix A
- T1.4: Update config.schema.json (parallel)

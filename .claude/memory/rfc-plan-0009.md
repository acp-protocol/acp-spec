# Implementation Plan: RFC-0009 Extended Annotation Types

**RFC**: 0009
**Title**: Extended Annotation Types
**Status**: Draft
**Generated**: 2025-12-24

---

## Executive Summary

RFC-0009 extends ACP's annotation vocabulary with 30+ new annotation types across 7 categories: file-level, symbol-level, signature, behavioral, lifecycle, documentation, and performance. This enables ACP to provide complete documentation coverage without relying on native documentation systems.

---

## Affected Components

### Specification (acp-spec)

| Component | Impact | Changes Required |
|-----------|--------|------------------|
| `spec/chapters/05-annotations.md` | HIGH | Add 7 new annotation categories with full syntax |
| `spec/chapters/03-cache-format.md` | HIGH | Document new cache fields |
| `spec/ACP-1.0.md` | MEDIUM | Update Appendix A with new annotations |
| `schemas/v1/cache.schema.json` | HIGH | Add behavioral, lifecycle, documentation, performance fields |
| `schemas/v1/config.schema.json` | LOW | Optional: Add annotation generation config |

### CLI (acp-cli)

| Component | Impact | Changes Required |
|-----------|--------|------------------|
| `src/parse/mod.rs` | HIGH | Parse 30+ new annotation types |
| `src/cache/types.rs` | HIGH | Add new annotation storage types |
| `src/index/indexer.rs` | MEDIUM | Extract and store new annotations |
| `src/annotate/` | MEDIUM | Generate new annotation types |
| `src/commands/query.rs` | LOW | Query support for new fields |

### Dependencies

```
RFC-0009 (Extended Annotation Types)
├── RFC-0001 (Self-Documenting Annotations) - Foundation
├── RFC-0003 (Provenance Tracking) - Source tracking
└── RFC-0010 (Documentation Generator) - Rendering
    └── Requires RFC-0009 annotations for badges/sections
```

---

## Phased Implementation

### Phase 1: Schema Design (3 days)

**Objective**: Define complete data structures for all annotation categories.

#### Tasks

1.1 **Design behavioral annotations schema**
   - Add `behavioral` object to `symbol_entry`
   - Fields: `pure`, `idempotent`, `memoized`, `async`, `generator`, `sideEffects`
   - Define TypeScript/Rust types

1.2 **Design lifecycle annotations schema**
   - Add `lifecycle` object to `symbol_entry` and `file_entry`
   - Fields: `deprecated`, `experimental`, `beta`, `internal`, `publicApi`, `since`
   - Include deprecation message and replacement info

1.3 **Design documentation annotations schema**
   - Add `documentation` object to `symbol_entry`
   - Fields: `examples[]`, `seeAlso[]`, `links[]`, `notes[]`, `warnings[]`, `todos[]`
   - Support multiline examples

1.4 **Design performance annotations schema**
   - Add `performance` object to `symbol_entry`
   - Fields: `complexity`, `memory`, `cached`
   - Allow custom notation (O(n), O(1), etc.)

1.5 **Design file-level extensions**
   - Add to `file_entry`: `version`, `since`, `license`, `author`, `domain` (already exists)
   - Enhance `module` with full name support

1.6 **Update cache.schema.json**
   - Integrate all new definitions
   - Validate with existing schema tests

#### Deliverables
- [ ] Updated `schemas/v1/cache.schema.json`
- [ ] Schema validation tests pass
- [ ] Draft type definitions for CLI

---

### Phase 2: Parser Extension (4 days)

**Objective**: Parse all 30+ new annotation types from source code.

#### Tasks

2.1 **Extend annotation regex patterns**
   - File-level: `@acp:purpose`, `@acp:module`, `@acp:domain`, `@acp:owner`, `@acp:layer`, `@acp:stability`, `@acp:version`, `@acp:since`, `@acp:license`, `@acp:author`
   - Symbol-level: `@acp:fn`, `@acp:class`, `@acp:method`, `@acp:interface`, `@acp:type`, `@acp:enum`, `@acp:const`, `@acp:var`, `@acp:property`

2.2 **Add signature annotation parsing**
   - `@acp:param`, `@acp:returns`, `@acp:throws`, `@acp:yields`
   - `@acp:async`, `@acp:generator`, `@acp:template`
   - Handle type annotations in braces `{string}`

2.3 **Add behavioral annotation parsing**
   - `@acp:pure`, `@acp:idempotent`, `@acp:memoized`
   - `@acp:throttled`, `@acp:transactional`, `@acp:side-effects`

2.4 **Add lifecycle annotation parsing**
   - `@acp:deprecated`, `@acp:experimental`, `@acp:beta`
   - `@acp:internal`, `@acp:public-api`

2.5 **Add documentation annotation parsing**
   - `@acp:example`, `@acp:see`, `@acp:link`, `@acp:note`, `@acp:warning`, `@acp:todo`
   - Support multiline content

2.6 **Add performance annotation parsing**
   - `@acp:perf`, `@acp:memory`, `@acp:cached`

2.7 **Update ParseResult struct**
   - Add fields for all new annotation categories
   - Ensure provenance tracking (RFC-0003) applies

#### Deliverables
- [ ] Extended regex patterns in `parse/mod.rs`
- [ ] Updated `ParseResult` struct
- [ ] Parser unit tests for each annotation type

---

### Phase 3: Cache Integration (3 days)

**Objective**: Store extracted annotations in cache.

#### Tasks

3.1 **Update cache types**
   - Add `BehavioralAnnotations` struct
   - Add `LifecycleAnnotations` struct
   - Add `DocumentationAnnotations` struct
   - Add `PerformanceAnnotations` struct

3.2 **Update FileEntry**
   - Add `version`, `since`, `license`, `author` fields
   - Enhance existing `stability` with full RFC-0009 lifecycle

3.3 **Update SymbolEntry**
   - Add `behavioral: Option<BehavioralAnnotations>`
   - Add `lifecycle: Option<LifecycleAnnotations>`
   - Add `documentation: Option<DocumentationAnnotations>`
   - Add `performance: Option<PerformanceAnnotations>`

3.4 **Update indexer**
   - Extract new annotations during file parsing
   - Populate cache entries with annotation data
   - Handle sparse representation (only store present annotations)

3.5 **Serialization tests**
   - JSON round-trip tests for all new types
   - Schema validation of generated cache

#### Deliverables
- [ ] New types in `cache/types.rs`
- [ ] Updated `FileEntry` and `SymbolEntry`
- [ ] Indexer integration
- [ ] Serialization tests

---

### Phase 4: Specification Documentation (2 days)

**Objective**: Update specification with complete documentation.

#### Tasks

4.1 **Update Chapter 05 (Annotations)**
   - Add Section 7.4: File-Level Annotations
   - Add Section 7.5: Symbol-Level Annotations
   - Add Section 7.6: Behavioral Annotations
   - Add Section 7.7: Lifecycle Annotations
   - Add Section 7.8: Documentation Annotations
   - Add Section 7.9: Performance Annotations

4.2 **Update Chapter 03 (Cache Format)**
   - Document new `file_entry` fields
   - Document new `symbol_entry` fields
   - Add examples

4.3 **Update Appendix A**
   - Add all new annotations to reserved annotations table
   - Include value types and examples

4.4 **Update ACP-1.0.md**
   - Add RFC-0009 to version history
   - Cross-reference new sections

#### Deliverables
- [ ] Updated `spec/chapters/05-annotations.md`
- [ ] Updated `spec/chapters/03-cache-format.md`
- [ ] Updated `spec/ACP-1.0.md` Appendix A

---

### Phase 5: Testing (2 days)

**Objective**: Comprehensive test coverage.

#### Tasks

5.1 **Parser unit tests**
   - Test each annotation type in isolation
   - Test value extraction
   - Test multiline handling
   - Test error cases

5.2 **Integration tests**
   - Full file parsing with multiple annotations
   - Cross-language support (TypeScript, Python, Rust)

5.3 **Schema validation tests**
   - Validate generated cache against schema
   - Test sparse representation

5.4 **Regression tests**
   - Ensure existing functionality unaffected
   - Run full test suite

#### Deliverables
- [ ] Parser tests in `parse/mod.rs`
- [ ] Integration tests in `tests/`
- [ ] All tests passing

---

## Timeline Summary

| Phase | Duration | Cumulative |
|-------|----------|------------|
| Phase 1: Schema Design | 3 days | Day 3 |
| Phase 2: Parser Extension | 4 days | Day 7 |
| Phase 3: Cache Integration | 3 days | Day 10 |
| Phase 4: Specification | 2 days | Day 12 |
| Phase 5: Testing | 2 days | Day 14 |
| **Total** | **14 days** | ~2 weeks |

---

## Risks and Mitigations

| Risk | Impact | Likelihood | Mitigation |
|------|--------|------------|------------|
| Annotation explosion (30+ types) | Cache size increase | Medium | Sparse representation (only store present) |
| Parser complexity | Bug introduction | Medium | Incremental implementation, comprehensive tests |
| Breaking changes | User migration | Low | All new fields optional |
| Overlap with existing | Confusion | Low | Clear documentation on when to use each |

---

## Success Criteria

1. **Schema Complete**: All 30+ annotations represented in cache.schema.json
2. **Parser Coverage**: All annotation types parsed correctly with tests
3. **Cache Integration**: Annotations stored in cache with provenance
4. **Backward Compatible**: Existing caches remain valid
5. **Documentation**: Spec updated with complete annotation reference
6. **Tests Pass**: Full test suite including new tests

---

## Open Questions

1. **Example multiline handling**: How should `@acp:example` handle multiline code blocks?
   - Proposal: Allow continuation with subsequent `@acp:example+` lines

2. **Deprecation format**: What format for `@acp:deprecated` replacement info?
   - Proposal: `@acp:deprecated "2.0" - Use newFunction instead`

3. **Performance notation**: Standardize complexity notation?
   - Proposal: Free-form string, common examples in docs

---

## Next Steps

1. Review and approve this plan
2. Begin Phase 1: Schema Design
3. Create task list in `.claude/memory/rfc-tasks-0009.md`

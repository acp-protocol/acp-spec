# Task Breakdown: RFC-0009 Extended Annotation Types

**RFC**: 0009
**Generated**: 2025-12-24
**Total Tasks**: 32
**Estimated Duration**: ~14 days

---

## Phase 1: Schema Design

### P1.1 Behavioral Annotations Schema
- [ ] **T1.1.1** Add `BehavioralAnnotations` definition to cache.schema.json
  - Properties: `pure`, `idempotent`, `memoized`, `async`, `generator`, `sideEffects[]`
  - Priority: HIGH

- [ ] **T1.1.2** Add `behavioral` field to `symbol_entry` definition
  - Reference: `$ref: "#/$defs/behavioral_annotations"`
  - Priority: HIGH

### P1.2 Lifecycle Annotations Schema
- [ ] **T1.2.1** Add `LifecycleAnnotations` definition to cache.schema.json
  - Properties: `deprecated`, `experimental`, `beta`, `internal`, `publicApi`, `since`
  - Priority: HIGH

- [ ] **T1.2.2** Add `lifecycle` field to `symbol_entry`
  - Priority: HIGH

- [ ] **T1.2.3** Add `lifecycle` field to `file_entry`
  - Properties: `deprecated`, `experimental`, `internal`
  - Priority: MEDIUM

### P1.3 Documentation Annotations Schema
- [ ] **T1.3.1** Add `DocumentationAnnotations` definition
  - Properties: `examples[]`, `seeAlso[]`, `links[]`, `notes[]`, `warnings[]`, `todos[]`
  - Priority: HIGH

- [ ] **T1.3.2** Add `documentation` field to `symbol_entry`
  - Priority: HIGH

### P1.4 Performance Annotations Schema
- [ ] **T1.4.1** Add `PerformanceAnnotations` definition
  - Properties: `complexity`, `memory`, `cached`
  - Priority: MEDIUM

- [ ] **T1.4.2** Add `performance` field to `symbol_entry`
  - Priority: MEDIUM

### P1.5 File-Level Extensions
- [ ] **T1.5.1** Add new fields to `file_entry`
  - Fields: `version`, `since`, `license`, `author`
  - Priority: MEDIUM

### P1.6 Schema Validation
- [ ] **T1.6.1** Run schema validation tests
  - Ensure all JSON Schema references resolve
  - Priority: HIGH

- [ ] **T1.6.2** Create test fixtures for new schema sections
  - Priority: HIGH

---

## Phase 2: Parser Extension

### P2.1 File-Level Annotation Parsing
- [ ] **T2.1.1** Add regex for file-level annotations
  - `@acp:purpose`, `@acp:module`, `@acp:version`, `@acp:since`, `@acp:license`, `@acp:author`
  - Priority: HIGH

- [ ] **T2.1.2** Update `ParseResult` with file-level fields
  - Priority: HIGH

### P2.2 Symbol-Level Annotation Parsing
- [ ] **T2.2.1** Add regex for symbol description annotations
  - `@acp:fn`, `@acp:class`, `@acp:method`, `@acp:interface`, `@acp:type`, `@acp:enum`, `@acp:const`, `@acp:var`, `@acp:property`
  - Priority: HIGH

### P2.3 Behavioral Annotation Parsing
- [ ] **T2.3.1** Add regex for behavioral annotations
  - `@acp:pure`, `@acp:idempotent`, `@acp:memoized`, `@acp:throttled`, `@acp:transactional`, `@acp:side-effects`
  - Priority: HIGH

- [ ] **T2.3.2** Handle parameterized behavioral annotations
  - `@acp:throttled "100/min"`, `@acp:side-effects "db,network"`
  - Priority: MEDIUM

### P2.4 Lifecycle Annotation Parsing
- [ ] **T2.4.1** Add regex for lifecycle annotations
  - `@acp:deprecated`, `@acp:experimental`, `@acp:beta`, `@acp:internal`, `@acp:public-api`
  - Priority: HIGH

- [ ] **T2.4.2** Parse deprecation version and replacement
  - `@acp:deprecated "2.0" - Use newFn instead`
  - Priority: HIGH

### P2.5 Documentation Annotation Parsing
- [ ] **T2.5.1** Add regex for documentation annotations
  - `@acp:example`, `@acp:see`, `@acp:link`, `@acp:note`, `@acp:warning`, `@acp:todo`
  - Priority: HIGH

- [ ] **T2.5.2** Handle multiline examples
  - Support code block continuation
  - Priority: MEDIUM

### P2.6 Performance Annotation Parsing
- [ ] **T2.6.1** Add regex for performance annotations
  - `@acp:perf`, `@acp:memory`, `@acp:cached`
  - Priority: MEDIUM

### P2.7 Parser Tests
- [ ] **T2.7.1** Unit tests for each annotation type
  - Minimum 2 tests per annotation
  - Priority: HIGH

---

## Phase 3: Cache Integration

### P3.1 Type Definitions
- [ ] **T3.1.1** Define `BehavioralAnnotations` struct in Rust
  - Priority: HIGH

- [ ] **T3.1.2** Define `LifecycleAnnotations` struct
  - Priority: HIGH

- [ ] **T3.1.3** Define `DocumentationAnnotations` struct
  - Priority: HIGH

- [ ] **T3.1.4** Define `PerformanceAnnotations` struct
  - Priority: MEDIUM

### P3.2 Entry Updates
- [ ] **T3.2.1** Update `FileEntry` with new fields
  - Priority: HIGH

- [ ] **T3.2.2** Update `SymbolEntry` with new annotation fields
  - Priority: HIGH

### P3.3 Indexer Integration
- [ ] **T3.3.1** Extract annotations in file parsing loop
  - Priority: HIGH

- [ ] **T3.3.2** Populate `SymbolEntry` with behavioral/lifecycle/doc/perf
  - Priority: HIGH

### P3.4 Serialization
- [ ] **T3.4.1** Serde serialization for new types
  - Priority: HIGH

- [ ] **T3.4.2** JSON round-trip tests
  - Priority: HIGH

---

## Phase 4: Specification Documentation

### P4.1 Annotation Chapter
- [ ] **T4.1.1** Add Section 7.4-7.9 to Chapter 05
  - Document all annotation categories
  - Priority: HIGH

### P4.2 Cache Format Chapter
- [ ] **T4.2.1** Update Chapter 03 with new fields
  - Priority: MEDIUM

### P4.3 Appendix Updates
- [ ] **T4.3.1** Update Appendix A with all new annotations
  - Priority: MEDIUM

---

## Phase 5: Testing

### P5.1 Unit Tests
- [ ] **T5.1.1** Parser tests (~30 tests)
  - Priority: HIGH

### P5.2 Integration Tests
- [ ] **T5.2.1** Full file parsing tests
  - Priority: HIGH

### P5.3 Regression
- [ ] **T5.3.1** Run full test suite
  - Priority: HIGH

---

## Task Summary

| Phase | Tasks | Critical | High | Medium |
|-------|-------|----------|------|--------|
| P1: Schema | 12 | 0 | 10 | 2 |
| P2: Parser | 8 | 0 | 6 | 2 |
| P3: Cache | 8 | 0 | 7 | 1 |
| P4: Spec | 3 | 0 | 1 | 2 |
| P5: Testing | 3 | 0 | 3 | 0 |
| **Total** | **34** | **0** | **27** | **7** |

---

## Dependency Graph

```
P1.1 (Behavioral Schema) ─┬─> P2.3 (Behavioral Parser) ─> P3.1.1 (Behavioral Types)
P1.2 (Lifecycle Schema) ──┼─> P2.4 (Lifecycle Parser) ──> P3.1.2 (Lifecycle Types)
P1.3 (Documentation)  ────┼─> P2.5 (Doc Parser) ────────> P3.1.3 (Doc Types)
P1.4 (Performance) ───────┴─> P2.6 (Perf Parser) ───────> P3.1.4 (Perf Types)
                                     │
                                     v
                              P3.3 (Indexer Integration)
                                     │
                                     v
                              P4 (Documentation)
                                     │
                                     v
                              P5 (Testing)
```

---

## Acceptance Criteria

- [ ] All 30+ annotation types defined in schema
- [ ] All annotation types parsed correctly
- [ ] Cache stores annotations with provenance
- [ ] 34+ tests pass
- [ ] Specification complete
- [ ] Backward compatible (no breaking changes)

---

## Notes

1. **Sparse Representation**: Only store annotations that are present (omit empty objects)
2. **Provenance**: All annotations use RFC-0003 provenance tracking
3. **Multiline**: `@acp:example` may need special multiline handling
4. **RFC-0010 Integration**: Annotations rendered in docs (future RFC)

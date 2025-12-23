# RFC-0006 Task List

**RFC**: RFC-0006 Documentation System Bridging
**Created**: 2025-12-23
**Total Tasks**: 19
**Estimated Time**: 26.5 hours

---

## Task Summary

| Task ID | Phase | Description | Depends On | Est. | Status |
|---------|-------|-------------|------------|------|--------|
| T1.1 | 1 | Update config.schema.json | - | 1h | Pending |
| T1.2 | 1 | Update cache.schema.json | - | 1h | Pending |
| T1.3 | 1 | Update Rust cache types | T1.2 | 1h | Pending |
| T2.1 | 2 | Create bridge module structure | T1.3 | 30m | Pending |
| T2.2 | 2 | Implement BridgeConfig | T2.1 | 1h | Pending |
| T2.3 | 2 | Implement FormatDetector | T2.1 | 1.5h | Pending |
| T2.4 | 2 | Implement BridgeMerger | T2.2, T2.3 | 3h | Pending |
| T3.1 | 3 | Add bridge to Indexer struct | T2.4 | 1h | Pending |
| T3.2 | 3 | Integrate JSDoc bridging | T3.1 | 1.5h | Pending |
| T3.3 | 3 | Integrate Python docstring bridging | T3.1 | 1.5h | Pending |
| T3.4 | 3 | Track bridge statistics | T3.2, T3.3 | 1h | Pending |
| T4.1 | 4 | Add --bridge flag to index | T3.4 | 30m | Pending |
| T4.2 | 4 | Create bridge command | T3.4 | 1h | Pending |
| T4.3 | 4 | Implement bridge status | T4.2 | 1.5h | Pending |
| T4.4 | 4 | Update acp init for bridging | T4.1 | 1h | Pending |
| T5.1 | 5 | Unit tests for bridge module | T2.4 | 2h | Pending |
| T5.2 | 5 | Integration tests | T4.3 | 3h | Pending |
| T5.3 | 5 | Performance benchmarks | T3.4 | 1h | Pending |
| T6.1 | 6 | Update CLI help text | T4.4 | 30m | Pending |
| T6.2 | 6 | Create bridging spec chapter | T5.2 | 2h | Pending |

---

## Phase 1: Foundation (3h)

### T1.1: Update config.schema.json

| Field | Value |
|-------|-------|
| **Phase** | 1 |
| **Component** | schemas |
| **File** | `schemas/v1/config.schema.json` |
| **Depends On** | None |
| **Estimated Time** | 1h |
| **Status** | Pending |

**Description**:
Add the `bridge` configuration section to config schema with all bridging options.

**Acceptance Criteria**:
- [ ] `bridge.enabled` boolean (default: false)
- [ ] `bridge.precedence` enum: acp-first, native-first, merge
- [ ] `bridge.strictness` enum: permissive, strict
- [ ] `bridge.jsdoc.enabled` boolean
- [ ] `bridge.python.enabled` boolean
- [ ] `bridge.python.docstringStyle` enum: auto, google, numpy, sphinx
- [ ] Schema validates successfully

---

### T1.2: Update cache.schema.json

| Field | Value |
|-------|-------|
| **Phase** | 1 |
| **Component** | schemas |
| **File** | `schemas/v1/cache.schema.json` |
| **Depends On** | None |
| **Estimated Time** | 1h |
| **Status** | Pending |

**Description**:
Add bridging provenance fields to cache schema for tracking annotation sources.

**Acceptance Criteria**:
- [ ] `param_entry.typeSource` enum: type_hint, jsdoc, docstring
- [ ] `param_entry.source` enum: explicit, converted, merged
- [ ] `param_entry.sourceFormat` string
- [ ] `param_entry.sourceFormats` array
- [ ] `file_entry.bridge_metadata` object
- [ ] `symbol_entry.bridge_metadata` object
- [ ] All fields optional (backward compatible)

---

### T1.3: Update Rust cache types

| Field | Value |
|-------|-------|
| **Phase** | 1 |
| **Component** | cli |
| **File** | `cli/src/cache/types.rs` |
| **Depends On** | T1.2 |
| **Estimated Time** | 1h |
| **Status** | Pending |

**Description**:
Add corresponding Rust types for cache schema additions.

**Acceptance Criteria**:
- [ ] `BridgeMetadata` struct defined
- [ ] `TypeSource` enum defined
- [ ] `BridgeSource` enum: Explicit, Converted, Merged
- [ ] Fields added to `SymbolEntry`
- [ ] Serde serialization correct
- [ ] Compiles without warnings

---

## Phase 2: Bridge Module (6h)

### T2.1: Create bridge module structure

| Field | Value |
|-------|-------|
| **Phase** | 2 |
| **Component** | cli |
| **Files** | `cli/src/bridge/mod.rs` |
| **Depends On** | T1.3 |
| **Estimated Time** | 30m |
| **Status** | Pending |

**Description**:
Create the bridge module with submodules and exports.

**Acceptance Criteria**:
- [ ] `cli/src/bridge/` directory created
- [ ] `mod.rs` with module docs and exports
- [ ] `config.rs` file stub
- [ ] `merger.rs` file stub
- [ ] `detector.rs` file stub
- [ ] Added to `lib.rs`

---

### T2.2: Implement BridgeConfig

| Field | Value |
|-------|-------|
| **Phase** | 2 |
| **Component** | cli |
| **File** | `cli/src/bridge/config.rs` |
| **Depends On** | T2.1 |
| **Estimated Time** | 1h |
| **Status** | Pending |

**Description**:
Configuration loading and language enable checks.

**Acceptance Criteria**:
- [ ] `BridgeConfig` struct with all fields
- [ ] `Precedence` enum: AcpFirst, NativeFirst, Merge
- [ ] `Strictness` enum: Permissive, Strict
- [ ] `PythonConfig` with docstring_style
- [ ] `from_config(&Config)` constructor
- [ ] `is_language_enabled(Language)` method
- [ ] Defaults match schema defaults

---

### T2.3: Implement FormatDetector

| Field | Value |
|-------|-------|
| **Phase** | 2 |
| **Component** | cli |
| **File** | `cli/src/bridge/detector.rs` |
| **Depends On** | T2.1 |
| **Estimated Time** | 1.5h |
| **Status** | Pending |

**Description**:
Detect Python docstring format from content.

**Acceptance Criteria**:
- [ ] `DocstringStyle` enum: Google, NumPy, Sphinx, Unknown
- [ ] `detect_docstring_style(content)` function
- [ ] Regex patterns for each style
- [ ] Google style: `Args:`, `Returns:`
- [ ] NumPy style: `Parameters\n----------`
- [ ] Sphinx style: `:param`, `:returns:`
- [ ] `has_doc_blocks(content, language)` function

---

### T2.4: Implement BridgeMerger

| Field | Value |
|-------|-------|
| **Phase** | 2 |
| **Component** | cli |
| **File** | `cli/src/bridge/merger.rs` |
| **Depends On** | T2.2, T2.3 |
| **Estimated Time** | 3h |
| **Status** | Pending |

**Description**:
Core merge logic applying precedence rules.

**Acceptance Criteria**:
- [ ] `BridgeMerger` struct
- [ ] `merge()` method with signature
- [ ] `MergeResult` struct with annotations and metadata
- [ ] acp-first mode: ACP directive overrides native
- [ ] native-first mode: native description authoritative
- [ ] merge mode: concatenate descriptions
- [ ] Provenance tracked for each field
- [ ] Count converted/merged/explicit

---

## Phase 3: Indexer Integration (5h)

### T3.1: Add bridge to Indexer struct

| Field | Value |
|-------|-------|
| **Phase** | 3 |
| **Component** | cli |
| **File** | `cli/src/index/indexer.rs` |
| **Depends On** | T2.4 |
| **Estimated Time** | 1h |
| **Status** | Pending |

**Description**:
Add bridge configuration and merger to Indexer.

**Acceptance Criteria**:
- [ ] `bridge_config: BridgeConfig` field
- [ ] `bridge_merger: BridgeMerger` field
- [ ] Initialize in `Indexer::new()`
- [ ] Skip bridging when disabled
- [ ] Compiles without errors

---

### T3.2: Integrate JSDoc bridging

| Field | Value |
|-------|-------|
| **Phase** | 3 |
| **Component** | cli |
| **File** | `cli/src/index/indexer.rs` |
| **Depends On** | T3.1 |
| **Estimated Time** | 1.5h |
| **Status** | Pending |

**Description**:
Call JSDoc converter during JS/TS file processing.

**Acceptance Criteria**:
- [ ] Extract doc comments from JS/TS files
- [ ] Parse with `JsDocParser` from converters
- [ ] Call `bridge_merger.merge()`
- [ ] Add merged annotations to symbol entry
- [ ] Provenance marked as "converted" or "merged"
- [ ] Works for .js, .ts, .jsx, .tsx files

---

### T3.3: Integrate Python docstring bridging

| Field | Value |
|-------|-------|
| **Phase** | 3 |
| **Component** | cli |
| **File** | `cli/src/index/indexer.rs` |
| **Depends On** | T3.1 |
| **Estimated Time** | 1.5h |
| **Status** | Pending |

**Description**:
Call docstring converter during Python file processing.

**Acceptance Criteria**:
- [ ] Detect docstring style (or use config)
- [ ] Parse with `DocstringParser` from converters
- [ ] Call `bridge_merger.merge()`
- [ ] Add merged annotations to symbol entry
- [ ] Works for .py files
- [ ] Style stored in metadata

---

### T3.4: Track bridge statistics

| Field | Value |
|-------|-------|
| **Phase** | 3 |
| **Component** | cli |
| **File** | `cli/src/index/indexer.rs` |
| **Depends On** | T3.2, T3.3 |
| **Estimated Time** | 1h |
| **Status** | Pending |

**Description**:
Aggregate bridging statistics into cache.

**Acceptance Criteria**:
- [ ] Count converted annotations
- [ ] Count merged annotations
- [ ] Count explicit (ACP-only) annotations
- [ ] Track by source format (jsdoc, google, etc.)
- [ ] Store in `cache.bridge_metadata`
- [ ] Stats visible in JSON output

---

## Phase 4: CLI Commands (4h)

### T4.1: Add --bridge flag to index

| Field | Value |
|-------|-------|
| **Phase** | 4 |
| **Component** | cli |
| **Files** | `cli/src/main.rs`, `cli/src/commands/index.rs` |
| **Depends On** | T3.4 |
| **Estimated Time** | 30m |
| **Status** | Pending |

**Description**:
Add command-line flags to control bridging.

**Acceptance Criteria**:
- [ ] `--bridge` flag enables bridging
- [ ] `--no-bridge` flag disables bridging
- [ ] Flags override config setting
- [ ] Help text accurate

---

### T4.2: Create bridge command

| Field | Value |
|-------|-------|
| **Phase** | 4 |
| **Component** | cli |
| **File** | `cli/src/commands/bridge.rs` |
| **Depends On** | T3.4 |
| **Estimated Time** | 1h |
| **Status** | Pending |

**Description**:
Create `acp bridge` command structure.

**Acceptance Criteria**:
- [ ] `BridgeSubcommand` enum defined
- [ ] `BridgeOptions` struct defined
- [ ] `execute_bridge()` function
- [ ] Registered in `commands/mod.rs`
- [ ] Registered in `main.rs`

---

### T4.3: Implement bridge status

| Field | Value |
|-------|-------|
| **Phase** | 4 |
| **Component** | cli |
| **File** | `cli/src/commands/bridge.rs` |
| **Depends On** | T4.2 |
| **Estimated Time** | 1.5h |
| **Status** | Pending |

**Description**:
Show bridging configuration and statistics.

**Acceptance Criteria**:
- [ ] Show enabled/disabled status
- [ ] Show precedence mode
- [ ] Show per-language settings
- [ ] Show annotation counts from cache
- [ ] Show breakdown by format
- [ ] `--json` flag for machine output

---

### T4.4: Update acp init for bridging

| Field | Value |
|-------|-------|
| **Phase** | 4 |
| **Component** | cli |
| **File** | `cli/src/commands/init.rs` |
| **Depends On** | T4.1 |
| **Estimated Time** | 1h |
| **Status** | Pending |

**Description**:
Add interactive bridging setup to init command.

**Acceptance Criteria**:
- [ ] Prompt: "Enable documentation bridging?"
- [ ] Explanation of what bridging does
- [ ] If yes, prompt for precedence mode
- [ ] Write bridge config to .acp.config.json
- [ ] `--yes` flag uses defaults
- [ ] Works in non-interactive mode

---

## Phase 5: Testing (6h)

### T5.1: Unit tests for bridge module

| Field | Value |
|-------|-------|
| **Phase** | 5 |
| **Component** | cli |
| **File** | `cli/src/bridge/*.rs` (inline tests) |
| **Depends On** | T2.4 |
| **Estimated Time** | 2h |
| **Status** | Pending |

**Description**:
Unit tests for all bridge module components.

**Tests Required**:
- [ ] FormatDetector: Google style detection
- [ ] FormatDetector: NumPy style detection
- [ ] FormatDetector: Sphinx style detection
- [ ] FormatDetector: Unknown fallback
- [ ] BridgeConfig: from_config loading
- [ ] BridgeConfig: is_language_enabled
- [ ] BridgeMerger: acp-first precedence
- [ ] BridgeMerger: native-first precedence
- [ ] BridgeMerger: merge mode
- [ ] BridgeMerger: empty native docs
- [ ] BridgeMerger: empty ACP annotations
- [ ] BridgeMerger: provenance tracking
- [ ] Edge: malformed docs with permissive
- [ ] Edge: malformed docs with strict
- [ ] Edge: conflicting types

---

### T5.2: Integration tests

| Field | Value |
|-------|-------|
| **Phase** | 5 |
| **Component** | cli |
| **File** | `cli/tests/bridge_tests.rs` |
| **Depends On** | T4.3 |
| **Estimated Time** | 3h |
| **Status** | Pending |

**Description**:
End-to-end integration tests.

**Tests Required**:
- [ ] JSDoc file → cache with correct provenance
- [ ] Python Google docstring → cache
- [ ] Mixed ACP + JSDoc → correct merge
- [ ] Bridge disabled → no conversion
- [ ] `--bridge` flag overrides config
- [ ] `--no-bridge` flag overrides config
- [ ] `acp bridge status` output format
- [ ] `acp bridge status --json` output
- [ ] Multi-file project statistics
- [ ] Incremental reindex (changed file only)

---

### T5.3: Performance benchmarks

| Field | Value |
|-------|-------|
| **Phase** | 5 |
| **Component** | cli |
| **File** | `cli/benches/bridge_bench.rs` |
| **Depends On** | T3.4 |
| **Estimated Time** | 1h |
| **Status** | Pending |

**Description**:
Performance benchmarks for bridging overhead.

**Benchmarks Required**:
- [ ] Index baseline (no bridging)
- [ ] Index with bridging (JS project)
- [ ] Index with bridging (Python project)
- [ ] Per-file JSDoc parse time
- [ ] Per-file docstring parse time
- [ ] Format detection time

**Target**: < 15% overhead vs baseline

---

## Phase 6: Documentation (2.5h)

### T6.1: Update CLI help text

| Field | Value |
|-------|-------|
| **Phase** | 6 |
| **Component** | cli |
| **File** | `cli/src/main.rs` |
| **Depends On** | T4.4 |
| **Estimated Time** | 30m |
| **Status** | Pending |

**Description**:
Update CLI help for bridging commands.

**Acceptance Criteria**:
- [ ] `--bridge` flag help accurate
- [ ] `acp bridge` command help
- [ ] `acp bridge status` help
- [ ] Examples in help text

---

### T6.2: Create bridging spec chapter

| Field | Value |
|-------|-------|
| **Phase** | 6 |
| **Component** | spec |
| **File** | `spec/chapters/15-bridging.md` |
| **Depends On** | T5.2 |
| **Estimated Time** | 2h |
| **Status** | Pending |

**Description**:
Write specification chapter for documentation bridging.

**Sections Required**:
- [ ] 15.1 Overview
- [ ] 15.2 Configuration
- [ ] 15.3 Precedence Rules
- [ ] 15.4 Supported Formats
- [ ] 15.5 Format Detection
- [ ] 15.6 Provenance Tracking
- [ ] 15.7 CLI Commands
- [ ] 15.8 Examples

---

## Completion Checklist

### Phase 1 Complete
- [ ] T1.1: config.schema.json updated
- [ ] T1.2: cache.schema.json updated
- [ ] T1.3: Rust types updated
- [ ] All schemas validate

### Phase 2 Complete
- [ ] T2.1: Module structure created
- [ ] T2.2: BridgeConfig implemented
- [ ] T2.3: FormatDetector implemented
- [ ] T2.4: BridgeMerger implemented
- [ ] Bridge module compiles

### Phase 3 Complete
- [ ] T3.1: Indexer updated
- [ ] T3.2: JSDoc bridging works
- [ ] T3.3: Python bridging works
- [ ] T3.4: Statistics tracked
- [ ] `acp index --bridge` works

### Phase 4 Complete
- [ ] T4.1: Flags work
- [ ] T4.2: Command registered
- [ ] T4.3: Status displays
- [ ] T4.4: Init prompts

### Phase 5 Complete
- [ ] T5.1: Unit tests pass
- [ ] T5.2: Integration tests pass
- [ ] T5.3: Performance acceptable

### Phase 6 Complete
- [ ] T6.1: Help text updated
- [ ] T6.2: Spec chapter written

---

## Notes

- Converters already exist in `src/annotate/converters/`
- `DocStandardParser` trait provides common interface
- `ParsedDocumentation` struct standardizes output
- Phase 3+ languages (Rust, Java, Go) deferred to later phases

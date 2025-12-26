# Changelog

All notable changes to the AI Context Protocol specification and reference implementation will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.6.0] - 2025-12-25

### Added - RFC-0004: Tiered Interface Primers

This release implements RFC-0004, which introduces a simplified primer system with tiered interface documentation for AI agents. It replaces the multi-dimensional scoring system with a straightforward budget-aware tier selection algorithm.

#### New CLI Command

- `acp primer` - Generate AI bootstrap primers with tiered content selection
  - `--budget <N>` - Token budget for the primer (default: 200)
  - `--capabilities <caps>` - Filter by capabilities (comma-separated: shell, mcp)
  - `--json` - Output as JSON with metadata
  - `--cache <path>` - Include project warnings from cache

#### Tier System

| Tier | Budget Range | Content Depth |
|------|--------------|---------------|
| Minimal | <80 tokens remaining | Command + one-line purpose |
| Standard | 80-299 tokens | + options, output shape, usage |
| Full | 300+ tokens | + examples, patterns |

#### Bootstrap Block (~20 tokens)

Always included regardless of budget:
```
This project uses ACP. @acp:* comments are directives for you.
Before editing: acp constraints <path>
More: acp primer --budget N
```

#### Schema Updates

- **primer.schema.json**: Added RFC-0004 tiered structure definitions
  - `bootstrap` - Core awareness block (always included)
  - `interface` - Tiered CLI/MCP/daemon command documentation
  - `command` - Command with priority, critical flag, and tiered content
  - `tierContent` - Tokens and template per tier level
  - `projectConfig` - Dynamic project-specific content settings

#### CLI Implementation (acp-cli)

- New `src/commands/primer.rs` module with:
  - `Tier::from_budget()` - Tier selection based on remaining tokens
  - `generate_primer()` - Budget-aware content selection
  - `get_project_warnings()` - Extract frozen/restricted symbols from cache
  - Default command set with 8 commands at 3 tier levels
- 5 new unit tests + 2 schema validation tests

### Changed

- RFC-0004 status updated to Implemented
- Primer schema `sections` field now optional (backward compatible)

## [0.5.0] - 2025-12-24

### Added - RFC-0006: Documentation System Bridging

This release implements RFC-0006, which enables ACP to leverage existing documentation from JSDoc, Python docstrings (Google/NumPy/Sphinx), Rustdoc, and other language-specific documentation systems.

#### Schema Updates

- **config.schema.json**: Added `bridge` configuration section
  - `enabled` - Enable/disable documentation bridging
  - `precedence` - Merge priority: `acp-first`, `native-first`, `merge`
  - `strictness` - Error handling: `permissive`, `strict`
  - `jsdoc` - JSDoc/TSDoc settings (enabled, extractTypes, convertTags)
  - `python` - Python docstring settings (enabled, docstringStyle, extractTypeHints)
  - `rust` - Rust doc settings (enabled, convertSections)
  - `provenance` - Tracking settings (markConverted, includeSourceFormat)

- **cache.schema.json**: Added bridging support
  - `bridge` top-level section for aggregate statistics
  - `bridge_metadata` per-file bridging information
  - `source`, `sourceFormat`, `sourceFormats` fields on param/returns/throws entries
  - `TypeSource`, `BridgeSource`, `SourceFormat` enums

#### New CLI Commands

- `acp bridge status` - Show bridging configuration and statistics
- `acp index --bridge` - Enable bridging during indexing
- `acp index --no-bridge` - Disable bridging (overrides config)

#### Specification Updates

- **Chapter 15 (Bridging)**: New chapter covering documentation bridging architecture
  - Format detection for JSDoc, Python docstrings (Google/NumPy/Sphinx), Rustdoc
  - Precedence rules for merging native docs with ACP annotations
  - Provenance tracking for converted annotations

#### CLI Implementation (acp-cli)

- New `src/bridge/` module with:
  - `config.rs` - BridgeConfig, JsDocConfig, PythonConfig, RustConfig
  - `detector.rs` - FormatDetector with auto-detection for all formats
  - `merger.rs` - BridgeMerger with precedence modes
  - `mod.rs` - BridgeResult type and module structure
- Indexer integration with format detection and statistics
- 39 new tests (21 unit + 18 integration)

### Changed

- RFC-0006 status updated to Implemented
- Cache schema version remains compatible (additive changes only)

## [0.4.0] - 2025-12-22

### Added - RFC-0002: Documentation References and Style Guides

This release implements RFC-0002, which formalizes the `@acp:ref` and `@acp:style` annotation system with project-level configuration, schema support, and AI behavioral guidelines.

#### Schema Updates

- **config.schema.json**: Added `documentation` configuration section
  - `approvedSources[]` - Define trusted documentation sources with IDs
  - `styleGuides{}` - Custom style guide definitions with inheritance
  - `defaults` - Project-wide documentation defaults
  - `validation` - Reference validation settings

- **cache.schema.json**: Added documentation storage
  - `refs[]` in file_entry - Documentation references per file
  - `style` object in file_entry - Enhanced style configuration
  - `documentation` top-level index - Project-wide documentation aggregation
  - `$defs/ref_entry` and `$defs/style_entry` definitions

#### New Annotations

- `@acp:ref-version` - Specify documentation version
- `@acp:ref-section` - Reference specific section within documentation
- `@acp:ref-fetch` - Control whether AI should proactively fetch documentation
- `@acp:style-extends` - Style guide inheritance

#### Specification Updates

- **ACP-1.0.md Appendix A**: Updated reserved annotations table with RFC-0002 additions
- **Chapter 03 (Cache Format)**: Added `refs[]`, `style`, and `documentation` index documentation
- **Chapter 04 (Config Format)**: New Section 9 for documentation configuration
- **Chapter 05 (Annotations)**: Extended documentation reference annotations, added built-in style guide registry
- **Chapter 06 (Constraints)**: Updated style constraints with inheritance and built-in guide URLs
- **Chapter 11 (Tool Integration)**: New Section 10 for AI behavior with documentation references

#### Built-in Style Guide Registry

Added 14 built-in style guides with official documentation URLs:
- `google-typescript`, `google-javascript`, `google-python`, `google-java`, `google-cpp`, `google-go`
- `airbnb-javascript`, `airbnb-react`
- `pep8`, `black`
- `prettier`, `rustfmt`, `standardjs`, `tailwindcss-v3`

### Changed

- RFC-0002 status updated to Implemented
- RFC-0002 open questions resolved
- RFC directory structure flattened (all RFCs in `rfcs/`, status tracked in header)

## [0.3.0] - 2025-12-21

### Added

- Document all six ACP JSON files in specification Section 3
  - Added Section 3.4: Attempts File (`.acp/acp.attempts.json`)
  - Added Section 3.5: Sync File (`.acp/acp.sync.json`)
  - Added Section 3.6: Primer File (`.acp.primer.json`)
- Add cross-references in Chapter 13 (Debug Sessions) for attempts.json
- Add cross-references in Chapter 14 (Bootstrap) for primer.json

### Changed

- Updated file count from "three JSON files" to "six JSON files" in Section 3
- Expanded file format table with all six schema-backed files
- Added implementation notes for Level 2 compliance with new file support

## [0.2.0] - 2025-12-20

### Added

- Add acp annotate command for automated annotation generation

### Changed

- Initial ACP spec release

- Refactor CLI architecture and reorganize specification

- Restructure CLI into modular architecture (constraints, vars, cache modules)
- Rewrite CLI README with complete command documentation
- Renumber spec chapters to sequential 01-12 format
- Add formal W3C EBNF grammar with railroad diagrams
- Add specification examples (minimal, complete, edge-cases)
- Add CLI implementation guide for Rust developers
- Add GitHub issue templates for community contributions

- Fix CLI indexer bugs and build warnings

- Fix glob pattern matching by using relative paths for pattern comparison
- Fix vars file naming (.acp.vars.json instead of .acp.cache.vars.json)
- Enhance parser to extract @acp: annotations and build symbols
- Fix config resolution when indexing subdirectories
- Remove unused cfg attribute and prefix unused variables with underscore

- Update CLI to be fully compliant with JSON schemas

- cache/types.rs: Update SymbolType, Stability, add Language/Visibility enums,
  restructure FileEntry and SymbolEntry, add source_files to Cache
- vars/mod.rs: Simplify VarsFile and VarEntry to match vars.schema.json
- config/mod.rs: Add schema-compliant structure with optional sections
- index/indexer.rs: Populate source_files, detect language, update generate_vars
- parse/mod.rs: Use new schema-compliant types
- main.rs: Fix references to removed fields, add output directory creation
- query.rs: Update to work with new Cache structure
- .gitignore: Add .acp/ generated output directory

- Add variable inheritance and extended variable types

- vars.schema.json: Add refs array for inheritance chains, source/lines
  for origin tracking, layer/pattern/context types
- CLI: Restore chain traversal logic, populate refs from call graph
- Generate layer variables from file entries
- Update CHANGELOG with new features

- Fix path normalization in check command

Handle ./prefix variations when looking up files in cache

- Add constraint parsing and display to check command

- Parser: Fix regex to match hyphenated annotation names (ai-careful, ai-readonly)
- Parser: Extract @acp:lock, @acp:ai-careful, @acp:hack annotations from source
- Cache: Add ai_hints field to FileEntry for behavioral hints
- Indexer: Build ConstraintIndex from parsed lock levels and hack markers
- Check command: Display lock level, AI hints, and requirement warnings

- Fix attempt file tracking when creating checkpoints

When creating a checkpoint, also track those files in the current
active attempt so that file counts and cleanup work correctly.

- Require config and prevent empty cache creation

- Fail all commands (except init) if .acp.config.json is missing
- Show helpful message directing users to run 'acp init'
- Fail index command if no files match include patterns
- Display current patterns to help debug configuration issues

- Allow validate command without config file

- Move attempts file to .acp/ and add schema

- Change attempts file location from .acp.attempts.json to .acp/acp.attempts.json
- Add $schema field to AttemptTracker for validation
- Create attempts.schema.json defining all attempt tracking types

- Add sync schema and improve schema validation

- Add schemas/v1/sync.schema.json for AI tool synchronization config
  - Support built-in tools (cursor, claude-code, copilot, etc.)
  - Support custom tools via pattern ^custom-[a-z0-9-]+$
  - Conditional sectionMarker requirement for section merge strategy
  - Content options, custom adapters, hooks, and templates

- Improve attempts.schema.json validation
  - Add additionalProperties: false for strict validation
  - Add minLength constraints on identifier fields
  - Add pattern validation for MD5 hashes and git commit SHA
  - Add git_commit field to tracked_checkpoint

- Add sync section to config.schema.json
  - Support inline sync config or boolean enable/disable

- Update CLI attempts.rs
  - Add git_commit field to TrackedCheckpoint struct
  - Capture git commit SHA when creating checkpoints

- Add primer schema, enhance schema validation, and add CI workflow

- Add primer.schema.json for AI context bootstrapping definitions
- Enhance schema.rs with full jsonschema validation
- Add test fixtures for all 6 schemas (26 tests passing)
- Add GitHub Actions workflow for schema validation
- Update README with all Schema Store catalog entries

🤖 Generated with [Claude Code](https://claude.com/claude-code)

Co-Authored-By: Claude Opus 4.5 <noreply@anthropic.com>

- Add tree-sitter AST parsing and git2 integration

New features:
- Tree-sitter based AST parsing for 6 languages (TypeScript, JavaScript,
  Rust, Python, Go, Java)
- Git2 integration with blame tracking, file history, and contributor metadata
- Symbol extraction with signatures, visibility, generics, and doc comments
- Git metadata per file (last commit, author, contributors) and per symbol
  (code age, last modifier)

Changes:
- Add cli/src/ast/ module with AstParser and 6 language extractors
- Add cli/src/git/ module with GitRepository, BlameInfo, FileHistory
- Update cache types with GitFileInfo and GitSymbolInfo
- Update cache.schema.json with git metadata definitions
- Integrate AST parsing and git metadata into indexer
- Update README with current capabilities, CLI commands, and roadmap
- Add comprehensive testing guide
---

[Unreleased]: https://github.com/acp-protocol/acp-spec/compare/HEAD

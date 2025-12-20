# Changelog

All notable changes to the AI Context Protocol specification and reference implementation will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

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

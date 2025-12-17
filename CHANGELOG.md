# Changelog

All notable changes to the AI Context Protocol specification and reference implementation will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Initial public release preparation
- JSON schemas for Schema Store submission

---

## [1.0.0] - 2024-XX-XX

### Added

#### Specification
- Complete ACP 1.0 specification
- 12 main sections covering all protocol aspects
- 3 appendices (Annotation Reference, Schema Reference, Language Support)

#### Core Features
- **Annotation System**: `@acp:` prefix annotations in code comments
    - Module and symbol annotations
    - Domain and layer organization
    - Constraint annotations (lock, style, behavior, quality)
    - Debug session tracking (hack, debug)
    - Variable definitions

- **Constraint System**: Graduated protection levels
    - `frozen` - Never modify
    - `restricted` - Explain changes first
    - `approval-required` - Ask before changing
    - `tests-required` - Must include tests
    - `docs-required` - Must update documentation
    - `review-required` - Flag for human review
    - `normal` - No restrictions (default)
    - `experimental` - Extra caution advised

- **Variable System**: Token-efficient references
    - Symbol variables (`$SYM_*`)
    - File variables (`$FILE_*`)
    - Domain variables (`$DOM_*`)
    - Pattern variables (`$PAT_*`)
    - Context variables (`$CTX_*`)
    - Config variables (`$CFG_*`)

- **Conformance Levels**: Three-tier implementation system
    - Level 1 (Reader): Parse and query cache files
    - Level 2 (Standard): Generate cache, variables, constraints
    - Level 3 (Full): MCP integration, debug sessions, watch mode

- **Error Handling**: Configurable strictness
    - Permissive mode (default): Warn and continue
    - Strict mode: Fail on first error
    - Standardized error codes

#### File Formats
- `.acp.config.json` - Project configuration
- `.acp.cache.json` - Indexed codebase cache
- `.acp.vars.json` - Variable definitions

#### JSON Schemas
- `cache.schema.json` - Cache file validation
- `config.schema.json` - Configuration validation
- `vars.schema.json` - Variables file validation

#### Documentation
- Getting started guide
- Annotation reference
- Constraint reference
- Variable reference
- Integration guides (Claude Desktop, Cursor, GitHub Copilot)

#### Reference Implementation
- Rust CLI (`acp-protocol-cli`)
- Commands: `init`, `index`, `query`, `vars`, `constraints`, `check`, `watch`
- Multi-language support: TypeScript, Python, Rust, Go, Java

### Technical Decisions
- JSON field naming: `snake_case` throughout
- Symbol qualification: `file/path:class.function` format
- Extension namespace: `@acp:x-vendor:feature` convention
- Default lock level: `normal` (no restrictions)
- Undefined variables: Warn and leave literal
- Staleness detection: Git-aware with timestamp fallback

---

## [0.1.0] - 2024-XX-XX (Internal)

### Added
- Initial draft specification
- Proof of concept CLI
- Basic annotation parsing

---

[Unreleased]: https://github.com/acp-protocol/acp-spec/compare/v1.0.0...HEAD
[1.0.0]: https://github.com/acp-protocol/acp-spec/releases/tag/v1.0.0
[0.1.0]: https://github.com/acp-protocol/acp-spec/releases/tag/v0.1.0
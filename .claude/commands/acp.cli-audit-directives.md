---
description: Audit the ACP CLI implementation for RFC-001 self-documenting annotation compliance. Analyzes cli/src/* for required directive support.
handoffs:
  - label: CLI Remediation Plan
    agent: acp.cli-remediate
    prompt: Create remediation plan based on audit findings
    send: true
  - label: Spec Audit
    agent: acp.spec-audit-directives
    prompt: Re-verify spec compliance before CLI changes
  - label: Generate Tasks
    agent: acp.cli-tasks
    prompt: Generate implementation tasks from audit
---

## User Input

```text
$ARGUMENTS
```

You **MUST** consider the user input before proceeding (if not empty). The user input may specify:
- `--module <name>` to audit specific module (parse, cache, constraints, etc.)
- `--json` to output findings in JSON format
- `--strict` to treat SHOULD as errors
- `--decisions <path>` to load spec decisions for consistency
- `--skip-tests` to skip test coverage analysis

## Purpose

This command audits the ACP CLI (Rust implementation) against RFC-001 requirements. The CLI must be updated to:

1. **Parse directive suffixes** from annotations
2. **Store directives** in cache output
3. **Display directives** in constraint/query output
4. **Implement new commands** (map, migrate)
5. **Support hierarchical annotations** (file/symbol/inline)

## Outline

1. **Initialize audit** by running the analysis script:
   ```bash
   ./scripts/audit-cli-directives.sh --json
   ```

   Parse JSON response:
   ```json
   {
     "status": "findings|clean",
     "cli_version": "0.2.0",
     "rfc_version": "001",
     "modules_audited": [
       "cli/src/parse/mod.rs",
       "cli/src/cache/types.rs",
       "cli/src/constraints/mod.rs",
       ...
     ],
     "findings": {
       "blockers": [...],
       "violations": [...],
       "warnings": [...],
       "info": [...]
     },
     "feature_coverage": {
       "directive_parsing": "missing|partial|complete",
       "directive_storage": "missing|partial|complete",
       "directive_display": "missing|partial|complete",
       "symbol_annotations": "missing|partial|complete",
       "inline_annotations": "missing|partial|complete",
       "map_command": "missing|partial|complete",
       "migrate_command": "missing|partial|complete"
     }
   }
   ```

2. **Analyze by module**:

   **Parser Module** (`cli/src/parse/`):
    - Does regex extract directive suffix?
    - Does it handle multi-line directives?
    - Does it parse symbol-level annotations?
    - Does it extract inline annotations?

   **Cache Module** (`cli/src/cache/`):
    - Does `Annotation` struct have `directive` field?
    - Does `FileEntry` have `purpose` field?
    - Does `FileEntry` have `symbols` array?
    - Does `FileEntry` have `inline` array?
    - Does schema validation include new fields?

   **Constraints Module** (`cli/src/constraints/`):
    - Does output include directive text?
    - Does aggregation combine directives?

   **Commands**:
    - Is `acp map` implemented?
    - Is `acp migrate` implemented?
    - Does `acp query symbol` return line ranges?
    - Does `acp constraints` show directives?

3. **Generate audit report** with findings and tasks

## RFC-001 CLI Requirements

### Parser Updates

| ID | Requirement | File | Status |
|----|-------------|------|--------|
| P01 | Extract directive suffix after ` - ` | parse/mod.rs | ○ |
| P02 | Handle multi-line directives (indented continuation) | parse/mod.rs | ○ |
| P03 | Parse `@acp:purpose` for file metadata | parse/mod.rs | ○ |
| P04 | Parse symbol-level annotations (`@acp:fn`, `@acp:class`, etc.) | parse/mod.rs | ○ |
| P05 | Parse inline annotations (`@acp:hack`, `@acp:todo`, etc.) | parse/mod.rs | ○ |
| P06 | Extract line numbers for symbols | parse/mod.rs | ○ |
| P07 | Warn on missing directive suffix (per Q07 decision) | index/indexer.rs | ○ |
| P08 | Auto-generate default directives (per Q04 decision) | parse/mod.rs | ○ |

### Cache Structure Updates

| ID | Requirement | File | Status |
|----|-------------|------|--------|
| C01 | Add `directive: String` to Annotation struct | cache/types.rs | ○ |
| C02 | Add `auto_generated: bool` to Annotation (per Q08) | cache/types.rs | ○ |
| C03 | Add `purpose: Option<String>` to FileEntry | cache/types.rs | ○ |
| C04 | Add `symbols: Vec<SymbolAnnotation>` to FileEntry | cache/types.rs | ○ |
| C05 | Add `inline: Vec<InlineAnnotation>` to FileEntry | cache/types.rs | ○ |
| C06 | Add `lines: LineRange` to SymbolEntry | cache/types.rs | ○ |
| C07 | Add `signature: Option<String>` to SymbolEntry | cache/types.rs | ○ |
| C08 | Update JSON serialization for new fields | cache/types.rs | ○ |

### Constraint Updates

| ID | Requirement | File | Status |
|----|-------------|------|--------|
| N01 | Include directive in constraint output | constraints/mod.rs | ○ |
| N02 | Aggregate directives from multiple annotations | constraints/mod.rs | ○ |
| N03 | Apply conflict resolution per Q03 decision | constraints/mod.rs | ○ |

### New Commands

| ID | Requirement | File | Status |
|----|-------------|------|--------|
| M01 | Implement `acp map <path>` command | commands/map.rs | ○ |
| M02 | Implement `acp migrate --add-directives` | commands/migrate.rs | ○ |
| M03 | Update `acp query file` with purpose/symbols | commands/query.rs | ○ |
| M04 | Update `acp query symbol` with line ranges | commands/query.rs | ○ |
| M05 | Update `acp constraints` with directives | commands/check.rs | ○ |

### Index Command Updates

| ID | Requirement | File | Status |
|----|-------------|------|--------|
| I01 | Warn on missing directives (first-N + summary per Q07) | commands/index.rs | ○ |
| I02 | Extract and store file purpose | index/indexer.rs | ○ |
| I03 | Build symbol annotation index | index/indexer.rs | ○ |
| I04 | Build inline annotation index | index/indexer.rs | ○ |

**Legend**: ○ Pending | ◐ Partial | ● Complete | ✗ Missing

## Current Implementation Analysis

Based on CLI source analysis:

### Annotation Struct (Current)

```rust
// cli/src/parse/mod.rs
pub struct Annotation {
    pub name: String,
    pub value: Option<String>,
    pub line: usize,
}
```

**Missing**: `directive` field, `auto_generated` flag

### Parser Regex (Current)

```rust
let pattern = regex::Regex::new(r"@acp:(\w+)(?:\s+(.+))?").unwrap();
```

**Issue**: Does not extract directive suffix after ` - `

### FileEntry (Current)

```rust
// cli/src/cache/types.rs  
pub struct FileEntry {
    pub path: String,
    pub module: String,
    pub lines: usize,
    pub domains: Vec<String>,
    pub layer: Option<String>,
    // ... etc
}
```

**Missing**: `purpose`, `symbols`, `inline` fields

## Script Reference

| Script | Purpose | When Used |
|--------|---------|-----------|
| `audit-cli-directives.sh` | Scan CLI source for RFC-001 gaps | Always |
| `analyze-parser.sh` | Deep analysis of parser module | On demand |
| `check-cache-types.sh` | Verify cache struct fields | On demand |

## Output Format

Generate report in this format:

```markdown
# ACP CLI Directive Audit Report

**Generated**: [TIMESTAMP]
**CLI Version**: [VERSION]
**RFC Reference**: RFC-001

## Summary

| Category | Count |
|----------|-------|
| Blockers | X |
| Violations | X |
| Warnings | X |

## Feature Coverage

| Feature | Status | Gap |
|---------|--------|-----|
| Directive Parsing | [STATUS] | [GAP] |
| Directive Storage | [STATUS] | [GAP] |
| ... | ... | ... |

## Module Analysis

### Parser (cli/src/parse/)
[detailed findings]

### Cache (cli/src/cache/)
[detailed findings]

### Commands
[detailed findings]

## Implementation Tasks

[prioritized task list with estimates]
```

## Error Conditions

| Error | Cause | Resolution |
|-------|-------|------------|
| "CLI source not found" | Missing cli/src/ | Ensure running from repo root |
| "Cannot parse Rust" | Malformed source | Check syntax |
| "Decisions not loaded" | Missing spec decisions | Run spec-clarify first |

## Completion Criteria

### Audit Complete When:
- [ ] All CLI modules scanned
- [ ] Parser regex analyzed
- [ ] Cache structures analyzed
- [ ] Command implementations checked
- [ ] Findings categorized
- [ ] Tasks generated with estimates
- [ ] Report produced

## Integration with Spec Workflow

This audit should run AFTER spec remediation:

```
┌─────────────────────┐     ┌─────────────────────┐
│ Spec Remediation    │────▶│ CLI Audit           │ ← You are here
│ (spec updated)      │     │ (find gaps)         │
└─────────────────────┘     └─────────┬───────────┘
                                      │
                                      ▼
                            ┌─────────────────────┐
                            │ CLI Remediation     │
                            │ (update code)       │
                            └─────────┬───────────┘
                                      │
                                      ▼
                            ┌─────────────────────┐
                            │ CLI Verification    │
                            │ (test output)       │
                            └─────────────────────┘
```

Load spec decisions to ensure CLI implements consistent behavior.
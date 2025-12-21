---
description: Remediate the ACP CLI implementation to support RFC-001 self-documenting annotations. Updates parser, cache structures, and commands.
handoffs:
  - label: Back to CLI Audit
    agent: acp.cli-audit-directives
    prompt: Re-audit to check remediation completeness
  - label: Verify CLI Output
    agent: acp.cli-verify
    prompt: Verify CLI produces correct cache and output
    send: true
  - label: Run Tests
    agent: acp.cli-test
    prompt: Run CLI test suite after changes
  - label: Update Docs
    agent: acp.cli-docs
    prompt: Update CLI documentation for new features
---

## User Input

```text
$ARGUMENTS
```

You **MUST** consider the user input before proceeding (if not empty). The user input may specify:
- `--decisions <path>` to load spec decisions (default: `.acp/spec-decisions.json`)
- `--module <n>` to remediate specific module only
- `--dry-run` to show proposed changes without applying
- `--backup` to create backups before modifying (default: true)
- `--task <ID>` to implement specific task only
- `--skip-tests` to skip updating tests

## Purpose

This command updates the ACP CLI (Rust implementation) to comply with RFC-001. Changes span multiple modules:

1. **Parser** - Extract directive suffixes from annotations
2. **Cache Types** - Add fields for directives, purpose, symbols, inline
3. **Indexer** - Build enhanced cache with new fields
4. **Commands** - Implement `map`, `migrate`, update `query`/`constraints`
5. **Tests** - Add tests for new functionality

## Prerequisites

Before running:
1. **Spec remediation complete**: Spec defines directive requirements
2. **Decisions captured**: `.acp/spec-decisions.json` exists
3. **CLI audit complete**: Know what needs to change
4. **Rust toolchain**: `cargo` available for building/testing

## Outline

1. **Load context**:
   ```bash
   ./scripts/load-cli-remediation-context.sh --json
   ```

   Expected output:
   ```json
   {
     "decisions_file": ".acp/spec-decisions.json",
     "decisions_valid": true,
     "cli_audit": ".acp/cli-audit-findings.json",
     "cli_version": "0.2.0",
     "target_version": "0.3.0",
     "modules_to_update": [
       "cli/src/parse/mod.rs",
       "cli/src/cache/types.rs",
       ...
     ],
     "backup_dir": ".acp/backups/cli-[TIMESTAMP]",
     "ready": true
   }
   ```

2. **Create backups** (unless `--no-backup`)

3. **Apply remediations** by module (see detailed sections)

4. **Run cargo check** to verify compilation

5. **Update tests**

6. **Generate summary**

## Remediation Tasks

### Module: Parser (`cli/src/parse/`)

#### Task P01: Update Annotation Struct

**File**: `cli/src/parse/mod.rs` (or wherever Annotation is defined)

```rust
// BEFORE
pub struct Annotation {
    pub name: String,
    pub value: Option<String>,
    pub line: usize,
}

// AFTER
/// Parsed annotation from source with directive support
#[derive(Debug, Clone, serde::Serialize)]
pub struct Annotation {
    /// Annotation type (e.g., "lock", "ref", "hack")
    pub name: String,
    /// Primary value after the annotation name
    pub value: Option<String>,
    /// Self-documenting directive text after ` - `
    pub directive: Option<String>,
    /// Whether directive was auto-generated from defaults
    #[serde(default, skip_serializing_if = "is_false")]
    pub auto_generated: bool,
    /// Source line number (1-indexed)
    pub line: usize,
}

fn is_false(b: &bool) -> bool { !*b }
```

---

#### Task P02: Update Parser Regex

**File**: `cli/src/parse/mod.rs`

```rust
// BEFORE
let pattern = regex::Regex::new(r"@acp:(\w+)(?:\s+(.+))?").unwrap();

// AFTER - Extract directive suffix
lazy_static::lazy_static! {
    /// Matches: @acp:name [value] [- directive]
    /// Groups: 1=name, 2=value (before -), 3=directive (after -)
    static ref ANNOTATION_PATTERN: regex::Regex = regex::Regex::new(
        r"@acp:([\w-]+)(?:\s+([^-\n]+?))?(?:\s+-\s+(.+))?"
    ).unwrap();
}
```

---

#### Task P03: Update `parse_annotations` Function

```rust
/// Parse @acp: annotations from source comments
pub fn parse_annotations(&self, content: &str) -> Vec<Annotation> {
    let mut annotations = Vec::new();
    let mut pending_multiline: Option<(usize, &mut Annotation)> = None;
    
    for (line_num, line) in content.lines().enumerate() {
        let line_1indexed = line_num + 1;
        
        // Check for multiline continuation (indented line after annotation)
        if let Some((start_line, ref mut ann)) = pending_multiline {
            if line.starts_with("//") {
                let trimmed = line.trim_start_matches('/').trim();
                if trimmed.starts_with("  ") || trimmed.starts_with('\t') {
                    // Continuation line - append to directive
                    if let Some(ref mut dir) = ann.directive {
                        dir.push(' ');
                        dir.push_str(trimmed.trim());
                    }
                    continue;
                }
            }
            pending_multiline = None;
        }
        
        // Look for new annotations
        for cap in ANNOTATION_PATTERN.captures_iter(line) {
            let name = cap.get(1).unwrap().as_str().to_string();
            let value = cap.get(2).map(|m| m.as_str().trim().to_string());
            let directive = cap.get(3).map(|m| m.as_str().trim().to_string());
            
            // Auto-generate directive if missing (per Q04 decision)
            let (final_directive, auto_generated) = match directive {
                Some(d) => (Some(d), false),
                None => (self.default_directive(&name, value.as_deref()), true),
            };
            
            let mut annotation = Annotation {
                name,
                value,
                directive: final_directive,
                auto_generated,
                line: line_1indexed,
            };
            
            annotations.push(annotation);
        }
    }
    
    annotations
}

/// Get default directive for annotation type (per Q04 = B)
fn default_directive(&self, name: &str, value: Option<&str>) -> Option<String> {
    match name {
        "lock" => match value {
            Some("frozen") => Some("MUST NOT modify this file under any circumstances".into()),
            Some("restricted") => Some("Explain proposed changes and wait for explicit approval".into()),
            Some("approval-required") => Some("Propose changes and request confirmation".into()),
            Some("tests-required") => Some("All changes must include corresponding tests".into()),
            Some("normal") | None => Some("Safe to modify following project conventions".into()),
            _ => None,
        },
        "ref" => value.map(|url| format!("Consult {} before making changes", url)),
        "hack" => Some("Temporary workaround - check expiry before modifying".into()),
        "deprecated" => Some("Do not use or extend - see replacement annotation".into()),
        "todo" => Some("Pending work item".into()),
        "fixme" => Some("Known issue requiring fix".into()),
        _ => None,
    }
}
```

---

### Module: Cache Types (`cli/src/cache/`)

#### Task C01-C08: Update Cache Structures

**File**: `cli/src/cache/types.rs`

```rust
// Add new types for RFC-001 compliance

/// Line range for symbol location
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct LineRange {
    pub start: usize,
    pub end: usize,
}

/// Symbol-level annotation from source
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SymbolAnnotation {
    /// Symbol name
    pub name: String,
    /// Symbol type (function, class, method, const)
    #[serde(rename = "type")]
    pub symbol_type: String,
    /// Line range in file
    pub lines: LineRange,
    /// Purpose/description from @acp:fn, @acp:class, etc.
    pub purpose: Option<String>,
    /// Function/method signature
    pub signature: Option<String>,
    /// Symbol-specific constraints
    #[serde(skip_serializing_if = "Option::is_none")]
    pub constraints: Option<SymbolConstraint>,
    /// Parameters (for functions)
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub params: Vec<ParamAnnotation>,
    /// Return type annotation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub returns: Option<String>,
}

/// Parameter annotation
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ParamAnnotation {
    pub name: String,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub param_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// Symbol-level constraint
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SymbolConstraint {
    pub level: LockLevel,
    pub directive: String,
}

/// Inline annotation (hack, todo, fixme, critical)
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct InlineAnnotation {
    /// Line number
    pub line: usize,
    /// Annotation type
    #[serde(rename = "type")]
    pub annotation_type: String,
    /// Self-documenting directive
    pub directive: String,
    /// Expiry date for hacks
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires: Option<String>,
    /// Related ticket/issue
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ticket: Option<String>,
}

// Update FileEntry
pub struct FileEntry {
    pub path: String,
    pub module: String,
    pub lines: usize,
    
    // NEW: RFC-001 fields
    /// File purpose from @acp:purpose annotation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub purpose: Option<String>,
    
    /// Symbol-level annotations
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub symbols: Vec<SymbolAnnotation>,
    
    /// Inline annotations (hacks, todos, etc.)
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub inline: Vec<InlineAnnotation>,
    
    // Existing fields...
    pub domains: Vec<String>,
    pub layer: Option<String>,
    // ...
}
```

---

### Module: Constraints (`cli/src/constraints/`)

#### Task N01-N03: Update Constraint Output

**File**: `cli/src/constraints/mod.rs`

```rust
/// File constraint with directive
#[derive(Debug, Clone, serde::Serialize)]
pub struct FileConstraint {
    pub level: LockLevel,
    /// Aggregated directive from annotations
    pub directive: String,
    /// All annotations contributing to this constraint
    pub annotations: Vec<AnnotationRef>,
}

impl FileConstraint {
    /// Aggregate constraints from multiple annotations
    /// Applies conflict resolution per Q03 decision (most specific wins)
    pub fn aggregate(annotations: &[Annotation], file_level: LockLevel) -> Self {
        let mut directives = Vec::new();
        let mut refs = Vec::new();
        
        for ann in annotations {
            if let Some(ref dir) = ann.directive {
                directives.push(dir.clone());
            }
            refs.push(AnnotationRef {
                annotation_type: ann.name.clone(),
                value: ann.value.clone(),
                directive: ann.directive.clone(),
                line: ann.line,
            });
        }
        
        Self {
            level: file_level,
            directive: directives.join(" "),
            annotations: refs,
        }
    }
}
```

---

### Module: Commands

#### Task M01: Implement `acp map` Command

**New File**: `cli/src/commands/map.rs`

```rust
//! @acp:module "Map Command"
//! @acp:summary "Display directory/file structure with annotations"

use std::path::PathBuf;
use clap::Args;

#[derive(Args)]
pub struct MapArgs {
    /// Path to map (file or directory)
    #[arg(default_value = ".")]
    pub path: PathBuf,
    
    /// Maximum depth for directory recursion
    #[arg(short, long, default_value = "3")]
    pub depth: usize,
    
    /// Show inline annotations (hacks, todos)
    #[arg(long)]
    pub inline: bool,
    
    /// Output format (tree, flat, json)
    #[arg(short, long, default_value = "tree")]
    pub format: String,
}

pub fn execute(args: MapArgs, cache: &Cache) -> Result<()> {
    match args.format.as_str() {
        "tree" => print_tree(&args.path, &cache, args.depth, args.inline),
        "flat" => print_flat(&args.path, &cache, args.inline),
        "json" => print_json(&args.path, &cache),
        _ => Err(AcpError::InvalidArgument("format".into())),
    }
}

fn print_tree(path: &Path, cache: &Cache, depth: usize, show_inline: bool) -> Result<()> {
    // Output format per RFC-001 Section 5.7:
    // src/auth/
    //   session.ts (restricted)
    //     Session management and JWT validation
    //     ├─ validateSession (fn:45) [frozen]
    //     ├─ refreshToken (fn:89)
    //     └─ SessionStore (class:120)
    //   Active Issues:
    //     session.ts:56 - @acp:hack expires 2024-06-01
    
    // Implementation...
    Ok(())
}
```

---

#### Task M02: Implement `acp migrate` Command

**New File**: `cli/src/commands/migrate.rs`

```rust
//! @acp:module "Migrate Command"
//! @acp:summary "Add directive suffixes to existing annotations"

use std::path::PathBuf;
use clap::Args;

#[derive(Args)]
pub struct MigrateArgs {
    /// Paths to migrate (default: all indexed files)
    pub paths: Vec<PathBuf>,
    
    /// Dry run - show changes without applying
    #[arg(long)]
    pub dry_run: bool,
    
    /// Interactive mode - confirm each change
    #[arg(short, long)]
    pub interactive: bool,
    
    /// Backup files before modifying
    #[arg(long, default_value = "true")]
    pub backup: bool,
}

pub fn execute(args: MigrateArgs) -> Result<MigrationReport> {
    // Find annotations without directives
    // Generate default directives
    // Show/apply changes
    // Output report
    
    // Per RFC-001 Migration Command example:
    // $ acp migrate --dry-run
    // Would update 47 annotations:
    //   src/auth/session.ts:1
    //     - // @acp:lock frozen
    //     + // @acp:lock frozen - MUST NOT modify this file under any circumstances
    
    Ok(MigrationReport::default())
}
```

---

#### Task M03-M04: Update Query Commands

**File**: `cli/src/commands/query.rs` or `cli/src/query/engine.rs`

```rust
// Update query file output to include purpose and symbols
pub fn query_file(path: &str, cache: &Cache) -> Result<FileQueryResult> {
    let file = cache.files.get(path)
        .ok_or_else(|| AcpError::NotFound(path.into()))?;
    
    Ok(FileQueryResult {
        path: file.path.clone(),
        purpose: file.purpose.clone(),
        lines: file.lines,
        constraints: get_file_constraints(path, cache),
        symbols: file.symbols.iter().map(|s| SymbolSummary {
            name: s.name.clone(),
            symbol_type: s.symbol_type.clone(),
            lines: s.lines.clone(),
            purpose: s.purpose.clone(),
            constraint_level: s.constraints.as_ref().map(|c| c.level.clone()),
        }).collect(),
        inline_count: file.inline.len(),
    })
}

// Update query symbol to include line ranges per RFC-001
pub fn query_symbol(name: &str, cache: &Cache) -> Result<SymbolQueryResult> {
    // Find symbol across all files
    for (path, file) in &cache.files {
        for symbol in &file.symbols {
            if symbol.name == name {
                return Ok(SymbolQueryResult {
                    name: symbol.name.clone(),
                    file: path.clone(),
                    lines: symbol.lines.clone(),  // NEW: line range
                    purpose: symbol.purpose.clone(),
                    signature: symbol.signature.clone(),
                    constraints: symbol.constraints.clone(),
                    params: symbol.params.clone(),
                    returns: symbol.returns.clone(),
                });
            }
        }
    }
    
    Err(AcpError::NotFound(name.into()))
}
```

---

#### Task M05: Update Constraints Command

**File**: `cli/src/commands/check.rs`

```rust
// Update output to include directives
pub fn print_constraints(path: &str, cache: &Cache) -> Result<()> {
    let constraint = cache.constraints.by_file.get(path);
    
    println!("File: {}", path);
    
    if let Some(c) = constraint {
        println!("Level: {:?}", c.level);
        println!("Directive: {}", c.directive);  // NEW
        
        if !c.annotations.is_empty() {
            println!("\nAnnotations:");
            for ann in &c.annotations {
                println!("  @acp:{} {}", ann.annotation_type, 
                    ann.value.as_deref().unwrap_or(""));
                if let Some(ref dir) = ann.directive {
                    println!("    → {}", dir);  // Show directive
                }
            }
        }
    }
    
    // Show symbol-level constraints
    if let Some(file) = cache.files.get(path) {
        let frozen_symbols: Vec<_> = file.symbols.iter()
            .filter(|s| s.constraints.as_ref().map(|c| c.level == LockLevel::Frozen).unwrap_or(false))
            .collect();
        
        if !frozen_symbols.is_empty() {
            println!("\nFrozen Symbols:");
            for s in frozen_symbols {
                println!("  {} (lines {}-{})", s.name, s.lines.start, s.lines.end);
                if let Some(ref c) = s.constraints {
                    println!("    → {}", c.directive);
                }
            }
        }
    }
    
    Ok(())
}
```

---

### Module: Indexer Updates

#### Task I01-I04: Enhanced Indexing

**File**: `cli/src/index/indexer.rs`

```rust
impl Indexer {
    /// Index with RFC-001 directive support
    pub async fn index_with_directives<P: AsRef<Path>>(&self, root: P) -> Result<Cache> {
        let mut cache = self.index(root).await?;
        
        // Track missing directive warnings (per Q07 = D: first-N then summary)
        let mut missing_count = 0;
        let max_warnings = 5;
        
        for (path, file) in &mut cache.files {
            // Extract purpose from @acp:purpose annotation
            if let Some(purpose_ann) = file.annotations.iter()
                .find(|a| a.name == "purpose") 
            {
                file.purpose = purpose_ann.directive.clone()
                    .or_else(|| purpose_ann.value.clone());
            }
            
            // Warn on missing directives
            for ann in &file.annotations {
                if ann.directive.is_none() && !ann.auto_generated {
                    missing_count += 1;
                    if missing_count <= max_warnings {
                        eprintln!("Warning: {}:{} @acp:{} missing directive",
                            path, ann.line, ann.name);
                    }
                }
            }
        }
        
        if missing_count > max_warnings {
            eprintln!("... and {} more annotations missing directives", 
                missing_count - max_warnings);
            eprintln!("Run 'acp migrate --add-directives' to fix");
        }
        
        Ok(cache)
    }
}
```

---

## Build Verification

After applying changes, verify:

```bash
# Check compilation
cargo check

# Run tests
cargo test

# Build release
cargo build --release

# Test on sample project
./target/release/acp index --verbose test-project/
```

## Test Updates

Add tests for new functionality:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_parse_annotation_with_directive() {
        let parser = Parser::new();
        let content = r#"
            // @acp:lock frozen - MUST NOT modify this file
            export function test() {}
        "#;
        
        let annotations = parser.parse_annotations(content);
        assert_eq!(annotations.len(), 1);
        assert_eq!(annotations[0].name, "lock");
        assert_eq!(annotations[0].value, Some("frozen".into()));
        assert_eq!(annotations[0].directive, 
            Some("MUST NOT modify this file".into()));
        assert!(!annotations[0].auto_generated);
    }
    
    #[test]
    fn test_auto_generate_directive() {
        let parser = Parser::new();
        let content = "// @acp:lock frozen\n";
        
        let annotations = parser.parse_annotations(content);
        assert!(annotations[0].directive.is_some());
        assert!(annotations[0].auto_generated);
    }
    
    #[test]
    fn test_multiline_directive() {
        let parser = Parser::new();
        let content = r#"
            // @acp:lock restricted - Explain changes and wait for approval.
            //   This file handles payment processing.
            //   Contact: payments-team@company.com
        "#;
        
        let annotations = parser.parse_annotations(content);
        assert!(annotations[0].directive.unwrap().contains("payments-team"));
    }
}
```

## Completion Criteria

### Remediation Complete When:
- [ ] All P tasks (Parser) complete
- [ ] All C tasks (Cache) complete
- [ ] All N tasks (Constraints) complete
- [ ] All M tasks (Commands) complete
- [ ] All I tasks (Indexer) complete
- [ ] `cargo check` passes
- [ ] `cargo test` passes
- [ ] Sample project indexes correctly
- [ ] Cache output matches new schema
- [ ] Summary generated

## Rollback

If changes break the build:

```bash
# Restore from backup
./scripts/rollback-cli.sh --backup .acp/backups/cli-TIMESTAMP

# Or use git
git checkout -- cli/src/
```
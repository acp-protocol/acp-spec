# RFC-0005: CLI Implementation for Annotation Provenance Tracking

- **RFC ID**: 0005
- **Title**: CLI Implementation for Annotation Provenance Tracking
- **Author**: ACP Protocol Team
- **Status**: Implemented
- **Created**: 2025-12-22
- **Updated**: 2025-12-22
- **Accepted**: 2025-12-22
- **Implemented**: 2025-12-22
- **Implements**: RFC-0003
- **Discussion**: TBD

---

## Summary

This RFC specifies the CLI implementation changes required to support RFC-0003 (Annotation Provenance Tracking). It details enhancements to the parser, cache types, indexer, annotate command, query command, and introduces a new `acp review` command for managing annotation review workflows.

## Motivation

### Problem Statement

RFC-0003 defines the specification for annotation provenance tracking, including new annotations (`@acp:source`, `@acp:source-confidence`, `@acp:source-reviewed`, `@acp:source-id`), cache schema extensions, and config schema extensions. However, the ACP reference CLI implementation needs corresponding updates to:

1. Parse and recognize the new provenance annotations
2. Track provenance data during indexing
3. Emit provenance markers when generating annotations
4. Support querying by provenance attributes
5. Provide provenance statistics
6. Enable review workflows for auto-generated annotations

### Goals

1. Extend the annotation parser to recognize `@acp:source*` annotations
2. Extend cache types to store provenance metadata per annotation
3. Enhance the indexer to capture and track provenance during indexing
4. Update the `acp annotate` command to emit provenance markers
5. Add provenance filters (`--source`, `--confidence`, `--needs-review`) to `acp query`
6. Add `--provenance` flag to `acp query stats` for provenance dashboard
7. Implement new `acp review` command for annotation review workflows

### Non-Goals

- Changing the provenance annotation syntax (defined in RFC-0003)
- Modifying the cache or config schemas (completed in RFC-0003)
- Implementing AI-assisted annotation refinement (future work)
- Adding GUI or IDE integration (separate effort)

## Detailed Design

### Overview

The implementation is organized into six components:

1. **Parser Extension** - Recognize and parse `@acp:source*` annotations
2. **Cache Types Extension** - Add `AnnotationProvenance` struct and related types
3. **Indexer Extension** - Extract and track provenance during indexing
4. **Annotate Command Extension** - Emit provenance markers when generating annotations
5. **Query Command Extension** - Add provenance filters
6. **Review Command (New)** - Interactive review workflow

### Component 1: Parser Extension

**Files**: `cli/src/parse/mod.rs`, `cli/src/parse/annotations.rs`

#### New Types

```rust
/// Source origin for annotation provenance
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SourceOrigin {
    Explicit,
    Converted,
    Heuristic,
    Refined,
    Inferred,
}

impl Default for SourceOrigin {
    fn default() -> Self {
        SourceOrigin::Explicit
    }
}

/// Provenance metadata for an annotation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProvenanceMarker {
    pub source: SourceOrigin,
    pub confidence: Option<f64>,
    pub reviewed: Option<bool>,
    pub generation_id: Option<String>,
}

/// Extended annotation with provenance
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnnotationWithProvenance {
    pub annotation: Annotation,
    pub provenance: Option<ProvenanceMarker>,
}
```

#### Parser Changes

The parser must recognize these patterns:

```rust
// Regex patterns for provenance annotations
lazy_static! {
    static ref SOURCE_PATTERN: Regex =
        Regex::new(r"@acp:source\s+(explicit|converted|heuristic|refined|inferred)(?:\s+-\s+(.+))?").unwrap();

    static ref CONFIDENCE_PATTERN: Regex =
        Regex::new(r"@acp:source-confidence\s+(\d+\.?\d*)(?:\s+-\s+(.+))?").unwrap();

    static ref REVIEWED_PATTERN: Regex =
        Regex::new(r"@acp:source-reviewed\s+(true|false)(?:\s+-\s+(.+))?").unwrap();

    static ref ID_PATTERN: Regex =
        Regex::new(r"@acp:source-id\s+([a-zA-Z0-9\-]+)(?:\s+-\s+(.+))?").unwrap();
}
```

**Parsing Algorithm**:

1. After parsing a block of annotations, look for trailing `@acp:source*` annotations
2. Associate provenance with ALL immediately preceding non-provenance annotations
3. Validate confidence is in range [0.0, 1.0]
4. Validate source origin is a known value

#### Validation

| Condition                  | Permissive Mode              | Strict Mode   |
|----------------------------|------------------------------|---------------|
| Unknown source origin      | Warn, default to `heuristic` | Error         |
| Confidence out of range    | Warn, clamp to [0.0, 1.0]    | Error         |
| Multiple provenance blocks | Warn, use first              | Error         |

### Component 2: Cache Types Extension

**Files**: `cli/src/cache/types.rs`

#### New Types

```rust
/// Provenance metadata for a single annotation value
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AnnotationProvenance {
    /// The annotation value
    pub value: String,

    /// Origin of the annotation
    #[serde(default, skip_serializing_if = "is_explicit")]
    pub source: SourceOrigin,

    /// Confidence score (0.0-1.0), only for auto-generated
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confidence: Option<f64>,

    /// Whether annotation is flagged for review
    #[serde(default, skip_serializing_if = "is_false")]
    pub needs_review: bool,

    /// Whether annotation has been reviewed
    #[serde(default, skip_serializing_if = "is_false")]
    pub reviewed: bool,

    /// When the annotation was reviewed (ISO 8601)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reviewed_at: Option<String>,

    /// When the annotation was generated (ISO 8601)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generated_at: Option<String>,

    /// Generation batch identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generation_id: Option<String>,
}

/// Top-level provenance statistics
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProvenanceStats {
    pub summary: ProvenanceSummary,
    pub low_confidence: Vec<LowConfidenceEntry>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_generation: Option<GenerationInfo>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProvenanceSummary {
    pub total: u64,
    pub by_source: SourceCounts,
    pub needs_review: u64,
    pub reviewed: u64,
    pub average_confidence: HashMap<String, f64>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SourceCounts {
    pub explicit: u64,
    pub converted: u64,
    pub heuristic: u64,
    pub refined: u64,
    pub inferred: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LowConfidenceEntry {
    pub target: String,
    pub annotation: String,
    pub confidence: f64,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GenerationInfo {
    pub id: String,
    pub timestamp: String,
    pub annotations_generated: u64,
    pub files_affected: u64,
}
```

#### FileEntry Extension

```rust
pub struct FileEntry {
    // ... existing fields ...

    /// Annotation provenance tracking (RFC-0003)
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub annotations: HashMap<String, AnnotationProvenance>,
}
```

#### SymbolEntry Extension

```rust
pub struct SymbolEntry {
    // ... existing fields ...

    /// Annotation provenance tracking (RFC-0003)
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub annotations: HashMap<String, AnnotationProvenance>,
}
```

#### Cache Extension

```rust
pub struct Cache {
    // ... existing fields ...

    /// Provenance statistics (RFC-0003)
    #[serde(default, skip_serializing_if = "ProvenanceStats::is_empty")]
    pub provenance: ProvenanceStats,
}
```

### Component 3: Indexer Extension

**Files**: `cli/src/index/indexer.rs`

#### Changes

1. **Extract Provenance During Parsing**

```rust
fn extract_provenance(
    annotations: &[AnnotationWithProvenance],
) -> HashMap<String, AnnotationProvenance> {
    let mut result = HashMap::new();

    for ann in annotations {
        if let Some(ref prov) = ann.provenance {
            let key = annotation_key(&ann.annotation);
            result.insert(key, AnnotationProvenance {
                value: ann.annotation.value.clone(),
                source: prov.source,
                confidence: prov.confidence,
                needs_review: prov.confidence.map_or(false, |c| c < 0.8),
                reviewed: prov.reviewed.unwrap_or(false),
                reviewed_at: None,
                generated_at: Some(chrono::Utc::now().to_rfc3339()),
                generation_id: prov.generation_id.clone(),
            });
        }
    }

    result
}
```

2. **Compute Provenance Statistics**

```rust
fn compute_provenance_stats(cache: &Cache) -> ProvenanceStats {
    let mut stats = ProvenanceStats::default();
    let mut confidence_sums: HashMap<String, (f64, u64)> = HashMap::new();

    // Iterate over all files and symbols
    for file in cache.files.values() {
        for (key, prov) in &file.annotations {
            stats.summary.total += 1;
            increment_source_count(&mut stats.summary.by_source, prov.source);

            if prov.needs_review {
                stats.summary.needs_review += 1;
            }
            if prov.reviewed {
                stats.summary.reviewed += 1;
            }
            if let Some(conf) = prov.confidence {
                let source_key = format!("{:?}", prov.source).to_lowercase();
                let entry = confidence_sums.entry(source_key).or_insert((0.0, 0));
                entry.0 += conf;
                entry.1 += 1;

                // Track low confidence
                if conf < 0.5 {
                    stats.low_confidence.push(LowConfidenceEntry {
                        target: file.path.clone(),
                        annotation: key.clone(),
                        confidence: conf,
                        value: prov.value.clone(),
                    });
                }
            }
        }
    }

    // Same for symbols...

    // Calculate averages
    for (source, (sum, count)) in confidence_sums {
        if count > 0 {
            stats.summary.average_confidence.insert(source, sum / count as f64);
        }
    }

    stats
}
```

3. **Update Index Function**

```rust
pub async fn index<P: AsRef<Path>>(
    root: P,
    config: &Config,
) -> Result<Cache> {
    // ... existing logic ...

    // After indexing all files
    cache.provenance = compute_provenance_stats(&cache);

    Ok(cache)
}
```

### Component 4: Annotate Command Extension

**Files**: `cli/src/commands/annotate.rs`, `cli/src/annotate/writer.rs`

#### New Options

```rust
pub struct AnnotateOptions {
    // ... existing options ...

    /// Disable provenance markers
    #[arg(long)]
    pub no_provenance: bool,

    /// Minimum confidence threshold (don't emit below this)
    #[arg(long, default_value = "0.5")]
    pub min_confidence: f64,

    /// Mark all as needing review
    #[arg(long)]
    pub mark_needs_review: bool,
}
```

#### Writer Changes

When writing annotations, emit provenance markers:

```rust
fn write_annotation_with_provenance(
    writer: &mut impl Write,
    annotation: &Annotation,
    source: SourceOrigin,
    confidence: f64,
    generation_id: &str,
    options: &AnnotateOptions,
) -> Result<()> {
    // Write the main annotation
    writeln!(writer, " * @acp:{} \"{}\" - {}",
        annotation.name,
        annotation.value,
        annotation.directive)?;

    // Write provenance markers (unless disabled)
    if !options.no_provenance {
        writeln!(writer, " * @acp:source {} - Auto-generated by acp annotate",
            source.as_str())?;

        writeln!(writer, " * @acp:source-confidence {:.2} - Confidence score",
            confidence)?;

        if confidence < 0.8 || options.mark_needs_review {
            writeln!(writer, " * @acp:source-reviewed false - Flagged for human review")?;
        }

        writeln!(writer, " * @acp:source-id {} - Generation batch ID",
            generation_id)?;
    }

    Ok(())
}
```

#### Generation ID

Generate unique batch IDs for each run:

```rust
fn generate_batch_id() -> String {
    let now = chrono::Utc::now();
    let random: u16 = rand::random();
    format!("gen-{}-{:03}", now.format("%Y%m%d"), random % 1000)
}
```

### Component 5: Query Command Extension

**Files**: `cli/src/commands/query.rs`

#### New Subcommand Options

```rust
#[derive(Args)]
pub struct QueryOptions {
    // ... existing options ...

    /// Filter by source origin
    #[arg(long)]
    pub source: Option<SourceOrigin>,

    /// Filter by confidence (e.g., "<0.7", ">=0.9")
    #[arg(long)]
    pub confidence: Option<String>,

    /// Show only annotations needing review
    #[arg(long)]
    pub needs_review: bool,
}
```

#### New Stats Options

```rust
#[derive(Args)]
pub struct StatsOptions {
    // ... existing options ...

    /// Show provenance statistics
    #[arg(long)]
    pub provenance: bool,
}
```

#### Confidence Filter Parsing

```rust
fn parse_confidence_filter(expr: &str) -> Result<ConfidenceFilter> {
    let expr = expr.trim();

    if let Some(val) = expr.strip_prefix("<=") {
        return Ok(ConfidenceFilter::LessOrEqual(val.parse()?));
    }
    if let Some(val) = expr.strip_prefix(">=") {
        return Ok(ConfidenceFilter::GreaterOrEqual(val.parse()?));
    }
    if let Some(val) = expr.strip_prefix('<') {
        return Ok(ConfidenceFilter::Less(val.parse()?));
    }
    if let Some(val) = expr.strip_prefix('>') {
        return Ok(ConfidenceFilter::Greater(val.parse()?));
    }
    if let Some(val) = expr.strip_prefix('=') {
        return Ok(ConfidenceFilter::Equal(val.parse()?));
    }

    Err(anyhow!("Invalid confidence filter: {}", expr))
}

enum ConfidenceFilter {
    Less(f64),
    LessOrEqual(f64),
    Greater(f64),
    GreaterOrEqual(f64),
    Equal(f64),
}
```

#### Provenance Stats Display

```rust
fn display_provenance_stats(cache: &Cache) {
    let stats = &cache.provenance;

    println!("Annotation Provenance Statistics");
    println!("================================");
    println!();
    println!("Total annotations tracked: {}", stats.summary.total);
    println!();
    println!("By Source:");
    println!("  explicit:  {:>5} ({:.1}%)",
        stats.summary.by_source.explicit,
        percentage(stats.summary.by_source.explicit, stats.summary.total));
    println!("  converted: {:>5} ({:.1}%)",
        stats.summary.by_source.converted,
        percentage(stats.summary.by_source.converted, stats.summary.total));
    println!("  heuristic: {:>5} ({:.1}%)",
        stats.summary.by_source.heuristic,
        percentage(stats.summary.by_source.heuristic, stats.summary.total));
    println!("  refined:   {:>5} ({:.1}%)",
        stats.summary.by_source.refined,
        percentage(stats.summary.by_source.refined, stats.summary.total));
    println!("  inferred:  {:>5} ({:.1}%)",
        stats.summary.by_source.inferred,
        percentage(stats.summary.by_source.inferred, stats.summary.total));
    println!();
    println!("Review Status:");
    println!("  Needs review: {}", stats.summary.needs_review);
    println!("  Reviewed:     {}", stats.summary.reviewed);
    println!();
    println!("Average Confidence:");
    for (source, avg) in &stats.summary.average_confidence {
        println!("  {}: {:.2}", source, avg);
    }

    if !stats.low_confidence.is_empty() {
        println!();
        println!("Low Confidence Annotations ({}):", stats.low_confidence.len());
        for entry in stats.low_confidence.iter().take(10) {
            println!("  {} [{}]: {} ({:.2})",
                entry.target, entry.annotation, entry.value, entry.confidence);
        }
        if stats.low_confidence.len() > 10 {
            println!("  ... and {} more", stats.low_confidence.len() - 10);
        }
    }
}
```

### Component 6: Review Command (New)

**Files**: `cli/src/commands/review.rs` (new)

#### Command Structure

```rust
#[derive(Parser)]
pub struct ReviewCommand {
    #[command(subcommand)]
    pub subcommand: Option<ReviewSubcommand>,

    /// Filter by source origin
    #[arg(long)]
    pub source: Option<SourceOrigin>,

    /// Filter by confidence (e.g., "<0.7")
    #[arg(long)]
    pub confidence: Option<String>,

    /// Filter by domain
    #[arg(long)]
    pub domain: Option<String>,

    /// Path to cache file
    #[arg(long, default_value = ".acp.cache.json")]
    pub cache: PathBuf,

    /// Output format
    #[arg(long, default_value = "human")]
    pub format: OutputFormat,
}

#[derive(Subcommand)]
pub enum ReviewSubcommand {
    /// List annotations needing review
    List,

    /// Mark annotations as reviewed
    Mark(MarkOptions),

    /// Interactive review mode
    Interactive,
}

#[derive(Args)]
pub struct MarkOptions {
    /// Mark as reviewed
    #[arg(long)]
    pub reviewed: bool,

    /// Mark specific file
    #[arg(long)]
    pub file: Option<PathBuf>,

    /// Mark specific symbol
    #[arg(long)]
    pub symbol: Option<String>,

    /// Mark all matching filters as reviewed
    #[arg(long)]
    pub all: bool,
}
```

#### List Subcommand

```rust
async fn list_for_review(
    cache: &Cache,
    options: &ReviewCommand,
) -> Result<Vec<ReviewItem>> {
    let mut items = Vec::new();

    for (path, file) in &cache.files {
        for (key, prov) in &file.annotations {
            if should_include(prov, options) {
                items.push(ReviewItem {
                    target: path.clone(),
                    annotation: key.clone(),
                    value: prov.value.clone(),
                    source: prov.source,
                    confidence: prov.confidence,
                    line: None, // Would need to track in cache
                });
            }
        }
    }

    // Sort by confidence (lowest first)
    items.sort_by(|a, b| {
        a.confidence.unwrap_or(1.0)
            .partial_cmp(&b.confidence.unwrap_or(1.0))
            .unwrap()
    });

    Ok(items)
}

fn should_include(prov: &AnnotationProvenance, options: &ReviewCommand) -> bool {
    // Must need review
    if prov.reviewed {
        return false;
    }

    // Source filter
    if let Some(ref source) = options.source {
        if prov.source != *source {
            return false;
        }
    }

    // Confidence filter
    if let Some(ref conf_expr) = options.confidence {
        if let Ok(filter) = parse_confidence_filter(conf_expr) {
            if let Some(conf) = prov.confidence {
                if !filter.matches(conf) {
                    return false;
                }
            }
        }
    }

    true
}
```

#### Mark Subcommand

```rust
async fn mark_reviewed(
    cache: &mut Cache,
    options: &MarkOptions,
    filters: &ReviewCommand,
) -> Result<u64> {
    let mut count = 0;
    let now = chrono::Utc::now().to_rfc3339();

    for file in cache.files.values_mut() {
        // Check file filter
        if let Some(ref path) = options.file {
            if file.path != path.to_string_lossy() {
                continue;
            }
        }

        for prov in file.annotations.values_mut() {
            if should_include(prov, filters) {
                prov.reviewed = true;
                prov.needs_review = false;
                prov.reviewed_at = Some(now.clone());
                count += 1;
            }
        }
    }

    // Same for symbols...

    // Recompute provenance stats
    cache.provenance = compute_provenance_stats(cache);

    Ok(count)
}
```

#### Interactive Mode

```rust
async fn interactive_review(
    cache: &mut Cache,
    options: &ReviewCommand,
) -> Result<()> {
    let items = list_for_review(cache, options).await?;

    if items.is_empty() {
        println!("No annotations need review!");
        return Ok(());
    }

    println!("Interactive Review Mode");
    println!("=======================");
    println!("{} annotations to review", items.len());
    println!();
    println!("Commands: [a]ccept, [s]kip, [e]dit, [q]uit");
    println!();

    for item in items {
        println!("File: {}", item.target);
        println!("Annotation: @acp:{}", item.annotation);
        println!("Value: \"{}\"", item.value);
        println!("Source: {:?}", item.source);
        if let Some(conf) = item.confidence {
            println!("Confidence: {:.2}", conf);
        }
        println!();

        print!("> ");
        std::io::stdout().flush()?;

        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;

        match input.trim().chars().next() {
            Some('a') | Some('A') => {
                mark_annotation_reviewed(cache, &item)?;
                println!("Marked as reviewed.");
            }
            Some('s') | Some('S') => {
                println!("Skipped.");
            }
            Some('e') | Some('E') => {
                println!("Edit mode not yet implemented.");
            }
            Some('q') | Some('Q') => {
                println!("Exiting review.");
                break;
            }
            _ => {
                println!("Unknown command. Skipping.");
            }
        }

        println!();
    }

    Ok(())
}
```

### CLI Integration

**File**: `cli/src/main.rs`

Add the review command:

```rust
#[derive(Subcommand)]
enum Commands {
    // ... existing commands ...

    /// Review auto-generated annotations
    Review(ReviewCommand),
}

async fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        // ... existing handlers ...

        Commands::Review(cmd) => {
            handle_review(cmd).await?;
        }
    }

    Ok(())
}
```

## Drawbacks

- **Complexity**: Adds significant complexity to the parser and indexer
- **Cache Size**: Provenance tracking increases cache file size
- **Breaking Change**: Existing caches won't have provenance data
- **Performance**: Additional processing during indexing and annotation generation

## Alternatives

### Alternative A: External Provenance Store

Store provenance data in a separate `.acp.provenance.json` file instead of in the cache.

**Pros:**
- Doesn't increase cache size
- Easier to clear provenance without rebuilding cache

**Cons:**
- Two files to manage
- Must keep in sync

**Why rejected:** Increased operational complexity; cache should be self-contained.

### Alternative B: Git-Based Provenance

Track annotation changes through git history instead of explicit markers.

**Pros:**
- No annotation overhead
- Natural history tracking

**Cons:**
- Requires git
- Harder to query
- Doesn't capture confidence

**Why rejected:** Not all projects use git; explicit markers are more queryable.

## Compatibility

### Backward Compatibility

- Existing caches will have empty `provenance` and `annotations` fields (optional fields)
- Existing annotations without `@acp:source` markers are assumed to be `explicit`
- No breaking changes to existing CLI behavior

### Forward Compatibility

- Future versions can add more source types
- Confidence scoring can be improved without breaking changes
- Review workflow can be extended

### Migration Path

No migration required. Provenance tracking is additive:

1. Re-index with new CLI version to populate provenance stats
2. Run `acp annotate` on new files to generate provenance markers
3. Use `acp review` to review existing auto-generated annotations

## Implementation

### Phase 1: Core Types and Parser (~4 hours)

**Files:**
- `cli/src/parse/mod.rs` - Add provenance annotation parsing
- `cli/src/cache/types.rs` - Add provenance types

**Deliverables:**
- `SourceOrigin` enum
- `AnnotationProvenance` struct
- `ProvenanceStats` and related types
- Parser recognizes `@acp:source*` annotations

### Phase 2: Indexer Integration (~3 hours)

**Files:**
- `cli/src/index/indexer.rs` - Extract and track provenance

**Deliverables:**
- Provenance extracted during indexing
- Provenance stats computed for cache
- File/symbol entries populated with `annotations` field

### Phase 3: Annotate Command Extension (~3 hours)

**Files:**
- `cli/src/commands/annotate.rs` - Add provenance options
- `cli/src/annotate/writer.rs` - Emit provenance markers

**Deliverables:**
- `--no-provenance` flag
- `--min-confidence` flag
- `--mark-needs-review` flag
- Provenance markers written to files

### Phase 4: Query Command Extension (~2 hours)

**Files:**
- `cli/src/commands/query.rs` - Add provenance filters

**Deliverables:**
- `--source` filter
- `--confidence` filter
- `--needs-review` filter
- `acp query stats --provenance` output

### Phase 5: Review Command (~4 hours)

**Files:**
- `cli/src/commands/review.rs` (new)
- `cli/src/main.rs` - Register command

**Deliverables:**
- `acp review list` subcommand
- `acp review mark` subcommand
- `acp review interactive` subcommand
- Cache update and save

### Phase 6: Testing (~3 hours)

**Files:**
- `cli/tests/provenance.rs` (new)
- `cli/tests/review.rs` (new)

**Deliverables:**
- Unit tests for parser
- Unit tests for cache types
- Integration tests for indexer
- Integration tests for commands

## Rollout Plan

1. **Phase 1-2**: Implement behind `--features provenance` flag
2. **Phase 3-4**: Enable by default, add CLI options
3. **Phase 5**: Release `acp review` command
4. **Phase 6**: Remove feature flag, full release

## Open Questions

1. Should we support bulk edit in interactive review mode?
2. Should confidence thresholds be configurable per-project?
3. How should we handle annotation merging during `acp annotate --apply`?

## Resolved Questions

1. **Q**: Should provenance be stored in cache or separate file?
   **A**: In cache, for self-containment.

2. **Q**: Should `@acp:source` be required or optional?
   **A**: Optional. Missing `@acp:source` implies `explicit`.

3. **Q**: What happens to provenance when annotation is manually edited?
   **A**: Source changes to `explicit`, confidence and reviewed fields are cleared.

## References

- [RFC-0003: Annotation Provenance Tracking](./rfc-0003-annotation-provenance-tracking.md)
- [ACP-1.0.md Section 11: Annotation Provenance](../spec/chapters/05-annotations.md#11-annotation-provenance-rfc-0003)
- [cache.schema.json](../schemas/v1/cache.schema.json)
- [config.schema.json](../schemas/v1/config.schema.json)

---

## Appendix

### A. Example CLI Sessions

**Generating annotations with provenance:**

```bash
$ acp annotate src/auth/ --apply --level standard

Annotating files...
  src/auth/session.ts: 3 annotations generated
  src/auth/jwt.ts: 2 annotations generated

Summary:
  Files processed: 2
  Annotations added: 5
  Average confidence: 0.78
  Flagged for review: 2

Generation ID: gen-20251222-042
```

**Querying by provenance:**

```bash
$ acp query stats --provenance

Annotation Provenance Statistics
================================

Total annotations tracked: 150

By Source:
  explicit:   80 (53.3%)
  converted:  20 (13.3%)
  heuristic:  45 (30.0%)
  refined:     5 (3.3%)
  inferred:    0 (0.0%)

Review Status:
  Needs review: 12
  Reviewed:     58

Average Confidence:
  converted: 0.92
  heuristic: 0.76

Low Confidence Annotations (3):
  src/utils/helpers.ts [domain]: utility (0.45)
  src/api/router.ts [fn]: route handler (0.52)
  src/db/models.ts [layer]: persistence (0.48)
```

**Reviewing annotations:**

```bash
$ acp review --confidence "<0.7" --source heuristic

Annotations needing review: 12

1. src/utils/helpers.ts
   @acp:domain "utility" - Consider domain context when making changes
   Source: heuristic, Confidence: 0.45

   [a]ccept  [s]kip  [e]dit  [q]uit
   > a
   Marked as reviewed.

2. src/api/router.ts
   @acp:fn "route handler" - Use this understanding when calling
   Source: heuristic, Confidence: 0.52

   [a]ccept  [s]kip  [e]dit  [q]uit
   > s
   Skipped.
```

### B. Estimated Effort

| Phase | Tasks | Estimated Time |
|-------|-------|----------------|
| Phase 1: Core Types | Parser + Types | 4 hours |
| Phase 2: Indexer | Integration | 3 hours |
| Phase 3: Annotate | Command Extension | 3 hours |
| Phase 4: Query | Filters + Stats | 2 hours |
| Phase 5: Review | New Command | 4 hours |
| Phase 6: Testing | Unit + Integration | 3 hours |
| **Total** | **6 phases** | **~19 hours** |

---

## Changelog

| Date | Change |
|------|--------|
| 2025-12-22 | Initial draft |

---

<!--
## RFC Process Checklist (for maintainers)

- [X] RFC number assigned
- [X] Added to rfcs/
- [N/A] Discussion link added
- [X] Initial review complete
- [N/A] Community feedback period (2+ weeks)
- [N/A] FCP announced
- [N/A] FCP complete (10 days)
- [X] Decision made
- [X] Status updated
-->

# Implementation Plan: RFC-0005

**RFC**: RFC-0005 CLI Implementation for Annotation Provenance Tracking
**Created**: 2025-12-22
**Status**: Ready for Implementation
**Implements**: RFC-0003

---

## Overview

RFC-0005 specifies CLI changes to support RFC-0003 (Annotation Provenance Tracking). This plan details the implementation across 6 phases with specific file modifications, code changes, and acceptance criteria.

### Goals

1. Extend parser to recognize `@acp:source*` provenance annotations
2. Add provenance types to cache module
3. Enhance indexer to track provenance during indexing
4. Update `acp annotate` to emit provenance markers
5. Add provenance filters to `acp query`
6. Implement new `acp review` command

### Non-Goals

- Changing annotation syntax (RFC-0003 scope)
- Modifying cache/config schemas (completed in RFC-0003 Phase 1)
- AI-assisted refinement (future work)

---

## Phase 1: Core Types and Parser (~4 hours)

### T1.1: Add SourceOrigin enum to parse/mod.rs

**Component**: Parser
**File**: `cli/src/parse/mod.rs`
**Depends On**: None
**Estimated Time**: 30 minutes

**Description**:
Add the `SourceOrigin` enum for annotation provenance tracking.

**Code Changes**:
```rust
// Add after existing imports
use serde::{Deserialize, Serialize};

/// Source origin for annotation provenance (RFC-0003)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum SourceOrigin {
    #[default]
    Explicit,
    Converted,
    Heuristic,
    Refined,
    Inferred,
}

impl SourceOrigin {
    pub fn as_str(&self) -> &'static str {
        match self {
            SourceOrigin::Explicit => "explicit",
            SourceOrigin::Converted => "converted",
            SourceOrigin::Heuristic => "heuristic",
            SourceOrigin::Refined => "refined",
            SourceOrigin::Inferred => "inferred",
        }
    }
}

impl std::str::FromStr for SourceOrigin {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "explicit" => Ok(SourceOrigin::Explicit),
            "converted" => Ok(SourceOrigin::Converted),
            "heuristic" => Ok(SourceOrigin::Heuristic),
            "refined" => Ok(SourceOrigin::Refined),
            "inferred" => Ok(SourceOrigin::Inferred),
            _ => Err(format!("Unknown source origin: {}", s)),
        }
    }
}
```

**Acceptance Criteria**:
- [ ] SourceOrigin enum compiles
- [ ] Default is `Explicit`
- [ ] Serializes to lowercase
- [ ] FromStr parses all variants

---

### T1.2: Add ProvenanceMarker struct to parse/mod.rs

**Component**: Parser
**File**: `cli/src/parse/mod.rs`
**Depends On**: T1.1
**Estimated Time**: 30 minutes

**Description**:
Add the `ProvenanceMarker` struct to hold parsed provenance data.

**Code Changes**:
```rust
/// Provenance metadata for an annotation (RFC-0003)
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ProvenanceMarker {
    /// Source origin (explicit, converted, heuristic, refined, inferred)
    pub source: SourceOrigin,
    /// Confidence score (0.0-1.0), only for auto-generated
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confidence: Option<f64>,
    /// Whether annotation has been reviewed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reviewed: Option<bool>,
    /// Generation batch identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generation_id: Option<String>,
}

/// Extended annotation with provenance (RFC-0003)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnnotationWithProvenance {
    pub annotation: Annotation,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provenance: Option<ProvenanceMarker>,
}
```

**Acceptance Criteria**:
- [ ] ProvenanceMarker compiles
- [ ] Optional fields serialize correctly
- [ ] AnnotationWithProvenance wraps Annotation

---

### T1.3: Add provenance regex patterns

**Component**: Parser
**File**: `cli/src/parse/mod.rs`
**Depends On**: T1.2
**Estimated Time**: 45 minutes

**Description**:
Add regex patterns to recognize `@acp:source*` annotations.

**Code Changes**:
```rust
// Add after existing LazyLock patterns

/// Regex for @acp:source annotation
static SOURCE_PATTERN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"@acp:source\s+(explicit|converted|heuristic|refined|inferred)(?:\s+-\s+(.+))?$")
        .unwrap()
});

/// Regex for @acp:source-confidence annotation
static CONFIDENCE_PATTERN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"@acp:source-confidence\s+(\d+\.?\d*)(?:\s+-\s+(.+))?$").unwrap()
});

/// Regex for @acp:source-reviewed annotation
static REVIEWED_PATTERN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"@acp:source-reviewed\s+(true|false)(?:\s+-\s+(.+))?$").unwrap()
});

/// Regex for @acp:source-id annotation
static ID_PATTERN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"@acp:source-id\s+([a-zA-Z0-9\-]+)(?:\s+-\s+(.+))?$").unwrap()
});
```

**Acceptance Criteria**:
- [ ] All patterns compile
- [ ] SOURCE_PATTERN matches all 5 origin values
- [ ] CONFIDENCE_PATTERN captures decimal numbers
- [ ] REVIEWED_PATTERN captures true/false
- [ ] ID_PATTERN captures alphanumeric IDs with hyphens

---

### T1.4: Implement parse_provenance method

**Component**: Parser
**File**: `cli/src/parse/mod.rs`
**Depends On**: T1.3
**Estimated Time**: 1 hour

**Description**:
Add method to parse provenance annotations following a block of annotations.

**Code Changes**:
```rust
impl Parser {
    /// Parse provenance annotations from comment lines (RFC-0003)
    ///
    /// Returns a ProvenanceMarker if any @acp:source* annotations are found.
    pub fn parse_provenance(&self, lines: &[&str], start_idx: usize) -> Option<ProvenanceMarker> {
        let mut marker = ProvenanceMarker::default();
        let mut found_any = false;

        for i in start_idx..lines.len() {
            let line = lines[i];

            // Check for @acp:source
            if let Some(cap) = SOURCE_PATTERN.captures(line) {
                if let Ok(origin) = cap.get(1).unwrap().as_str().parse() {
                    marker.source = origin;
                    found_any = true;
                }
            }

            // Check for @acp:source-confidence
            if let Some(cap) = CONFIDENCE_PATTERN.captures(line) {
                if let Ok(conf) = cap.get(1).unwrap().as_str().parse::<f64>() {
                    // Clamp to valid range
                    marker.confidence = Some(conf.clamp(0.0, 1.0));
                    found_any = true;
                }
            }

            // Check for @acp:source-reviewed
            if let Some(cap) = REVIEWED_PATTERN.captures(line) {
                marker.reviewed = Some(cap.get(1).unwrap().as_str() == "true");
                found_any = true;
            }

            // Check for @acp:source-id
            if let Some(cap) = ID_PATTERN.captures(line) {
                marker.generation_id = Some(cap.get(1).unwrap().as_str().to_string());
                found_any = true;
            }

            // Stop if we hit a non-provenance annotation or non-comment line
            if !line.contains("@acp:source") && !line.trim().starts_with("//")
                && !line.trim().starts_with("*") && !line.trim().starts_with("#") {
                break;
            }
        }

        if found_any { Some(marker) } else { None }
    }
}
```

**Acceptance Criteria**:
- [ ] Method parses all 4 provenance annotation types
- [ ] Confidence is clamped to [0.0, 1.0]
- [ ] Returns None if no provenance found
- [ ] Stops at non-comment lines

---

### T1.5: Update parse_annotations to include provenance

**Component**: Parser
**File**: `cli/src/parse/mod.rs`
**Depends On**: T1.4
**Estimated Time**: 1 hour

**Description**:
Modify `parse_annotations` to detect and associate provenance with annotations.

**Code Changes**:
```rust
impl Parser {
    /// Parse @acp: annotations with provenance support (RFC-0003)
    pub fn parse_annotations_with_provenance(&self, content: &str) -> Vec<AnnotationWithProvenance> {
        let annotations = self.parse_annotations(content);
        let lines: Vec<&str> = content.lines().collect();

        let mut result: Vec<AnnotationWithProvenance> = Vec::new();
        let mut i = 0;

        while i < annotations.len() {
            let ann = &annotations[i];

            // Check if next annotation is a provenance marker
            let provenance = if ann.line < lines.len() {
                self.parse_provenance(&lines, ann.line) // 0-indexed from 1-indexed line
            } else {
                None
            };

            result.push(AnnotationWithProvenance {
                annotation: ann.clone(),
                provenance,
            });

            i += 1;
        }

        result
    }
}
```

**Acceptance Criteria**:
- [ ] Returns AnnotationWithProvenance for each annotation
- [ ] Provenance is associated with preceding annotation
- [ ] Existing parse_annotations behavior preserved

---

## Phase 2: Cache Types Extension (~2 hours)

### T2.1: Add AnnotationProvenance struct to cache/types.rs

**Component**: Cache
**File**: `cli/src/cache/types.rs`
**Depends On**: T1.1
**Estimated Time**: 45 minutes

**Description**:
Add provenance tracking struct matching RFC-0003 schema.

**Code Changes**:
```rust
// Add to imports
use crate::parse::SourceOrigin;

/// Provenance metadata for a single annotation value (RFC-0003)
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

fn is_explicit(source: &SourceOrigin) -> bool {
    matches!(source, SourceOrigin::Explicit)
}
```

**Acceptance Criteria**:
- [ ] Struct compiles and matches schema
- [ ] Optional fields skip serialization when empty
- [ ] Source defaults to Explicit

---

### T2.2: Add ProvenanceStats struct to cache/types.rs

**Component**: Cache
**File**: `cli/src/cache/types.rs`
**Depends On**: T2.1
**Estimated Time**: 45 minutes

**Description**:
Add top-level provenance statistics structure.

**Code Changes**:
```rust
/// Top-level provenance statistics (RFC-0003)
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProvenanceStats {
    pub summary: ProvenanceSummary,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub low_confidence: Vec<LowConfidenceEntry>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_generation: Option<GenerationInfo>,
}

impl ProvenanceStats {
    pub fn is_empty(&self) -> bool {
        self.summary.total == 0
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProvenanceSummary {
    pub total: u64,
    pub by_source: SourceCounts,
    pub needs_review: u64,
    pub reviewed: u64,
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub average_confidence: HashMap<String, f64>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SourceCounts {
    #[serde(default)]
    pub explicit: u64,
    #[serde(default)]
    pub converted: u64,
    #[serde(default)]
    pub heuristic: u64,
    #[serde(default)]
    pub refined: u64,
    #[serde(default)]
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

**Acceptance Criteria**:
- [ ] All structs compile
- [ ] is_empty() correctly detects empty stats
- [ ] Matches RFC-0003 cache schema

---

### T2.3: Add annotations field to FileEntry and SymbolEntry

**Component**: Cache
**File**: `cli/src/cache/types.rs`
**Depends On**: T2.1
**Estimated Time**: 30 minutes

**Description**:
Add `annotations` HashMap to existing entry types.

**Code Changes**:
```rust
// In FileEntry struct, add:
    /// Annotation provenance tracking (RFC-0003)
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub annotations: HashMap<String, AnnotationProvenance>,

// In SymbolEntry struct, add:
    /// Annotation provenance tracking (RFC-0003)
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub annotations: HashMap<String, AnnotationProvenance>,
```

**Acceptance Criteria**:
- [ ] Both structs have annotations field
- [ ] Field skips serialization when empty
- [ ] Existing tests pass

---

### T2.4: Add provenance field to Cache struct

**Component**: Cache
**File**: `cli/src/cache/types.rs`
**Depends On**: T2.2
**Estimated Time**: 15 minutes

**Description**:
Add top-level provenance statistics to Cache.

**Code Changes**:
```rust
// In Cache struct, add:
    /// Provenance statistics (RFC-0003)
    #[serde(default, skip_serializing_if = "ProvenanceStats::is_empty")]
    pub provenance: ProvenanceStats,
```

**Acceptance Criteria**:
- [ ] Cache has provenance field
- [ ] Defaults to empty ProvenanceStats
- [ ] Skips serialization when empty

---

## Phase 3: Indexer Extension (~3 hours)

### T3.1: Add extract_provenance function to indexer.rs

**Component**: Indexer
**File**: `cli/src/index/indexer.rs`
**Depends On**: T2.1
**Estimated Time**: 1 hour

**Description**:
Add function to extract provenance from parsed annotations.

**Code Changes**:
```rust
use crate::cache::AnnotationProvenance;
use crate::parse::{AnnotationWithProvenance, SourceOrigin};

/// Extract provenance data from parsed annotations (RFC-0003)
fn extract_provenance(
    annotations: &[AnnotationWithProvenance],
    threshold: f64,
) -> HashMap<String, AnnotationProvenance> {
    let mut result = HashMap::new();

    for ann in annotations {
        // Skip provenance-only annotations
        if ann.annotation.name.starts_with("source") {
            continue;
        }

        let key = format!("@acp:{}", ann.annotation.name);

        let prov = if let Some(ref marker) = ann.provenance {
            AnnotationProvenance {
                value: ann.annotation.value.clone().unwrap_or_default(),
                source: marker.source,
                confidence: marker.confidence,
                needs_review: marker.confidence.map_or(false, |c| c < threshold),
                reviewed: marker.reviewed.unwrap_or(false),
                reviewed_at: None,
                generated_at: Some(chrono::Utc::now().to_rfc3339()),
                generation_id: marker.generation_id.clone(),
            }
        } else {
            // No provenance = explicit annotation
            AnnotationProvenance {
                value: ann.annotation.value.clone().unwrap_or_default(),
                source: SourceOrigin::Explicit,
                confidence: None,
                needs_review: false,
                reviewed: true, // Explicit annotations are considered reviewed
                reviewed_at: None,
                generated_at: None,
                generation_id: None,
            }
        };

        result.insert(key, prov);
    }

    result
}
```

**Acceptance Criteria**:
- [ ] Extracts provenance for all annotations
- [ ] Flags low confidence as needs_review
- [ ] Explicit annotations marked as reviewed

---

### T3.2: Add compute_provenance_stats function

**Component**: Indexer
**File**: `cli/src/index/indexer.rs`
**Depends On**: T3.1
**Estimated Time**: 1 hour

**Description**:
Compute aggregate provenance statistics for the cache.

**Code Changes**:
```rust
use crate::cache::{ProvenanceStats, ProvenanceSummary, SourceCounts, LowConfidenceEntry};

/// Compute provenance statistics from cache (RFC-0003)
fn compute_provenance_stats(cache: &Cache, low_conf_threshold: f64) -> ProvenanceStats {
    let mut stats = ProvenanceStats::default();
    let mut confidence_sums: HashMap<String, (f64, u64)> = HashMap::new();

    // Process file annotations
    for file in cache.files.values() {
        for (key, prov) in &file.annotations {
            update_stats(&mut stats, &mut confidence_sums, key, prov, &file.path, low_conf_threshold);
        }
    }

    // Process symbol annotations
    for symbol in cache.symbols.values() {
        for (key, prov) in &symbol.annotations {
            let target = format!("{}:{}", symbol.file, symbol.name);
            update_stats(&mut stats, &mut confidence_sums, key, prov, &target, low_conf_threshold);
        }
    }

    // Calculate average confidence per source
    for (source, (sum, count)) in confidence_sums {
        if count > 0 {
            stats.summary.average_confidence.insert(source, sum / count as f64);
        }
    }

    // Sort low confidence by confidence ascending
    stats.low_confidence.sort_by(|a, b| {
        a.confidence.partial_cmp(&b.confidence).unwrap_or(std::cmp::Ordering::Equal)
    });

    stats
}

fn update_stats(
    stats: &mut ProvenanceStats,
    confidence_sums: &mut HashMap<String, (f64, u64)>,
    key: &str,
    prov: &AnnotationProvenance,
    target: &str,
    low_conf_threshold: f64,
) {
    stats.summary.total += 1;

    // Count by source
    match prov.source {
        SourceOrigin::Explicit => stats.summary.by_source.explicit += 1,
        SourceOrigin::Converted => stats.summary.by_source.converted += 1,
        SourceOrigin::Heuristic => stats.summary.by_source.heuristic += 1,
        SourceOrigin::Refined => stats.summary.by_source.refined += 1,
        SourceOrigin::Inferred => stats.summary.by_source.inferred += 1,
    }

    if prov.needs_review {
        stats.summary.needs_review += 1;
    }
    if prov.reviewed {
        stats.summary.reviewed += 1;
    }

    // Track confidence
    if let Some(conf) = prov.confidence {
        let source_key = prov.source.as_str().to_string();
        let entry = confidence_sums.entry(source_key).or_insert((0.0, 0));
        entry.0 += conf;
        entry.1 += 1;

        // Track low confidence
        if conf < low_conf_threshold {
            stats.low_confidence.push(LowConfidenceEntry {
                target: target.to_string(),
                annotation: key.to_string(),
                confidence: conf,
                value: prov.value.clone(),
            });
        }
    }
}
```

**Acceptance Criteria**:
- [ ] Counts all source types
- [ ] Computes average confidence per source
- [ ] Collects low confidence entries

---

### T3.3: Integrate provenance into index function

**Component**: Indexer
**File**: `cli/src/index/indexer.rs`
**Depends On**: T3.1, T3.2
**Estimated Time**: 1 hour

**Description**:
Update the main index function to extract and store provenance.

**Code Changes**:
```rust
// In index() method, after parsing files:

// Update the parallel processing section to use parse_annotations_with_provenance
let results: Vec<_> = files
    .par_iter()
    .filter_map(|path| {
        let mut parse_result = annotation_parser.parse(path).ok()?;

        // Parse provenance (RFC-0003)
        if let Ok(content) = std::fs::read_to_string(path) {
            let annotations_with_prov = annotation_parser.parse_annotations_with_provenance(&content);
            let review_threshold = self.config.annotate
                .as_ref()
                .and_then(|a| a.provenance.as_ref())
                .map(|p| p.review_threshold)
                .unwrap_or(0.8);

            // Extract provenance for file entry
            parse_result.file.annotations = extract_provenance(&annotations_with_prov, review_threshold);
        }

        // ... rest of parsing logic ...

        Some(parse_result)
    })
    .collect();

// At the end, before returning cache:
let low_conf_threshold = self.config.annotate
    .as_ref()
    .and_then(|a| a.provenance.as_ref())
    .map(|p| p.min_confidence)
    .unwrap_or(0.5);

let mut cache = builder.build();
cache.provenance = compute_provenance_stats(&cache, low_conf_threshold);

Ok(cache)
```

**Acceptance Criteria**:
- [ ] Provenance extracted during indexing
- [ ] File entries have annotations populated
- [ ] Cache provenance stats computed
- [ ] Config thresholds respected

---

## Phase 4: Annotate Command Extension (~3 hours)

### T4.1: Add provenance options to AnnotateOptions

**Component**: Annotate Command
**File**: `cli/src/commands/annotate.rs`
**Depends On**: T2.1
**Estimated Time**: 30 minutes

**Description**:
Add CLI options for provenance control.

**Code Changes**:
```rust
/// Options for the annotate command
#[derive(Debug, Clone)]
pub struct AnnotateOptions {
    // ... existing fields ...

    /// Disable provenance markers (RFC-0003)
    pub no_provenance: bool,

    /// Minimum confidence threshold (don't emit below this)
    pub min_confidence: f64,

    /// Mark all generated annotations as needing review
    pub mark_needs_review: bool,
}

impl Default for AnnotateOptions {
    fn default() -> Self {
        Self {
            // ... existing defaults ...
            no_provenance: false,
            min_confidence: 0.5,
            mark_needs_review: false,
        }
    }
}
```

**Acceptance Criteria**:
- [ ] Options added to struct
- [ ] Defaults are sensible
- [ ] Options parsed from CLI

---

### T4.2: Add generation ID generator

**Component**: Annotate Command
**File**: `cli/src/commands/annotate.rs`
**Depends On**: None
**Estimated Time**: 15 minutes

**Description**:
Generate unique batch IDs for annotation generation runs.

**Code Changes**:
```rust
use rand::Rng;

/// Generate unique batch ID for annotation generation (RFC-0003)
fn generate_batch_id() -> String {
    let now = chrono::Utc::now();
    let random: u16 = rand::thread_rng().gen_range(0..1000);
    format!("gen-{}-{:03}", now.format("%Y%m%d"), random)
}
```

**Acceptance Criteria**:
- [ ] IDs are unique per run
- [ ] Format matches RFC spec

---

### T4.3: Update Writer to emit provenance markers

**Component**: Annotate Writer
**File**: `cli/src/annotate/writer.rs`
**Depends On**: T4.1, T4.2
**Estimated Time**: 1.5 hours

**Description**:
Modify annotation writer to include provenance annotations.

**Code Changes**:
```rust
use crate::parse::SourceOrigin;

/// Write annotation with provenance markers (RFC-0003)
fn write_annotation_with_provenance(
    writer: &mut impl std::io::Write,
    annotation_type: &str,
    value: &str,
    directive: &str,
    source: SourceOrigin,
    confidence: f64,
    generation_id: &str,
    options: &WriteOptions,
) -> Result<()> {
    // Write the main annotation
    writeln!(writer, " * @acp:{} \"{}\" - {}",
        annotation_type, value, directive)?;

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

**Acceptance Criteria**:
- [ ] Main annotation written first
- [ ] Provenance markers follow
- [ ] --no-provenance skips markers
- [ ] Low confidence flagged for review

---

### T4.4: Update execute_annotate for provenance

**Component**: Annotate Command
**File**: `cli/src/commands/annotate.rs`
**Depends On**: T4.2, T4.3
**Estimated Time**: 1 hour

**Description**:
Integrate provenance into the annotate command flow.

**Code Changes**:
```rust
pub fn execute_annotate(options: AnnotateOptions, config: Config) -> Result<()> {
    // Generate batch ID at start (RFC-0003)
    let generation_id = generate_batch_id();

    // ... existing code ...

    // Update statistics output to include provenance info
    match options.format {
        OutputFormat::Summary => {
            // ... existing output ...

            if !options.no_provenance {
                println!("\n{}:", style("Provenance").bold());
                println!("  Generation ID:    {}", generation_id);
                println!("  Flagged for review: {}", flagged_count);
            }
        }
        OutputFormat::Json => {
            // Add provenance to JSON output
            let output = serde_json::json!({
                // ... existing fields ...
                "provenance": {
                    "generation_id": generation_id,
                    "enabled": !options.no_provenance,
                    "flagged_for_review": flagged_count,
                }
            });
            println!("{}", serde_json::to_string_pretty(&output)?);
        }
        // ...
    }

    Ok(())
}
```

**Acceptance Criteria**:
- [ ] Generation ID created per run
- [ ] Provenance info in summary output
- [ ] Provenance info in JSON output

---

## Phase 5: Query Command Extension (~2 hours)

### T5.1: Add provenance filter options

**Component**: Query Command
**File**: `cli/src/commands/query.rs`
**Depends On**: T2.1
**Estimated Time**: 30 minutes

**Description**:
Add CLI options for filtering by provenance.

**Code Changes**:
```rust
use crate::parse::SourceOrigin;

/// Options for the query command (RFC-0003 extended)
#[derive(Debug, Clone)]
pub struct QueryOptions {
    pub cache: PathBuf,
    pub json: bool,

    /// Filter by source origin (RFC-0003)
    pub source: Option<SourceOrigin>,

    /// Filter by confidence expression (e.g., "<0.7", ">=0.9")
    pub confidence: Option<String>,

    /// Show only annotations needing review
    pub needs_review: bool,
}

/// Query subcommand types (RFC-0003 extended)
#[derive(Debug, Clone)]
pub enum QuerySubcommand {
    // ... existing variants ...

    /// Show provenance statistics
    Provenance,
}
```

**Acceptance Criteria**:
- [ ] Options added to struct
- [ ] New Provenance subcommand variant

---

### T5.2: Add confidence filter parser

**Component**: Query Command
**File**: `cli/src/commands/query.rs`
**Depends On**: T5.1
**Estimated Time**: 30 minutes

**Description**:
Parse confidence filter expressions.

**Code Changes**:
```rust
#[derive(Debug, Clone)]
enum ConfidenceFilter {
    Less(f64),
    LessOrEqual(f64),
    Greater(f64),
    GreaterOrEqual(f64),
    Equal(f64),
}

impl ConfidenceFilter {
    fn parse(expr: &str) -> Result<Self> {
        let expr = expr.trim();

        if let Some(val) = expr.strip_prefix("<=") {
            return Ok(Self::LessOrEqual(val.parse()?));
        }
        if let Some(val) = expr.strip_prefix(">=") {
            return Ok(Self::GreaterOrEqual(val.parse()?));
        }
        if let Some(val) = expr.strip_prefix('<') {
            return Ok(Self::Less(val.parse()?));
        }
        if let Some(val) = expr.strip_prefix('>') {
            return Ok(Self::Greater(val.parse()?));
        }
        if let Some(val) = expr.strip_prefix('=') {
            return Ok(Self::Equal(val.parse()?));
        }

        Err(anyhow!("Invalid confidence filter: {}", expr))
    }

    fn matches(&self, confidence: f64) -> bool {
        match self {
            Self::Less(v) => confidence < *v,
            Self::LessOrEqual(v) => confidence <= *v,
            Self::Greater(v) => confidence > *v,
            Self::GreaterOrEqual(v) => confidence >= *v,
            Self::Equal(v) => (confidence - v).abs() < 0.001,
        }
    }
}
```

**Acceptance Criteria**:
- [ ] Parses all comparison operators
- [ ] matches() returns correct results

---

### T5.3: Implement query_provenance function

**Component**: Query Command
**File**: `cli/src/commands/query.rs`
**Depends On**: T5.2
**Estimated Time**: 1 hour

**Description**:
Display provenance statistics dashboard.

**Code Changes**:
```rust
fn query_provenance(cache_data: &Cache, json: bool) -> Result<()> {
    let stats = &cache_data.provenance;

    if json {
        println!("{}", serde_json::to_string_pretty(stats)?);
        return Ok(());
    }

    println!("{}", style("Annotation Provenance Statistics").bold());
    println!("{}", "=".repeat(40));
    println!();

    println!("Total annotations tracked: {}", stats.summary.total);
    println!();

    println!("{}:", style("By Source").bold());
    let total = stats.summary.total as f64;
    println!("  explicit:  {:>5} ({:.1}%)",
        stats.summary.by_source.explicit,
        (stats.summary.by_source.explicit as f64 / total) * 100.0);
    println!("  converted: {:>5} ({:.1}%)",
        stats.summary.by_source.converted,
        (stats.summary.by_source.converted as f64 / total) * 100.0);
    println!("  heuristic: {:>5} ({:.1}%)",
        stats.summary.by_source.heuristic,
        (stats.summary.by_source.heuristic as f64 / total) * 100.0);
    println!("  refined:   {:>5} ({:.1}%)",
        stats.summary.by_source.refined,
        (stats.summary.by_source.refined as f64 / total) * 100.0);
    println!("  inferred:  {:>5} ({:.1}%)",
        stats.summary.by_source.inferred,
        (stats.summary.by_source.inferred as f64 / total) * 100.0);

    println!();
    println!("{}:", style("Review Status").bold());
    println!("  Needs review: {}", stats.summary.needs_review);
    println!("  Reviewed:     {}", stats.summary.reviewed);

    if !stats.summary.average_confidence.is_empty() {
        println!();
        println!("{}:", style("Average Confidence").bold());
        for (source, avg) in &stats.summary.average_confidence {
            println!("  {}: {:.2}", source, avg);
        }
    }

    if !stats.low_confidence.is_empty() {
        println!();
        println!("{} ({}):", style("Low Confidence Annotations").bold(),
            stats.low_confidence.len());
        for entry in stats.low_confidence.iter().take(10) {
            println!("  {} [{}]: {} ({:.2})",
                entry.target, entry.annotation, entry.value, entry.confidence);
        }
        if stats.low_confidence.len() > 10 {
            println!("  ... and {} more", stats.low_confidence.len() - 10);
        }
    }

    Ok(())
}
```

**Acceptance Criteria**:
- [ ] Displays all stats from cache
- [ ] JSON mode outputs raw stats
- [ ] Shows top 10 low confidence

---

## Phase 6: Review Command (~4 hours)

### T6.1: Create review.rs command file

**Component**: Review Command
**File**: `cli/src/commands/review.rs` (NEW)
**Depends On**: T2.1, T5.2
**Estimated Time**: 30 minutes

**Description**:
Create new file with command structure.

**Code Changes**:
```rust
//! @acp:module "Review Command"
//! @acp:summary "Review and manage annotation provenance"
//! @acp:domain cli
//! @acp:layer handler

use std::collections::HashMap;
use std::io::{self, Write};
use std::path::PathBuf;

use anyhow::Result;
use console::style;

use crate::cache::{AnnotationProvenance, Cache};
use crate::parse::SourceOrigin;

/// Options for the review command (RFC-0003)
#[derive(Debug, Clone)]
pub struct ReviewOptions {
    /// Cache file path
    pub cache: PathBuf,
    /// Filter by source origin
    pub source: Option<SourceOrigin>,
    /// Filter by confidence expression
    pub confidence: Option<String>,
    /// Filter by domain
    pub domain: Option<String>,
    /// Output as JSON
    pub json: bool,
}

impl Default for ReviewOptions {
    fn default() -> Self {
        Self {
            cache: PathBuf::from(".acp.cache.json"),
            source: None,
            confidence: None,
            domain: None,
            json: false,
        }
    }
}

/// Review subcommands
#[derive(Debug, Clone)]
pub enum ReviewSubcommand {
    /// List annotations needing review
    List,
    /// Mark annotations as reviewed
    Mark { file: Option<PathBuf>, symbol: Option<String>, all: bool },
    /// Interactive review mode
    Interactive,
}

/// Item for review display
#[derive(Debug, Clone)]
pub struct ReviewItem {
    pub target: String,
    pub annotation: String,
    pub value: String,
    pub source: SourceOrigin,
    pub confidence: Option<f64>,
}
```

**Acceptance Criteria**:
- [ ] File created with correct structure
- [ ] Types compile

---

### T6.2: Implement list subcommand

**Component**: Review Command
**File**: `cli/src/commands/review.rs`
**Depends On**: T6.1
**Estimated Time**: 1 hour

**Description**:
List annotations needing review.

**Code Changes**:
```rust
/// Execute the review command
pub fn execute_review(
    options: ReviewOptions,
    subcommand: ReviewSubcommand,
) -> Result<()> {
    let cache = Cache::from_json(&options.cache)?;

    match subcommand {
        ReviewSubcommand::List => list_for_review(&cache, &options),
        ReviewSubcommand::Mark { file, symbol, all } => {
            let mut cache = cache;
            mark_reviewed(&mut cache, &options, file.as_ref(), symbol.as_deref(), all)?;
            cache.write_json(&options.cache)?;
            Ok(())
        }
        ReviewSubcommand::Interactive => {
            let mut cache = cache;
            interactive_review(&mut cache, &options)?;
            cache.write_json(&options.cache)?;
            Ok(())
        }
    }
}

fn list_for_review(cache: &Cache, options: &ReviewOptions) -> Result<()> {
    let items = collect_review_items(cache, options);

    if items.is_empty() {
        println!("{} No annotations need review!", style("✓").green());
        return Ok(());
    }

    if options.json {
        let json_items: Vec<_> = items.iter().map(|item| {
            serde_json::json!({
                "target": item.target,
                "annotation": item.annotation,
                "value": item.value,
                "source": item.source.as_str(),
                "confidence": item.confidence,
            })
        }).collect();
        println!("{}", serde_json::to_string_pretty(&json_items)?);
        return Ok(());
    }

    println!("{} annotations need review:", items.len());
    println!();

    for (i, item) in items.iter().enumerate() {
        println!("{}. {}", i + 1, style(&item.target).cyan());
        println!("   @acp:{} \"{}\"", item.annotation, item.value);
        println!("   Source: {:?}", item.source);
        if let Some(conf) = item.confidence {
            println!("   Confidence: {:.2}", conf);
        }
        println!();
    }

    Ok(())
}

fn collect_review_items(cache: &Cache, options: &ReviewOptions) -> Vec<ReviewItem> {
    let mut items = Vec::new();
    let conf_filter = options.confidence.as_ref()
        .and_then(|c| ConfidenceFilter::parse(c).ok());

    // Collect from files
    for (path, file) in &cache.files {
        for (key, prov) in &file.annotations {
            if should_include(prov, options, &conf_filter) {
                items.push(ReviewItem {
                    target: path.clone(),
                    annotation: key.trim_start_matches("@acp:").to_string(),
                    value: prov.value.clone(),
                    source: prov.source,
                    confidence: prov.confidence,
                });
            }
        }
    }

    // Collect from symbols
    for symbol in cache.symbols.values() {
        for (key, prov) in &symbol.annotations {
            if should_include(prov, options, &conf_filter) {
                items.push(ReviewItem {
                    target: format!("{}:{}", symbol.file, symbol.name),
                    annotation: key.trim_start_matches("@acp:").to_string(),
                    value: prov.value.clone(),
                    source: prov.source,
                    confidence: prov.confidence,
                });
            }
        }
    }

    // Sort by confidence (lowest first)
    items.sort_by(|a, b| {
        a.confidence.unwrap_or(1.0)
            .partial_cmp(&b.confidence.unwrap_or(1.0))
            .unwrap_or(std::cmp::Ordering::Equal)
    });

    items
}

fn should_include(
    prov: &AnnotationProvenance,
    options: &ReviewOptions,
    conf_filter: &Option<ConfidenceFilter>,
) -> bool {
    // Must need review
    if prov.reviewed || !prov.needs_review {
        return false;
    }

    // Source filter
    if let Some(ref source) = options.source {
        if prov.source != *source {
            return false;
        }
    }

    // Confidence filter
    if let Some(ref filter) = conf_filter {
        if let Some(conf) = prov.confidence {
            if !filter.matches(conf) {
                return false;
            }
        }
    }

    true
}
```

**Acceptance Criteria**:
- [ ] Lists all items needing review
- [ ] Filters by source work
- [ ] Filters by confidence work
- [ ] Sorts by confidence ascending

---

### T6.3: Implement mark subcommand

**Component**: Review Command
**File**: `cli/src/commands/review.rs`
**Depends On**: T6.2
**Estimated Time**: 1 hour

**Description**:
Mark annotations as reviewed.

**Code Changes**:
```rust
fn mark_reviewed(
    cache: &mut Cache,
    options: &ReviewOptions,
    file: Option<&PathBuf>,
    symbol: Option<&str>,
    all: bool,
) -> Result<()> {
    let now = chrono::Utc::now().to_rfc3339();
    let conf_filter = options.confidence.as_ref()
        .and_then(|c| ConfidenceFilter::parse(c).ok());
    let mut count = 0;

    // Mark file annotations
    for (path, file_entry) in cache.files.iter_mut() {
        // Check file filter
        if let Some(filter_path) = file {
            if !path.contains(&filter_path.to_string_lossy().to_string()) {
                continue;
            }
        }

        for prov in file_entry.annotations.values_mut() {
            if (all || should_include(prov, options, &conf_filter)) && !prov.reviewed {
                prov.reviewed = true;
                prov.needs_review = false;
                prov.reviewed_at = Some(now.clone());
                count += 1;
            }
        }
    }

    // Mark symbol annotations
    for sym in cache.symbols.values_mut() {
        // Check symbol filter
        if let Some(sym_filter) = symbol {
            if sym.name != sym_filter {
                continue;
            }
        }

        // Check file filter for symbol
        if let Some(filter_path) = file {
            if !sym.file.contains(&filter_path.to_string_lossy().to_string()) {
                continue;
            }
        }

        for prov in sym.annotations.values_mut() {
            if (all || should_include(prov, options, &conf_filter)) && !prov.reviewed {
                prov.reviewed = true;
                prov.needs_review = false;
                prov.reviewed_at = Some(now.clone());
                count += 1;
            }
        }
    }

    // Recompute provenance stats
    cache.provenance = compute_provenance_stats(cache, 0.5);

    println!("{} Marked {} annotations as reviewed", style("✓").green(), count);

    Ok(())
}
```

**Acceptance Criteria**:
- [ ] Marks matching annotations as reviewed
- [ ] Updates reviewed_at timestamp
- [ ] Recomputes provenance stats
- [ ] Saves cache

---

### T6.4: Implement interactive subcommand

**Component**: Review Command
**File**: `cli/src/commands/review.rs`
**Depends On**: T6.2, T6.3
**Estimated Time**: 1.5 hours

**Description**:
Interactive review workflow.

**Code Changes**:
```rust
fn interactive_review(cache: &mut Cache, options: &ReviewOptions) -> Result<()> {
    let items = collect_review_items(cache, options);

    if items.is_empty() {
        println!("{} No annotations need review!", style("✓").green());
        return Ok(());
    }

    println!("{}", style("Interactive Review Mode").bold());
    println!("{}", "=".repeat(40));
    println!("{} annotations to review", items.len());
    println!();
    println!("Commands: [a]ccept, [s]kip, [q]uit");
    println!();

    let now = chrono::Utc::now().to_rfc3339();
    let mut reviewed_count = 0;
    let mut skipped_count = 0;

    for item in &items {
        println!("{}", style(&item.target).cyan());
        println!("  @acp:{} \"{}\"", item.annotation, item.value);
        println!("  Source: {:?}", item.source);
        if let Some(conf) = item.confidence {
            println!("  Confidence: {:.2}", conf);
        }
        println!();

        print!("{} ", style(">").yellow());
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        match input.trim().chars().next() {
            Some('a') | Some('A') => {
                // Mark as reviewed in cache
                mark_single_reviewed(cache, &item, &now)?;
                println!("{} Marked as reviewed", style("✓").green());
                reviewed_count += 1;
            }
            Some('s') | Some('S') => {
                println!("Skipped");
                skipped_count += 1;
            }
            Some('q') | Some('Q') => {
                println!("\nExiting review");
                break;
            }
            _ => {
                println!("Unknown command, skipping");
                skipped_count += 1;
            }
        }

        println!();
    }

    // Recompute stats
    cache.provenance = compute_provenance_stats(cache, 0.5);

    println!("{}", "=".repeat(40));
    println!("Reviewed: {}, Skipped: {}", reviewed_count, skipped_count);

    Ok(())
}

fn mark_single_reviewed(
    cache: &mut Cache,
    item: &ReviewItem,
    timestamp: &str,
) -> Result<()> {
    let key = format!("@acp:{}", item.annotation);

    // Check if target is a file or symbol
    if item.target.contains(':') {
        // Symbol (format: file:name)
        let parts: Vec<&str> = item.target.rsplitn(2, ':').collect();
        if parts.len() == 2 {
            let sym_name = parts[0];
            if let Some(sym) = cache.symbols.get_mut(sym_name) {
                if let Some(prov) = sym.annotations.get_mut(&key) {
                    prov.reviewed = true;
                    prov.needs_review = false;
                    prov.reviewed_at = Some(timestamp.to_string());
                }
            }
        }
    } else {
        // File
        if let Some(file) = cache.files.get_mut(&item.target) {
            if let Some(prov) = file.annotations.get_mut(&key) {
                prov.reviewed = true;
                prov.needs_review = false;
                prov.reviewed_at = Some(timestamp.to_string());
            }
        }
    }

    Ok(())
}
```

**Acceptance Criteria**:
- [ ] Shows items one at a time
- [ ] Accept marks as reviewed
- [ ] Skip moves to next
- [ ] Quit exits early
- [ ] Stats recomputed at end

---

### T6.5: Register review command in main.rs

**Component**: CLI Main
**File**: `cli/src/main.rs`
**Depends On**: T6.1
**Estimated Time**: 30 minutes

**Description**:
Add review command to CLI.

**Code Changes**:
```rust
// Add to commands/mod.rs or main.rs imports
mod review;
use review::{execute_review, ReviewOptions, ReviewSubcommand};

// In Commands enum
#[derive(Subcommand)]
enum Commands {
    // ... existing commands ...

    /// Review auto-generated annotations (RFC-0003)
    Review {
        #[command(subcommand)]
        subcommand: Option<ReviewSubcommandCli>,

        /// Filter by source origin
        #[arg(long)]
        source: Option<String>,

        /// Filter by confidence (e.g., "<0.7")
        #[arg(long)]
        confidence: Option<String>,

        /// Cache file path
        #[arg(long, default_value = ".acp.cache.json")]
        cache: PathBuf,

        /// Output as JSON
        #[arg(long)]
        json: bool,
    },
}

#[derive(Subcommand)]
enum ReviewSubcommandCli {
    /// List annotations needing review
    List,
    /// Mark annotations as reviewed
    Mark {
        #[arg(long)]
        file: Option<PathBuf>,
        #[arg(long)]
        symbol: Option<String>,
        #[arg(long)]
        all: bool,
    },
    /// Interactive review mode
    Interactive,
}

// In main match
Commands::Review { subcommand, source, confidence, cache, json } => {
    let options = ReviewOptions {
        cache,
        source: source.and_then(|s| s.parse().ok()),
        confidence,
        domain: None,
        json,
    };
    let sub = match subcommand {
        Some(ReviewSubcommandCli::List) | None => ReviewSubcommand::List,
        Some(ReviewSubcommandCli::Mark { file, symbol, all }) =>
            ReviewSubcommand::Mark { file, symbol, all },
        Some(ReviewSubcommandCli::Interactive) => ReviewSubcommand::Interactive,
    };
    execute_review(options, sub)?;
}
```

**Acceptance Criteria**:
- [ ] `acp review` works
- [ ] `acp review list` works
- [ ] `acp review mark` works
- [ ] `acp review interactive` works
- [ ] All filters work

---

## Phase 7: Testing (~3 hours)

### T7.1: Parser tests

**Component**: Tests
**File**: `cli/tests/provenance_parser.rs` (NEW)
**Depends On**: Phase 1
**Estimated Time**: 1 hour

**Test Cases**:
- [ ] Parse @acp:source with all 5 values
- [ ] Parse @acp:source-confidence with valid float
- [ ] Parse @acp:source-confidence clamps out-of-range
- [ ] Parse @acp:source-reviewed true/false
- [ ] Parse @acp:source-id with valid ID
- [ ] Parse combined provenance block
- [ ] Missing provenance returns None

---

### T7.2: Cache types tests

**Component**: Tests
**File**: `cli/tests/provenance_cache.rs` (NEW)
**Depends On**: Phase 2
**Estimated Time**: 30 minutes

**Test Cases**:
- [ ] AnnotationProvenance serializes correctly
- [ ] ProvenanceStats is_empty() works
- [ ] FileEntry with annotations serializes
- [ ] SymbolEntry with annotations serializes
- [ ] Cache with provenance roundtrips

---

### T7.3: Indexer tests

**Component**: Tests
**File**: `cli/tests/provenance_indexer.rs` (NEW)
**Depends On**: Phase 3
**Estimated Time**: 1 hour

**Test Cases**:
- [ ] extract_provenance creates correct entries
- [ ] compute_provenance_stats counts correctly
- [ ] Low confidence entries collected
- [ ] Average confidence calculated correctly
- [ ] Explicit annotations marked as reviewed

---

### T7.4: Command integration tests

**Component**: Tests
**File**: `cli/tests/provenance_commands.rs` (NEW)
**Depends On**: Phases 4-6
**Estimated Time**: 30 minutes

**Test Cases**:
- [ ] `acp annotate` emits provenance markers
- [ ] `acp annotate --no-provenance` skips markers
- [ ] `acp query stats --provenance` shows stats
- [ ] `acp review list` shows items
- [ ] `acp review mark --all` marks all

---

## Dependencies

```
Phase 1: Core Types + Parser
  T1.1 ── T1.2 ── T1.3 ── T1.4 ── T1.5

Phase 2: Cache Types
  T1.1 ── T2.1 ── T2.2
              └── T2.3 ── T2.4

Phase 3: Indexer
  T2.1 ── T3.1 ── T3.2 ── T3.3

Phase 4: Annotate Command
  T2.1 ── T4.1 ── T4.2 ── T4.3 ── T4.4

Phase 5: Query Command
  T2.1 ── T5.1 ── T5.2 ── T5.3

Phase 6: Review Command
  T2.1 ── T6.1 ── T6.2 ── T6.3 ── T6.4 ── T6.5

Phase 7: Testing
  All phases ── T7.1, T7.2, T7.3, T7.4
```

---

## Risk Mitigation

| Risk | Likelihood | Impact | Mitigation |
|------|------------|--------|------------|
| Parser regex complexity | Medium | Medium | Test extensively with edge cases |
| Cache size increase | Low | Low | Use skip_serializing_if liberally |
| Interactive mode issues | Low | Low | Provide non-interactive fallback |
| Config integration | Low | Medium | Use sensible defaults |

---

## Estimated Effort

| Phase | Tasks | Estimated Time |
|-------|-------|----------------|
| Phase 1: Core Types + Parser | 5 tasks | 4 hours |
| Phase 2: Cache Types | 4 tasks | 2 hours |
| Phase 3: Indexer | 3 tasks | 3 hours |
| Phase 4: Annotate Command | 4 tasks | 3 hours |
| Phase 5: Query Command | 3 tasks | 2 hours |
| Phase 6: Review Command | 5 tasks | 4 hours |
| Phase 7: Testing | 4 tasks | 3 hours |
| **Total** | **28 tasks** | **~21 hours** |

---

## Success Criteria

RFC-0005 implementation is complete when:
- [ ] Parser recognizes all @acp:source* annotations
- [ ] Cache types store provenance correctly
- [ ] Indexer extracts and tracks provenance
- [ ] `acp annotate` emits provenance markers
- [ ] `acp query stats --provenance` shows dashboard
- [ ] `acp review` command works
- [ ] All tests pass
- [ ] RFC status updated to Implemented

---

## Next Steps

Ready for `/rfc.implement` to begin Phase 1.

Start with:
- T1.1: Add SourceOrigin enum
- T1.2: Add ProvenanceMarker struct

**Note**: Implementation should be done in the `acp-cli` repository at `/Users/dunnock/projects/acp-protocol/acp-cli/`.

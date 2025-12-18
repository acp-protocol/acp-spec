//! @acp:module "Cache Types"
//! @acp:summary "Data structures matching the .acp.cache.json schema"
//! @acp:domain cli
//! @acp:layer model
//!
//! These types serialize directly to/from `.acp.cache.json`

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::path::Path;

use crate::constraints::ConstraintIndex;
use crate::error::Result;

/// @acp:summary "Complete ACP cache file structure"
/// @acp:lock normal
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cache {
    /// Schema version
    pub version: String,
    /// Generation timestamp
    pub generated_at: DateTime<Utc>,
    /// Project metadata
    pub project: ProjectInfo,
    /// Aggregate statistics
    pub stats: Stats,
    /// Files indexed by path (O(1) lookup)
    pub files: HashMap<String, FileEntry>,
    /// Symbols indexed by name (O(1) lookup)
    pub symbols: HashMap<String, SymbolEntry>,
    /// Bidirectional call graph
    pub graph: CallGraph,
    /// Domain groupings
    pub domains: HashMap<String, DomainEntry>,
    /// Layer groupings
    pub layers: HashMap<String, Vec<String>>,
    /// Security-sensitive code index
    pub security: SecurityIndex,
    /// Frequently-called symbols
    pub hotpaths: Vec<HotpathEntry>,
    /// Stability classifications
    pub stability: StabilityIndex,
    /// AI behavioral constraints
    #[serde(default)]
    pub constraints: ConstraintIndex,
}

impl Cache {
    /// @acp:summary "Create a new empty cache"
    pub fn new(project_name: &str, root: &str) -> Self {
        Self {
            version: crate::VERSION.to_string(),
            generated_at: Utc::now(),
            project: ProjectInfo {
                name: project_name.to_string(),
                root: root.to_string(),
                description: None,
            },
            stats: Stats::default(),
            files: HashMap::new(),
            symbols: HashMap::new(),
            graph: CallGraph::default(),
            domains: HashMap::new(),
            layers: HashMap::new(),
            security: SecurityIndex::default(),
            hotpaths: Vec::new(),
            stability: StabilityIndex::default(),
            constraints: ConstraintIndex::default(),
        }
    }

    /// @acp:summary "Load cache from JSON file"
    pub fn from_json<P: AsRef<Path>>(path: P) -> Result<Self> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let cache = serde_json::from_reader(reader)?;
        Ok(cache)
    }

    /// @acp:summary "Write cache to JSON file"
    pub fn write_json<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let file = File::create(path)?;
        let writer = BufWriter::new(file);
        serde_json::to_writer_pretty(writer, self)?;
        Ok(())
    }

    /// @acp:summary "Get a symbol by name - O(1) lookup"
    pub fn get_symbol(&self, name: &str) -> Option<&SymbolEntry> {
        self.symbols.get(name)
    }

    /// @acp:summary "Get a file by path - O(1) lookup"
    pub fn get_file(&self, path: &str) -> Option<&FileEntry> {
        self.files.get(path)
    }

    /// @acp:summary "Get callers of a symbol from reverse call graph"
    pub fn get_callers(&self, symbol: &str) -> Option<&Vec<String>> {
        self.graph.reverse.get(symbol)
    }

    /// @acp:summary "Get callees of a symbol from forward call graph"
    pub fn get_callees(&self, symbol: &str) -> Option<&Vec<String>> {
        self.graph.forward.get(symbol)
    }

    /// @acp:summary "Get all files in a domain"
    pub fn get_domain_files(&self, domain: &str) -> Option<&Vec<String>> {
        self.domains.get(domain).map(|d| &d.files)
    }

    /// @acp:summary "Get all files in an architectural layer"
    pub fn get_layer_files(&self, layer: &str) -> Option<&Vec<String>> {
        self.layers.get(layer)
    }

    /// @acp:summary "Recalculate statistics after indexing"
    pub fn update_stats(&mut self) {
        self.stats.files = self.files.len();
        self.stats.symbols = self.symbols.len();
        self.stats.lines = self.files.values().map(|f| f.lines).sum();

        let annotated = self.symbols.values()
            .filter(|s| s.summary.is_some())
            .count();

        if self.stats.symbols > 0 {
            self.stats.annotation_coverage =
                (annotated as f64 / self.stats.symbols as f64) * 100.0;
        }
    }
}

/// @acp:summary "Builder for incremental cache construction"
pub struct CacheBuilder {
    cache: Cache,
}

impl CacheBuilder {
    pub fn new(project_name: &str, root: &str) -> Self {
        Self {
            cache: Cache::new(project_name, root),
        }
    }

    pub fn add_file(mut self, file: FileEntry) -> Self {
        let path = file.path.clone();
        self.cache.files.insert(path, file);
        self
    }

    pub fn add_symbol(mut self, symbol: SymbolEntry) -> Self {
        let name = symbol.name.clone();
        self.cache.symbols.insert(name, symbol);
        self
    }

    pub fn add_call_edge(mut self, from: &str, to: Vec<String>) -> Self {
        self.cache.graph.forward.insert(from.to_string(), to.clone());

        // Build reverse graph
        for callee in to {
            self.cache.graph.reverse
                .entry(callee)
                .or_default()
                .push(from.to_string());
        }
        self
    }

    pub fn add_domain(mut self, domain: DomainEntry) -> Self {
        let name = domain.name.clone();
        self.cache.domains.insert(name, domain);
        self
    }

    pub fn build(mut self) -> Cache {
        self.cache.update_stats();
        self.cache
    }
}

/// @acp:summary "Project metadata"
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectInfo {
    pub name: String,
    pub root: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// @acp:summary "Aggregate statistics"
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Stats {
    pub files: usize,
    pub symbols: usize,
    pub lines: usize,
    #[serde(default)]
    pub annotation_coverage: f64,
}

/// @acp:summary "File entry with metadata and guardrails"
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileEntry {
    pub path: String,
    pub module: String,
    pub lines: usize,
    #[serde(default)]
    pub domains: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub layer: Option<String>,
    #[serde(default)]
    pub stability: Stability,
    #[serde(default)]
    pub depends: Vec<String>,
    #[serde(default)]
    pub exports: Vec<String>,
    #[serde(default)]
    pub symbols: Vec<String>,
    #[serde(default)]
    pub keywords: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hash: Option<String>,
    /// Guardrail annotations parsed from this file
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub guardrails: Option<crate::constraints::FileGuardrails>,
}

/// @acp:summary "Symbol entry with metadata"
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SymbolEntry {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fqn: Option<String>,
    #[serde(rename = "type")]
    pub symbol_type: SymbolType,
    pub file: String,
    pub lines: [usize; 2],
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature: Option<String>,
    #[serde(default)]
    pub async_fn: bool,
    #[serde(default)]
    pub exported: bool,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub calls: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub throws: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub flags: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub side_effects: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub complexity: Option<String>,
}

/// @acp:summary "Symbol type enumeration"
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SymbolType {
    #[default]
    Fn,
    Class,
    Method,
    Const,
    Type,
    Interface,
    Var,
}

/// @acp:summary "Stability classification"
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Stability {
    Frozen,
    Stable,
    #[default]
    Active,
    Volatile,
}

/// @acp:summary "Bidirectional call graph"
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CallGraph {
    /// Forward: caller -> [callees]
    #[serde(default)]
    pub forward: HashMap<String, Vec<String>>,
    /// Reverse: callee -> [callers]
    #[serde(default)]
    pub reverse: HashMap<String, Vec<String>>,
}

/// @acp:summary "Domain grouping"
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DomainEntry {
    pub name: String,
    pub files: Vec<String>,
    #[serde(default)]
    pub symbols: Vec<String>,
    #[serde(default)]
    pub file_count: usize,
    #[serde(default)]
    pub symbol_count: usize,
}

/// @acp:summary "Security-sensitive code index"
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SecurityIndex {
    #[serde(default)]
    pub pii: Vec<String>,
    #[serde(default)]
    pub phi: Vec<String>,
    #[serde(default)]
    pub financial: Vec<String>,
    #[serde(default)]
    pub credentials: Vec<String>,
    #[serde(default)]
    pub auth_required: Vec<String>,
    #[serde(default)]
    pub audit_logged: Vec<String>,
    #[serde(default)]
    pub compliance: HashMap<String, Vec<String>>,
}

/// @acp:summary "Hotpath entry for frequently-called code"
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HotpathEntry {
    pub symbol: String,
    pub file: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency: Option<String>,
    #[serde(default)]
    pub callers: usize,
}

/// @acp:summary "Stability index groupings"
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StabilityIndex {
    #[serde(default)]
    pub frozen: Vec<String>,
    #[serde(default)]
    pub stable: Vec<String>,
    #[serde(default)]
    pub active: Vec<String>,
    #[serde(default)]
    pub volatile: Vec<String>,
    #[serde(default)]
    pub deprecated: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cache_roundtrip() {
        let cache = CacheBuilder::new("test", "/test")
            .add_symbol(SymbolEntry {
                name: "test_fn".to_string(),
                fqn: None,
                symbol_type: SymbolType::Fn,
                file: "test.rs".to_string(),
                lines: [1, 10],
                summary: Some("Test function".to_string()),
                signature: None,
                async_fn: false,
                exported: true,
                calls: vec![],
                throws: vec![],
                flags: vec![],
                side_effects: vec![],
                complexity: None,
            })
            .build();

        let json = serde_json::to_string_pretty(&cache).unwrap();
        let parsed: Cache = serde_json::from_str(&json).unwrap();

        assert_eq!(parsed.project.name, "test");
        assert!(parsed.symbols.contains_key("test_fn"));
    }
}
//! Cache data structures matching the JSON schema
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

/// Complete ACP cache file
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cache {
    pub version: String,
    pub generated_at: DateTime<Utc>,
    pub project: ProjectInfo,
    pub stats: Stats,
    pub files: HashMap<String, FileEntry>,
    pub symbols: HashMap<String, SymbolEntry>,
    pub graph: CallGraph,
    pub domains: HashMap<String, DomainEntry>,
    pub layers: HashMap<String, Vec<String>>,
    pub security: SecurityIndex,
    pub hotpaths: Vec<HotpathEntry>,
    pub stability: StabilityIndex,
    /// Constraint index for AI guardrails
    #[serde(default)]
    pub constraints: ConstraintIndex,
}

impl Cache {
    /// Create a new empty cache
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

    /// Load cache from JSON file
    pub fn from_json<P: AsRef<Path>>(path: P) -> Result<Self> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let cache = serde_json::from_reader(reader)?;
        Ok(cache)
    }

    /// Write cache to JSON file
    pub fn write_json<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let file = File::create(path)?;
        let writer = BufWriter::new(file);
        serde_json::to_writer_pretty(writer, self)?;
        Ok(())
    }

    /// Get a symbol by name (O(1))
    pub fn get_symbol(&self, name: &str) -> Option<&SymbolEntry> {
        self.symbols.get(name)
    }

    /// Get a file by path (O(1))
    pub fn get_file(&self, path: &str) -> Option<&FileEntry> {
        self.files.get(path)
    }

    /// Get callers of a symbol
    pub fn get_callers(&self, symbol: &str) -> Option<&Vec<String>> {
        self.graph.reverse.get(symbol)
    }

    /// Get callees of a symbol
    pub fn get_callees(&self, symbol: &str) -> Option<&Vec<String>> {
        self.graph.forward.get(symbol)
    }

    /// Get files in a domain
    pub fn get_domain_files(&self, domain: &str) -> Option<&Vec<String>> {
        self.domains.get(domain).map(|d| &d.files)
    }

    /// Get files in a layer
    pub fn get_layer_files(&self, layer: &str) -> Option<&Vec<String>> {
        self.layers.get(layer)
    }

    /// Update stats after indexing
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

/// Builder for creating cache incrementally
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectInfo {
    pub name: String,
    pub root: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Stats {
    pub files: usize,
    pub symbols: usize,
    pub lines: usize,
    #[serde(default)]
    pub annotation_coverage: f64,
}

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
    /// Guardrail annotations for this file
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub guardrails: Option<crate::guardrails::FileGuardrails>,
}

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

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Stability {
    Frozen,
    Stable,
    #[default]
    Active,
    Volatile,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CallGraph {
    /// caller -> [callees]
    #[serde(default)]
    pub forward: HashMap<String, Vec<String>>,
    /// callee -> [callers]
    #[serde(default)]
    pub reverse: HashMap<String, Vec<String>>,
}

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

//! Configuration for ACP

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// ACP configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// Project root directory
    #[serde(default = "default_root")]
    pub root: PathBuf,

    /// File patterns to include
    #[serde(default = "default_include")]
    pub include: Vec<String>,

    /// File patterns to exclude
    #[serde(default = "default_exclude")]
    pub exclude: Vec<String>,

    /// Output paths
    #[serde(default)]
    pub output: OutputConfig,

    /// Parser settings
    #[serde(default)]
    pub parser: ParserConfig,

    /// Indexer settings
    #[serde(default)]
    pub indexer: IndexerConfig,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            root: default_root(),
            include: default_include(),
            exclude: default_exclude(),
            output: OutputConfig::default(),
            parser: ParserConfig::default(),
            indexer: IndexerConfig::default(),
        }
    }
}

impl Config {
    /// Load config from .acp.config.json
    pub fn load<P: AsRef<std::path::Path>>(path: P) -> crate::Result<Self> {
        let content = std::fs::read_to_string(path)?;
        Ok(serde_json::from_str(&content)?)
    }

    /// Load from default location or create default
    pub fn load_or_default() -> Self {
        Self::load(".acp.config.json").unwrap_or_default()
    }
}

fn default_root() -> PathBuf {
    PathBuf::from(".")
}

fn default_include() -> Vec<String> {
    vec![
        "**/*.ts".to_string(),
        "**/*.tsx".to_string(),
        "**/*.js".to_string(),
        "**/*.jsx".to_string(),
        "**/*.rs".to_string(),
        "**/*.py".to_string(),
        "**/*.go".to_string(),
        "**/*.java".to_string(),
    ]
}

fn default_exclude() -> Vec<String> {
    vec![
        "**/node_modules/**".to_string(),
        "**/dist/**".to_string(),
        "**/build/**".to_string(),
        "**/target/**".to_string(),
        "**/.git/**".to_string(),
        "**/vendor/**".to_string(),
        "**/__pycache__/**".to_string(),
    ]
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutputConfig {
    /// Cache file path
    #[serde(default = "default_cache_path")]
    pub cache: PathBuf,

    /// Vars file path
    #[serde(default = "default_vars_path")]
    pub vars: PathBuf,

    /// Whether to also output SQLite
    #[serde(default)]
    pub sqlite: bool,
}

impl Default for OutputConfig {
    fn default() -> Self {
        Self {
            cache: default_cache_path(),
            vars: default_vars_path(),
            sqlite: false,
        }
    }
}

fn default_cache_path() -> PathBuf {
    PathBuf::from(".acp.cache.json")
}

fn default_vars_path() -> PathBuf {
    PathBuf::from(".acp.vars.json")
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParserConfig {
    /// Extract doc comments
    #[serde(default = "default_true")]
    pub extract_docs: bool,

    /// Parse function signatures
    #[serde(default = "default_true")]
    pub extract_signatures: bool,

    /// Track call graph
    #[serde(default = "default_true")]
    pub track_calls: bool,
}

impl Default for ParserConfig {
    fn default() -> Self {
        Self {
            extract_docs: true,
            extract_signatures: true,
            track_calls: true,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexerConfig {
    /// Number of parallel workers
    #[serde(default = "default_workers")]
    pub workers: usize,

    /// Generate vars automatically
    #[serde(default = "default_true")]
    pub auto_vars: bool,

    /// Infer domains from paths
    #[serde(default = "default_true")]
    pub infer_domains: bool,

    /// Infer layers from paths
    #[serde(default = "default_true")]
    pub infer_layers: bool,
}

impl Default for IndexerConfig {
    fn default() -> Self {
        Self {
            workers: default_workers(),
            auto_vars: true,
            infer_domains: true,
            infer_layers: true,
        }
    }
}

fn default_true() -> bool {
    true
}

fn default_workers() -> usize {
    num_cpus::get().max(1)
}

// Add num_cpus to Cargo.toml if not using default
#[cfg(not(feature = "num_cpus"))]
mod num_cpus {
    pub fn get() -> usize {
        4
    }
}

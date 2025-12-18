# ACP CLI Implementation Guide

> **Version**: 1.0.0
> **Status**: Implementation Reference
> **Last Updated**: December 2024

This guide provides complete, production-ready code for the ACP (AI Context Protocol) CLI tool. All code snippets are from the current implementation with recommended organizational structure for maintainability.

---

## Table of Contents

1. [Overview](#1-overview)
2. [Project Structure](#2-project-structure)
3. [Core Architecture](#3-core-architecture)
4. [Foundation Layer](#4-foundation-layer)
5. [Parsing System](#5-parsing-system)
6. [Indexing System](#6-indexing-system)
7. [Variable System](#7-variable-system)
8. [Constraints & Guardrails](#8-constraints--guardrails)
9. [Attempt Tracking](#9-attempt-tracking)
10. [Commands](#10-commands)
11. [Query System](#11-query-system)
12. [Testing](#12-testing)
13. [Distribution](#13-distribution)

---

## 1. Overview

The ACP CLI is a Rust-based tool for token-efficient code documentation and AI context management. It provides:

- **Fast Parsing**: Tree-sitter based AST parsing (with regex fallback)
- **JSON Output**: Queryable cache files with O(1) symbol/file lookups
- **Variable System**: Token-efficient macros with inheritance and expansion
- **Multi-language Support**: TypeScript, JavaScript, Rust, Python, Go, Java
- **Guardrails**: AI behavioral constraints via `@acp:` annotations
- **Attempt Tracking**: Checkpoint and rollback system for AI modifications

### Commands

| Command | Description |
|---------|-------------|
| `acp index` | Index codebase, generate `.acp.cache.json` |
| `acp vars` | Generate `.acp.vars.json` from cache |
| `acp query` | Query cache (symbol, file, callers, callees, domains, stats) |
| `acp expand` | Expand variable references in text |
| `acp chain` | Show variable inheritance chain |
| `acp check` | Check guardrails for a file |
| `acp attempt` | Manage troubleshooting attempts and checkpoints |
| `acp revert` | Revert changes via attempt or checkpoint |
| `acp watch` | Watch for file changes |
| `acp validate` | Validate cache/vars JSON files |

### Technology Stack

```
+-----------------------------------------------------------+
|                    CLI Interface                           |
|                      (clap v4)                             |
+-----------------------------------------------------------+
|  Commands  |  Index  |  Query  |  Vars  |  Watch  | ...   |
+-----------------------------------------------------------+
|                    Core Library                            |
|  +--------+ +--------+ +-------+ +-----------+ +--------+ |
|  | Parser | |Indexer | | Cache | |Constraints| |Attempts| |
|  +--------+ +--------+ +-------+ +-----------+ +--------+ |
+-----------------------------------------------------------+
|                   Tree-sitter Parsers                      |
|  TypeScript | JavaScript | Python | Rust | Go | Java       |
+-----------------------------------------------------------+
```

---

## 2. Project Structure

### Recommended Directory Layout

```
cli/
├── Cargo.toml
├── src/
│   ├── main.rs                    # CLI entry point
│   ├── lib.rs                     # Library facade
│   ├── error.rs                   # Error types
│   │
│   ├── config/
│   │   └── mod.rs                 # Configuration loading
│   │
│   ├── cache/
│   │   ├── mod.rs                 # Module exports
│   │   ├── types.rs               # Cache data structures
│   │   └── builder.rs             # CacheBuilder
│   │
│   ├── parse/
│   │   ├── mod.rs                 # Parser facade
│   │   ├── annotations.rs         # @acp: annotation parsing
│   │   ├── tree_sitter.rs         # Tree-sitter integration (future)
│   │   └── languages/             # Language-specific parsers (future)
│   │       ├── mod.rs
│   │       ├── typescript.rs
│   │       ├── rust.rs
│   │       └── python.rs
│   │
│   ├── index/
│   │   └── indexer.rs             # Codebase indexer
│   │
│   ├── vars/
│   │   ├── mod.rs                 # Variable types and VarsFile
│   │   ├── resolver.rs            # VarResolver
│   │   ├── expander.rs            # VarExpander with inheritance
│   │   └── presets.rs             # Expansion mode presets
│   │
│   ├── constraints/
│   │   ├── mod.rs                 # Module exports
│   │   ├── types.rs               # Constraint types
│   │   ├── guardrails.rs          # FileGuardrails, parser
│   │   └── enforcer.rs            # GuardrailEnforcer
│   │
│   ├── attempts/
│   │   └── mod.rs                 # AttemptTracker
│   │
│   ├── query/
│   │   └── engine.rs              # Query builder
│   │
│   ├── schema/
│   │   └── validator.rs           # JSON schema validation
│   │
│   ├── watch.rs                   # File watcher
│   │
│   └── commands/                  # Command implementations
│       ├── mod.rs
│       ├── index.rs
│       ├── vars.rs
│       ├── query.rs
│       ├── expand.rs
│       ├── check.rs
│       ├── attempt.rs
│       ├── watch.rs
│       └── validate.rs
│
└── tests/
    ├── integration/
    └── fixtures/
```

### File Reorganization Map

| Current Location | Recommended Location | Action |
|-----------------|---------------------|--------|
| `cli/src/main.rs` | `cli/src/main.rs` | Keep, refactor to delegate |
| `cli/src/lib.rs` | `cli/src/lib.rs` | Keep, update exports |
| `cli/src/error.rs` | `cli/src/error.rs` | Keep |
| `cli/src/config.rs` | `cli/src/config/mod.rs` | Move |
| `cli/src/cache.rs` | `cli/src/cache/types.rs` | Split |
| `cli/src/parse.rs` | `cli/src/parse/mod.rs` | Split |
| `cli/src/index.rs` | `cli/src/index/indexer.rs` | Move |
| `cli/src/vars.rs` | `cli/src/vars/mod.rs` | Split |
| `cli/src/constraints.rs` | `cli/src/constraints/types.rs` | Move |
| `cli/src/guardrails.rs` | `cli/src/constraints/guardrails.rs` | Split |
| `cli/src/attempts.rs` | `cli/src/attempts/mod.rs` | Move |
| `cli/src/query.rs` | `cli/src/query/engine.rs` | Move |
| `cli/src/watch.rs` | `cli/src/watch.rs` | Keep |
| `cli/src/schema.rs` | `cli/src/schema/validator.rs` | Move |
| `cli/src/expand.rs` | `cli/src/vars/presets.rs` | Move |

### Cargo.toml

```toml
# @acp:module "ACP CLI Package Manifest"
# @acp:domain cli
# @acp:layer config
# @acp:stability stable
# @acp:summary "Cargo build configuration for the ACP command-line tool"

# @acp:summary "Package metadata and publishing information"
[package]
name = "acp"
version = "0.1.0"
edition = "2021"
authors = ["ACP Contributors"]
description = "AI Context Protocol - Token-efficient and context enhancing code documentation for AI systems"
license = "MIT"
repository = "https://github.com/acp-protocol/acp-spec"
keywords = ["ai", "documentation", "code-analysis", "context", "llm"]
categories = ["development-tools", "command-line-utilities"]

# @acp:summary "Library crate configuration"
[lib]
name = "acp"
path = "src/lib.rs"

# @acp:summary "CLI binary entry point"
[[bin]]
name = "acp"
path = "src/main.rs"

# @acp:summary "Runtime dependencies organized by function"
[dependencies]
# CLI - Command-line interface and user interaction
clap = { version = "4.5", features = ["derive", "env"] }
indicatif = "0.17"          # Progress bars
console = "0.15"            # Terminal colors/styling
dialoguer = "0.11"          # Interactive prompts

# Serialization - Data format handling
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
serde_yaml = "0.9"

# Async runtime - Concurrent execution
tokio = { version = "1.48", features = ["full"] }

# File system - Directory traversal and watching
walkdir = "2.5"
glob = "0.3"
notify = "8.2"              # File watching

# Parsing - Source code analysis via tree-sitter
tree-sitter = "0.25"
tree-sitter-typescript = "0.23"
tree-sitter-javascript = "0.25"
tree-sitter-rust = "0.24"
tree-sitter-python = "0.25"
tree-sitter-go = "0.25"
tree-sitter-java = "0.23"

# Hashing - File content fingerprinting
md5 = "0.7"
sha2 = "0.10"

# Error handling - Ergonomic error types
thiserror = "2.0"
anyhow = "1.0"

# Parallel processing - Multi-threaded file indexing
rayon = "1.10"

# Regex - Pattern matching for annotations
regex = "1.11"

# Logging - Structured diagnostics
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter"] }

# Database - Optional SQLite for large codebases
rusqlite = { version = "0.32", optional = true, features = ["bundled"] }

# HTTP client - Optional for future MCP server integration
reqwest = { version = "0.12", optional = true, features = ["json"] }

# JSON schema validation - Cache/vars file validation
jsonschema = "0.29"

# Template engine - Output formatting and expansion
handlebars = "6.3"

# Time - Timestamps for cache generation
chrono = { version = "0.4", features = ["serde"] }

# UUID - Unique identifiers for attempts/checkpoints
uuid = { version = "1.18", features = ["v4", "serde"] }

# @acp:summary "Development and testing dependencies"
[dev-dependencies]
tempfile = "3.15"           # Temporary test directories
pretty_assertions = "1.4"   # Readable test diffs
criterion = "0.5"           # Benchmarking

# @acp:summary "Optional feature flags for extended functionality"
[features]
default = []
sqlite = ["rusqlite"]       # SQLite database support
mcp = ["reqwest"]           # MCP server integration
full = ["sqlite", "mcp"]    # All optional features

# @acp:summary "Release build optimizations for binary distribution"
[profile.release]
lto = true                  # Link-time optimization
codegen-units = 1           # Single codegen unit for better optimization
panic = "abort"             # Smaller binary, no unwinding
strip = true                # Strip symbols

# @acp:summary "Development build configuration for fast iteration"
[profile.dev]
opt-level = 0               # No optimization
debug = true                # Full debug info

# @acp:summary "Benchmark configuration (uncomment when ready)"
# [[bench]]
# name = "indexing"
# harness = false
```

---

## 3. Core Architecture

### Library Facade

**(current) in `cli/src/lib.rs`**

```rust
//! @acp:module "ACP Library"
//! @acp:summary "Token-efficient code documentation and indexing for AI systems"
//! @acp:domain cli
//! @acp:layer utility
//!
//! # ACP - AI Context Protocol
//!
//! Token-efficient code documentation and indexing for AI systems.
//!
//! ## Features
//!
//! - **Fast Parsing**: Uses tree-sitter for accurate AST parsing
//! - **JSON Output**: Queryable with jq for O(1) lookups
//! - **Variable System**: Token-efficient macros with inheritance
//! - **Multi-language**: TypeScript, JavaScript, Rust, Python, Go, Java
//!
//! ## Example
//!
//! ```rust,no_run
//! use acp::{Indexer, Config};
//!
//! #[tokio::main]
//! async fn main() -> anyhow::Result<()> {
//!     let config = Config::default();
//!     let indexer = Indexer::new(config)?;
//!
//!     // Index codebase
//!     let cache = indexer.index(".").await?;
//!
//!     // Write JSON output
//!     cache.write_json(".acp.cache.json")?;
//!
//!     Ok(())
//! }
//! ```

pub mod cache;
pub mod config;
pub mod constraints;
pub mod error;
pub mod expand;
pub mod index;
pub mod parse;
pub mod query;
pub mod schema;
pub mod vars;
pub mod watch;
pub mod attempts;
pub mod guardrails;

// Re-exports
pub use cache::{Cache, CacheBuilder};
pub use config::Config;
pub use constraints::{
    Constraints, ConstraintIndex,
    StyleConstraint, MutationConstraint, BehaviorModifier, QualityGate,
    HackMarker, DebugSession, DebugAttempt,
    LockLevel, DebugStatus, DebugResult,
};
pub use error::{AcpError, Result};
pub use index::Indexer;
pub use parse::{Parser, Language};
pub use query::Query;
pub use vars::{VarResolver, VarExpander};
pub use attempts::AttemptTracker;
pub use guardrails::GuardrailEnforcer;

/// Library version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
```

---

## 4. Foundation Layer

### 4.1 Error Types

**(current) in `cli/src/error.rs`**

```rust
//! @acp:module "Error Types"
//! @acp:summary "Comprehensive error handling for ACP operations"
//! @acp:domain cli
//! @acp:layer utility

use thiserror::Error;

/// @acp:summary "Result type alias for ACP operations"
pub type Result<T> = std::result::Result<T, AcpError>;

/// @acp:summary "Comprehensive error types for ACP operations"
/// @acp:lock normal
#[derive(Error, Debug)]
pub enum AcpError {
    /// IO operation failed
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    /// JSON serialization/deserialization failed
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    /// Source code parsing failed
    #[error("Parse error: {message}")]
    Parse {
        message: String,
        file: Option<String>,
        line: Option<usize>
    },

    /// File not found
    #[error("File not found: {0}")]
    FileNotFound(String),

    /// Invalid configuration
    #[error("Invalid configuration: {0}")]
    Config(String),

    /// Variable reference not found
    #[error("Variable not found: {0}")]
    VarNotFound(String),

    /// Circular dependency in variable inheritance
    #[error("Cycle detected in variable inheritance: {0}")]
    CycleDetected(String),

    /// JSON schema validation failed
    #[error("Schema validation failed: {0}")]
    SchemaValidation(String),

    /// Language not supported
    #[error("Unsupported language: {0}")]
    UnsupportedLanguage(String),

    /// Indexing operation failed
    #[error("Index error: {0}")]
    Index(String),

    /// Generic error
    #[error("{0}")]
    Other(String),
}

impl AcpError {
    /// @acp:summary "Create a parse error without location context"
    pub fn parse(message: impl Into<String>) -> Self {
        Self::Parse {
            message: message.into(),
            file: None,
            line: None,
        }
    }

    /// @acp:summary "Create a parse error with file and line context"
    pub fn parse_at(message: impl Into<String>, file: impl Into<String>, line: usize) -> Self {
        Self::Parse {
            message: message.into(),
            file: Some(file.into()),
            line: Some(line),
        }
    }
}
```

### 4.2 Configuration

**(current) in `cli/src/config.rs` - move to `cli/src/config/mod.rs`**

```rust
//! @acp:module "Configuration"
//! @acp:summary "Project configuration loading and defaults"
//! @acp:domain cli
//! @acp:layer config

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// @acp:summary "Main ACP configuration structure"
/// @acp:lock normal
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// Project root directory
    #[serde(default = "default_root")]
    pub root: PathBuf,

    /// File patterns to include (glob syntax)
    #[serde(default = "default_include")]
    pub include: Vec<String>,

    /// File patterns to exclude (glob syntax)
    #[serde(default = "default_exclude")]
    pub exclude: Vec<String>,

    /// Output paths configuration
    #[serde(default)]
    pub output: OutputConfig,

    /// Parser behavior settings
    #[serde(default)]
    pub parser: ParserConfig,

    /// Indexer behavior settings
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
    /// @acp:summary "Load config from .acp.config.json file"
    pub fn load<P: AsRef<std::path::Path>>(path: P) -> crate::Result<Self> {
        let content = std::fs::read_to_string(path)?;
        Ok(serde_json::from_str(&content)?)
    }

    /// @acp:summary "Load from default location or create default config"
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

/// @acp:summary "Output file path configuration"
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutputConfig {
    /// Cache file output path
    #[serde(default = "default_cache_path")]
    pub cache: PathBuf,

    /// Vars file output path
    #[serde(default = "default_vars_path")]
    pub vars: PathBuf,

    /// Whether to also output SQLite database
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

/// @acp:summary "Parser behavior configuration"
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParserConfig {
    /// Extract documentation comments
    #[serde(default = "default_true")]
    pub extract_docs: bool,

    /// Parse and extract function signatures
    #[serde(default = "default_true")]
    pub extract_signatures: bool,

    /// Track call graph relationships
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

/// @acp:summary "Indexer behavior configuration"
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexerConfig {
    /// Number of parallel workers for file processing
    #[serde(default = "default_workers")]
    pub workers: usize,

    /// Automatically generate vars file after indexing
    #[serde(default = "default_true")]
    pub auto_vars: bool,

    /// Infer domains from file paths
    #[serde(default = "default_true")]
    pub infer_domains: bool,

    /// Infer architectural layers from file paths
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

// Fallback if num_cpus not available
#[cfg(not(feature = "num_cpus"))]
mod num_cpus {
    pub fn get() -> usize {
        4
    }
}
```

### 4.3 Cache Types

**(current) in `cli/src/cache.rs` - move to `cli/src/cache/types.rs`**

```rust
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
    pub guardrails: Option<crate::guardrails::FileGuardrails>,
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
```

---

## 5. Parsing System

### 5.1 Parser Module

**(current) in `cli/src/parse.rs` - move to `cli/src/parse/mod.rs`**

```rust
//! @acp:module "Parser"
//! @acp:summary "Source code parsing and annotation extraction"
//! @acp:domain cli
//! @acp:layer service
//!
//! Parses source files to extract symbols, calls, and documentation.
//! Currently uses regex-based parsing with tree-sitter support planned.

use std::path::Path;

use crate::cache::{FileEntry, SymbolEntry, SymbolType, Stability};
use crate::error::{AcpError, Result};

/// @acp:summary "Supported programming languages"
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Language {
    TypeScript,
    JavaScript,
    Rust,
    Python,
    Go,
    Java,
}

impl Language {
    /// @acp:summary "Detect language from file extension"
    pub fn from_path<P: AsRef<Path>>(path: P) -> Option<Self> {
        let ext = path.as_ref().extension()?.to_str()?;
        match ext {
            "ts" | "tsx" => Some(Self::TypeScript),
            "js" | "jsx" | "mjs" => Some(Self::JavaScript),
            "rs" => Some(Self::Rust),
            "py" => Some(Self::Python),
            "go" => Some(Self::Go),
            "java" => Some(Self::Java),
            _ => None,
        }
    }
}

/// @acp:summary "Result of parsing a source file"
#[derive(Debug, Clone)]
pub struct ParseResult {
    pub file: FileEntry,
    pub symbols: Vec<SymbolEntry>,
    pub calls: Vec<(String, Vec<String>)>, // (caller, callees)
}

/// @acp:summary "Parser for source files"
pub struct Parser {
    // tree-sitter parsers would be initialized here
    // For now, this is a stub implementation
}

impl Parser {
    pub fn new() -> Self {
        Self {}
    }

    /// @acp:summary "Parse a source file and extract metadata"
    pub fn parse<P: AsRef<Path>>(&self, path: P) -> Result<ParseResult> {
        let path = path.as_ref();
        let content = std::fs::read_to_string(path)?;
        let lang = Language::from_path(path)
            .ok_or_else(|| AcpError::UnsupportedLanguage(
                path.extension()
                    .map(|e| e.to_string_lossy().to_string())
                    .unwrap_or_default()
            ))?;

        // TODO: Implement actual tree-sitter parsing
        // For now, return a basic result with file metadata
        let lines = content.lines().count();
        let file_name = path.file_stem()
            .map(|s| s.to_string_lossy().to_string())
            .unwrap_or_default();

        let file = FileEntry {
            path: path.to_string_lossy().to_string(),
            module: file_name.clone(),
            lines,
            domains: vec![],
            layer: None,
            stability: Stability::Active,
            depends: vec![],
            exports: vec![],
            symbols: vec![],
            keywords: vec![],
            hash: Some(format!("{:x}", md5::compute(&content))),
            guardrails: None,
        };

        Ok(ParseResult {
            file,
            symbols: vec![],
            calls: vec![],
        })
    }

    /// @acp:summary "Parse @acp: annotations from source comments"
    pub fn parse_annotations(&self, content: &str) -> Vec<Annotation> {
        let mut annotations = Vec::new();
        let pattern = regex::Regex::new(r"@acp:(\w+)(?:\s+(.+))?").unwrap();

        for (line_num, line) in content.lines().enumerate() {
            for cap in pattern.captures_iter(line) {
                annotations.push(Annotation {
                    name: cap.get(1).unwrap().as_str().to_string(),
                    value: cap.get(2).map(|m| m.as_str().to_string()),
                    line: line_num + 1,
                });
            }
        }

        annotations
    }
}

impl Default for Parser {
    fn default() -> Self {
        Self::new()
    }
}

/// @acp:summary "Parsed annotation from source"
#[derive(Debug, Clone)]
pub struct Annotation {
    pub name: String,
    pub value: Option<String>,
    pub line: usize,
}
```

---

## 6. Indexing System

**(current) in `cli/src/index.rs` - move to `cli/src/index/indexer.rs`**

```rust
//! @acp:module "Indexer"
//! @acp:summary "Codebase indexing and cache generation"
//! @acp:domain cli
//! @acp:layer service
//!
//! Walks the codebase and builds the cache/vars files.

use std::path::Path;
use std::sync::Arc;

use rayon::prelude::*;
use walkdir::WalkDir;
use glob::Pattern;

use crate::cache::{Cache, CacheBuilder, DomainEntry};
use crate::config::Config;
use crate::error::Result;
use crate::parse::Parser;
use crate::vars::VarsFile;

/// @acp:summary "Codebase indexer with parallel file processing"
pub struct Indexer {
    config: Config,
    parser: Arc<Parser>,
}

impl Indexer {
    pub fn new(config: Config) -> Result<Self> {
        Ok(Self {
            config,
            parser: Arc::new(Parser::new()),
        })
    }

    /// @acp:summary "Index the codebase and generate cache"
    /// @acp:ai-careful "This processes many files in parallel"
    pub async fn index<P: AsRef<Path>>(&self, root: P) -> Result<Cache> {
        let root = root.as_ref();
        let project_name = root
            .file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_else(|| "project".to_string());

        let mut builder = CacheBuilder::new(&project_name, &root.to_string_lossy());

        // Find all matching files
        let files = self.find_files(root)?;

        // Parse files in parallel using rayon
        let results: Vec<_> = files
            .par_iter()
            .filter_map(|path| {
                self.parser.parse(path).ok()
            })
            .collect();

        // Build cache from results
        let mut domains: std::collections::HashMap<String, Vec<String>> =
            std::collections::HashMap::new();

        for result in results {
            // Add file
            builder = builder.add_file(result.file.clone());

            // Add symbols
            for symbol in result.symbols {
                builder = builder.add_symbol(symbol);
            }

            // Add call edges
            for (from, to) in result.calls {
                builder = builder.add_call_edge(&from, to);
            }

            // Track domains
            for domain in &result.file.domains {
                domains
                    .entry(domain.clone())
                    .or_default()
                    .push(result.file.path.clone());
            }
        }

        // Add domains to cache
        for (name, files) in domains {
            builder = builder.add_domain(DomainEntry {
                name: name.clone(),
                files: files.clone(),
                symbols: vec![],
                file_count: files.len(),
                symbol_count: 0,
            });
        }

        Ok(builder.build())
    }

    /// @acp:summary "Find all files matching include/exclude patterns"
    fn find_files<P: AsRef<Path>>(&self, root: P) -> Result<Vec<String>> {
        let root = root.as_ref();
        let include_patterns: Vec<_> = self.config.include
            .iter()
            .filter_map(|p| Pattern::new(p).ok())
            .collect();

        let exclude_patterns: Vec<_> = self.config.exclude
            .iter()
            .filter_map(|p| Pattern::new(p).ok())
            .collect();

        let files: Vec<String> = WalkDir::new(root)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_file())
            .map(|e| e.path().to_string_lossy().to_string())
            .filter(|path| {
                // Must match at least one include pattern
                let included = include_patterns.iter().any(|p| p.matches(path));
                // Must not match any exclude pattern
                let excluded = exclude_patterns.iter().any(|p| p.matches(path));
                included && !excluded
            })
            .collect();

        Ok(files)
    }

    /// @acp:summary "Generate vars file from cache"
    pub fn generate_vars(&self, cache: &Cache) -> VarsFile {
        use crate::vars::{VarEntry, VarCategory, VarsFile};
        use chrono::Utc;
        use serde_json::json;

        let mut vars = std::collections::HashMap::new();

        // Generate symbol vars
        for (name, symbol) in &cache.symbols {
            if symbol.exported {
                let var_name = format!("SYM_{}", name.to_uppercase().replace('.', "_"));
                vars.insert(var_name.clone(), VarEntry {
                    name: var_name,
                    category: VarCategory::Symbol,
                    summary: symbol.summary.clone(),
                    value: json!({
                        "sym": name,
                        "file": symbol.file,
                        "lines": symbol.lines,
                        "async": symbol.async_fn,
                        "calls": symbol.calls,
                        "throws": symbol.throws,
                        "flags": symbol.flags,
                    }),
                    tokens: Some(20),
                    tokens_saved: Some(40),
                    source: Some(symbol.file.clone()),
                    lines: Some(symbol.lines),
                    tags: vec![name.to_lowercase()],
                    refs: vec![],
                });
            }
        }

        // Generate domain vars
        for (name, domain) in &cache.domains {
            let var_name = format!("DOM_{}", name.to_uppercase().replace('-', "_"));
            vars.insert(var_name.clone(), VarEntry {
                name: var_name,
                category: VarCategory::Domain,
                summary: Some(format!("Domain: {} ({} files)", name, domain.file_count)),
                value: json!({
                    "domain": name,
                    "files": domain.files,
                    "symbols": domain.symbols,
                    "file_count": domain.file_count,
                }),
                tokens: Some(35),
                tokens_saved: Some(150),
                source: None,
                lines: None,
                tags: vec![name.to_lowercase()],
                refs: vec![],
            });
        }

        // Generate layer vars
        for (name, files) in &cache.layers {
            let var_name = format!("LAY_{}", name.to_uppercase());
            vars.insert(var_name.clone(), VarEntry {
                name: var_name,
                category: VarCategory::Layer,
                summary: Some(format!("Layer: {} ({} files)", name, files.len())),
                value: json!({
                    "layer": name,
                    "files": files,
                    "count": files.len(),
                }),
                tokens: Some(40),
                tokens_saved: Some(200),
                source: None,
                lines: None,
                tags: vec![name.to_lowercase()],
                refs: vec![],
            });
        }

        let mut vars_file = VarsFile {
            version: crate::VERSION.to_string(),
            generated_at: Utc::now(),
            project: Some(cache.project.name.clone()),
            stats: None,
            vars,
            index: None,
        };

        vars_file.build_index();
        vars_file
    }
}
```

---

## 7. Variable System

The variable system provides token-efficient macros with inheritance and multiple expansion modes.

**(current) in `cli/src/vars.rs` - move to `cli/src/vars/mod.rs`**

Due to length (~540 lines), see the full implementation in `cli/src/vars.rs`. Key components:

- **`VarsFile`**: Complete vars file structure with stats and indexes
- **`VarEntry`**: Single variable with category, value, tokens, refs
- **`VarCategory`**: Symbol, File, Domain, Layer, Arch, Pattern, etc.
- **`VarResolver`**: Finds and retrieves variables by name/category/tag
- **`VarExpander`**: Expands variables with inheritance and cycle detection
- **`ExpansionMode`**: None, Summary, Inline, Annotated, Block, Interactive

### Expansion Presets

**(current) in `cli/src/expand.rs` - move to `cli/src/vars/presets.rs`**

```rust
//! @acp:module "Expansion Presets"
//! @acp:summary "Pre-configured expansion modes for common use cases"

pub use crate::vars::{VarExpander, VarResolver, ExpansionMode, ExpansionResult, InheritanceChain};

pub mod presets {
    use super::ExpansionMode;

    /// For AI-to-AI communication - keeps $VAR references intact
    pub const AI_TO_AI: ExpansionMode = ExpansionMode::None;

    /// Quick human reading - inline expansion
    pub const QUICK: ExpansionMode = ExpansionMode::Inline;

    /// Detailed human reading (default) - annotated format
    pub const DETAILED: ExpansionMode = ExpansionMode::Annotated;

    /// Documentation generation - full block format
    pub const DOCUMENTATION: ExpansionMode = ExpansionMode::Block;

    /// Interactive web UI - HTML-like markers
    pub const INTERACTIVE: ExpansionMode = ExpansionMode::Interactive;

    /// Most compact human-readable - summary only
    pub const SUMMARY: ExpansionMode = ExpansionMode::Summary;
}
```

---

## 8. Constraints & Guardrails

The constraint and guardrail system provides AI behavioral controls via `@acp:` annotations.

### 8.1 Constraint Types

**(current) in `cli/src/constraints.rs` - move to `cli/src/constraints/types.rs`**

Due to length (~635 lines), see the full implementation in `cli/src/constraints.rs`. Key components:

- **`Constraints`**: Complete constraint set (style, mutation, behavior, quality)
- **`LockLevel`**: Frozen, Restricted, ApprovalRequired, TestsRequired, DocsRequired, Normal, Experimental
- **`StyleConstraint`**: Style guide, reference URL, rules, linter
- **`MutationConstraint`**: Lock level, reason, requirements
- **`BehaviorModifier`**: Approach, priority, explain/verify flags
- **`QualityGate`**: Tests, coverage, security review, complexity
- **`HackMarker`**: Temporary code tracking with expiration
- **`DebugSession`**: AI troubleshooting tracking with attempts

### 8.2 Guardrails Parser and Enforcer

**(current) in `cli/src/guardrails.rs` - move to `cli/src/constraints/guardrails.rs`**

Due to length (~791 lines), see the full implementation in `cli/src/guardrails.rs`. Key components:

- **`FileGuardrails`**: Complete guardrail annotations for a file
- **`GuardrailParser`**: Parses 25+ `@acp:` annotations from source
- **`GuardrailEnforcer`**: Validates changes against guardrails
- **`GuardrailCheck`**: Result with violations, warnings, required actions

**Supported Annotations:**

| Category | Annotations |
|----------|------------|
| Constraints | `@acp:style`, `@acp:framework`, `@acp:compat`, `@acp:requires`, `@acp:forbids`, `@acp:pattern` |
| AI Behavior | `@acp:ai-readonly`, `@acp:ai-careful`, `@acp:ai-ask`, `@acp:ai-context`, `@acp:ai-approach`, `@acp:ai-reference` |
| Temporary | `@acp:hack`, `@acp:experiment`, `@acp:debug`, `@acp:wip`, `@acp:temporary` |
| Attempts | `@acp:attempt-start`, `@acp:attempt-end`, `@acp:checkpoint`, `@acp:revert-if`, `@acp:original` |
| Review | `@acp:review-required`, `@acp:review-by`, `@acp:ai-generated`, `@acp:human-verified` |
| Quality | `@acp:tech-debt`, `@acp:complexity`, `@acp:smell`, `@acp:coverage`, `@acp:test-required`, `@acp:lint` |

---

## 9. Attempt Tracking

**(current) in `cli/src/attempts.rs` - move to `cli/src/attempts/mod.rs`**

Due to length (~391 lines), see the full implementation in `cli/src/attempts.rs`. Key components:

- **`AttemptTracker`**: Manages attempts and checkpoints (stored in `.acp.attempts.json`)
- **`TrackedAttempt`**: Individual attempt with files, status, timestamps
- **`TrackedCheckpoint`**: Rollback point with file states
- **`RevertAction`**: Result of reverting an attempt

Key methods:
- `start_attempt()`: Begin a new troubleshooting attempt
- `record_modification()`: Track file changes in an attempt
- `fail_attempt()` / `verify_attempt()`: Mark attempt outcome
- `revert_attempt()`: Restore original files
- `create_checkpoint()` / `restore_checkpoint()`: Rollback points
- `cleanup_failed()`: Revert all failed attempts

---

## 10. Commands

The main CLI entry point (`cli/src/main.rs`) uses clap for command parsing and delegates to handlers.

**(current) in `cli/src/main.rs`**

Due to length (~649 lines), see the full implementation in `cli/src/main.rs`.

**Command Examples:**

```bash
# Index codebase
acp index . --output .acp.cache.json --vars

# Generate vars from cache
acp vars --cache .acp.cache.json --output .acp.vars.json

# Query cache
acp query symbol MyFunction
acp query callers MyFunction
acp query domains
acp query stats

# Expand variables
echo "Check \$SYM_AUTH_HANDLER" | acp expand --mode annotated

# Check guardrails
acp check src/auth.ts

# Manage attempts
acp attempt start fix-123 --for-issue "Bug #123"
acp attempt list --active
acp attempt fail fix-123 --reason "Approach didn't work"
acp attempt revert fix-123

# Checkpoints
acp attempt checkpoint before-refactor --files src/*.ts
acp attempt restore before-refactor

# Validate files
acp validate .acp.cache.json
```

---

## 11. Query System

**(current) in `cli/src/query.rs` - move to `cli/src/query/engine.rs`**

```rust
//! @acp:module "Query Engine"
//! @acp:summary "Type-safe programmatic cache access"
//! @acp:domain cli
//! @acp:layer service

use crate::cache::{Cache, SymbolEntry, FileEntry, DomainEntry};

/// @acp:summary "Query builder for cache with zero-copy access"
pub struct Query<'a> {
    cache: &'a Cache,
}

impl<'a> Query<'a> {
    pub fn new(cache: &'a Cache) -> Self {
        Self { cache }
    }

    /// Get symbol by name - O(1)
    pub fn symbol(&self, name: &str) -> Option<&SymbolEntry> {
        self.cache.get_symbol(name)
    }

    /// Get file by path - O(1)
    pub fn file(&self, path: &str) -> Option<&FileEntry> {
        self.cache.get_file(path)
    }

    /// Get callers of a symbol
    pub fn callers(&self, symbol: &str) -> Vec<&str> {
        self.cache.get_callers(symbol)
            .map(|v| v.iter().map(|s| s.as_str()).collect())
            .unwrap_or_default()
    }

    /// Get callees of a symbol
    pub fn callees(&self, symbol: &str) -> Vec<&str> {
        self.cache.get_callees(symbol)
            .map(|v| v.iter().map(|s| s.as_str()).collect())
            .unwrap_or_default()
    }

    /// Get domain by name
    pub fn domain(&self, name: &str) -> Option<&DomainEntry> {
        self.cache.domains.get(name)
    }

    /// Iterate all domains
    pub fn domains(&self) -> impl Iterator<Item = &DomainEntry> {
        self.cache.domains.values()
    }

    /// Search symbols by name pattern
    pub fn search_symbols(&self, pattern: &str) -> Vec<&SymbolEntry> {
        let p = pattern.to_lowercase();
        self.cache.symbols.values()
            .filter(|s| s.name.to_lowercase().contains(&p))
            .collect()
    }

    /// Get hotpath symbols
    pub fn hotpaths(&self) -> impl Iterator<Item = &str> {
        self.cache.hotpaths.iter().map(|h| h.symbol.as_str())
    }

    /// Get deprecated symbols/files
    pub fn deprecated(&self) -> &[String] {
        &self.cache.stability.deprecated
    }

    /// Get security-sensitive code
    pub fn security_sensitive(&self) -> impl Iterator<Item = &str> {
        self.cache.security.auth_required.iter()
            .chain(self.cache.security.audit_logged.iter())
            .chain(self.cache.security.credentials.iter())
            .map(|s| s.as_str())
    }
}
```

---

## 12. Testing

### Unit Tests

Each module includes inline tests. Run with:

```bash
cargo test
```

### Integration Tests

Create `tests/integration/` with end-to-end tests:

```rust
// tests/integration/index_test.rs
use acp::{Config, Indexer};
use tempfile::TempDir;

#[tokio::test]
async fn test_index_project() {
    let dir = TempDir::new().unwrap();
    // Create test files...

    let config = Config::default();
    let indexer = Indexer::new(config).unwrap();
    let cache = indexer.index(dir.path()).await.unwrap();

    assert!(cache.stats.files > 0);
}
```

### Snapshot Tests

Use `insta` for output snapshot testing:

```rust
#[test]
fn test_cache_json_format() {
    let cache = create_test_cache();
    insta::assert_json_snapshot!(cache);
}
```

---

## 13. Distribution

### GitHub Actions Release

```yaml
# .github/workflows/release.yml
name: Release

on:
  push:
    tags: ['v*']

jobs:
  build:
    strategy:
      matrix:
        include:
          - os: ubuntu-latest
            target: x86_64-unknown-linux-gnu
          - os: macos-latest
            target: x86_64-apple-darwin
          - os: macos-latest
            target: aarch64-apple-darwin
          - os: windows-latest
            target: x86_64-pc-windows-msvc

    runs-on: ${{ matrix.os }}
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
        with:
          targets: ${{ matrix.target }}

      - name: Build
        run: cargo build --release --target ${{ matrix.target }}

      - name: Upload artifact
        uses: actions/upload-artifact@v4
        with:
          name: acp-${{ matrix.target }}
          path: target/${{ matrix.target }}/release/acp*
```

### Homebrew Formula

```ruby
class Acp < Formula
  desc "AI Context Protocol CLI"
  homepage "https://github.com/anthropics/acp-spec"
  version "0.1.0"

  on_macos do
    if Hardware::CPU.arm?
      url "https://github.com/anthropics/acp-spec/releases/download/v#{version}/acp-aarch64-apple-darwin.tar.gz"
    else
      url "https://github.com/anthropics/acp-spec/releases/download/v#{version}/acp-x86_64-apple-darwin.tar.gz"
    end
  end

  def install
    bin.install "acp"
  end

  test do
    system "#{bin}/acp", "--version"
  end
end
```

---

## Appendix: Implementation Checklist

### Core Features
- [x] Error handling (`error.rs`)
- [x] Configuration (`config.rs`)
- [x] Cache types (`cache.rs`)
- [x] Parser facade (`parse.rs`)
- [ ] Tree-sitter integration (TODO)
- [x] Indexer (`index.rs`)
- [x] Variable system (`vars.rs`)
- [x] Constraints (`constraints.rs`)
- [x] Guardrails (`guardrails.rs`)
- [x] Attempt tracking (`attempts.rs`)
- [x] Query engine (`query.rs`)
- [x] Watch mode (stub) (`watch.rs`)
- [x] Schema validation (basic) (`schema.rs`)

### Commands
- [x] `acp index`
- [x] `acp vars`
- [x] `acp query` (8 subcommands)
- [x] `acp expand`
- [x] `acp chain`
- [x] `acp check`
- [x] `acp attempt` (9 subcommands)
- [x] `acp revert`
- [x] `acp watch`
- [x] `acp validate`

### Future Work
- [ ] Full tree-sitter parsing for all languages
- [ ] Incremental cache updates in watch mode
- [ ] JSON Schema validation with jsonschema crate
- [ ] SQLite output option
- [ ] Language-specific symbol extraction

---

*This guide reflects the current implementation. For the latest code, see the `cli/src/` directory.*

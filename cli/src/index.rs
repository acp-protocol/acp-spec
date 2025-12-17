//! Indexer module
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

/// Codebase indexer
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

    /// Index the codebase and generate cache
    pub async fn index<P: AsRef<Path>>(&self, root: P) -> Result<Cache> {
        let root = root.as_ref();
        let project_name = root
            .file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_else(|| "project".to_string());

        let mut builder = CacheBuilder::new(&project_name, &root.to_string_lossy());

        // Find all matching files
        let files = self.find_files(root)?;

        // Parse files in parallel
        let results: Vec<_> = files
            .par_iter()
            .filter_map(|path| {
                self.parser.parse(path).ok()
            })
            .collect();

        // Build cache from results
        let mut domains: std::collections::HashMap<String, Vec<String>> = std::collections::HashMap::new();

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

        // Add domains
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

    /// Find all files matching include/exclude patterns
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

    /// Generate vars file from cache
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

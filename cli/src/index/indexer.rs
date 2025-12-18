//! @acp:module "Indexer"
//! @acp:summary "Codebase indexing and cache generation (schema-compliant)"
//! @acp:domain cli
//! @acp:layer service
//!
//! Walks the codebase and builds the cache/vars files.

use std::path::Path;
use std::sync::Arc;
use std::fs;

use chrono::{DateTime, Utc};
use rayon::prelude::*;
use walkdir::WalkDir;
use glob::Pattern;

use crate::cache::{Cache, CacheBuilder, DomainEntry, Language};
use crate::config::Config;
use crate::error::Result;
use crate::parse::Parser;
use crate::vars::{VarsFile, VarEntry};

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

        // Add source_files with modification times
        for file_path in &files {
            if let Ok(metadata) = fs::metadata(file_path) {
                if let Ok(modified) = metadata.modified() {
                    let modified_dt: DateTime<Utc> = modified.into();
                    let relative_path = Path::new(file_path)
                        .strip_prefix(root)
                        .map(|p| p.to_string_lossy().to_string())
                        .unwrap_or_else(|_| file_path.clone());
                    builder = builder.add_source_file(relative_path, modified_dt);
                }
            }
        }

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
                description: None,
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
            .filter_map(|e| {
                // Get path relative to root for pattern matching
                let full_path = e.path().to_string_lossy().to_string();
                let relative_path = e.path()
                    .strip_prefix(root)
                    .map(|p| p.to_string_lossy().to_string())
                    .unwrap_or_else(|_| full_path.clone());

                // Must match at least one include pattern
                let match_opts = glob::MatchOptions {
                    case_sensitive: true,
                    require_literal_separator: false,
                    require_literal_leading_dot: false,
                };
                let included = include_patterns.is_empty() ||
                    include_patterns.iter().any(|p| p.matches_with(&relative_path, match_opts));
                // Must not match any exclude pattern
                let excluded = exclude_patterns.iter().any(|p| p.matches_with(&relative_path, match_opts));

                if included && !excluded {
                    Some(full_path)
                } else {
                    None
                }
            })
            .collect();

        Ok(files)
    }

    /// @acp:summary "Generate vars file from cache (schema-compliant)"
    pub fn generate_vars(&self, cache: &Cache) -> VarsFile {
        let mut vars_file = VarsFile::new();

        // Generate symbol vars
        for (name, symbol) in &cache.symbols {
            if symbol.exported {
                let var_name = format!("SYM_{}", name.to_uppercase().replace('.', "_"));
                vars_file.add_variable(
                    var_name,
                    VarEntry::symbol(
                        symbol.qualified_name.clone(),
                        symbol.summary.clone(),
                    ),
                );
            }
        }

        // Generate domain vars
        for (name, domain) in &cache.domains {
            let var_name = format!("DOM_{}", name.to_uppercase().replace('-', "_"));
            vars_file.add_variable(
                var_name,
                VarEntry::domain(
                    name.clone(),
                    Some(format!("Domain: {} ({} files)", name, domain.files.len())),
                ),
            );
        }

        // Generate file vars for important files
        for (path, file) in &cache.files {
            // Only generate vars for files with modules or summaries
            if file.module.is_some() || file.summary.is_some() {
                let var_name = format!("FILE_{}",
                    path.replace('/', "_")
                        .replace('.', "_")
                        .to_uppercase());
                vars_file.add_variable(
                    var_name,
                    VarEntry::file(
                        path.clone(),
                        file.summary.clone().or_else(|| file.module.clone()),
                    ),
                );
            }
        }

        vars_file
    }
}

/// Detect language from file extension
pub fn detect_language(path: &str) -> Option<Language> {
    let path = Path::new(path);
    let ext = path.extension()?.to_str()?;

    match ext.to_lowercase().as_str() {
        "ts" | "tsx" => Some(Language::Typescript),
        "js" | "jsx" | "mjs" | "cjs" => Some(Language::Javascript),
        "py" | "pyw" => Some(Language::Python),
        "rs" => Some(Language::Rust),
        "go" => Some(Language::Go),
        "java" => Some(Language::Java),
        "cs" => Some(Language::CSharp),
        "cpp" | "cxx" | "cc" | "hpp" | "hxx" => Some(Language::Cpp),
        "c" | "h" => Some(Language::C),
        "rb" => Some(Language::Ruby),
        "php" => Some(Language::Php),
        "swift" => Some(Language::Swift),
        "kt" | "kts" => Some(Language::Kotlin),
        _ => None,
    }
}

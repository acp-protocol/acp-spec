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
use crate::constraints::{ConstraintIndex, Constraints, MutationConstraint, LockLevel, HackMarker, HackType};
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
        let mut constraint_index = ConstraintIndex::default();

        for result in &results {
            // Add file
            builder = builder.add_file(result.file.clone());

            // Add symbols
            for symbol in &result.symbols {
                builder = builder.add_symbol(symbol.clone());
            }

            // Add call edges
            for (from, to) in &result.calls {
                builder = builder.add_call_edge(from, to.clone());
            }

            // Track domains
            for domain in &result.file.domains {
                domains
                    .entry(domain.clone())
                    .or_default()
                    .push(result.file.path.clone());
            }

            // Build constraints from parse result
            if result.lock_level.is_some() || !result.ai_hints.is_empty() {
                let lock_level = result.lock_level.as_ref().map(|l| {
                    match l.to_lowercase().as_str() {
                        "frozen" => LockLevel::Frozen,
                        "restricted" => LockLevel::Restricted,
                        "approval-required" => LockLevel::ApprovalRequired,
                        "tests-required" => LockLevel::TestsRequired,
                        "docs-required" => LockLevel::DocsRequired,
                        "experimental" => LockLevel::Experimental,
                        _ => LockLevel::Normal,
                    }
                }).unwrap_or(LockLevel::Normal);

                let constraints = Constraints {
                    mutation: Some(MutationConstraint {
                        level: lock_level.clone(),
                        reason: None,
                        contact: None,
                        requires_approval: matches!(lock_level, LockLevel::ApprovalRequired),
                        requires_tests: matches!(lock_level, LockLevel::TestsRequired),
                        requires_docs: matches!(lock_level, LockLevel::DocsRequired),
                        max_lines_changed: None,
                        allowed_operations: None,
                        forbidden_operations: None,
                    }),
                    ..Default::default()
                };
                constraint_index.by_file.insert(result.file.path.clone(), constraints);

                // Track by lock level
                let level_str = format!("{:?}", lock_level).to_lowercase();
                constraint_index.by_lock_level
                    .entry(level_str)
                    .or_default()
                    .push(result.file.path.clone());
            }

            // Build hack markers
            for hack in &result.hacks {
                let hack_marker = HackMarker {
                    id: format!("{}:{}", result.file.path, hack.line),
                    hack_type: HackType::Workaround,
                    file: result.file.path.clone(),
                    line: Some(hack.line),
                    created_at: Utc::now(),
                    author: None,
                    reason: hack.reason.clone().unwrap_or_else(|| "Temporary hack".to_string()),
                    ticket: hack.ticket.clone(),
                    expires: hack.expires.as_ref().and_then(|e| {
                        chrono::NaiveDate::parse_from_str(e, "%Y-%m-%d")
                            .ok()
                            .map(|d| d.and_hms_opt(0, 0, 0).unwrap())
                            .map(|dt| DateTime::<Utc>::from_naive_utc_and_offset(dt, Utc))
                    }),
                    original_code: None,
                    revert_instructions: None,
                };
                constraint_index.hacks.push(hack_marker);
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

        // Add constraints if any were found
        if !constraint_index.by_file.is_empty() || !constraint_index.hacks.is_empty() {
            builder = builder.set_constraints(constraint_index);
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

        // Build a map of symbol names to var names for ref resolution
        let mut symbol_to_var: std::collections::HashMap<String, String> = std::collections::HashMap::new();
        for (name, symbol) in &cache.symbols {
            if symbol.exported {
                let var_name = format!("SYM_{}", name.to_uppercase().replace('.', "_"));
                symbol_to_var.insert(name.clone(), var_name);
            }
        }

        // Generate symbol vars with refs from call graph
        for (name, symbol) in &cache.symbols {
            if symbol.exported {
                let var_name = format!("SYM_{}", name.to_uppercase().replace('.', "_"));

                // Build refs from symbols this one calls
                let refs: Vec<String> = symbol.calls.iter()
                    .filter_map(|callee| symbol_to_var.get(callee).cloned())
                    .collect();

                let entry = VarEntry {
                    var_type: crate::vars::VarType::Symbol,
                    value: symbol.qualified_name.clone(),
                    description: symbol.summary.clone(),
                    refs,
                    source: Some(symbol.file.clone()),
                    lines: Some(symbol.lines),
                };

                vars_file.add_variable(var_name, entry);
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

        // Generate layer vars from unique layers
        let mut layers: std::collections::HashSet<String> = std::collections::HashSet::new();
        for file in cache.files.values() {
            if let Some(layer) = &file.layer {
                layers.insert(layer.clone());
            }
        }
        for layer in layers {
            let var_name = format!("LAYER_{}", layer.to_uppercase().replace('-', "_"));
            let file_count = cache.files.values()
                .filter(|f| f.layer.as_ref() == Some(&layer))
                .count();
            vars_file.add_variable(
                var_name,
                VarEntry::layer(
                    layer.clone(),
                    Some(format!("Layer: {} ({} files)", layer, file_count)),
                ),
            );
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

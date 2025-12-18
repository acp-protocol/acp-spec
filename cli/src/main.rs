//! ACP Command Line Interface

use std::path::PathBuf;

use clap::{Parser, Subcommand};
use console::style;

use acp::{Config, Indexer, Cache, Query};
use acp::vars::{VarsFile, VarResolver, VarExpander, ExpansionMode};

#[derive(Parser)]
#[command(name = "acp")]
#[command(about = "AI Context Protocol - Token-efficient code documentation")]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// Config file path
    #[arg(short, long, global = true, default_value = ".acp.config.json")]
    config: PathBuf,

    /// Verbose output
    #[arg(short, long, global = true)]
    verbose: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize a new ACP project
    Init {
        /// Force overwrite existing config
        #[arg(short, long)]
        force: bool,

        /// File patterns to include (can specify multiple)
        #[arg(long)]
        include: Vec<String>,

        /// File patterns to exclude (can specify multiple)
        #[arg(long)]
        exclude: Vec<String>,

        /// Cache file output path
        #[arg(long)]
        cache_path: Option<PathBuf>,

        /// Vars file output path
        #[arg(long)]
        vars_path: Option<PathBuf>,

        /// Number of parallel workers
        #[arg(long)]
        workers: Option<usize>,

        /// Skip interactive prompts (use defaults + CLI args)
        #[arg(short = 'y', long)]
        yes: bool,
    },

    /// Index the codebase and generate cache
    Index {
        /// Root directory to index
        #[arg(default_value = ".")]
        root: PathBuf,

        /// Output cache file path
        #[arg(short, long, default_value = ".acp/acp.cache.json")]
        output: PathBuf,

        /// Also generate vars file
        #[arg(long)]
        vars: bool,
    },

    /// Generate vars file from cache
    Vars {
        /// Cache file to read
        #[arg(short, long, default_value = ".acp/acp.cache.json")]
        cache: PathBuf,

        /// Output vars file path
        #[arg(short, long, default_value = ".acp/acp.vars.json")]
        output: PathBuf,
    },

    /// Query the cache
    Query {
        /// Query type
        #[command(subcommand)]
        query: QueryCommands,

        /// Cache file to query
        #[arg(short, long, default_value = ".acp/acp.cache.json")]
        cache: PathBuf,
    },

    /// Expand variable references in text
    Expand {
        /// Text to expand (reads from stdin if not provided)
        text: Option<String>,

        /// Expansion mode
        #[arg(short, long, default_value = "annotated")]
        mode: String,

        /// Vars file path
        #[arg(long, default_value = ".acp/acp.vars.json")]
        vars: PathBuf,

        /// Show inheritance chains
        #[arg(long)]
        chains: bool,
    },

    /// Show variable inheritance chain
    Chain {
        /// Variable name
        name: String,

        /// Vars file path
        #[arg(long, default_value = ".acp/acp.vars.json")]
        vars: PathBuf,

        /// Show as tree
        #[arg(long)]
        tree: bool,
    },

    /// Manage troubleshooting attempts
    Attempt {
        #[command(subcommand)]
        cmd: AttemptCommands,
    },

    /// Check guardrails for a file
    Check {
        /// File to check
        file: PathBuf,

        /// Cache file
        #[arg(short, long, default_value = ".acp/acp.cache.json")]
        cache: PathBuf,
    },

    /// Revert changes
    Revert {
        /// Attempt ID to revert
        #[arg(long)]
        attempt: Option<String>,

        /// Checkpoint name to restore
        #[arg(long)]
        checkpoint: Option<String>,
    },

    /// Watch for changes and update cache
    Watch {
        /// Root directory to watch
        #[arg(default_value = ".")]
        root: PathBuf,
    },

    /// Validate cache/vars files
    Validate {
        /// File to validate
        file: PathBuf,
    },
}

#[derive(Subcommand)]
enum AttemptCommands {
    /// Start a new attempt
    Start {
        /// Unique attempt ID
        id: String,

        /// Issue this is for
        #[arg(long, short = 'f')]
        for_issue: Option<String>,

        /// Description
        #[arg(long, short)]
        description: Option<String>,
    },

    /// List attempts
    List {
        /// Show only active attempts
        #[arg(long)]
        active: bool,

        /// Show only failed attempts
        #[arg(long)]
        failed: bool,

        /// Show history
        #[arg(long)]
        history: bool,
    },

    /// Mark attempt as failed
    Fail {
        /// Attempt ID
        id: String,

        /// Failure reason
        #[arg(long)]
        reason: Option<String>,
    },

    /// Mark attempt as verified
    Verify {
        /// Attempt ID
        id: String,
    },

    /// Revert an attempt
    Revert {
        /// Attempt ID
        id: String,
    },

    /// Clean up all failed attempts
    Cleanup,

    /// Create a checkpoint
    Checkpoint {
        /// Checkpoint name
        name: String,

        /// Files to checkpoint
        #[arg(long, short)]
        files: Vec<String>,

        /// Description
        #[arg(long)]
        description: Option<String>,
    },

    /// List checkpoints
    Checkpoints,

    /// Restore to checkpoint
    Restore {
        /// Checkpoint name
        name: String,
    },
}

#[derive(Subcommand)]
enum QueryCommands {
    /// Query a symbol
    Symbol {
        /// Symbol name
        name: String,
    },

    /// Query a file
    File {
        /// File path
        path: String,
    },

    /// Get callers of a symbol
    Callers {
        /// Symbol name
        symbol: String,
    },

    /// Get callees of a symbol
    Callees {
        /// Symbol name
        symbol: String,
    },

    /// List domains
    Domains,

    /// Query a domain
    Domain {
        /// Domain name
        name: String,
    },

    /// List hotpaths
    Hotpaths,

    /// Show stats
    Stats,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    // Load config
    let config = if cli.config.exists() {
        Config::load(&cli.config)?
    } else {
        Config::default()
    };

    match cli.command {
        Commands::Init { force, include, exclude, cache_path, vars_path, workers, yes } => {
            use dialoguer::{theme::ColorfulTheme, Confirm, Input, MultiSelect};
            use acp::scan::scan_project;

            let config_path = PathBuf::from(".acp.config.json");

            if config_path.exists() && !force {
                eprintln!("{} Config file already exists. Use --force to overwrite.", style("✗").red());
                std::process::exit(1);
            }

            let mut config = Config::default();

            // Interactive mode if no CLI options and not using --yes
            let interactive = !yes && include.is_empty() && exclude.is_empty()
                && cache_path.is_none() && vars_path.is_none() && workers.is_none();

            if interactive {
                println!("{} ACP Project Setup\n", style("→").cyan());

                // Scan project to detect languages
                println!("{} Scanning project...", style("→").dim());
                let scan = scan_project(".");

                if scan.languages.is_empty() {
                    println!("{} No supported languages detected\n", style("⚠").yellow());
                } else {
                    println!("{} Detected languages:", style("✓").green());
                    for lang in &scan.languages {
                        println!("    {} ({} files)", style(lang.name).cyan(), lang.file_count);
                    }
                    println!();

                    // Auto-populate include patterns from detected languages
                    let mut include_patterns: Vec<String> = vec![];
                    for lang in &scan.languages {
                        include_patterns.extend(lang.patterns.iter().map(|s| s.to_string()));
                    }
                    config.include = include_patterns;

                    // Ask to confirm or modify
                    let use_detected = Confirm::with_theme(&ColorfulTheme::default())
                        .with_prompt("Use detected languages?")
                        .default(true)
                        .interact()?;

                    if !use_detected {
                        // Fall back to manual selection
                        let all_languages = vec![
                            ("TypeScript/TSX", vec!["**/*.ts", "**/*.tsx"]),
                            ("JavaScript/JSX", vec!["**/*.js", "**/*.jsx", "**/*.mjs"]),
                            ("Rust", vec!["**/*.rs"]),
                            ("Python", vec!["**/*.py"]),
                            ("Go", vec!["**/*.go"]),
                            ("Java", vec!["**/*.java"]),
                        ];

                        let items: Vec<&str> = all_languages.iter().map(|(name, _)| *name).collect();
                        let selections = MultiSelect::with_theme(&ColorfulTheme::default())
                            .with_prompt("Select languages to index")
                            .items(&items)
                            .interact()?;

                        config.include = selections.iter()
                            .flat_map(|&idx| all_languages[idx].1.iter().map(|s| s.to_string()))
                            .collect();
                    }
                }

                // Custom excludes
                let add_excludes = Confirm::with_theme(&ColorfulTheme::default())
                    .with_prompt("Add custom exclude patterns? (node_modules, dist, etc. already excluded)")
                    .default(false)
                    .interact()?;

                if add_excludes {
                    let custom: String = Input::with_theme(&ColorfulTheme::default())
                        .with_prompt("Enter patterns (comma-separated)")
                        .interact_text()?;
                    config.exclude.extend(custom.split(',').map(|s| s.trim().to_string()));
                }

                // Workers - using system default (parallel workers is internal config)

                // TODO: MCP detection (commented out for future)
                // if scan.mcp_available {
                //     println!("{} MCP server detected", style("✓").green());
                //     // Could offer enhanced MCP-based features here
                // }
            } else {
                // Apply CLI options
                if !include.is_empty() {
                    config.include = include;
                }
                if !exclude.is_empty() {
                    config.exclude.extend(exclude);
                }
                // Output paths are now handled via Config helper methods
                // cache_path and vars_path can be passed to commands directly
                let _ = cache_path; // Suppress unused warning
                let _ = vars_path;
                let _ = workers;
            }

            // Create .acp directory
            let acp_dir = PathBuf::from(".acp");
            if !acp_dir.exists() {
                std::fs::create_dir(&acp_dir)?;
                println!("{} Created .acp/ directory", style("✓").green());
            }

            // Write config
            config.save(&config_path)?;
            println!("{} Created {}", style("✓").green(), config_path.display());

            // Print next steps
            println!("\nNext steps:");
            println!("  1. Run {} to index your codebase", style("acp index").cyan());
        }

        Commands::Index { root, output, vars } => {
            println!("{} Indexing codebase...", style("→").cyan());

            // Use config from target root if it exists, otherwise use defaults
            // This avoids pattern mismatches when indexing a subdirectory
            let config = {
                let root_config = root.join(".acp.config.json");
                let root_str = root.to_string_lossy();
                if root_config.exists() {
                    // Config in target directory takes precedence
                    Config::load(&root_config).unwrap_or_default()
                } else if root_str != "." && root_str != "./" {
                    // Indexing a subdirectory - use defaults to avoid pattern mismatches
                    // (parent's "src/**/*" won't match files when root IS src/)
                    Config::default()
                } else {
                    config
                }
            };

            let indexer = Indexer::new(config)?;
            let cache = indexer.index(&root).await?;

            // Create output directory if needed
            if let Some(parent) = output.parent() {
                if !parent.as_os_str().is_empty() && !parent.exists() {
                    std::fs::create_dir_all(parent)?;
                }
            }

            cache.write_json(&output)?;
            println!("{} Cache written to {}", style("✓").green(), output.display());
            println!("  Files: {}", cache.stats.files);
            println!("  Symbols: {}", cache.stats.symbols);
            println!("  Lines: {}", cache.stats.lines);

            if vars {
                let vars_file = indexer.generate_vars(&cache);
                // Replace acp.cache.json with acp.vars.json
                let output_str = output.to_string_lossy();
                let vars_path = if output_str.contains("acp.cache.json") {
                    PathBuf::from(output_str.replace("acp.cache.json", "acp.vars.json"))
                } else if output_str.contains("cache.json") {
                    PathBuf::from(output_str.replace("cache.json", "vars.json"))
                } else {
                    output.with_extension("vars.json")
                };
                vars_file.write_json(&vars_path)?;
                println!("{} Vars written to {}", style("✓").green(), vars_path.display());
            }
        }

        Commands::Vars { cache, output } => {
            println!("{} Generating vars...", style("→").cyan());
            
            let cache_data = Cache::from_json(&cache)?;
            let config = Config::default();
            let indexer = Indexer::new(config)?;
            let vars_file = indexer.generate_vars(&cache_data);

            vars_file.write_json(&output)?;
            println!("{} Vars written to {}", style("✓").green(), output.display());
            println!("  Variables: {}", vars_file.variables.len());
        }

        Commands::Query { query, cache } => {
            let cache_data = Cache::from_json(&cache)?;
            let q = Query::new(&cache_data);

            match query {
                QueryCommands::Symbol { name } => {
                    if let Some(sym) = q.symbol(&name) {
                        println!("{}", serde_json::to_string_pretty(sym)?);
                    } else {
                        eprintln!("{} Symbol not found: {}", style("✗").red(), name);
                    }
                }

                QueryCommands::File { path } => {
                    if let Some(file) = q.file(&path) {
                        println!("{}", serde_json::to_string_pretty(file)?);
                    } else {
                        eprintln!("{} File not found: {}", style("✗").red(), path);
                    }
                }

                QueryCommands::Callers { symbol } => {
                    let callers = q.callers(&symbol);
                    if callers.is_empty() {
                        println!("No callers found for {}", symbol);
                    } else {
                        for caller in callers {
                            println!("{}", caller);
                        }
                    }
                }

                QueryCommands::Callees { symbol } => {
                    let callees = q.callees(&symbol);
                    if callees.is_empty() {
                        println!("No callees found for {}", symbol);
                    } else {
                        for callee in callees {
                            println!("{}", callee);
                        }
                    }
                }

                QueryCommands::Domains => {
                    for domain in q.domains() {
                        println!("{}: {} files, {} symbols",
                            style(&domain.name).cyan(),
                            domain.files.len(),
                            domain.symbols.len()
                        );
                    }
                }

                QueryCommands::Domain { name } => {
                    if let Some(domain) = q.domain(&name) {
                        println!("{}", serde_json::to_string_pretty(domain)?);
                    } else {
                        eprintln!("{} Domain not found: {}", style("✗").red(), name);
                    }
                }

                QueryCommands::Hotpaths => {
                    for hp in q.hotpaths() {
                        println!("{}", hp);
                    }
                }

                QueryCommands::Stats => {
                    println!("Files: {}", cache_data.stats.files);
                    println!("Symbols: {}", cache_data.stats.symbols);
                    println!("Lines: {}", cache_data.stats.lines);
                    println!("Coverage: {:.1}%", cache_data.stats.annotation_coverage);
                    println!("Domains: {}", cache_data.domains.len());
                }
            }
        }

        Commands::Expand { text, mode, vars, chains } => {
            let vars_file = VarsFile::from_json(&vars)?;
            let resolver = VarResolver::new(vars_file);
            let mut expander = VarExpander::new(resolver);

            let input = match text {
                Some(t) => t,
                None => {
                    use std::io::Read;
                    let mut buf = String::new();
                    std::io::stdin().read_to_string(&mut buf)?;
                    buf
                }
            };

            let expansion_mode = match mode.as_str() {
                "none" => ExpansionMode::None,
                "summary" => ExpansionMode::Summary,
                "inline" => ExpansionMode::Inline,
                "annotated" => ExpansionMode::Annotated,
                "block" => ExpansionMode::Block,
                "interactive" => ExpansionMode::Interactive,
                _ => ExpansionMode::Annotated,
            };

            let result = expander.expand_text(&input, expansion_mode);
            println!("{}", result.expanded);

            if chains && !result.inheritance_chains.is_empty() {
                println!("\n{}", style("Inheritance Chains:").bold());
                for chain in &result.inheritance_chains {
                    println!("  {} → {}", 
                        style(&chain.root).cyan(),
                        chain.chain.join(" → ")
                    );
                }
            }
        }

        Commands::Chain { name, vars, tree } => {
            let vars_file = VarsFile::from_json(&vars)?;
            let resolver = VarResolver::new(vars_file);
            let expander = VarExpander::new(resolver);

            let name = name.trim_start_matches('$');
            let chain = expander.get_inheritance_chain(name);

            if tree {
                println!("{}", style(format!("${}", name)).cyan().bold());
                print_chain_tree(&chain.chain, 0);
            } else {
                println!("Root: {}", style(&chain.root).cyan());
                println!("Depth: {}", chain.depth);
                println!("Chain: {}", chain.chain.join(" → "));
            }
        }

        Commands::Watch { root } => {
            let watcher = acp::watch::FileWatcher::new(config);
            watcher.watch(&root)?;
        }

        Commands::Attempt { cmd } => {
            let mut tracker = acp::AttemptTracker::load_or_create();

            match cmd {
                AttemptCommands::Start { id, for_issue, description } => {
                    tracker.start_attempt(&id, for_issue.as_deref(), description.as_deref());
                    tracker.save()?;
                    println!("{} Started attempt: {}", style("✓").green(), id);
                }

                AttemptCommands::List { active, failed, history } => {
                    if history {
                        println!("{}", style("Attempt History:").bold());
                        for entry in &tracker.history {
                            let status_color = match entry.status {
                                acp::constraints::AttemptStatus::Verified => style("✓").green(),
                                acp::constraints::AttemptStatus::Failed => style("✗").red(),
                                acp::constraints::AttemptStatus::Reverted => style("↩").yellow(),
                                _ => style("?").dim(),
                            };
                            println!("  {} {} - {:?} ({} files)",
                                status_color,
                                entry.id,
                                entry.status,
                                entry.files_modified
                            );
                        }
                    } else {
                        println!("{}", style("Active Attempts:").bold());
                        for attempt in tracker.attempts.values() {
                            if active && attempt.status != acp::constraints::AttemptStatus::Active {
                                continue;
                            }
                            if failed && attempt.status != acp::constraints::AttemptStatus::Failed {
                                continue;
                            }
                            
                            let status_color = match attempt.status {
                                acp::constraints::AttemptStatus::Active => style("●").cyan(),
                                acp::constraints::AttemptStatus::Testing => style("◐").yellow(),
                                acp::constraints::AttemptStatus::Failed => style("✗").red(),
                                _ => style("?").dim(),
                            };
                            
                            println!("  {} {} - {:?}", status_color, attempt.id, attempt.status);
                            if let Some(issue) = &attempt.for_issue {
                                println!("    For: {}", issue);
                            }
                            println!("    Files: {}", attempt.files.len());
                        }
                    }
                }

                AttemptCommands::Fail { id, reason } => {
                    tracker.fail_attempt(&id, reason.as_deref())?;
                    tracker.save()?;
                    println!("{} Marked attempt as failed: {}", style("✗").red(), id);
                }

                AttemptCommands::Verify { id } => {
                    tracker.verify_attempt(&id)?;
                    tracker.save()?;
                    println!("{} Verified attempt: {}", style("✓").green(), id);
                }

                AttemptCommands::Revert { id } => {
                    let actions = tracker.revert_attempt(&id)?;
                    println!("{} Reverted attempt: {}", style("↩").yellow(), id);
                    for action in &actions {
                        println!("  {} {}", style(&action.action).dim(), action.file);
                    }
                }

                AttemptCommands::Cleanup => {
                    let actions = tracker.cleanup_failed()?;
                    println!("{} Cleaned up {} files from failed attempts",
                        style("✓").green(),
                        actions.len()
                    );
                }

                AttemptCommands::Checkpoint { name, files, description } => {
                    let file_refs: Vec<&str> = files.iter().map(|s| s.as_str()).collect();
                    tracker.create_checkpoint(&name, &file_refs, description.as_deref())?;
                    println!("{} Created checkpoint: {} ({} files)",
                        style("✓").green(),
                        name,
                        files.len()
                    );
                }

                AttemptCommands::Checkpoints => {
                    println!("{}", style("Checkpoints:").bold());
                    for (name, cp) in &tracker.checkpoints {
                        println!("  {} - {} files, created {}",
                            style(name).cyan(),
                            cp.files.len(),
                            cp.created_at.format("%Y-%m-%d %H:%M")
                        );
                    }
                }

                AttemptCommands::Restore { name } => {
                    let actions = tracker.restore_checkpoint(&name)?;
                    println!("{} Restored checkpoint: {}", style("↩").yellow(), name);
                    for action in &actions {
                        println!("  {} {}", style(&action.action).dim(), action.file);
                    }
                }
            }
        }

        Commands::Check { file, cache } => {
            let cache_data = Cache::from_json(&cache)?;

            // Try multiple path formats to find the file
            let file_str = file.to_string_lossy().to_string();
            let file_entry = cache_data.files.get(&file_str)
                .or_else(|| cache_data.files.get(&format!("./{}", file_str)))
                .or_else(|| {
                    // Try stripping ./ prefix if present
                    let stripped = file_str.strip_prefix("./").unwrap_or(&file_str);
                    cache_data.files.get(stripped)
                });

            if let Some(file_entry) = file_entry {
                // File found in cache - show basic info
                println!("{} File found in cache", style("✓").green());
                println!("  Path: {}", file_entry.path);
                println!("  Lines: {}", file_entry.lines);
                println!("  Language: {:?}", file_entry.language);
                if let Some(stability) = &file_entry.stability {
                    println!("  Stability: {:?}", stability);
                }

                // Check constraints if available
                if let Some(ref constraints) = cache_data.constraints {
                    if let Some(file_constraints) = constraints.by_file.get(&file_entry.path) {
                        if let Some(mutation) = &file_constraints.mutation {
                            println!("  Lock level: {:?}", mutation.level);
                        }
                    }
                }
            } else {
                eprintln!("{} File not in cache: {}", style("✗").red(), file.display());
            }
        }

        Commands::Revert { attempt, checkpoint } => {
            let mut tracker = acp::AttemptTracker::load_or_create();

            if let Some(id) = attempt {
                let actions = tracker.revert_attempt(&id)?;
                println!("{} Reverted attempt: {}", style("↩").yellow(), id);
                for action in &actions {
                    println!("  {} {}", style(&action.action).dim(), action.file);
                }
            } else if let Some(name) = checkpoint {
                let actions = tracker.restore_checkpoint(&name)?;
                println!("{} Restored checkpoint: {}", style("↩").yellow(), name);
                for action in &actions {
                    println!("  {} {}", style(&action.action).dim(), action.file);
                }
            } else {
                eprintln!("Specify --attempt or --checkpoint");
            }
        }

        Commands::Validate { file } => {
            let content = std::fs::read_to_string(&file)?;
            
            if file.to_string_lossy().contains("cache") {
                acp::schema::validate_cache(&content)?;
                println!("{} Cache file is valid", style("✓").green());
            } else if file.to_string_lossy().contains("vars") {
                acp::schema::validate_vars(&content)?;
                println!("{} Vars file is valid", style("✓").green());
            } else {
                eprintln!("Unknown file type. Expected 'cache' or 'vars' in filename.");
            }
        }
    }

    Ok(())
}

fn print_chain_tree(chain: &[String], depth: usize) {
    for (i, item) in chain.iter().skip(1).enumerate() {
        let prefix = if i == chain.len() - 2 { "└── " } else { "├── " };
        let indent = "│   ".repeat(depth);
        println!("{}{}${}", indent, prefix, style(item).cyan());
    }
}

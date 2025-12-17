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
    /// Index the codebase and generate .acp.cache.json
    Index {
        /// Root directory to index
        #[arg(default_value = ".")]
        root: PathBuf,

        /// Output cache file path
        #[arg(short, long, default_value = ".acp.cache.json")]
        output: PathBuf,

        /// Also generate vars file
        #[arg(long)]
        vars: bool,
    },

    /// Generate .acp.vars.json from cache
    Vars {
        /// Cache file to read
        #[arg(short, long, default_value = ".acp.cache.json")]
        cache: PathBuf,

        /// Output vars file path
        #[arg(short, long, default_value = ".acp.vars.json")]
        output: PathBuf,
    },

    /// Query the cache
    Query {
        /// Query type
        #[command(subcommand)]
        query: QueryCommands,

        /// Cache file to query
        #[arg(short, long, default_value = ".acp.cache.json")]
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
        #[arg(long, default_value = ".acp.vars.json")]
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
        #[arg(long, default_value = ".acp.vars.json")]
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
        #[arg(short, long, default_value = ".acp.cache.json")]
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
        Commands::Index { root, output, vars } => {
            println!("{} Indexing codebase...", style("→").cyan());
            
            let indexer = Indexer::new(config)?;
            let cache = indexer.index(&root).await?;

            cache.write_json(&output)?;
            println!("{} Cache written to {}", style("✓").green(), output.display());
            println!("  Files: {}", cache.stats.files);
            println!("  Symbols: {}", cache.stats.symbols);
            println!("  Lines: {}", cache.stats.lines);

            if vars {
                let vars_file = indexer.generate_vars(&cache);
                let vars_path = output.with_extension("vars.json");
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
            
            if let Some(stats) = &vars_file.stats {
                println!("  Variables: {}", stats.total_vars);
                println!("  Total tokens: {}", stats.total_tokens);
            }
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
                            domain.file_count,
                            domain.symbol_count
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
                    println!("Layers: {}", cache_data.layers.len());
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
                                acp::guardrails::AttemptStatus::Verified => style("✓").green(),
                                acp::guardrails::AttemptStatus::Failed => style("✗").red(),
                                acp::guardrails::AttemptStatus::Reverted => style("↩").yellow(),
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
                            if active && attempt.status != acp::guardrails::AttemptStatus::Active {
                                continue;
                            }
                            if failed && attempt.status != acp::guardrails::AttemptStatus::Failed {
                                continue;
                            }
                            
                            let status_color = match attempt.status {
                                acp::guardrails::AttemptStatus::Active => style("●").cyan(),
                                acp::guardrails::AttemptStatus::Testing => style("◐").yellow(),
                                acp::guardrails::AttemptStatus::Failed => style("✗").red(),
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
            
            if let Some(file_entry) = cache_data.files.get(&file.to_string_lossy().to_string()) {
                if let Some(guardrails) = &file_entry.guardrails {
                    let check = acp::GuardrailEnforcer::can_modify(guardrails);
                    
                    if check.passed {
                        println!("{} Guardrails check passed", style("✓").green());
                    } else {
                        println!("{} Guardrails check failed", style("✗").red());
                    }

                    if !check.violations.is_empty() {
                        println!("\n{}:", style("Violations").red().bold());
                        for v in &check.violations {
                            println!("  {} [{}] {}", style("✗").red(), v.rule, v.message);
                        }
                    }

                    if !check.warnings.is_empty() {
                        println!("\n{}:", style("Warnings").yellow().bold());
                        for w in &check.warnings {
                            println!("  {} [{}] {}", style("⚠").yellow(), w.rule, w.message);
                        }
                    }

                    if !check.required_actions.is_empty() {
                        println!("\n{}:", style("Required Actions").cyan().bold());
                        for a in &check.required_actions {
                            println!("  {} {} - {}", style("→").cyan(), a.action, a.reason);
                        }
                    }
                } else {
                    println!("{} No guardrails defined for this file", style("○").dim());
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

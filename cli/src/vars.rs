//! Variable system with inheritance and expansion
//!
//! Variables are token-efficient macros that can reference other variables.

use chrono::{DateTime, Utc};
use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::path::Path;

use crate::error::Result;

/// Complete vars file
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VarsFile {
    pub version: String,
    pub generated_at: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stats: Option<VarsStats>,
    pub vars: HashMap<String, VarEntry>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index: Option<VarsIndex>,
}

impl VarsFile {
    /// Load from JSON file
    pub fn from_json<P: AsRef<Path>>(path: P) -> Result<Self> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        Ok(serde_json::from_reader(reader)?)
    }

    /// Write to JSON file
    pub fn write_json<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let file = File::create(path)?;
        let writer = BufWriter::new(file);
        serde_json::to_writer_pretty(writer, self)?;
        Ok(())
    }

    /// Build indexes after loading/modification
    pub fn build_index(&mut self) {
        let mut by_category: HashMap<String, Vec<String>> = HashMap::new();
        let mut by_tag: HashMap<String, Vec<String>> = HashMap::new();
        let mut inheritance: HashMap<String, Vec<String>> = HashMap::new();

        for (name, var) in &self.vars {
            // Category index
            by_category
                .entry(var.category.to_string())
                .or_default()
                .push(name.clone());

            // Tag index
            for tag in &var.tags {
                by_tag
                    .entry(tag.clone())
                    .or_default()
                    .push(name.clone());
            }

            // Inheritance index
            if !var.refs.is_empty() {
                inheritance.insert(name.clone(), var.refs.clone());
            }
        }

        self.index = Some(VarsIndex {
            by_category,
            by_tag,
            inheritance,
        });

        // Update stats
        self.stats = Some(VarsStats {
            total_vars: self.vars.len(),
            total_tokens: self.vars.values().map(|v| v.tokens.unwrap_or(0)).sum(),
            by_category: self.index.as_ref()
                .map(|i| i.by_category.iter()
                    .map(|(k, v)| (k.clone(), v.len()))
                    .collect())
                .unwrap_or_default(),
        });
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VarsStats {
    pub total_vars: usize,
    pub total_tokens: usize,
    #[serde(default)]
    pub by_category: HashMap<String, usize>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VarEntry {
    pub name: String,
    pub category: VarCategory,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    pub value: Value, // Can be string or structured object
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tokens: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tokens_saved: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lines: Option<[usize; 2]>,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default)]
    pub refs: Vec<String>, // Variables this references
}

impl VarEntry {
    /// Get value as string (flattening if structured)
    pub fn value_string(&self) -> String {
        match &self.value {
            Value::String(s) => s.clone(),
            Value::Object(obj) => {
                obj.iter()
                    .map(|(k, v)| format!("{}:{}", k, value_to_string(v)))
                    .collect::<Vec<_>>()
                    .join("|")
            }
            _ => self.value.to_string(),
        }
    }
}

fn value_to_string(v: &Value) -> String {
    match v {
        Value::String(s) => s.clone(),
        Value::Array(arr) => arr.iter()
            .map(value_to_string)
            .collect::<Vec<_>>()
            .join(","),
        Value::Bool(b) => b.to_string(),
        Value::Number(n) => n.to_string(),
        _ => v.to_string(),
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum VarCategory {
    Symbol,
    File,
    Domain,
    Layer,
    Arch,
    Pattern,
    Procedure,
    Query,
    Context,
    Config,
    Custom,
}

impl std::fmt::Display for VarCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::Symbol => "symbol",
            Self::File => "file",
            Self::Domain => "domain",
            Self::Layer => "layer",
            Self::Arch => "arch",
            Self::Pattern => "pattern",
            Self::Procedure => "procedure",
            Self::Query => "query",
            Self::Context => "context",
            Self::Config => "config",
            Self::Custom => "custom",
        };
        write!(f, "{}", s)
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct VarsIndex {
    #[serde(default)]
    pub by_category: HashMap<String, Vec<String>>,
    #[serde(default)]
    pub by_tag: HashMap<String, Vec<String>>,
    #[serde(default)]
    pub inheritance: HashMap<String, Vec<String>>,
}

/// Resolves variable references
pub struct VarResolver {
    vars: HashMap<String, VarEntry>,
    var_pattern: Regex,
}

impl VarResolver {
    pub fn new(vars_file: VarsFile) -> Self {
        Self {
            vars: vars_file.vars,
            var_pattern: Regex::new(r"\$([A-Z][A-Z0-9_]+)(?:\.(\w+))?").unwrap(),
        }
    }

    /// Get a variable by name
    pub fn get(&self, name: &str) -> Option<&VarEntry> {
        self.vars.get(name)
    }

    /// Find all variable references in text
    pub fn find_references(&self, text: &str) -> Vec<VarReference> {
        self.var_pattern
            .captures_iter(text)
            .map(|cap| VarReference {
                full_match: cap.get(0).unwrap().as_str().to_string(),
                name: cap.get(1).unwrap().as_str().to_string(),
                modifier: cap.get(2).map(|m| m.as_str().to_string()),
                start: cap.get(0).unwrap().start(),
                end: cap.get(0).unwrap().end(),
            })
            .collect()
    }

    /// Get variables by category
    pub fn by_category(&self, category: VarCategory) -> Vec<&VarEntry> {
        self.vars
            .values()
            .filter(|v| v.category == category)
            .collect()
    }

    /// Get variables by tag
    pub fn by_tag(&self, tag: &str) -> Vec<&VarEntry> {
        self.vars
            .values()
            .filter(|v| v.tags.contains(&tag.to_string()))
            .collect()
    }

    /// Search variables
    pub fn search(&self, query: &str) -> Vec<&VarEntry> {
        let q = query.to_lowercase();
        self.vars
            .values()
            .filter(|v| {
                v.name.to_lowercase().contains(&q)
                    || v.summary.as_ref().map(|s| s.to_lowercase().contains(&q)).unwrap_or(false)
                    || v.tags.iter().any(|t| t.to_lowercase().contains(&q))
            })
            .collect()
    }
}

#[derive(Debug, Clone)]
pub struct VarReference {
    pub full_match: String,
    pub name: String,
    pub modifier: Option<String>,
    pub start: usize,
    pub end: usize,
}

/// Expands variable references with inheritance support
pub struct VarExpander {
    resolver: VarResolver,
    expansion_cache: HashMap<String, String>,
}

impl VarExpander {
    pub fn new(resolver: VarResolver) -> Self {
        Self {
            resolver,
            expansion_cache: HashMap::new(),
        }
    }

    /// Expand a single variable, resolving inheritance
    pub fn expand_var(
        &mut self,
        name: &str,
        max_depth: usize,
        visited: &mut HashSet<String>,
    ) -> Option<String> {
        // Check cache
        if let Some(cached) = self.expansion_cache.get(name) {
            return Some(cached.clone());
        }

        // Cycle detection
        if visited.contains(name) {
            return Some(format!("[CYCLE:${}]", name));
        }

        let var = self.resolver.get(name)?;
        visited.insert(name.to_string());

        let mut value = var.value_string();

        // Recursively expand nested references
        if max_depth > 0 {
            let refs = self.resolver.find_references(&value);
            for r in refs.iter().rev() {
                if let Some(expanded) = self.expand_var(&r.name, max_depth - 1, visited) {
                    value = format!(
                        "{}{}{}",
                        &value[..r.start],
                        expanded,
                        &value[r.end..]
                    );
                }
            }
        }

        self.expansion_cache.insert(name.to_string(), value.clone());
        Some(value)
    }

    /// Expand all variable references in text
    pub fn expand_text(&mut self, text: &str, mode: ExpansionMode) -> ExpansionResult {
        let refs = self.resolver.find_references(text);
        let tokens_before = estimate_tokens(text);
        let mut expanded = text.to_string();
        let mut vars_expanded = Vec::new();
        let mut vars_unresolved = Vec::new();
        let mut chains = Vec::new();

        // Process in reverse to preserve positions
        for r in refs.iter().rev() {
            if let Some(var) = self.resolver.get(&r.name) {
                vars_expanded.push(r.name.clone());

                // Track inheritance chain
                let chain = self.get_inheritance_chain(&r.name);
                chains.push(chain);

                // Format based on mode
                let replacement = self.format_var(var, r.modifier.as_deref(), mode);
                expanded = format!(
                    "{}{}{}",
                    &expanded[..r.start],
                    replacement,
                    &expanded[r.end..]
                );
            } else {
                vars_unresolved.push(r.name.clone());
            }
        }

        ExpansionResult {
            original: text.to_string(),
            expanded,
            vars_expanded,
            vars_unresolved,
            tokens_before,
            tokens_after: estimate_tokens(&text), // Recalculate after expansion
            inheritance_chains: chains,
        }
    }

    /// Get full inheritance chain for a variable
    pub fn get_inheritance_chain(&self, name: &str) -> InheritanceChain {
        let mut chain = vec![name.to_string()];
        let mut visited = HashSet::new();
        visited.insert(name.to_string());

        self.build_chain(name, &mut chain, &mut visited);

        InheritanceChain {
            root: name.to_string(),
            chain: chain.clone(),
            depth: chain.len() - 1,
            has_cycle: false, // Would be detected during build
        }
    }

    fn build_chain(&self, name: &str, chain: &mut Vec<String>, visited: &mut HashSet<String>) {
        if let Some(var) = self.resolver.get(name) {
            for ref_name in &var.refs {
                if !visited.contains(ref_name) {
                    visited.insert(ref_name.clone());
                    chain.push(ref_name.clone());
                    self.build_chain(ref_name, chain, visited);
                }
            }
        }
    }

    fn format_var(&mut self, var: &VarEntry, modifier: Option<&str>, mode: ExpansionMode) -> String {
        // Handle modifier
        if let Some(m) = modifier {
            return match m {
                "summary" => var.summary.clone().unwrap_or_default(),
                "value" => var.value_string(),
                "source" => var.source.clone().unwrap_or_default(),
                "lines" => var.lines.map(|l| format!("{}-{}", l[0], l[1])).unwrap_or_default(),
                _ => var.value_string(),
            };
        }

        // Handle expansion mode
        match mode {
            ExpansionMode::None => format!("${}", var.name),
            ExpansionMode::Summary => var.summary.clone().unwrap_or_else(|| var.name.clone()),
            ExpansionMode::Inline => {
                let mut visited = HashSet::new();
                self.expand_var(&var.name, 3, &mut visited)
                    .unwrap_or_else(|| var.value_string())
            }
            ExpansionMode::Annotated => {
                format!(
                    "**${}** → {}",
                    var.name,
                    self.humanize_value(&var.value_string())
                )
            }
            ExpansionMode::Block => self.format_block(var),
            ExpansionMode::Interactive => {
                format!(
                    r#"<acp-var name="{}" summary="{}">{}</acp-var>"#,
                    var.name,
                    var.summary.as_deref().unwrap_or(&var.name),
                    var.summary.as_deref().unwrap_or(&var.name)
                )
            }
        }
    }

    fn format_block(&self, var: &VarEntry) -> String {
        let mut lines = Vec::new();
        lines.push(format!("> **{}**: {}", var.name, var.summary.as_deref().unwrap_or("")));
        lines.push(">".to_string());
        
        if let Value::Object(obj) = &var.value {
            for (k, v) in obj {
                lines.push(format!("> - **{}**: {}", k, value_to_string(v)));
            }
        } else {
            lines.push(format!("> {}", var.value_string()));
        }
        
        if let Some(source) = &var.source {
            lines.push(format!("> - *Source*: `{}`", source));
        }
        
        lines.join("\n")
    }

    fn humanize_value(&self, value: &str) -> String {
        // Convert pipe-separated to readable
        value
            .split('|')
            .filter_map(|part| {
                let mut iter = part.splitn(2, ':');
                let key = iter.next()?;
                let val = iter.next()?;
                Some(format!("{}: {}", capitalize(key), val))
            })
            .collect::<Vec<_>>()
            .join(" | ")
    }

    pub fn clear_cache(&mut self) {
        self.expansion_cache.clear();
    }
}

fn capitalize(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().chain(c).collect(),
    }
}

fn estimate_tokens(text: &str) -> usize {
    (text.len() + 3) / 4 // Rough estimate: ~4 chars per token
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExpansionMode {
    /// Keep $VAR as-is
    None,
    /// Replace with summary only
    Summary,
    /// Replace with full value inline
    Inline,
    /// Show both: **$VAR** → value
    Annotated,
    /// Full formatted block
    Block,
    /// HTML-like markers for UI
    Interactive,
}

#[derive(Debug, Clone)]
pub struct ExpansionResult {
    pub original: String,
    pub expanded: String,
    pub vars_expanded: Vec<String>,
    pub vars_unresolved: Vec<String>,
    pub tokens_before: usize,
    pub tokens_after: usize,
    pub inheritance_chains: Vec<InheritanceChain>,
}

#[derive(Debug, Clone)]
pub struct InheritanceChain {
    pub root: String,
    pub chain: Vec<String>,
    pub depth: usize,
    pub has_cycle: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_references() {
        let vars_file = VarsFile {
            version: "1.0.0".to_string(),
            generated_at: Utc::now(),
            project: None,
            stats: None,
            vars: HashMap::new(),
            index: None,
        };
        let resolver = VarResolver::new(vars_file);

        let refs = resolver.find_references("Check $SYM_TEST and $ARCH_FLOW.value");
        assert_eq!(refs.len(), 2);
        assert_eq!(refs[0].name, "SYM_TEST");
        assert_eq!(refs[1].name, "ARCH_FLOW");
        assert_eq!(refs[1].modifier, Some("value".to_string()));
    }
}

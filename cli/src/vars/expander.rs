//! @acp:module "Variable Expander"
//! @acp:summary "Expands variable references with inheritance support"
//! @acp:domain cli
//! @acp:layer service

use serde_json::Value;
use std::collections::{HashMap, HashSet};

use super::{capitalize, estimate_tokens, value_to_string, VarEntry, VarResolver};

/// @acp:summary "Expands variable references with caching and inheritance"
pub struct VarExpander {
    resolver: VarResolver,
    expansion_cache: HashMap<String, String>,
}

impl VarExpander {
    /// Create a new expander from a resolver
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
            if let Some(var) = self.resolver.get(&r.name).cloned() {
                vars_expanded.push(r.name.clone());

                // Track inheritance chain
                let chain = self.get_inheritance_chain(&r.name);
                chains.push(chain);

                // Format based on mode
                let replacement = self.format_var(&var, r.modifier.as_deref(), mode);
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
            tokens_after: estimate_tokens(&text),
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
            has_cycle: false,
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

    /// Clear the expansion cache
    pub fn clear_cache(&mut self) {
        self.expansion_cache.clear();
    }
}

/// @acp:summary "Variable expansion mode controlling output format"
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

/// @acp:summary "Result of variable expansion operation"
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

/// @acp:summary "Inheritance chain for a variable"
#[derive(Debug, Clone)]
pub struct InheritanceChain {
    pub root: String,
    pub chain: Vec<String>,
    pub depth: usize,
    pub has_cycle: bool,
}

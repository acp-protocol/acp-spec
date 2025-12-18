//! @acp:module "Variables"
//! @acp:summary "Variable system with inheritance and expansion for token-efficient macros"
//! @acp:domain cli
//! @acp:layer model
//! @acp:stability stable

mod resolver;
mod expander;

pub mod presets;

pub use resolver::{VarResolver, VarReference};
pub use expander::{VarExpander, ExpansionMode, ExpansionResult, InheritanceChain};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::path::Path;

use crate::error::Result;

/// @acp:summary "Complete vars file structure for .acp.vars.json"
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

/// @acp:summary "Statistics about the vars file"
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VarsStats {
    pub total_vars: usize,
    pub total_tokens: usize,
    #[serde(default)]
    pub by_category: HashMap<String, usize>,
}

/// @acp:summary "A single variable entry with metadata"
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VarEntry {
    pub name: String,
    pub category: VarCategory,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    pub value: Value,
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
    pub refs: Vec<String>,
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

/// @acp:summary "Convert JSON value to display string"
pub fn value_to_string(v: &Value) -> String {
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

/// @acp:summary "Variable category classification"
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

/// @acp:summary "Lookup indexes for fast variable access"
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct VarsIndex {
    #[serde(default)]
    pub by_category: HashMap<String, Vec<String>>,
    #[serde(default)]
    pub by_tag: HashMap<String, Vec<String>>,
    #[serde(default)]
    pub inheritance: HashMap<String, Vec<String>>,
}

/// @acp:summary "Estimate token count from text length"
pub fn estimate_tokens(text: &str) -> usize {
    (text.len() + 3) / 4
}

/// @acp:summary "Capitalize first character of string"
pub fn capitalize(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().chain(c).collect(),
    }
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

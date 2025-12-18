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
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
        let _lang = Language::from_path(path)
            .ok_or_else(|| AcpError::UnsupportedLanguage(
                path.extension()
                    .map(|e| e.to_string_lossy().to_string())
                    .unwrap_or_default()
            ))?;

        let lines = content.lines().count();
        let file_name = path.file_stem()
            .map(|s| s.to_string_lossy().to_string())
            .unwrap_or_default();
        let file_path = path.to_string_lossy().to_string();

        // Parse @acp: annotations from source
        let annotations = self.parse_annotations(&content);

        // Extract file-level metadata from annotations
        let mut module_name = file_name.clone();
        let mut domains = vec![];
        let mut layer = None;
        let mut symbols = vec![];
        let mut symbol_names = vec![];
        let mut calls = vec![];

        // Track current symbol context for multi-line annotations
        let mut current_symbol: Option<SymbolBuilder> = None;

        for ann in &annotations {
            match ann.name.as_str() {
                "module" => {
                    if let Some(val) = &ann.value {
                        module_name = val.trim_matches('"').to_string();
                    }
                }
                "domain" => {
                    if let Some(val) = &ann.value {
                        domains.push(val.trim_matches('"').to_string());
                    }
                }
                "layer" => {
                    if let Some(val) = &ann.value {
                        layer = Some(val.trim_matches('"').to_string());
                    }
                }
                "symbol" => {
                    // Save previous symbol if exists
                    if let Some(builder) = current_symbol.take() {
                        let sym = builder.build(&file_path);
                        symbol_names.push(sym.name.clone());
                        symbols.push(sym);
                    }
                    // Start new symbol
                    if let Some(val) = &ann.value {
                        current_symbol = Some(SymbolBuilder::new(
                            val.trim_matches('"').to_string(),
                            ann.line,
                        ));
                    }
                }
                "summary" => {
                    if let Some(ref mut builder) = current_symbol {
                        if let Some(val) = &ann.value {
                            builder.summary = Some(val.trim_matches('"').to_string());
                        }
                    }
                }
                "calls" => {
                    if let Some(ref mut builder) = current_symbol {
                        if let Some(val) = &ann.value {
                            let callees: Vec<String> = val
                                .split(',')
                                .map(|s| s.trim().trim_matches('"').to_string())
                                .collect();
                            builder.calls.extend(callees);
                        }
                    }
                }
                _ => {}
            }
        }

        // Save last symbol
        if let Some(builder) = current_symbol {
            let sym = builder.build(&file_path);
            if !sym.calls.is_empty() {
                calls.push((sym.name.clone(), sym.calls.clone()));
            }
            symbol_names.push(sym.name.clone());
            symbols.push(sym);
        }

        // Build call edges for earlier symbols
        for sym in &symbols {
            if !sym.calls.is_empty() {
                calls.push((sym.name.clone(), sym.calls.clone()));
            }
        }

        let file = FileEntry {
            path: file_path,
            module: module_name,
            lines,
            domains,
            layer,
            stability: Stability::Active,
            depends: vec![],
            exports: symbol_names.clone(),
            symbols: symbol_names,
            keywords: vec![],
            hash: Some(format!("{:x}", md5::compute(&content))),
            guardrails: None,
        };

        Ok(ParseResult {
            file,
            symbols,
            calls,
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

/// Helper to build SymbolEntry from annotations
struct SymbolBuilder {
    name: String,
    line: usize,
    summary: Option<String>,
    calls: Vec<String>,
}

impl SymbolBuilder {
    fn new(name: String, line: usize) -> Self {
        Self {
            name,
            line,
            summary: None,
            calls: vec![],
        }
    }

    fn build(self, file_path: &str) -> SymbolEntry {
        SymbolEntry {
            name: self.name,
            fqn: None,
            symbol_type: SymbolType::Fn,
            file: file_path.to_string(),
            lines: [self.line, self.line + 10], // Approximate
            summary: self.summary,
            signature: None,
            exported: true,
            async_fn: false,
            calls: self.calls,
            throws: vec![],
            flags: vec![],
            side_effects: vec![],
            complexity: None,
        }
    }
}
//! @acp:module "Parser"
//! @acp:summary "Source code parsing and annotation extraction (schema-compliant)"
//! @acp:domain cli
//! @acp:layer service
//!
//! Parses source files to extract symbols, calls, and documentation.
//! Currently uses regex-based parsing with tree-sitter support planned.

use std::path::Path;

use crate::cache::{FileEntry, SymbolEntry, SymbolType, Visibility};
use crate::error::{AcpError, Result};
use crate::index::detect_language;

/// @acp:summary "Result of parsing a source file"
#[derive(Debug, Clone)]
pub struct ParseResult {
    pub file: FileEntry,
    pub symbols: Vec<SymbolEntry>,
    pub calls: Vec<(String, Vec<String>)>, // (caller, callees)
    pub lock_level: Option<String>,         // from @acp:lock
    pub ai_hints: Vec<String>,              // from @acp:ai-careful, @acp:ai-readonly, etc.
    pub hacks: Vec<HackAnnotation>,         // from @acp:hack
}

/// @acp:summary "Parsed hack annotation"
#[derive(Debug, Clone)]
pub struct HackAnnotation {
    pub line: usize,
    pub expires: Option<String>,
    pub ticket: Option<String>,
    pub reason: Option<String>,
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
        let file_path = path.to_string_lossy().to_string();

        let language = detect_language(&file_path)
            .ok_or_else(|| AcpError::UnsupportedLanguage(
                path.extension()
                    .map(|e| e.to_string_lossy().to_string())
                    .unwrap_or_default()
            ))?;

        let lines = content.lines().count();
        let _file_name = path.file_stem()
            .map(|s| s.to_string_lossy().to_string())
            .unwrap_or_default();

        // Parse @acp: annotations from source
        let annotations = self.parse_annotations(&content);

        // Extract file-level metadata from annotations
        let mut module_name = None;
        let mut file_summary = None;
        let mut domains = vec![];
        let mut layer = None;
        let mut symbols = vec![];
        let mut exports = vec![];
        let mut imports = vec![];
        let mut calls = vec![];
        let mut lock_level = None;
        let mut ai_hints = vec![];
        let mut hacks = vec![];

        // Track current symbol context for multi-line annotations
        let mut current_symbol: Option<SymbolBuilder> = None;

        for ann in &annotations {
            match ann.name.as_str() {
                "module" => {
                    if let Some(val) = &ann.value {
                        module_name = Some(val.trim_matches('"').to_string());
                    }
                }
                "summary" => {
                    if let Some(ref mut builder) = current_symbol {
                        if let Some(val) = &ann.value {
                            builder.summary = Some(val.trim_matches('"').to_string());
                        }
                    } else if let Some(val) = &ann.value {
                        // File-level summary
                        file_summary = Some(val.trim_matches('"').to_string());
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
                "lock" => {
                    if let Some(val) = &ann.value {
                        lock_level = Some(val.trim_matches('"').to_string());
                    }
                }
                "ai-careful" | "ai-readonly" | "ai-avoid" | "ai-no-modify" => {
                    let hint = if let Some(val) = &ann.value {
                        format!("{}: {}", ann.name, val.trim_matches('"'))
                    } else {
                        ann.name.clone()
                    };
                    ai_hints.push(hint);
                }
                "hack" => {
                    // Parse hack annotation: @acp:hack expires=2025-03-01 ticket=JIRA-123 "reason"
                    let mut hack = HackAnnotation {
                        line: ann.line,
                        expires: None,
                        ticket: None,
                        reason: None,
                    };
                    if let Some(val) = &ann.value {
                        // Parse key=value pairs and quoted reason
                        for part in val.split_whitespace() {
                            if let Some(expires) = part.strip_prefix("expires=") {
                                hack.expires = Some(expires.to_string());
                            } else if let Some(ticket) = part.strip_prefix("ticket=") {
                                hack.ticket = Some(ticket.to_string());
                            } else if part.starts_with('"') {
                                // Capture the rest as reason
                                hack.reason = Some(val.split('"').nth(1).unwrap_or("").to_string());
                                break;
                            }
                        }
                    }
                    hacks.push(hack);
                }
                "symbol" => {
                    // Save previous symbol if exists
                    if let Some(builder) = current_symbol.take() {
                        let sym = builder.build(&file_path);
                        exports.push(sym.name.clone());
                        symbols.push(sym);
                    }
                    // Start new symbol
                    if let Some(val) = &ann.value {
                        current_symbol = Some(SymbolBuilder::new(
                            val.trim_matches('"').to_string(),
                            ann.line,
                            &file_path,
                        ));
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
                "imports" | "depends" => {
                    if let Some(val) = &ann.value {
                        let import_list: Vec<String> = val
                            .split(',')
                            .map(|s| s.trim().trim_matches('"').to_string())
                            .collect();
                        imports.extend(import_list);
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
            exports.push(sym.name.clone());
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
            lines,
            language,
            exports,
            imports,
            module: module_name,
            summary: file_summary,
            domains,
            layer,
            stability: None,
            ai_hints: ai_hints.clone(),
            git: None,
        };

        Ok(ParseResult {
            file,
            symbols,
            calls,
            lock_level,
            ai_hints,
            hacks,
        })
    }

    /// @acp:summary "Parse @acp: annotations from source comments"
    pub fn parse_annotations(&self, content: &str) -> Vec<Annotation> {
        let mut annotations = Vec::new();
        let pattern = regex::Regex::new(r"@acp:([\w-]+)(?:\s+(.+))?").unwrap();

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
    qualified_name: String,
    line: usize,
    summary: Option<String>,
    calls: Vec<String>,
}

impl SymbolBuilder {
    fn new(name: String, line: usize, file_path: &str) -> Self {
        let qualified_name = format!("{}:{}", file_path, name);
        Self {
            name,
            qualified_name,
            line,
            summary: None,
            calls: vec![],
        }
    }

    fn build(self, file_path: &str) -> SymbolEntry {
        SymbolEntry {
            name: self.name,
            qualified_name: self.qualified_name,
            symbol_type: SymbolType::Function,
            file: file_path.to_string(),
            lines: [self.line, self.line + 10], // Approximate
            exported: true,
            signature: None,
            summary: self.summary,
            async_fn: false,
            visibility: Visibility::Public,
            calls: self.calls,
            called_by: vec![], // Populated later by indexer
            git: None,
        }
    }
}

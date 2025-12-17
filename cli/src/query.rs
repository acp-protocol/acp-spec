//! Query module for programmatic cache access
//!
//! Provides type-safe queries similar to jq but in Rust.

use crate::cache::{Cache, SymbolEntry, FileEntry, DomainEntry};

/// Query builder for cache
pub struct Query<'a> {
    cache: &'a Cache,
}

impl<'a> Query<'a> {
    pub fn new(cache: &'a Cache) -> Self {
        Self { cache }
    }

    /// Get symbol by name
    pub fn symbol(&self, name: &str) -> Option<&SymbolEntry> {
        self.cache.get_symbol(name)
    }

    /// Get file by path
    pub fn file(&self, path: &str) -> Option<&FileEntry> {
        self.cache.get_file(path)
    }

    /// Get callers of a symbol
    pub fn callers(&self, symbol: &str) -> Vec<&str> {
        self.cache.get_callers(symbol)
            .map(|v| v.iter().map(|s| s.as_str()).collect())
            .unwrap_or_default()
    }

    /// Get callees of a symbol
    pub fn callees(&self, symbol: &str) -> Vec<&str> {
        self.cache.get_callees(symbol)
            .map(|v| v.iter().map(|s| s.as_str()).collect())
            .unwrap_or_default()
    }

    /// Get domain by name
    pub fn domain(&self, name: &str) -> Option<&DomainEntry> {
        self.cache.domains.get(name)
    }

    /// Get all domains
    pub fn domains(&self) -> impl Iterator<Item = &DomainEntry> {
        self.cache.domains.values()
    }

    /// Get files by domain
    pub fn files_in_domain(&self, domain: &str) -> Vec<&str> {
        self.cache.get_domain_files(domain)
            .map(|v| v.iter().map(|s| s.as_str()).collect())
            .unwrap_or_default()
    }

    /// Get files by layer
    pub fn files_in_layer(&self, layer: &str) -> Vec<&str> {
        self.cache.get_layer_files(layer)
            .map(|v| v.iter().map(|s| s.as_str()).collect())
            .unwrap_or_default()
    }

    /// Search symbols by name pattern
    pub fn search_symbols(&self, pattern: &str) -> Vec<&SymbolEntry> {
        let p = pattern.to_lowercase();
        self.cache.symbols.values()
            .filter(|s| s.name.to_lowercase().contains(&p))
            .collect()
    }

    /// Get hotpath symbols
    pub fn hotpaths(&self) -> impl Iterator<Item = &str> {
        self.cache.hotpaths.iter().map(|h| h.symbol.as_str())
    }

    /// Get deprecated symbols/files
    pub fn deprecated(&self) -> &[String] {
        &self.cache.stability.deprecated
    }

    /// Get security-sensitive symbols
    pub fn security_sensitive(&self) -> impl Iterator<Item = &str> {
        self.cache.security.auth_required.iter()
            .chain(self.cache.security.audit_logged.iter())
            .chain(self.cache.security.credentials.iter())
            .map(|s| s.as_str())
    }
}

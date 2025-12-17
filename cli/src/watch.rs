//! File watching module
//!
//! Watches for file changes and updates cache/vars incrementally.

use std::path::Path;
use std::sync::mpsc;

use notify::{Watcher, RecursiveMode, watcher};

use crate::config::Config;
use crate::error::Result;

/// File watcher for incremental updates
pub struct FileWatcher {
    config: Config,
}

impl FileWatcher {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// Start watching for changes
    pub fn watch<P: AsRef<Path>>(&self, root: P) -> Result<()> {
        let (tx, rx) = mpsc::channel();

        let mut watcher = watcher(tx, std::time::Duration::from_secs(2))
            .map_err(|e| crate::error::AcpError::Other(e.to_string()))?;

        watcher.watch(root.as_ref(), RecursiveMode::Recursive)
            .map_err(|e| crate::error::AcpError::Other(e.to_string()))?;

        println!("Watching for changes...");

        loop {
            match rx.recv() {
                Ok(event) => {
                    println!("Change detected: {:?}", event);
                    // TODO: Incremental update
                }
                Err(e) => {
                    eprintln!("Watch error: {}", e);
                }
            }
        }
    }
}

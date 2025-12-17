//! Output expansion module
//!
//! Expands AI output with variable references for human reading.

pub use crate::vars::{
    VarExpander,
    VarResolver,
    ExpansionMode,
    ExpansionResult,
    InheritanceChain,
};

/// Preset expansion configurations
pub mod presets {
    use super::ExpansionMode;

    /// For AI-to-AI communication
    pub const AI_TO_AI: ExpansionMode = ExpansionMode::None;

    /// Quick human reading
    pub const QUICK: ExpansionMode = ExpansionMode::Inline;

    /// Detailed human reading (default)
    pub const DETAILED: ExpansionMode = ExpansionMode::Annotated;

    /// Documentation generation
    pub const DOCUMENTATION: ExpansionMode = ExpansionMode::Block;

    /// Interactive web UI
    pub const INTERACTIVE: ExpansionMode = ExpansionMode::Interactive;

    /// Most compact human-readable
    pub const SUMMARY: ExpansionMode = ExpansionMode::Summary;
}

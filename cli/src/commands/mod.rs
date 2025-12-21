//! @acp:module "Commands"
//! @acp:summary "CLI command implementations for RFC-001 features"
//! @acp:domain cli
//! @acp:layer service
//!
//! Provides implementations for:
//! - `acp map` - Directory/file structure with annotations
//! - `acp migrate` - Add directive suffixes to annotations

pub mod map;
pub mod migrate;
pub mod output;

pub use map::{execute_map, MapBuilder, MapFormat, MapOptions};
pub use migrate::{execute_migrate, DirectiveDefaults, MigrateOptions, MigrationScanner};
pub use output::{format_constraint_level, format_symbol_ref, format_symbol_ref_range, TreeRenderer};

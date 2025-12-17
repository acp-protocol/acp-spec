//! Schema validation module

use crate::error::Result;

/// Validate cache file against schema
pub fn validate_cache(json: &str) -> Result<()> {
    // TODO: Implement JSON schema validation
    let _: crate::cache::Cache = serde_json::from_str(json)?;
    Ok(())
}

/// Validate vars file against schema
pub fn validate_vars(json: &str) -> Result<()> {
    // TODO: Implement JSON schema validation
    let _: crate::vars::VarsFile = serde_json::from_str(json)?;
    Ok(())
}

//! @acp:module "Schema Validation"
//! @acp:summary "JSON schema validation for ACP files"
//! @acp:domain cli
//! @acp:layer utility

use crate::error::Result;

/// @acp:summary "Validate cache file against schema"
pub fn validate_cache(json: &str) -> Result<()> {
    // TODO: Implement JSON schema validation using jsonschema crate
    let _: crate::cache::Cache = serde_json::from_str(json)?;
    Ok(())
}

/// @acp:summary "Validate vars file against schema"
pub fn validate_vars(json: &str) -> Result<()> {
    // TODO: Implement JSON schema validation using jsonschema crate
    let _: crate::vars::VarsFile = serde_json::from_str(json)?;
    Ok(())
}

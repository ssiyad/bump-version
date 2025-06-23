use super::{get_version_from_source, parse_source, write_source};
use crate::{error::BumpVersionError, version::Version};

/// Get the version from the package.json file.
pub fn get_version(file_path: &str) -> Result<Version, BumpVersionError> {
    let source = parse_source(file_path)?;
    get_version_from_source(source, vec!["version".to_string()])
}

/// Update the version in the package.json file.
///
/// * `version`: The new version to set in the package.json file.
pub fn update_version(file_path: &str, version: &Version) -> Result<(), BumpVersionError> {
    // Get package.json.
    let mut source = parse_source(file_path)?;

    // Upsert the version string in the package.json.
    source.insert("version".to_string(), version.to_string().into());

    // Serialize the package.json.
    let content = serde_json::to_string_pretty(&source)?;

    // Write the updated package.json back to the file.
    write_source(file_path, &content)?;

    Ok(())
}

use super::{parse_source, write_source};
use crate::{error::BumpVersionError, version::Version};

// Define the name of the package.json file.
const PACKAGE_JSON: &str = "package.json";

/// Get the version from the package.json file.
pub fn get_version() -> Result<Version, BumpVersionError> {
    // Get package.json.
    let source = parse_source(PACKAGE_JSON)?;

    // Get the version string from the package.json.
    let version_str = source
        .get("version")
        .ok_or(BumpVersionError::Other("Version not found in package.json"))?
        .as_str()
        .ok_or(BumpVersionError::Other("Version is not a string"))?;

    // Convert the version string to a Version struct.
    Ok(Version::from(version_str))
}

/// Update the version in the package.json file.
///
/// * `version`: The new version to set in the package.json file.
pub fn update_version(version: &Version) -> Result<(), BumpVersionError> {
    // Get package.json.
    let mut source = parse_source(PACKAGE_JSON)?;

    // Upsert the version string in the package.json.
    source.insert("version".to_string(), version.to_string().into());

    // Serialize the package.json.
    let content = serde_json::to_string_pretty(&source)?;

    // Write the updated package.json back to the file.
    write_source(PACKAGE_JSON, &content)?;

    Ok(())
}

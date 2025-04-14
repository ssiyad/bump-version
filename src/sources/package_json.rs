use super::get_path;
use crate::version::Version;
use indexmap::IndexMap;
use std::fs;

// Define the name of the package.json file.
const PACKAGE_JSON: &str = "package.json";

/// Parse the package.json file and return its contents as an IndexMap.
fn parse() -> IndexMap<String, serde_json::Value> {
    // Read the package.json file.
    let path = get_path(PACKAGE_JSON);
    let content = fs::read(path).expect("Unable to read file");

    // Parse and return package.json.
    serde_json::from_slice(&content).expect("Unable to parse JSON")
}

/// Get the version from the package.json file.
pub fn get_version() -> Version {
    // Get package.json.
    let package = parse();

    // Get the version string from the package.json.
    let version_str = package
        .get("version")
        .expect("Version not found in package.json")
        .as_str()
        .expect("Version is not a string");

    // Convert the version string to a Version struct.
    Version::from(version_str)
}

/// Update the version in the package.json file.
///
/// * `version`: The new version to set in the package.json file.
pub fn update_version(version: &Version) {
    // Get package.json.
    let mut package = parse();

    // Upsert the version string in the package.json.
    let key = "version".to_string();
    let version = version.to_string();
    package.insert(key, version.into());

    // Serialize the package.json.
    let content = serde_json::to_vec_pretty(&package).expect("Unable to serialize JSON");

    // Write the updated package.json back to the file.
    let path = get_path(PACKAGE_JSON);
    fs::write(path, content).expect("Unable to write file");
}

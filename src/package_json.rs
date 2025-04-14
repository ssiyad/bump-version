use crate::version::Version;
use indexmap::IndexMap;
use std::fs;

// Define the path to the package.json file.
const PACKAGE_JSON: &str = "package.json";

fn parse() -> IndexMap<String, serde_json::Value> {
    // Read the package.json file.
    let content = fs::read(PACKAGE_JSON).expect("Unable to read file");

    // Parse and return package.json.
    serde_json::from_slice(&content).expect("Unable to parse JSON")
}

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
    fs::write(PACKAGE_JSON, content).expect("Unable to write file");
}

use crate::version::Version;
use git2::Repository;
use indexmap::IndexMap;
use std::fs;

/// Get the path to the package.json file. Looks first in the current directory, then in the git
/// repository root.
fn get_path() -> String {
    // Define `package.json` name.
    let package_json = "package.json".to_string();

    // If the file exists, return its path.
    if fs::exists(&package_json).is_ok_and(|exists| exists) {
        return package_json;
    }

    // Find git repository.
    let repo = Repository::discover(".").expect("Unable to discover repository");

    // Construct and return the path to the package.json file in git repo root.
    repo.path()
        .parent()
        .expect("Unable to get parent directory")
        .join(package_json)
        .to_str()
        .expect("Unable to convert path to string")
        .to_string()
}

/// Parse the package.json file and return its contents as an IndexMap.
fn parse() -> IndexMap<String, serde_json::Value> {
    let path = get_path();
    dbg!(path);

    // Read the package.json file.
    let content = fs::read(get_path()).expect("Unable to read file");

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
    fs::write(get_path(), content).expect("Unable to write file");
}

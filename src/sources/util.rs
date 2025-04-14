use git2::Repository;
use indexmap::IndexMap;
use std::fs;

/// Get the path to the `source` file. Looks first in the current directory, then in the git root.
///
/// * `source`: The source name.
fn get_path(source: &str) -> String {
    // If the file exists, return its path.
    if fs::exists(source).is_ok_and(|exists| exists) {
        return source.to_string();
    }

    // Find git repository.
    let repo = Repository::discover(".").expect("Unable to discover repository");

    // Construct and return the path to the package.json file in git repo root.
    repo.path()
        .parent()
        .expect("Unable to get parent directory")
        .join(source)
        .to_str()
        .expect("Unable to convert path to string")
        .to_string()
}

/// Parse the source file and return its contents as an IndexMap.
pub fn parse_source(source: &str) -> IndexMap<String, toml::Value> {
    // Read the cargo.toml file.
    let path = get_path(source);
    let content = std::fs::read_to_string(path).expect("Unable to read file");

    // Parse and return cargo.toml.
    toml::from_str(&content).expect("Unable to parse TOML")
}

/// Write the source file with the given content.
///
/// * `source`: The source name.
/// * `content`: The content to write to the source file.
pub fn write_source(source: &str, content: &str) {
    let path = get_path(source);
    std::fs::write(path, content).expect("Unable to write file");
}

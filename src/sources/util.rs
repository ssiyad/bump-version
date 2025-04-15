use crate::error::BumpVersionError;
use colored::Colorize;
use git2::Repository;
use indexmap::IndexMap;
use log::debug;
use std::fs;

/// Get the path to the `source` file. Looks first in the current directory, then in the git root.
///
/// * `source`: The source name.
fn get_path(source: &str) -> Result<String, BumpVersionError> {
    // If the file exists, return its path.
    if fs::exists(source).is_ok_and(|exists| exists) {
        return Ok(source.to_string());
    }

    // Find git repository.
    let repo = Repository::discover(".")?;

    // Construct and return the path to the package.json file in git repo root.
    let path = repo
        .path()
        .parent()
        .ok_or(BumpVersionError::Other("Unable to get parent directory"))?
        .join(source)
        .to_str()
        .ok_or(BumpVersionError::Other("Unable to convert path to string"))?
        .to_string();

    Ok(path)
}

/// Parse the source file and return its contents as an IndexMap.
pub fn parse_source(source: &str) -> Result<IndexMap<String, toml::Value>, BumpVersionError> {
    // Read the source.
    let path = get_path(source)?;
    let content = std::fs::read_to_string(path)?;

    // Parse and return source.
    let parsed = match source.split(".").last() {
        Some("toml") => toml::from_str(&content).map_err(BumpVersionError::from),
        Some("json") => serde_json::from_str(&content).map_err(BumpVersionError::from),
        _ => Err(BumpVersionError::Other("Unsupported file type")),
    };

    debug!("Parsed source: {}", source.yellow());
    parsed
}

/// Write the source file with the given content.
///
/// * `source`: The source name.
/// * `content`: The content to write to the source file.
pub fn write_source(source: &str, content: &str) -> Result<(), BumpVersionError> {
    let path = get_path(source)?;
    std::fs::write(path, content)?;
    debug!("Wrote source: {}", source.yellow());
    Ok(())
}

use crate::error::BumpVersionError;
use crate::version::Version;
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
// Note! Get rid of `toml::Value` as type.
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

/// Get the version from the source file.
///
/// * `source`: The source name.
/// * `keys`: The key to the version in the source file.
pub fn get_version<T>(
    source: IndexMap<String, toml::Value>,
    keys: T,
) -> Result<Version, BumpVersionError>
where
    T: IntoIterator<Item = String>,
{
    // Convert to an iterator.
    let mut iter = keys.into_iter();

    // Get the initial key.
    let initial_key = iter
        .next()
        .ok_or(BumpVersionError::Other("No keys provided"))?;

    // Get initial section.
    let mut section = source.get(&initial_key);

    // Iterate through the keys and get the section.
    for key in iter {
        section = section.and_then(|s| s.get(key.as_str()));
    }

    // Get the version string.
    let version_str = section
        .ok_or(BumpVersionError::Other("Section not found"))?
        .as_str()
        .ok_or(BumpVersionError::Other("Version is not a string"))?;

    // Parse the version string.
    let version = Version::from(version_str);
    debug!("Got version: {}", version.to_string().yellow());

    // Return the parsed version.
    Ok(version)
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

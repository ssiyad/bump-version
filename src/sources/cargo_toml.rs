use super::{get_version_from_source, parse_source, write_source};
use crate::{error::BumpVersionError, version::Version};

pub fn get_version(file_path: &str) -> Result<Version, BumpVersionError> {
    let source = parse_source(file_path)?;
    get_version_from_source(source, vec!["package".to_string(), "version".to_string()])
}

pub fn update_version(file_path: &str, version: &Version) -> Result<(), BumpVersionError> {
    // Get cargo.toml.
    let mut source = parse_source(file_path)?;

    // Upsert the version string in the cargo.toml.
    source
        .entry("package".to_string())
        .or_insert_with(|| toml::Value::Table(toml::Table::new()))
        .as_table_mut()
        .ok_or(BumpVersionError::Other("Package section is not a table"))?
        .insert("version".to_string(), version.to_string().into());

    // Serialize the cargo.toml.
    let content = toml::to_string_pretty(&source)?;

    // Write the updated cargo.toml back to the file.
    write_source(file_path, &content)?;

    Ok(())
}

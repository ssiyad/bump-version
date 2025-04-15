use super::{parse_source, write_source};
use crate::{error::BumpVersionError, version::Version};

const CARGO_TOML: &str = "Cargo.toml";

pub fn get_version() -> Result<Version, BumpVersionError> {
    // Get cargo.toml.
    let source = parse_source(CARGO_TOML)?;

    // Get the version string from the cargo.toml.
    let version_str = source
        .get("package")
        .ok_or(BumpVersionError::Other("Package section not found"))?
        .get("version")
        .ok_or(BumpVersionError::Other(
            "Version not found in package section",
        ))?
        .as_str()
        .ok_or(BumpVersionError::Other("Version is not a string"))?;

    // Convert the version string to a Version struct.
    Ok(Version::from(version_str))
}

pub fn update_version(version: &Version) -> Result<(), BumpVersionError> {
    // Get cargo.toml.
    let mut source = parse_source(CARGO_TOML)?;

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
    write_source(CARGO_TOML, &content)?;

    Ok(())
}

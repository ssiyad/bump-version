use super::{parse_source, write_source};
use crate::version::Version;

const CARGO_TOML: &str = "Cargo.toml";

pub fn get_version() -> Version {
    // Get cargo.toml.
    let source = parse_source(CARGO_TOML);

    // Get the version string from the cargo.toml.
    let version_str = source
        .get("package")
        .expect("Package not found in cargo.toml")
        .get("version")
        .expect("Version not found in package section")
        .as_str()
        .expect("Version is not a string");

    // Convert the version string to a Version struct.
    Version::from(version_str)
}

pub fn update_version(version: &Version) {
    // Get cargo.toml.
    let mut source = parse_source(CARGO_TOML);

    // Upsert the version string in the cargo.toml.
    source
        .entry("package".to_string())
        .or_insert_with(|| toml::Value::Table(toml::Table::new()))
        .as_table_mut()
        .expect("Package section is not a table")
        .insert("version".to_string(), version.to_string().into());

    // Serialize the cargo.toml.
    let content = toml::to_string_pretty(&source).expect("Unable to serialize TOML");

    // Write the updated cargo.toml back to the file.
    write_source(CARGO_TOML, &content);
}

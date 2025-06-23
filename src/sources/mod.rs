pub mod cargo_toml;
pub mod package_json;
mod util;

pub use util::get_version_from_source;
pub use util::parse_source;
pub use util::write_source;

use crate::{error::BumpVersionError, version::Version};

/// Get the version from the specified file
/// Get the version from the specified file.
pub fn get_version(file_path: &str, file_type: &str) -> Result<Version, BumpVersionError> {
    match file_type {
        "package.json" => package_json::get_version(file_path),
        "cargo.toml" => cargo_toml::get_version(file_path),
        _ => Err(BumpVersionError::UnsupportedFileType(file_type.to_string())),
    }
}

/// Update the version in the specified file
/// Update the version in the specified file.
pub fn update_version(file_path: &str, file_type: &str, version: &Version) -> Result<(), BumpVersionError> {
    match file_type {
        "package.json" => package_json::update_version(file_path, version),
        "cargo.toml" => cargo_toml::update_version(file_path, version),
        _ => Err(BumpVersionError::UnsupportedFileType(file_type.to_string())),
    }
}
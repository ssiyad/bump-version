// Source
// This module defines the `Source` struct, which is used to represent a source of data.
// Roles:
// - `Source`: Represents a source of data.
// - `get_version`: Retrieves the version from the source.
// - `parse_source`: Parses the source file.
// - `write_source`: Writes the source file.
// - `default_entry`: Provides a default entry for the source.
// - `get_path`: Retrieves the path to the source file.
// - `get_source`: Retrieves the source file.

use crate::{error::BumpVersionError, version::Version};
use colored::Colorize;
use git2::Repository;
use indexmap::IndexMap;
use log::debug;
use serde::de::{self, DeserializeOwned};
use serde_json::from_str;
use std::fs;

#[derive(Clone, Debug)]
enum Value<T> {
    Node(IndexMap<String, Value<T>>),
    Leaf(T),
}

#[derive(Debug)]
struct Source {
    name: String,
    keys: Vec<String>,
}

impl Source {
    /// Creates a new `Source` instance.
    fn new<T>(name: T, keys: Vec<String>) -> Self
    where
        T: ToString,
    {
        Source {
            // Convert the name to a string.
            name: name.to_string(),
            keys,
        }
    }

    fn get_path(&self) -> Result<String, BumpVersionError> {
        if fs::exists(&self.name).is_ok_and(|exists| exists) {
            // If the file exists, return its path.
            debug!("Found source at: {}", self.name.yellow());
            return Ok(self.name.clone());
        };

        Err(BumpVersionError::SourceNotFound)
    }

    /// Get the path to the source file, from Git root.
    fn get_path_from_git(&self) -> Result<String, BumpVersionError> {
        // Construct the path to the package.json file in git repo root.
        let path = Repository::discover(".")?
            .path()
            .parent()
            .ok_or(BumpVersionError::Other("Unable to get parent directory"))?
            .join(&self.name)
            .to_str()
            .ok_or(BumpVersionError::Other("Unable to convert path to string"))?
            .to_string();

        debug!("Found source at: {}", path.yellow());
        Ok(path)
    }

    /// Get the extension of the source file.
    fn extension(&self) -> &str {
        self.name.split('.').next_back().unwrap_or("toml")
    }

    /// Read the source file and return its contents as a string.
    ///
    /// * `path`: The path to the source file.
    fn read(&self, path: &str) -> Result<String, BumpVersionError> {
        // let path = self.get_path()?;
        let content = fs::read_to_string(path)?;
        debug!("Read source: {}", self.name.yellow());
        Ok(content)
    }

    /// Parse the source file and return its contents as an IndexMap.
    ///
    /// * `content`: The content of the source file.
    fn parse<T>(&self, content: &str) -> Result<IndexMap<String, T>, BumpVersionError>
    where
        T: de::DeserializeOwned + ToString,
    {
        let result = match self.extension() {
            "toml" => toml::from_str(content).map_err(|_| BumpVersionError::SourceNotFound)?,
            "json" => from_str(content).map_err(|_| BumpVersionError::SourceNotFound)?,
            _ => todo!(), // Handle unsupported file types.
        };

        debug!("Parsed source: {}", self.name.yellow());
        Ok(result)
    }

    fn insert_nested<T>(
        mut content: IndexMap<String, Value<T>>,
        keys: Vec<String>,
        value: T,
    ) -> IndexMap<String, Value<T>>
    where
        T: Clone,
    {
        content
    }

    /// Get version from the source.
    ///
    /// * `content`: The content of the source.
    // Todo! // Nested keys.
    // Todo! // Move entry logic to separate method.
    fn version<T>(&self, mut content: IndexMap<String, T>) -> Result<Version, BumpVersionError>
    where
        T: de::DeserializeOwned + ToString,
    {
        let key = self.keys.get(1).unwrap();
        let entry = content.entry(key.to_string());
        // let x = entry.or_insert_with(|| IndexMap::new().into());
        let version = content.get(key).unwrap();
        Ok(Version::from(version.to_string().as_str()))
    }

    fn set_version<T>(&self, version: Version) -> T
    where
        T: DeserializeOwned + ToString,
    {
        unimplemented!()
    }
}

use serde::{Deserialize, Serialize};
use std::fs;

const PACKAGE_JSON: &str = "package.json";

#[derive(Serialize, Deserialize, Debug)]
pub struct PackageJson {
    pub version: String,
}

pub fn get() -> Vec<u8> {
    fs::read(PACKAGE_JSON).expect("Unable to read file")
}

pub fn parse(content: &[u8]) -> PackageJson {
    serde_json::from_slice(content).expect("Unable to parse JSON")
}

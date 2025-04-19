use colored::Colorize;
use log::{error, info};
use std::process;

#[derive(Debug)]
pub struct Version {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
}

impl Version {
    /// Bumps the version number based on the specified bump type.
    ///
    /// * `bump_type`: The type of bump to perform ("major", "minor", or "patch").
    pub fn bump(&self, bump_type: &str) -> Version {
        let version = match bump_type {
            "major" => Version {
                major: self.major + 1,
                minor: 0,
                patch: 0,
            },
            "minor" => Version {
                major: self.major,
                minor: self.minor + 1,
                patch: 0,
            },
            "patch" => Version {
                major: self.major,
                minor: self.minor,
                patch: self.patch + 1,
            },
            _ => {
                error!("Invalid bump type: {}", bump_type);
                process::exit(1);
            }
        };

        info!(
            "Bumped version from {} to {}",
            self.to_string().yellow(),
            version.to_string().yellow()
        );
        version
    }
}

impl From<&str> for Version {
    fn from(version_str: &str) -> Self {
        let parts: Vec<&str> = version_str.split('.').collect();

        // Check if the version string has exactly 3 parts.
        if parts.len() != 3 {
            panic!("Invalid version string");
        }

        Version {
            major: parts[0].parse().unwrap(),
            minor: parts[1].parse().unwrap(),
            patch: parts[2].parse().unwrap(),
        }
    }
}

impl std::fmt::Display for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_str() {
        let version = Version::from("1.2.3");
        assert_eq!(version.major, 1);
        assert_eq!(version.minor, 2);
        assert_eq!(version.patch, 3);
    }

    #[test]
    fn test_display() {
        let version = Version::from("1.2.3");
        assert_eq!(version.to_string(), "1.2.3");
    }

    #[should_panic(expected = "Invalid version string")]
    #[test]
    fn test_invalid_version() {
        let _ = Version::from("1.2");
    }

    #[test]
    fn test_bump_major() {
        let version = Version {
            major: 1,
            minor: 2,
            patch: 3,
        };
        let bumped = version.bump("major");
        assert_eq!(bumped.major, 2);
        assert_eq!(bumped.minor, 0);
        assert_eq!(bumped.patch, 0);
    }

    #[test]
    fn test_bump_minor() {
        let version = Version {
            major: 1,
            minor: 2,
            patch: 3,
        };
        let bumped = version.bump("minor");
        assert_eq!(bumped.major, 1);
        assert_eq!(bumped.minor, 3);
        assert_eq!(bumped.patch, 0);
    }

    #[test]
    fn test_bump_patch() {
        let version = Version {
            major: 1,
            minor: 2,
            patch: 3,
        };
        let bumped = version.bump("patch");
        assert_eq!(bumped.major, 1);
        assert_eq!(bumped.minor, 2);
        assert_eq!(bumped.patch, 4);
    }
}

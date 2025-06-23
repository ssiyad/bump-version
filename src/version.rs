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
    /// Parse a version string into a Version struct
    pub fn parse_version(version_str: &str) -> Result<Version, String> {
        let parts: Vec<&str> = version_str.split('.').collect();

        // Check if the version string has exactly 3 parts
        // Check if the version string has exactly 3 parts.
        if parts.len() != 3 {
            return Err(format!("Invalid version string '{}': must have format major.minor.patch", version_str));
        }

        let major = parts[0].parse::<u32>()
            .map_err(|_| format!("Invalid major version '{}': must be a number", parts[0]))?;
        let minor = parts[1].parse::<u32>()
            .map_err(|_| format!("Invalid minor version '{}': must be a number", parts[1]))?;
        let patch = parts[2].parse::<u32>()
            .map_err(|_| format!("Invalid patch version '{}': must be a number", parts[2]))?;

        Ok(Version { major, minor, patch })
    }

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
        // Use try_from but panic on error to maintain backward compatibility
        // Use try_from but panic on error to maintain backward compatibility.
        match Version::parse_version(version_str) {
            Ok(version) => version,
            Err(e) => {
                panic!("{}", e);
            }
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
    
    #[test]
    fn test_parse_version_valid() {
        let result = Version::parse_version("1.2.3");
        assert!(result.is_ok());
        let version = result.unwrap();
        assert_eq!(version.major, 1);
        assert_eq!(version.minor, 2);
        assert_eq!(version.patch, 3);
    }
    
    #[test]
    fn test_parse_version_invalid_format() {
        let result = Version::parse_version("1.2");
        assert!(result.is_err());
    }
    
    #[test]
    fn test_parse_version_invalid_numbers() {
        let result = Version::parse_version("1.a.3");
        assert!(result.is_err());
    }
}

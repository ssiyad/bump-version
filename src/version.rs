pub struct Version {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
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
}

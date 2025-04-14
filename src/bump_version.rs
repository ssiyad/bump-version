use super::version::Version;

pub fn bump(version: Version, bump_type: &str) -> Version {
    match bump_type {
        "major" => Version {
            major: version.major + 1,
            minor: 0,
            patch: 0,
        },
        "minor" => Version {
            major: version.major,
            minor: version.minor + 1,
            patch: 0,
        },
        "patch" => Version {
            major: version.major,
            minor: version.minor,
            patch: version.patch + 1,
        },
        _ => panic!("Invalid bump type"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bump_major() {
        let version = Version {
            major: 1,
            minor: 2,
            patch: 3,
        };
        let bumped = bump(version, "major");
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
        let bumped = bump(version, "minor");
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
        let bumped = bump(version, "patch");
        assert_eq!(bumped.major, 1);
        assert_eq!(bumped.minor, 2);
        assert_eq!(bumped.patch, 4);
    }
}

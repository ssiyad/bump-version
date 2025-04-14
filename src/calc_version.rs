use crate::version::Version;

pub fn parse(version: &str) -> Version {
    let mut version_parts = version.split('.').collect::<Vec<_>>();
    if version_parts.len() < 3 {
        panic!("Version string must have at least three parts");
    }
    let major = version_parts.remove(0).parse::<u32>().unwrap();
    let minor = version_parts.remove(0).parse::<u32>().unwrap();
    let patch = version_parts.remove(0).parse::<u32>().unwrap();

    Version {
        major,
        minor,
        patch,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_1_2_3() {
        let version = parse("1.2.3");
        assert_eq!(version.major, 1);
        assert_eq!(version.minor, 2);
        assert_eq!(version.patch, 3);
    }

    #[test]
    fn test_parse_4_5_6() {
        let version = parse("4.5.6");
        assert_eq!(version.major, 4);
        assert_eq!(version.minor, 5);
        assert_eq!(version.patch, 6);
    }

    #[test]
    #[should_panic(expected = "Version string must have at least three parts")]
    fn test_parse_invalid() {
        parse("1.2");
    }
}

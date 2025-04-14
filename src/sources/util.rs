use git2::Repository;
use std::fs;

/// Get the path to the `source` file. Looks first in the current directory, then in the git root.
///
/// * `source`: The source name.
pub fn get_path(source: &str) -> String {
    // If the file exists, return its path.
    if fs::exists(source).is_ok_and(|exists| exists) {
        return source.to_string();
    }

    // Find git repository.
    let repo = Repository::discover(".").expect("Unable to discover repository");

    // Construct and return the path to the package.json file in git repo root.
    repo.path()
        .parent()
        .expect("Unable to get parent directory")
        .join(source)
        .to_str()
        .expect("Unable to convert path to string")
        .to_string()
}

use crate::version::Version;
use git2::{IndexAddOption, Repository};

/// Commit the version change to the git repository.
pub fn commit(old_version: &Version, new_version: &Version) {
    // Get the repository.
    let repo = Repository::discover(".").expect("Unable to discover repository");

    // Get git index;
    let mut index = repo.index().expect("Unable to get git index");

    // Stage all changes.
    index
        .add_all(["*"].iter(), IndexAddOption::DEFAULT, None)
        .expect("Unable to stage changes");

    // Confirm stage.
    index.write().expect("Unable to write index");

    // Construct the commit message.
    let message = format!(
        "chore: bump version from {} to {}",
        old_version, new_version,
    );

    // Get the head commit.
    let head = repo.head().expect("Unable to get git head");

    // Get the parent commit.
    let parent = head.peel_to_commit().expect("Unable to peel to commit");

    // Get the oid of the root tree.
    let oid = index.write_tree().expect("Unable to write tree");

    // Get the signature.
    let signature = repo.signature().expect("Unable to get git signature");

    // Get the tree from the index.
    let tree = repo.find_tree(oid).expect("Unable to find tree");

    // Commit with message.
    repo.commit(
        Some("HEAD"),
        &signature,
        &signature,
        &message,
        &tree,
        &[&parent],
    )
    .expect("Unable to commit changes");
}

/// Create a tag for the new version in the git repository.
///
/// * `new_version`: The new version to tag.
pub fn tag(new_version: &Version) {
    // Get the repository.
    let repo = Repository::discover(".").expect("Unable to discover repository");

    // Get the head commit.
    let target = repo.revparse_single("HEAD").expect("Unable to get HEAD");

    // Get signature.
    let signature = repo.signature().expect("Unable to get git signature");

    // Get tag.
    let tag = format!("v{}", new_version);

    // Create the tag.
    repo.tag(&tag, &target, &signature, "", false)
        .expect("Unable to create tag");
}

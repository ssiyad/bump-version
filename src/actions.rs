use crate::{error::BumpVersionError, version::Version};
use colored::Colorize;
use git2::{IndexAddOption, Repository};
use log::info;

/// Commit the version change to the git repository.
pub fn commit(old_version: &Version, new_version: &Version) -> Result<(), BumpVersionError> {
    // Get the repository.
    let repo = Repository::discover(".")?;

    // Get git index;
    let mut index = repo.index()?;

    // Stage all changes.
    index.add_all(["*"].iter(), IndexAddOption::DEFAULT, None)?;

    // Confirm stage.
    index.write()?;
    info!("Staged changes");

    // Construct the commit message.
    let message = format!(
        "chore: bump version from {} to {}",
        old_version, new_version,
    );

    // Get the head commit.
    let head = repo.head()?;

    // Get the parent commit.
    let parent = head.peel_to_commit()?;

    // Get the oid of the root tree.
    let oid = index.write_tree()?;

    // Get the signature.
    let signature = repo.signature()?;

    // Get the tree from the index.
    let tree = repo.find_tree(oid)?;

    // Commit with message.
    let commit = repo.commit(
        Some("HEAD"),
        &signature,
        &signature,
        &message,
        &tree,
        &[&parent],
    )?;

    // Print success message.
    info!("Created commit: {}", commit.to_string().yellow());

    Ok(())
}

/// Create a tag for the new version in the git repository.
///
/// * `new_version`: The new version to tag.
pub fn tag(new_version: &Version) -> Result<(), BumpVersionError> {
    // Get the repository.
    let repo = Repository::discover(".")?;

    // Get the head commit.
    let target = repo.revparse_single("HEAD")?;

    // Get signature.
    let signature = repo.signature()?;

    // Get tag.
    let tag = format!("v{}", new_version);

    // Create the tag.
    repo.tag(&tag, &target, &signature, "", false)?;

    // Print success message.
    info!("Created tag: {}", tag.yellow());

    Ok(())
}

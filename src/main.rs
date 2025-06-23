use log::{LevelFilter, error};

mod actions;
mod config;
mod error;
mod sources;
mod version;

fn main() {
    // Logger.
    env_logger::builder()
        .filter_level(LevelFilter::Trace)
        .format_timestamp(None)
        .format_target(false)
        .init();

    // Get configuration options.
    let options = config::get();

    // Run and handle errors, if any.
    if let Err(err) = run(
        options.file.as_deref(),
        &options.bump_type,
        options.dry_run,
        options.commit,
        options.tag,
    ) {
        error!("{}", err);
    };
}

fn run(
    file_path: Option<&str>,
    bump_type: &str,
    dry_run: bool,
    commit: bool,
    tag: bool,
) -> Result<(), error::BumpVersionError> {
    if file_path.is_none() {
        return Err(error::BumpVersionError::SourceNotSpecified);
    }

    let file_path = file_path.unwrap();

    // Auto-detect file type
    // Auto-detect file type.
    let file_type = if file_path.to_lowercase().ends_with("package.json") {
        "package.json"
    } else if file_path.to_lowercase().ends_with("cargo.toml") {
        "cargo.toml"
    } else {
        return Err(error::BumpVersionError::UnsupportedFileType(
            file_path.to_string(),
        ));
    };

    // Get the current version and bump it
    // Get the current version and bump it.
    let current = sources::get_version(file_path, file_type)?;
    let bumped = current.bump(bump_type);

    if !dry_run {
        sources::update_version(file_path, file_type, &bumped)?;
    }

    if commit && !dry_run {
        actions::commit(&current, &bumped, None)?;
    }

    if tag && !dry_run {
        actions::tag(&bumped, None)?;
    }

    Ok(())
}


use log::{LevelFilter, error};

mod actions;
mod config;
mod error;
mod source;
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
        &options.bump_type,
        options.package_json,
        options.cargo_toml,
        options.dry_run,
        options.no_commit,
        options.no_tag,
    ) {
        error!("{}", err);
    };
}

fn run(
    bump_type: &str,
    package_json: bool,
    cargo_toml: bool,
    dry_run: bool,
    no_commit: bool,
    no_tag: bool,
) -> Result<(), error::BumpVersionError> {
    if !package_json && !cargo_toml {
        return Err(error::BumpVersionError::SourceNotSpecified);
    }

    if package_json {
        let current = sources::package_json::get_version()?;
        let bumped = current.bump(bump_type);

        if !dry_run {
            sources::package_json::update_version(&bumped)?;
        }

        if !no_commit && !dry_run {
            actions::commit(&current, &bumped)?;
        }

        if !no_tag && !dry_run {
            actions::tag(&bumped)?;
        }
    }

    if cargo_toml {
        let current = sources::cargo_toml::get_version()?;
        let bumped = current.bump(bump_type);

        if !dry_run {
            sources::cargo_toml::update_version(&bumped)?;
        }

        if !no_commit && !dry_run {
            actions::commit(&current, &bumped)?;
        }

        if !no_tag && !dry_run {
            actions::tag(&bumped)?;
        }
    }

    Ok(())
}

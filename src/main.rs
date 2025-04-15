use clap::{Arg, command};
use log::{LevelFilter, error};

mod actions;
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

    // Parse command line arguments.
    let matches = command!()
        .arg(
            Arg::new("bump-type")
                .value_parser(["major", "minor", "patch"])
                .default_value("patch")
                .help("Bump type"),
        )
        .get_matches();

    let bump_type = matches
        .get_one::<String>("bump-type")
        .expect("Invalid bump type");

    // Run and handle errors, if any.
    if let Err(err) = run(bump_type) {
        error!("{}", err);
    };
}

fn run(bump_type: &str) -> Result<(), error::BumpVersionError> {
    let current = sources::package_json::get_version()?;
    let bumped = current.bump(bump_type);
    sources::package_json::update_version(&bumped)?;
    let current = sources::cargo_toml::get_version()?;
    let bumped = current.bump(bump_type);
    sources::cargo_toml::update_version(&bumped)?;
    actions::commit(&current, &bumped)?;
    actions::tag(&bumped)?;

    Ok(())
}

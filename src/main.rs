use clap::{Arg, ArgAction, command};
use log::{LevelFilter, error};

mod actions;
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

    // Parse command line arguments.
    let matches = command!()
        .arg(
            Arg::new("bump-type")
                .value_parser(["major", "minor", "patch"])
                .default_value("patch")
                .help("Bump type"),
        )
        .arg(
            Arg::new("package-json")
                .long("package-json")
                .action(ArgAction::SetTrue)
                .help("Update package.json"),
        )
        .arg(
            Arg::new("cargo-toml")
                .long("cargo-toml")
                .action(ArgAction::SetTrue)
                .help("Update Cargo.toml"),
        )
        .arg(
            Arg::new("dry-run")
                .long("dry-run")
                .action(ArgAction::SetTrue)
                .help("Do not write to sources"),
        )
        .arg(
            Arg::new("no-commit")
                .long("no-commit")
                .action(ArgAction::SetTrue)
                .help("Do not commit the version change"),
        )
        .arg(
            Arg::new("no-tag")
                .long("no-tag")
                .action(ArgAction::SetTrue)
                .help("Do not add a tag"),
        )
        .get_matches();

    let package_json = matches.get_flag("package-json");
    let cargo_toml = matches.get_flag("cargo-toml");
    let dry_run = matches.get_flag("dry-run");
    let no_commit = matches.get_flag("no-commit");
    let no_tag = matches.get_flag("no-tag");

    let bump_type = matches
        .get_one::<String>("bump-type")
        .expect("Invalid bump type");

    // Run and handle errors, if any.
    if let Err(err) = run(
        bump_type,
        package_json,
        cargo_toml,
        dry_run,
        no_commit,
        no_tag,
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

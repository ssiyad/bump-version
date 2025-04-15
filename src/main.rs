use clap::{Arg, command};

mod actions;
mod sources;
mod version;

fn main() {
    let matches = command!()
        .arg(
            Arg::new("bump-type")
                .default_value("patch")
                .help("Bump type"),
        )
        .get_matches();

    let bump_type = matches.get_one::<String>("bump-type").unwrap();
    let current = sources::package_json::get_version();
    let bumped = current.bump(&bump_type);
    sources::package_json::update_version(&bumped);
    let current = sources::cargo_toml::get_version();
    let bumped = current.bump(&bump_type);
    sources::cargo_toml::update_version(&bumped);
    actions::commit(&current, &bumped);
    actions::tag(&bumped);
    println!("Bumped version: {}", bumped);
}

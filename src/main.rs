use clap::command;
use std::env;

mod actions;
mod sources;
mod version;

fn main() {
    command!().get_matches();
    let mut args = env::args().skip(1);
    let bump_type = args.next().unwrap_or("patch".to_string());
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

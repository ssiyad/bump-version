use std::env;

mod actions;
mod package_json;
mod version;

fn main() {
    let mut args = env::args().skip(1);
    let bump_type = args.next().unwrap_or("patch".to_string());
    let current = package_json::get_version();
    let bumped = current.bump(&bump_type);
    package_json::update_version(&bumped);
    actions::commit(&current, &bumped);
    println!("Bumped version: {}", bumped);
}

use std::env;

mod bump_version;
mod package_json;
mod version;

fn main() {
    let mut args = env::args().skip(1);
    let bump_type = args.next().unwrap_or("patch".to_string());
    let version = package_json::get_version();
    let bumped = bump_version::bump(version, &bump_type);
    package_json::update_version(&bumped);
    println!("Bumped version: {}", bumped);
}

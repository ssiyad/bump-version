use std::env;

mod bump_version;
mod calc_version;
mod package_json;
mod version;

fn main() {
    let mut args = env::args().skip(1);
    let bump_type = args.next().unwrap_or("patch".to_string());
    let package = package_json::parse(&package_json::get());
    let parsed = calc_version::parse(&package.version);
    let bumped = bump_version::bump(parsed, &bump_type);
    println!("Bumped version: {}", bumped);
}

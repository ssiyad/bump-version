use std::env;

mod bump_version;
mod calc_version;
mod version;

fn main() {
    let mut args = env::args().skip(1);
    let version = args.next().expect("Please provide a version string");
    let bump_type = args.next().unwrap_or("patch".to_string());
    let parsed = calc_version::parse(&version);
    let bumped = bump_version::bump(parsed, &bump_type);
    println!("Bumped version: {}", bumped);
}

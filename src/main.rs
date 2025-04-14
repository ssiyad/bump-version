use std::env;

mod calc_version;

fn main() {
    let mut args = env::args().skip(1);
    let version = args.next().expect("Please provide a version string");
    let parsed = calc_version::parse(&version);
    println!("{}", parsed);
}

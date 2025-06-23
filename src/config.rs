use clap::Parser;

#[derive(Parser, Debug)]
#[command(about = "Bump the version in package.json or Cargo.toml files")]
pub struct Options {
    /// File path to the package.json or cargo.toml file
    pub file: Option<String>,

    #[arg(long, default_value = "patch")]
    pub bump_type: String,

    #[arg(long)]
    pub dry_run: bool,

    /// Create a commit after bumping version
    #[arg(long)]
    pub commit: bool,

    /// Create a tag after bumping version
    #[arg(long)]
    pub tag: bool,
}

/// Get the configuration from the command line arguments.
pub fn get() -> Options {
    Options::parse()
}
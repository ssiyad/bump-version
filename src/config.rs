use clap::CommandFactory;
use clap::Parser;
use clap_config::ClapConfig;
use std::{
    env, fs,
    path::{Path, PathBuf},
};

#[derive(Parser, Debug, ClapConfig)]
pub struct Options {
    #[clap(long, default_value = "patch")]
    pub bump_type: String,

    #[clap(long, default_value = "false")]
    pub package_json: bool,

    #[clap(long, default_value = "false")]
    pub cargo_toml: bool,

    #[clap(long, default_value = "false")]
    pub dry_run: bool,

    #[clap(long, default_value = "false")]
    pub no_commit: bool,

    #[clap(long, default_value = "false")]
    pub no_tag: bool,
}

pub fn get() -> Options {
    let matches = <Options as CommandFactory>::command().get_matches();
    Options::from_merged(matches, config_source())
}

fn config_source() -> Option<OptionsConfig> {
    config_path().map(|path| toml::from_str(&fs::read_to_string(path).unwrap()).unwrap())
}

fn config_path() -> Option<PathBuf> {
    let xdg_config = env::var("XDG_CONFIG_HOME").ok();
    let home_config = env::var("HOME").ok();

    if let Some(xdg) = xdg_config {
        let path = Path::new(&xdg).join("bumpversion").join("config.toml");

        if path.exists() {
            return Some(path);
        }
    }

    if let Some(home) = home_config {
        let path = Path::new(&home)
            .join(".config")
            .join("bumpversion")
            .join("config.toml");

        if path.exists() {
            return Some(path);
        }
    }

    None
}

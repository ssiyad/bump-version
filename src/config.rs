use clap::CommandFactory;
use clap::Parser;
use clap_config::ClapConfig;
use git2::Repository;
use std::{
    env, fs,
    path::{Path, PathBuf},
};

const CONFIG_DIR: &str = ".config";
const CONFIG_FILE: &str = "bump-version.toml";
const CONFIG_HOME: &str = "XDG_CONFIG_HOME";
const USER_HOME: &str = "HOME";

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

    pub template_commit: Option<String>,
    pub template_tag: Option<String>,
}

/// Get the configuration from the command line arguments or the config file.
pub fn get() -> Options {
    let matches = <Options as CommandFactory>::command().get_matches();
    Options::from_merged(matches, config_source())
}

/// Get the configuration from the config file if it exists.
fn config_source() -> Option<OptionsConfig> {
    config_root().map(|path| {
        let path = path.join(CONFIG_FILE);
        toml::from_str(&fs::read_to_string(path).unwrap()).unwrap()
    })
}

/// Get the path to the home configuration directory.
fn config_root() -> Option<PathBuf> {
    // Get the current directory.
    let current_dir = env::current_dir().unwrap();

    // If the config file exists in the current directory, use that.
    if fs::exists(CONFIG_FILE).is_ok_and(|exists| exists) {
        return Some(current_dir);
    }

    // If the config file exists in the current git repository, use that.
    if let Ok(git_root) = Repository::discover(current_dir) {
        return Some(git_root.path().to_path_buf());
    }

    // If the config file exists in the home directory, use that.
    match env::var(CONFIG_HOME) {
        Ok(xdg_home) => Some(Path::new(&xdg_home).to_path_buf()),
        Err(_) => {
            let home = env::var(USER_HOME).unwrap();
            Some(Path::new(&home).join(CONFIG_DIR))
        }
    }
}

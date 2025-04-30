use clap::CommandFactory;
use clap::Parser;
use clap_config::ClapConfig;

#[derive(Parser, Debug, ClapConfig)]
pub struct Config {
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

pub fn get() -> Config {
    let config_str = "".to_string();
    let matches = <Config as CommandFactory>::command().get_matches();
    let config_: ConfigConfig = toml::from_str(&config_str).unwrap();
    Config::from_merged(matches, Some(config_))
}

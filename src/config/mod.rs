mod cli;

#[derive(Debug)]
pub struct Config {
    // Path to the project being analyzed.
    pub base_path: String,
}

pub fn parse_config() -> Config {
    let args = cli::parse_cli_options();
    return Config {
        base_path: args.base_path.unwrap_or(String::from(".")),
    }
}
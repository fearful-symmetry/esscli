use std::fs::read_to_string;

use cli::{Cli, Commands};
use clap::Parser;
use config::Config;
use ess::{ServerlessClient, Projects};
use anyhow::{Result, anyhow, Ok};

mod cli;
mod ess;
mod config;

fn main() -> Result<()>{
    let cli = Cli::parse();
    let cfg = get_config()?;

   let selected_endpoint = cfg.endpoints.get(&cli.endpoint).ok_or_else(|| anyhow!(format!("endpoint {} does not exist", cli.endpoint)))?;

    

    match &cli.command{
        cli::Types::SF(cmd) => {

        },
        cli::Types::SL(cmd) => {
            let client = ServerlessClient::new(cfg.config.key_path.into(), cfg.config.project, selected_endpoint)?;
            cmd.exec(client);
            let projects = client.list()?;
            print_projects(projects);
           

        }
    };

    Ok(())
}

fn get_config() -> Result<Config> {
    let mut home = dirs::home_dir().ok_or_else(|| anyhow!("could not find home dir"))?;
    home.push(".config/ess/esscli.toml");

    let cfg_raw = read_to_string(home)?;
    let cfg: Config = toml::from_str(&cfg_raw)?;
    Ok(cfg)
}

fn print_projects(proj: Projects) {
    for item in proj.items{
        println!("{:#?}", item);
    }
}

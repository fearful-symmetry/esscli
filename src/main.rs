use cli::{Cli, ServerlessCmd, StatefulCmd, RawReq};
use clap::Parser;
use clients::{client::{ResultFormatting, ESSClient}, serverless::{CreateProject, ProjectOverrides, ApplicationOverride}, create_deployment::DeploymentCreateRequest};
use config::{get_config, DeploymentSpecificConfig};
use anyhow::{Result, Ok, anyhow, Context};
use serde::Serialize;
use url::Url;

mod cli;
mod clients;
mod config;

fn main() -> Result<()>{
    let cli = Cli::parse();
    let cfg = get_config()?;
    let client = clients::client::ESSClient::new(cfg.clone()).context("error reading config file")?;

    let lvl = match cli.verbose {
        false => log::Level::Info ,
        true => log::Level::Debug
        
    };
    simple_logger::init_with_level(lvl).context("error creating logger")?;
   
    match &cli.command{
        cli::Types::Setup{path} => {
            config::create_config(path)?;
        },
        cli::Types::SF(cmd) => {
            match cmd{
                StatefulCmd::List => {
                    let res = client.stateful()?.list().context("error listing projects")?;
                    print_generic_struct(cli.out, &res)?;
                }, 
                StatefulCmd::Get{id} => {
                    let res = client.stateful()?.get(id).context("error fetching project")?;
                    print_generic_struct(cli.out, &res)?;
                },
                StatefulCmd::Raw(raw) => {
                    let parsed = cfg.resolve_stateful();
                    let res = handle_raw_request(parsed, raw, client).context("error performing raw HTTP request")?;
                    println!("{}", res);
                },
                StatefulCmd::Shutdown { ids } => {
                    for id in ids {
                        let res = client.stateful()?.shutdown(id).context("error shutting down")?;
                        print_generic_struct(cli.out, &res)?;
                    }

                },
                StatefulCmd::Create { name, region, version } => {
                    let req = create_deployment_request_from_cli(region.clone(), 
                    name.clone(), 
                    version.clone(), 
                    cfg.config.default_deployment).context("error creating deployment request")?;
                    let resp = client.stateful()?.create(req).context("error creating cluster")?;
                    println!("{:#?}", resp);
                }
            }

        },
        cli::Types::SL(cmd) => {
            match cmd {
                ServerlessCmd::List => {
                    let res = client.serverless()?.list().context("error listing projects")?;
                    print_generic_struct(cli.out, &res)?;
                },
                ServerlessCmd::Get{id} => {
                    let res = client.serverless()?.get(id).context("error fetching project")?;
                    print_generic_struct(cli.out, &res)?;
                },
                ServerlessCmd::ResetCreds { id } => {
                    let res = client.serverless()?.reset_credentials(id).context("error resetting credentials")?;
                    print_generic_struct(cli.out, &res)?;
                },
                ServerlessCmd::Status { id } => {
                    let res = client.serverless()?.status(id).context("error fetching status")?;
                    print_generic_struct(cli.out, &res)?;
                },
                ServerlessCmd::Delete { ids } => {
                    for id in ids {
                        client.serverless()?.delete(id).context("error performing delete request")?;
                    }
                },
                ServerlessCmd::Create { name, region, wait, reset_creds, es_docker_override, 
                    kibana_docker_override, fleet_docker_override } => {
                    // check region values
                    let serverless_cfg = cfg.resolve_serverless();
                    if serverless_cfg.region.is_none() && region.is_none() {
                        return Err(anyhow!("region value must be set in CLI or config"))
                    }
                    // default to the config value, then let cli override
                    let mut region_final = serverless_cfg.region.unwrap_or_default();
                    if let Some(reg) = region {
                        region_final = reg.to_string();
                    }

                    let mut req = CreateProject{name: name.to_string(), region_id: region_final, overrides: None};
                    if es_docker_override.is_some() || kibana_docker_override.is_some() || fleet_docker_override.is_some() {
                        let mut overrides = ProjectOverrides::default();
                        if let Some(es) = es_docker_override {
                            overrides.elasticsearch = Some(ApplicationOverride{docker_image: es.to_string()});
                        }
                        if let Some(kibana) = kibana_docker_override{
                           overrides.kibana = Some(ApplicationOverride { docker_image: kibana.to_string() });
                        }
                        if let Some(fleet) = fleet_docker_override{
                            overrides.fleet = Some(ApplicationOverride { docker_image: fleet.to_string() });
                        }
                        req.overrides = Some(overrides);
                    }
                    let res = client.serverless()?.create(req, *wait).context("error creating project")?;
                    print_generic_struct(cli.out, &res)?;
                    if *reset_creds{
                        let res = client.serverless()?.reset_credentials(&res.id).context("error resetting credentials")?;
                        print_generic_struct(cli.out, &res)?;
                    }
                },
                ServerlessCmd::Regions => {
                    let regions = client.serverless()?.regions()?;
                    // some hackery needed, since we get a list from the server, not an object
                   if cli.out == cli::OutputType::Json {
                    let formatted = serde_json::to_string_pretty(&regions)?;
                    println!("{}", formatted);
                   } else {
                    for region in regions {
                        print_generic_struct(cli.out, &region)?;
                    }
                   }

                },
                ServerlessCmd::Raw(raw) => {
                    let parsed = cfg.resolve_serverless();
                    let res = handle_raw_request(parsed, raw, client).context("error performing raw HTTP request")?;
                    println!("{}", res);
                }
            }
        }
    };

    Ok(())
}

fn handle_raw_request(cfg: DeploymentSpecificConfig, raw: &RawReq, client: ESSClient) -> Result<String> {
    let endpoint = Url::parse(&cfg.url)?;
    let res = match raw {
        cli::RawReq::Get { path } => {
            client.get(&endpoint, path)?
            
        },
        cli::RawReq::Post { path, body }=> {
            client.post(&endpoint, path, body.clone())?
        }
    };
    let formatted = jsonxf::pretty_print(&res).map_err(|v|anyhow!("{}", v))?;
    Ok(formatted)
}

fn print_generic_struct<P>(form: cli::OutputType, item: &P) -> Result<()> 
where P: Serialize + std::fmt::Debug + ResultFormatting
{
    match form {
        cli::OutputType::Compact => {
            println!("{}", item.compact());
        },
        cli::OutputType::Json => {
            let res = serde_json::to_string_pretty(&item)?;
            println!("{}", res);
        },
        cli::OutputType::Struct => {
            println!("{:#?}", item);
        }
    }
    Ok(())
}

/// helpful wrapper to take a few basic cli commands and use it to format the massive DeploymentCreateRequest
/// note: this will not successfully create a deployment
pub fn create_deployment_request_from_cli(region: Option<String>, 
    name: Option<String>, 
    version: Option<String>, 
    dep_path: String) -> Result<DeploymentCreateRequest> {
    // fetch default file
    let expanded = shellexpand::tilde(&dep_path).to_string();
    let raw_dep = std::fs::read_to_string(&expanded).context(format!("error reading request template from {}", expanded))?;
    let mut rendered: DeploymentCreateRequest = serde_json::from_str(&raw_dep).context("error reading JSON deployment template")?;

    // set user variables
    rendered.name = name;
    rendered.region = region;
    rendered.version = version;

    Ok(rendered)
}
use clap::{Parser, Subcommand, ValueEnum};


#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// enable debug-level logging
    #[clap(long, short, default_value_t=false)]
    pub verbose: bool,
    /// the format to print the output in
    #[clap(value_enum, long, short, default_value_t=OutputType::Struct)]
    pub out: OutputType,
    #[command(subcommand)]
    pub command: Types,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum OutputType {
    Compact,
    Struct,
    Json
}

#[derive(Subcommand)]
pub enum Types{
    /// Create a basic config file and default deployment request template at the specified location
    Setup{
        #[clap(long, short, default_value_t=String::from("~/.config/ess/"))]
        path: String
    },
    /// Run a command against a serverless deployment or configuration
    #[command(subcommand)]
    SL(ServerlessCmd),
    /// Run a command against a traditional stateful deployment or configuration
    #[command(subcommand)]
    SF(StatefulCmd)
}

#[derive(Subcommand)]
pub enum ServerlessCmd{
    /// List all deployments
    List,
    /// Get a deployment by ID
    Get{id: String},
    /// Reset the serverless credentials for a serverless instance
    ResetCreds{id: String},
    /// Get the status of a deployment
    Status{id: String},
    /// Send a raw request to ESS
    #[command(subcommand)]
    Raw(RawReq),    
    /// Delete a deployment
    Delete{ids: Vec<String>},
    /// Create a deployment
    Create{
        /// Name of the project
        name: String,
        /// Region of the project. If not specified, will default to the value specified in the config file.
        region: Option<String>,
        /// Wait for the project to become available before returning
        #[clap(long, short, default_value_t=false)]
        wait: bool,
        /// Reset the credentials after creating. As of 8/23, this
        /// is needed to fully create auth for the project.
        #[clap(long, short, default_value_t=true)]
        reset_creds: bool,
        /// Override elasticsearch image. Only usable interally.
        #[clap(long)]
        es_docker_override: Option<String>,
        /// Override kibana image. Only usable internally.
        #[clap(long)]
        kibana_docker_override: Option<String>,
        /// Override fleet image. Only usable internally.
        #[clap(long)]
        fleet_docker_override: Option<String>

    },
    /// List all available regions
    Regions
}

#[derive(Subcommand)]
pub enum StatefulCmd {
    /// list all deployments
    List,
    /// Get a deployment by ID
    Get{id: String},
    /// Shutdown a deployment
    Shutdown{ids: Vec<String>},
    /// Send a raw GET/POST request to ESS
    #[command(subcommand)]
    Raw(RawReq),
    /// Create a new cluster from esscli's template, overriding any values written into the template.
    Create{
        #[clap(long, short)]
        name: Option<String>,
        #[clap(long, short)]
        region: Option<String>,
        #[clap(long, short)]
        version: Option<String>
    }
}

#[derive(Subcommand)]
pub enum RawReq {
    /// Send a raw GET request
    Get{path: String},
    /// Send a raw POST request
    Post{path: String, body: Option<String>}
}
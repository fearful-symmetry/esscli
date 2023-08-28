use clap::{Parser, Subcommand};

use crate::ess::EssHandler;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[arg(default_value_t = String::from("default"))]
    pub endpoint: String,
    #[command(subcommand)]
    pub command: Types,
}

#[derive(Subcommand)]
pub enum Types{
    #[command(subcommand)]
    SL(Commands),
    #[command(subcommand)]
    SF(Commands)
}

#[derive(Subcommand)]
pub enum Commands{
    List
}

impl Commands {
    pub fn exec<T>(&self, client: T)
    where 
    T: EssHandler {

    }
}
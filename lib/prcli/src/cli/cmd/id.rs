use clap::{Parser, FromArgMatches, Arg, ArgAction, ArgMatches, Subcommand, Args};
use strum::Display;

#[derive(Default, Parser, Debug)]
pub struct IdCmd {
    #[clap(subcommand)]
    pub subcmd: Option<IdSubCmd>,
}

#[derive(Default, Display, Subcommand, Debug)]
#[clap(name = "id", about = "Manage ids")]
pub enum IdSubCmd {
    /// List all ids    
    #[clap(name = "list")]
    List,
    /// Add an id
    #[clap(name = "add")]
    Add,
    /// Add an id
    #[clap(name = "remove")]
    Remove,
    /// Add an id
    #[clap(name = "update")]
    Update,
    /// Add an id
    #[clap(name = "show")]
    Show,
    /// Get Help
    #[clap(name = "id-help")]
    #[default]
    Help,
}

impl super::Subcmd for IdSubCmd {
    type Parent = IdCmd;
}
impl super::super::Cmd for IdCmd {
    type Sub = IdSubCmd;
}
use clap::{Parser, FromArgMatches, Arg, ArgAction, ArgMatches, Subcommand, Args};
use strum::Display;

#[derive(Default, Parser, Debug)]
pub struct HelpCmd {
    #[clap(subcommand)]
    pub subcmd: Option<HelpSubCmd>,
}

#[derive(Default, Display, Subcommand, Debug)]
#[clap(name = "Help", about = "Manage Helps")]
pub enum HelpSubCmd {
    /// List all Helps    
    #[clap(name = "list")]
    List,
    /// Add an Help
    #[clap(name = "add")]
    Add,
    /// Add an Help
    #[clap(name = "remove")]
    Remove,
    /// Add an Help
    #[clap(name = "update")]
    Update,
    /// Add an Help
    #[clap(name = "show")]
    Show,
    /// Get Help
    #[clap(name = "help-help")]
    #[default]
    Help,
}

impl super::Subcmd for HelpSubCmd {
    type Parent = HelpCmd;
}
impl super::super::Cmd for HelpCmd {
    type Sub = HelpSubCmd;
}
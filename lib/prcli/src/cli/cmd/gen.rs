use clap::{Parser, FromArgMatches, Arg, ArgAction, ArgMatches, Subcommand, Args};
use strum::Display;

#[derive(Default, Parser, Debug)]
pub struct GenCmd {
    #[clap(subcommand)]
    pub subcmd: Option<GenSubCmd>,
}

#[derive(Default, Display, Subcommand, Debug)]
#[clap(name = "Gen", about = "Manage Gens")]
pub enum GenSubCmd {
    /// List all Gens    
    #[clap(name = "list")]
    List,
    /// Add an Gen
    #[clap(name = "add")]
    Add,
    /// Add an Gen
    #[clap(name = "remove")]
    Remove,
    /// Add an Gen
    #[clap(name = "update")]
    Update,
    /// Add an Gen
    #[clap(name = "show")]
    Show,
    /// Get Gen
    #[clap(name = "gen-help")]
    #[default]
    Help,
}

impl super::Subcmd for GenSubCmd {
    type Parent = GenCmd;
}
impl super::super::Cmd for GenCmd {
    type Sub = GenSubCmd;
}

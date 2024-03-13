use clap::{Parser, Subcommand};
use strum::Display;

#[derive(Parser, Debug, Default)]
pub struct InitCmd {
    #[clap(subcommand)]
    pub cmd: Option<InitSubCmd>,
}

#[derive(Subcommand, Debug, Display, Default)]
pub enum InitSubCmd {
    /// List all edits
    #[clap(name = "init-help")]
    #[default]
    Help,

}

impl super::Subcmd for InitSubCmd {
    type Parent = InitCmd;
}

impl super::super::Cmd for InitCmd {
    type Sub = InitSubCmd;
}
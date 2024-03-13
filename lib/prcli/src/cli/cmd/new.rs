use clap::{Parser, Subcommand};
use strum::Display;

#[derive(Parser, Debug, Default)]
pub struct NewCmd {
    #[clap(subcommand)]
    pub cmd: Option<NewSubCmd>,
}

#[derive(Subcommand, Debug, Display, Default)]
pub enum NewSubCmd {
    /// List all edits
    #[default]
    #[clap(name = "new-help")]
    Help,

}

impl super::Subcmd for NewSubCmd {
    type Parent = NewCmd;
}

impl super::super::Cmd for NewCmd {
    type Sub = NewSubCmd;
}
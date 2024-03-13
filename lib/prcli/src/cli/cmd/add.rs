use clap::{Parser, Subcommand};
use strum::Display;

#[derive(Parser, Debug, Default)]
pub struct AddCmd {

    #[clap(subcommand)]
    pub cmd: Option<AddSubCmd>,
}

#[derive(Subcommand, Debug, Display, Default)]
pub enum AddSubCmd {
    /// List all edits
    #[default]
    #[clap(name = "add-help")]
    Help,

}

impl super::Subcmd for AddSubCmd {
    type Parent = AddCmd;
}

impl super::super::Cmd for AddCmd {
    type Sub = AddSubCmd;
}
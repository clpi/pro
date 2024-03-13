use clap::{Parser, Subcommand};
use strum::Display;

#[derive(Parser, Debug, Default)]
pub struct EditCmd {
    #[clap(subcommand)]
    pub cmd: Option<EditSubCmd>,

}

#[derive(Subcommand, Debug, Display, Default)]
pub enum EditSubCmd {
    /// List all edits
    #[clap(name = "edit-help")]
    #[default]
    Help,

}

impl super::Subcmd for EditSubCmd {
    type Parent = EditCmd;
}

impl super::super::Cmd for EditCmd {
    type Sub = EditSubCmd;
}
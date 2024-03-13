use clap::{Parser, Subcommand};
use strum::Display;

#[derive(Parser, Debug, Default)]
pub struct ShCmd {
    #[clap(subcommand)]
    pub cmd: Option<ShSubCmd>,
}

#[derive(Subcommand, Debug, Display, Default)]
pub enum ShSubCmd {
    /// List all edits
    #[clap(name = "sh-help")]
    #[default]
    Help,

}

impl super::Subcmd for ShSubCmd {
    type Parent = ShCmd;
}

impl super::super::Cmd for ShCmd {
    type Sub = ShSubCmd;
}
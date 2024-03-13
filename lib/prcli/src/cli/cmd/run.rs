use clap::{Parser, Subcommand};
use strum::Display;

#[derive(Parser, Debug, Default)]
pub struct RunCmd {
    #[clap(subcommand)]
    pub cmd: Option<RunSubCmd>,

}

#[derive(Subcommand, Debug, Display, Default)]
pub enum RunSubCmd {
    /// List all edits
    #[default]
    #[clap(name = "run-help")]
    Help,

}

impl super::Subcmd for RunSubCmd {
    type Parent = RunCmd;
}

impl super::super::Cmd for RunCmd {
    type Sub = RunSubCmd;
}
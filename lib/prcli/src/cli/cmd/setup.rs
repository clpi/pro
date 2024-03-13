use super::super::Cmd;
use super::Subcmd;
use strum::Display;
use clap::{clap_derive::ValueEnum, command, ArgMatches, Args, Parser, Subcommand};

#[derive(Default, Parser, Debug)]
pub struct SetupCmd {
    #[clap(subcommand)]
    pub cmd: Option<SetupWhatCmd>,
}

#[repr(C)]
#[derive(Display, Debug, Subcommand, Default)]
pub enum SetupWhatCmd {
    #[clap(name = "node")]
    Node {

    },
    #[clap(name = "identity")]
    Identity {

    },
    #[clap(name = "subnet")]
    Subnet {

    },
    #[clap(name = "setup-help")]
    #[default]
    Help,
}

impl super::Subcmd for SetupWhatCmd {
    type Parent = SetupCmd;

    fn from_cmd_args(&self, args: &mut impl clap::Args) -> Option<Self> {
        args.update_from_arg_matches(&ArgMatches::default());
        match self.to_string().as_str() {
            "node" => {
                Some(SetupWhatCmd::Node {})
            },
            "subnet" => {
                Some(SetupWhatCmd::Node {})
            },
            "identity" => {
                Some(SetupWhatCmd::Node {})
            },
            _ => {
                Some(SetupWhatCmd::Help)
            }
        }
    }
}
impl super::super::Cmd for SetupCmd {
    type Sub = SetupWhatCmd;
}
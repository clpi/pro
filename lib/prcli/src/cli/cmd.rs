#![feature(associated_type_defaults)]
pub mod bld;
pub mod gen;
pub mod id;
pub mod sh;
pub mod setup;
pub mod ls;
pub mod init;
pub mod run;
pub mod help;
pub mod add;
pub mod edit;
pub mod cfg;
pub mod new;

use strum::{AsRefStr, AsStaticRef, EnumString, FromRepr, IntoStaticStr};
use self::{
    bld::BuildCmd, help::HelpCmd, id::IdCmd, init::InitCmd, ls::LsCmd, new::NewCmd, run::RunCmd, setup::SetupCmd, sh::ShCmd
};
use clap::{CommandFactory, Parser, Subcommand, Args};
use strum::Display;
// #[strum(serialize_all = "snake_case")]
/// The main CLI commands
#[repr(C)]
#[derive(Display, Subcommand, Debug, AsRefStr, IntoStaticStr, EnumString)]
pub enum Cmd {
    // #[clap(short, long = true)]
    #[clap(name = "init")]
    Init(InitCmd),
    /// Run a command,
    #[clap(name = "run")]
    Run(RunCmd),
    /// Run a command,
    #[clap(name = "sh")]
    Sh(ShCmd),
    /// Run a command,
    #[clap(name = "build")]
    Build(BuildCmd),
    /// Run a command,
    #[clap(name = "ls")]
    Ls(LsCmd),
    /// Run a command,
    #[clap(name = "new")]
    New(NewCmd),
    /// Run a command,
    #[clap(name = "root-help")]
    Help(HelpCmd),
    /// Run a command,
    #[clap(name = "setup")]
    Setup(SetupCmd),

    #[clap(name = "id")]
    Id(IdCmd),
    /// Run a command,
    #[clap(name = "version")]
    Version,

}

impl Default for Cmd {
    fn default() -> Self {
        Self::Help(HelpCmd::default())
    }
}

pub trait Subcmd: std::fmt::Display + clap::Subcommand + clap::FromArgMatches + Sized + std::fmt::Debug + Default {
    type Parent: Parser + std::fmt::Debug + Default; 

    fn from_cmd_args(&self, args: &mut impl clap::Args) -> Option<Self> {
        return None;
    }
    fn exit(&self) -> () {
        tracing::info!("Exiting...");
        std::process::exit(1);
    }
    fn matches(&self, args: &[clap::Arg]) {
        tracing::info_span!("Matching...", ?self);
    }
    fn cback(&self, args: &[clap::Arg]) -> anyhow::Result<()> {
        tracing::info_span!("Checking...", ?self);
        Ok(())
    }
    fn init() -> Self {
        Self::default()
    }
    fn get_cmd_matches(&self) -> Self::Parent {
        let p = Self::Parent::parse();
        tracing::info!("Matching {self}");
        return p;
    }
    fn get_cmd(&self) -> Self::Parent {
        let par = Self::Parent::default();
        tracing::info_span!("Getting cmd...", ?par);
        return par;
    }
    fn get_args(&self) {
        tracing::info!("Getting args... {self}");
    }
    fn get_subcmd(&self) -> Self::Parent {
        tracing::info!("Getting subcmd... {self}");
        Self::Parent::parse()
    }
    fn get_subcmd_args(&self) -> () {
        tracing::info!("Get Subcmd Args... {self}");

    }
}

impl Subcmd for Cmd {
    type Parent = super::Root;
    fn init() -> Self {
        Self::default()
    }
}

mod vers {
    use super::*;

    #[derive(Parser, Debug, Default)]
    pub struct VersionCmd {

    }
}
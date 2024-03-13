pub mod cmd;
pub mod sh;

use clap::{Parser, Command, command, CommandFactory, Subcommand, Args};
use strum::{Display, EnumProperty};


/// The main CLI struct
#[derive(Parser, Debug, Default)]
#[command(author, version, long_about, about)]
pub struct Root {
    #[clap(subcommand)]
    pub cmd: cmd::Cmd,
    #[clap(short, long)]
    pub debug: bool,
    #[clap(short, long, default_value = "default")]
    pub profile: Option<String>,
}

pub trait Cmd : std::fmt::Debug + Parser + Default {

    // type ParentCmd : Parser + Default + std::fmt::Debug;
    type Sub: Subcommand + std::fmt::Debug + Default + Sized + cmd::Subcmd + 'static;

    fn init() -> Self {
        tracing::info!("Initializing...");
        return Self::parse();
    }

    fn get_cmd(&self) -> Self::Sub {
        Self::Sub::default()
    }

    fn matches(&self, args: &[clap::Arg]) {
        let m = Self::parse();
        tracing::info_span!("Matching...", ?m);
    }

    fn cback(&self, args: &[clap::Arg]) -> anyhow::Result<()> {
        Ok(())
    }

    fn exit(&self) -> () {
        tracing::info!("Exiting...");
        std::process::exit(1);
    }
}

/// Is the base command
impl Cmd for Root {
    type Sub = cmd::Cmd;

    fn matches(&self, args: &[clap::Arg]) {
        let m = Self::parse();
        tracing::info_span!("Matching...", ?m);
        
    }
    fn cback(&self, args: &[clap::Arg]) -> anyhow::Result<()> {
        let res = Self::parse();
        tracing::info!("cback for cmd impl for root: {res:?} from self.parse {args:?} {self:?}");
        Ok(())
    }
    fn init() -> Self {
        return Self::parse()
    }
    
    fn get_cmd(&self) -> Self::Sub {
        return Self::Sub::default()
    }
    
    fn exit(&self) -> () {
        tracing::info!("Exiting...");
        std::process::exit(1);
    }
}

impl Root {
    pub fn init() -> Self {
        Self::parse()

    }
}
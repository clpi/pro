#[allow(unused_imports)]

use clap::CommandFactory;
use clap::Parser;
use tracing::{debug, error, info, warn};
use std::{borrow::BorrowMut, fmt::Debug, process::Termination};
use pr_cli::util::init_log;



pub fn main() -> anyhow::Result<()> {
    init_log();
    debug!("Hi\n");
    warn!("hi");
    error!("Hi there");
    info!("Info!");
    let cli = pr_cli::cli::Root::parse();
    debug!("{cli:?}");
    
    Ok(())
}
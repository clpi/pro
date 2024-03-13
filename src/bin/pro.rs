use pro::init_log;
use tokio::{

};
use tracing::{info, warn, error};
use tracing_subscriber::{util::SubscriberInitExt, EnvFilter};

#[tokio::main(flavor = "current_thread")]
pub async fn main() -> anyhow::Result<()> {
    init_log();
    error!("HELLO WORLD");
    warn!("hello world");
    info!("hello world");
    Ok(())
}

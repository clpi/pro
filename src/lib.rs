use tracing::{info, warn, error};

pub fn init_log() {
    tracing_subscriber::FmtSubscriber::builder()
        .with_env_filter(EnvFilter::from_default_env())
        .with_line_number(true)
        .with_thread_names(true)
        .with_ansi(true)
        .with_max_level(tracing::Level::DEBUG)
        .finish()
        .init()
}


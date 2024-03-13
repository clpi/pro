use tracing::{level_filters::LevelFilter, Instrument, Level};
use tracing_subscriber::{filter::{Directive, FilterExt, Filtered}, fmt::Layer, layer::{Filter, SubscriberExt}, registry::LookupSpan, util::SubscriberInitExt, EnvFilter, };

pub fn init_log() -> () {
    tracing_subscriber::FmtSubscriber::builder()
        .with_line_number(true)
        .with_level(true)
        .with_thread_names(true)
        .with_thread_ids(true)
        .with_timer(tracing_subscriber::fmt::time::uptime())
        .with_writer(std::io::stderr)
        .pretty()
        .with_line_number(true)
        .with_ansi(true)
        .with_max_level(tracing::Level::TRACE)
        .finish()
        .init();
}

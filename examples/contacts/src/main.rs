use ::contacts::ContactsManifester;
use ::proj_server::prelude::*;
use ::tracing_subscriber::{layer::SubscriberExt as _, util::SubscriberInitExt as _};

fn main() {
    setup_logging();

    cli_start(vec![ Box::new(ContactsManifester {}) ]);
}

fn setup_logging() {
    let file = std::fs::File::create("/tmp/c-server.log").expect("Failed to create log file");

    let filter = tracing_subscriber::EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info"));

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::fmt::layer()
                .with_writer(std::sync::Arc::new(file))
                .with_ansi(false),
        )
        .with(filter)
        .init();

    tracing::info!("LSP Server initialized and logging setup complete.");
}

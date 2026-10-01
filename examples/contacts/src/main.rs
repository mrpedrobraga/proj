use ::contacts::ContactsManifester;
use ::proj_server::{
    repl::{
        tower_lsp::{LspService, Server},
        ProjectServer,
    },
    server::ProjectView,
};
use ::tracing_subscriber::{layer::SubscriberExt as _, util::SubscriberInitExt as _};

fn main() {
    setup_logging();

    let manifester = ContactsManifester {};

    let server = ProjectView::new(vec![Box::new(manifester)]);
    let (service, socket) = LspService::new(|client| ProjectServer {
        project_view: server,
        lsp_client: client,
    });
    let stdin = smol::Unblock::new(std::io::stdin());
    let stdout = smol::Unblock::new(std::io::stdout());
    let server_process = Server::new(stdin, stdout, socket).serve(service);
    smol::block_on(server_process);
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

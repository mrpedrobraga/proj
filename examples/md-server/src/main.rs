use proj_server::{
    modpath,
    project::Project as _,
    repl::{
        tower_lsp::{LspService, Server},
        ProjectRepl,
    },
    server::ProjectServer,
};
use tracing_subscriber::{layer::SubscriberExt as _, util::SubscriberInitExt as _};

use self::project::MarkdownProject;

pub mod project;

fn main() {
    setup_logging();

    let project_path =
        "/home/mrpedrobraga/Development/proj/crates/md-server/projects/Example Project";

    let md_project_view: MarkdownProject = MarkdownProject::new_from_directory(project_path);

    let server = ProjectServer {
        open_projects: vec![Box::new(md_project_view)],
    };

    let server_process = setup_lsp_service(server);
    smol::block_on(server_process);
}

fn setup_lsp_service(server: ProjectServer) -> impl Future<Output = ()> {
    let (service, socket) = LspService::new(|client| ProjectRepl { server, client });
    let stdin = smol::Unblock::new(std::io::stdin());
    let stdout = smol::Unblock::new(std::io::stdout());
    Server::new(stdin, stdout, socket).serve(service)
}

fn setup_logging() {
    let file = std::fs::File::create("/tmp/md-server.log").expect("Failed to create log file");

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::fmt::layer()
                .with_writer(std::sync::Arc::new(file))
                .with_ansi(false),
        )
        .with(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    tracing::info!("LSP Server initialized and logging setup complete.");
}

#[allow(unused)]
fn test_partial_reference_resolution(md_project_view: &MarkdownProject) {
    let suffix = modpath!(index);
    println!(
        "\n\nShowing all possible resolutions for a module named '{:#?}'.\n",
        suffix
    );
    let possible_paths = md_project_view
        .modules
        .possible_paths_for_module_name(suffix.clone())
        .unwrap();

    possible_paths.iter().for_each(|(_, module_index)| {
        let module = &md_project_view.modules.entries[*module_index];
        dbg!(&module.internal_path);
    });
}

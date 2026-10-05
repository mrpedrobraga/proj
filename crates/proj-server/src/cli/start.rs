use ::clap::Parser as _;
use ::tower_lsp::{LspService, Server};

use crate::{project::manifest::ProjectManifester, server::ProjectServer, view::ProjectView};

use super::CliArgs;

/// Starts the CLI and executes all the cool functions.
pub fn cli_start(manifesters: Vec<Box<dyn ProjectManifester>>) {
    let args = CliArgs::parse();

    #[allow(unused)]
    match args {
        CliArgs::New { project_name, path } => todo!(),
        CliArgs::Info => todo!(),
        CliArgs::Build => todo!(),
        CliArgs::Run => todo!(),
        CliArgs::Serve => todo!(),
        CliArgs::Lsp => {
            let server = ProjectView::new(manifesters);
            let (service, socket) = LspService::new(|client| ProjectServer {
                project_view: server,
                lsp_client: client,
            });
            let stdin = smol::Unblock::new(std::io::stdin());
            let stdout = smol::Unblock::new(std::io::stdout());
            let server_process = Server::new(stdin, stdout, socket).serve(service);
            smol::block_on(server_process);
        }
        CliArgs::Bundle => todo!(),
        CliArgs::Action(cli_args_task) => todo!(),
        CliArgs::Deps(cli_args_deps) => todo!(),
        CliArgs::Mod(cli_args_mod) => todo!(),
        CliArgs::Lint(cli_args_lint) => todo!(),
        CliArgs::Style(cli_args_style) => todo!(),
    }
}
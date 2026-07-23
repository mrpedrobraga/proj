use clap::Parser;
use proj::{ProjectManifester, SimpleManifester, cli::CliArgs};

pub mod md;

fn main() {
    let args = CliArgs::parse();

    let manifester: SimpleManifester<md::MdManifest> =
        SimpleManifester::new();
    let _man = md::MdProjectManager {};

    #[allow(unused)]
    match args {
        CliArgs::New { project_name, path } => unimplemented!(),
        CliArgs::Info => {
            let doesit = manifester
                .directory_contains_project("./examples/markdown-wikilinks/projects/example-project")
                .unwrap();

            if doesit {
                println!("Directory contains a project!");
            } else {
                println!("Directory does not contain a project!");
            }
        }
        CliArgs::Build => unimplemented!(),
        CliArgs::Run => unimplemented!(),
        CliArgs::DevServe => unimplemented!(),
        CliArgs::Bundle => unimplemented!(),
        CliArgs::Task(cli_args_task) => unimplemented!(),
        CliArgs::Deps(cli_args_deps) => unimplemented!(),
        CliArgs::Mod(cli_args_mod) => unimplemented!(),
        CliArgs::Lint(cli_args_lint) => unimplemented!(),
        CliArgs::Style(cli_args_style) => unimplemented!(),
    }
}

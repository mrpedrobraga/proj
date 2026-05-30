use clap::Parser;
use proj::{ProjectManager, ProjectManifester, SimpleManifester, cli::CliArgs};
use std::path::PathBuf;

fn main() {
    let args = CliArgs::parse();

    let manifester: SimpleManifester<MdManifest> =
        SimpleManifester::new(PathBuf::from("README.md"));
    let _man = MdManager {};

    #[allow(unused)]
    match args {
        CliArgs::New => unimplemented!(),
        CliArgs::Info => {
            let doesit = manifester
                .directory_contains_project(
                    "./examples/markdown-wikilinks/projects/example-project",
                )
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

struct MdManager {}

struct MdManifest {}

impl ProjectManager for MdManager {
    type Manifester = SimpleManifester<MdManifest>;
}

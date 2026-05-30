use std::path::PathBuf;

use proj::{ProjectManager, ProjectManifester, SimpleManifester};

fn main() {
    let manifester: SimpleManifester<MdManifest> =
        SimpleManifester::new(PathBuf::from("README.md"));

    let doesit = manifester
        .directory_contains_project("./examples/markdown-wikilinks/projects/example-project")
        .unwrap();

    if doesit {
        println!("Directory contains a project!");
    } else {
        println!("Directory does not contain a project!");
    }
}

struct MdManager {}

struct MdManifest {}

impl ProjectManager for MdManager {
    type Manifester = SimpleManifester<MdManifest>;
}

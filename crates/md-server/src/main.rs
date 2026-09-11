use proj::{modpath, project::ProjectView};

use self::project::MarkdownProject;

pub mod project;

fn main() {
    let project_path = "./projects/example-project";

    let md_project_view: ProjectView<MarkdownProject> = ProjectView::new_from_directory(project_path);
}


#[allow(unused)]
fn test_partial_reference_resolution(md_project_view: ProjectView<MarkdownProject>) {
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
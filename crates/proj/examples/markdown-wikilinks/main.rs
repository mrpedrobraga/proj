use proj::project::{ModulePath, ProjectView};

use self::md::MarkdownProject;

pub mod md;

macro_rules! modpath {
    () => { ModulePath::from([]) };
    ( $name:ident ) => { ModulePath::from([ stringify!($name) ]) };
    ( $name:ident $(:: $frag:ident)* ) => { ModulePath::from([ stringify!($name) $(, stringify!($frag))* ]) };
}

fn main() {
    let project_path = "./examples/markdown-wikilinks/projects/example-project";
    let md_project_view: ProjectView<MarkdownProject> =
        ProjectView::new_from_directory(project_path);

    let test_module = md_project_view.modules.module_at(modpath!( Documents::Other::index ));
    let test_module = test_module.unwrap();

    let test_module_content = &test_module.content;
    dbg!(test_module_content);

    //test_partial_reference_resolution(md_project_view);
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

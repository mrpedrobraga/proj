use proj::project::ProjectView;

use self::md::MarkdownProject;

pub mod md;

fn main () {
    let project_path = "./examples/markdown-wikilinks/projects/example-project";
    let md_project_view: ProjectView<MarkdownProject> = ProjectView::new_from_directory(project_path);

    println!("{:#?}", md_project_view);

    dbg!(md_project_view.modules.possible_paths_for_module_name(&"index".to_string()));

    for p in md_project_view.modules.iter_paths() {
        dbg!(p);
    }
}

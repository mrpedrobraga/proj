use proj::project::ProjectView;

use self::md::MarkdownProject;

pub mod md;

fn main () {
    let project_path = "./examples/markdown-wikilinks/projects/example-project";
    let md_project_view: ProjectView<MarkdownProject> = ProjectView::new_from_directory(project_path);

    dbg!(md_project_view);
}

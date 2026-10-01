use super::{MarkdownContent, MarkdownManifester, MarkdownProject, MANIFEST_PATH};
use ::proj_server::{
    project::manifest::{IncludeRule, ModuleInclude, ProjectLayout},
    ProjError,
};
use proj_server::{
    project::{manifest::ProjectManifester, ModuleContent, ModuleSet, PositionInText, Project},
    server::HoverInfo,
};
use std::sync::Arc;

impl ProjectManifester for MarkdownManifester {
    fn name(&self) -> &str {
        "Markdown Manifester"
    }

    fn directory_contains_project(
        &self,
        directory_path: std::path::PathBuf,
    ) -> Result<bool, ProjError> {
        Ok(std::fs::exists(directory_path.join(MANIFEST_PATH))?)
    }

    fn new_project_from_directory(&self, path: std::path::PathBuf) -> Box<dyn Project> {
        Box::new(MarkdownProject::new_from_directory(path))
    }
}

fn project_layout() -> ProjectLayout {
    ProjectLayout::new() //
        .include(IncludeRule::new("**/*.md".into(), |source| {
            Ok(ModuleInclude::new(
                source.file_name().unwrap(),
                Arc::new(parse_markdown(source.file_content)),
            ))
        }))
}

impl Project for MarkdownProject {
    fn new_from_directory<Pa: AsRef<std::path::Path>>(path: Pa) -> Self
    where
        Self: Sized,
    {
        MarkdownProject {
            modules: project_layout().gather(path.as_ref()),
        }
    }

    fn update_from_directory<Pa: AsRef<std::path::Path>>(_path: Pa) -> Self
    where
        Self: Sized,
    {
        unimplemented!()
    }

    fn layout(&self) -> ProjectLayout {
        project_layout()
    }

    fn modules(&self) -> &ModuleSet {
        &self.modules
    }
}

fn parse_markdown(markdown_source: &str) -> MarkdownContent {
    let arena = comrak::Arena::new();
    let options = comrak::Options {
        extension: comrak::options::Extension::builder()
            .wikilinks_title_after_pipe(true)
            .build(),
        parse: comrak::options::Parse::builder().build(),
        render: comrak::options::Render::default(),
    };
    let _root_node = comrak::parse_document(&arena, markdown_source, &options);

    let mut lines_which_are_headings = vec![];
    for (line_index, line) in markdown_source.lines().enumerate() {
        if line.starts_with("#") {
            lines_which_are_headings.push(line_index);
        }
    }

    // for node in root_node.descendants() {
    //     println!("{:#?}", node.collect_text());
    // }

    MarkdownContent {
        lines_which_are_headings,
        text: markdown_source.to_string(),
    }
}

impl ModuleContent for MarkdownContent {
    fn hover_information_at(
        &self,
        position_in_source_text: proj_server::project::PositionInText,
    ) -> Option<HoverInfo> {
        if let Some(line_index) = self
            .lines_which_are_headings
            .iter()
            .copied()
            .find(|index| *index == position_in_source_text.line as usize)
        {
            let heading = self
                .text
                .lines()
                .nth(line_index)
                .unwrap_or("Heading")
                .to_string();
            let range = (
                PositionInText {
                    line: line_index as u32,
                    column: 0,
                },
                PositionInText {
                    line: line_index as u32,
                    column: heading.len() as u32,
                },
            );

            return Some(HoverInfo {
                text: format!(
                    "{heading}\n\nA beautiful heading in my beautiful markdown project.\nReally, isn't it sweet?"
                ),
                range: Some(range),
            });
        }

        None
    }

    fn nth_line(&self, line_index: usize) -> Option<String> {
        self.text.lines().nth(line_index).map(str::to_string)
    }
}

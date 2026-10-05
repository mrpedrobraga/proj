use ::std::sync::Arc;

use ::proj_server::{
    project::{
        manifest::{IncludeRule, ModuleInclude, ProjectLayout, ProjectManifester},
        ModuleContent, ModuleSet, Project,
    },
    view::HoverInfo,
    ProjError,
};
use ::serde::{Deserialize, Serialize};

pub struct ContactsProject {
    pub manifest: Option<ContactsManifest>,
    modules: ModuleSet,
}

pub struct ContactsManifester {}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContactsManifest {}

pub struct ContactsModuleContent {
    parsed: ContactFile,
    raw: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContactFile {
    name: String,
    description: Option<String>,
}

impl ProjectManifester for ContactsManifester {
    fn name(&self) -> &str {
        "contacts"
    }

    fn directory_contains_project(&self, path: std::path::PathBuf) -> Result<bool, ProjError> {
        std::fs::exists(path.join("contacts.toml")).map_err(ProjError::Io)
    }

    fn new_project_from_directory(&self, path: std::path::PathBuf) -> Box<dyn Project> {
        Box::new(ContactsProject::new_from_directory(path))
    }
}

fn project_layout() -> ProjectLayout {
    ProjectLayout::new() //
        .include(IncludeRule::new("contacts/*.ron".into(), |source| {
            let parsed = ron::from_str::<ContactFile>(source.file_content).map_err(Box::new)?;

            Ok(ModuleInclude::new(
                source.file_name().unwrap(),
                Arc::new(ContactsModuleContent {
                    parsed,
                    raw: source.file_content.to_string(),
                }),
            ))
        }))
}

impl Project for ContactsProject {
    fn new_from_directory<Pa: AsRef<std::path::Path>>(path: Pa) -> Self
    where
        Self: Sized,
    {
        Self {
            modules: project_layout().gather(path.as_ref()),
            manifest: None,
        }
    }

    fn update_from_directory<Pa: AsRef<std::path::Path>>(_path: Pa) -> Self
    where
        Self: Sized,
    {
        unimplemented!()
    }

    fn layout(&self) -> proj_server::project::manifest::ProjectLayout {
        project_layout()
    }

    fn modules(&self) -> &ModuleSet {
        &self.modules
    }
}

impl ModuleContent for ContactsModuleContent {
    fn hover_information_at(
        &self,
        _: proj_server::project::PositionInText,
    ) -> Option<proj_server::view::HoverInfo> {
        Some(HoverInfo {
            text: format!("Contacts entry for {}", self.parsed.name),
            range: None,
        })
    }

    fn nth_line(&self, line_index: usize) -> Option<String> {
        self.raw.lines().nth(line_index).map(str::to_string)
    }
}

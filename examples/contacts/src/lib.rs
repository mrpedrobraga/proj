use ::std::sync::Arc;

use ::proj_server::{
    ProjError,
    project::{
        ModuleContent, ModuleEntry, ModuleOrigin, ModulePath, Project, manifest::ProjectManifester,
    },
    server::HoverInfo,
};
use ::serde::{Deserialize, Serialize};

pub struct ContactsProject {
    manifest: Option<ContactsManifest>,
    entries: Vec<ModuleEntry>,
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

impl Project for ContactsProject {
    fn new_from_directory<Pa: AsRef<std::path::Path>>(path: Pa) -> Self
    where
        Self: Sized,
    {
        let mut project = Self {
            entries: Vec::new(),
            manifest: None,
        };

        project.load_root_module(path.as_ref().to_path_buf());
        project.discover_other_modules(path.as_ref().to_path_buf());

        project
    }

    fn update_from_directory<Pa: AsRef<std::path::Path>>(path: Pa) -> Self
    where
        Self: Sized,
    {
        unimplemented!()
    }

    fn load_root_module(&mut self, directory_path: std::path::PathBuf)
    where
        Self: Sized,
    {
        tracing::info!("Loading manifest...");
        self.manifest = std::fs::read_to_string(directory_path.join("contacts.toml"))
            .ok()
            .and_then(|raw| toml::from_str(&raw).ok());
    }

    fn discover_other_modules(&mut self, directory_path: std::path::PathBuf)
    where
        Self: Sized,
    {
        let contacts_folder = directory_path.join("contacts");
        tracing::info!("Discovering contacts from '{}'", contacts_folder.display());
        if contacts_folder.exists() {
            let dir = std::fs::read_dir(contacts_folder).unwrap();

            tracing::info!("Discovering modules...");

            for dir_entry in dir.flatten() {
                let dir_entry_path = dir_entry.path();
                let raw = std::fs::read_to_string(&dir_entry_path).unwrap();
                let parsed: ContactFile = ron::from_str(&raw).unwrap();
                tracing::info!("Discovered module {:?}", dir_entry_path);
                self.entries.push(ModuleEntry {
                    name: parsed.name.clone(),
                    internal_path: ModulePath(vec![
                        dir_entry_path
                            .file_name()
                            .unwrap()
                            .to_str()
                            .unwrap()
                            .to_string(),
                    ]),
                    origin: ModuleOrigin::File(dir_entry_path),
                    content: Arc::new(ContactsModuleContent { parsed, raw }),
                });
            }
        }
    }

    fn module_at_file_path(
        &self,
        file_path: &std::path::Path,
    ) -> Option<&proj_server::project::ModuleEntry> {
        tracing::info!("Checking for found module at {}", file_path.display());

        for entry in self.entries.iter() {
            if let ModuleOrigin::File(path) = &entry.origin
                && same_file::is_same_file(path, file_path).ok()?
            {
                tracing::info!("Found.");
                return Some(entry);
            }
        }

        tracing::info!("Not found.");

        None
    }
}

impl ProjectManifester for ContactsManifester {
    fn name(&self) -> &str {
        "contacts"
    }

    fn directory_contains_project(
        &self,
        path: std::path::PathBuf,
    ) -> proj_server::project::manifest::Result<bool> {
        std::fs::exists(path.join("contacts.toml")).map_err(ProjError::Io)
    }

    fn new_project_from_directory(&self, path: std::path::PathBuf) -> Box<dyn Project> {
        Box::new(ContactsProject::new_from_directory(path))
    }
}

impl ModuleContent for ContactsModuleContent {
    fn hover_information_at(
        &self,
        _: proj_server::project::PositionInText,
    ) -> Option<proj_server::server::HoverInfo> {
        Some(HoverInfo {
            text: format!("Contacts entry for {}", self.parsed.name),
            range: None,
        })
    }

    fn nth_line(&self, line_index: usize) -> Option<String> {
        self.raw.lines().nth(line_index).map(str::to_string)
    }
}

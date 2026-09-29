use std::path::PathBuf;

use crate::ProjError;

use super::Project;

/// Handy alias so we don't have to specify the error type all the time...
pub type Result<T> = core::result::Result<T, ProjError>;

/// Trait for a component that manages the "existence" of a project
/// in a folder: creating, checking validity, getting information, etc.
pub trait ProjectManifester: Send + Sync {
    /// Returns a name identifying this manifester :-)
    fn name(&self) -> &str;

    /// Returns whether a directory contains a valid project.
    fn directory_contains_project(&self, path: PathBuf) -> Result<bool>;

    /// Creates a new project from a directory.
    fn new_project_from_directory(&self, path: PathBuf) -> Box<dyn Project>;

    /// Returns a path in array form from a string.
    fn parse_module_path_from_str(&self, string_path: &str) -> Result<Vec<String>> {
        Ok(string_path.split("::").map(str::to_string).collect())
    }
}

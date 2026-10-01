use super::{ModuleContent, Project};
use crate::ProjError;
use ::std::{path::Path, sync::Arc};
use std::path::PathBuf;

pub mod implementations;

/// Trait for a component that manages the "existence" of a project
/// in a folder: creating, checking validity, getting information, etc.
pub trait ProjectManifester: Send + Sync {
    /// Returns a name identifying this manifester :-)
    fn name(&self) -> &str;

    /// Returns whether a directory contains a valid project.
    fn directory_contains_project(&self, path: PathBuf) -> Result<bool, ProjError>;

    /// Creates a new project from a directory.
    fn new_project_from_directory(&self, path: PathBuf) -> Box<dyn Project>;

    /// Returns a path in array form from a string.
    fn parse_module_path_from_str(&self, string_path: &str) -> Result<Vec<String>, ProjError> {
        Ok(string_path.split("::").map(str::to_string).collect())
    }
}

/// A [ProjectLayout] describes how to understand what modules exist
/// from the file system!
#[derive(Default)]
pub struct ProjectLayout {
    rules: Vec<IncludeRule>,
}

#[derive(Clone)]
pub struct IncludeRule {
    glob_pattern: String,
    parse: Arc<ParseFn>,
}

/// Describes a file source for parsing and generating a module inclusion.
pub struct IncludeSource<'walk> {
    pub path: &'walk Path,
    // TODO: Use a virtualized view into the file instead of loading all of it.
    pub file_content: &'walk str,
    pub parent_module_path: &'walk [String],
}

/// Describes a module to be included in a module set.
pub struct ModuleInclude {
    pub name: String,
    pub content: Arc<dyn ModuleContent>,
    pub children_rules: Vec<IncludeRule>,
}

pub type DynError = Box<dyn ::std::error::Error + Send + Sync>;
pub type ParseFn = dyn Fn(&IncludeSource) -> core::result::Result<ModuleInclude, DynError>;

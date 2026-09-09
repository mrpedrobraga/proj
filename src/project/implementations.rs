use std::path::Path;

use super::{ProjectKind, ProjectOrigin, ProjectView};

impl<PKind: ProjectKind> ProjectView<PKind> {
    /// Creates a new project view from a directory.
    pub fn new_from_directory<Pa: AsRef<Path>>(path: Pa) -> Self {
        let origin = ProjectOrigin { local_path: path.as_ref().to_path_buf()};
        let modules = vec![];
        
        ProjectView { modules, origin }
    }

    /// Updates this project view by looking at this new directory.
    /// 
    /// This is faster than creating a new view from scratch if little has changed
    /// on disk since the current version, but way slower if the entire project structure changed.
    /// 
    /// TODO: Perhaps use immutable `ProjectView`s instead of taking `&mut self`?
    pub fn update_from_directory<Pa: AsRef<Path>>(path: Pa) -> Self {
        let origin = ProjectOrigin { local_path: path.as_ref().to_path_buf()};
        let modules = vec![];
        
        ProjectView { modules, origin }
    }

}
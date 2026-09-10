use std::{path::Path};

use super::{ModuleEntry, ModuleItem, ProjectKind, ProjectOrigin, ProjectView};

impl<PKind: ProjectKind> ProjectView<PKind> {
    /// Creates a new project view from a directory.
    pub fn new_from_directory<Pa: AsRef<Path>>(path: Pa) -> Self {
        let local_path = path.as_ref().to_path_buf();
        let origin = ProjectOrigin {
            local_path: local_path.clone(),
        };
        let mut modules = Vec::new();

        PKind::load_root_module(&mut modules, local_path.clone());
        PKind::discover_other_modules(&mut modules, local_path);

        ProjectView { modules, origin }
    }

    /// Updates this project view by looking at this new directory.
    ///
    /// This is faster than creating a new view from scratch if little has changed
    /// on disk since the current version, but way slower if the entire project structure changed.
    ///
    /// TODO: Perhaps use immutable `ProjectView`s instead of taking `&mut self`?
    pub fn update_from_directory<Pa: AsRef<Path>>(path: Pa) -> Self {
        let origin = ProjectOrigin {
            local_path: path.as_ref().to_path_buf(),
        };
        let modules = vec![];

        ProjectView { modules, origin }
    }
}

impl<P: ProjectKind> std::fmt::Debug for ProjectView<P>
where
    P: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "project ")?;
        f.debug_set().entries(self.modules.iter()).finish()
    }
}

impl<P: ProjectKind> std::fmt::Debug for ModuleEntry<P>
where
    P: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "mod '{}' ", self.name)?;
        f.debug_set().entries(self.items.iter()).finish()
    }
}

impl<P: ProjectKind> std::fmt::Debug for ModuleItem<P>
where
    P: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "item {:?}", self.item)
    }
}
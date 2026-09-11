use std::{fmt::Debug, path::Path};

use sequence_trie::SequenceTrie;

use super::{
    ModuleEntry, ModuleName, ModuleOrigin, ModulePath, ModuleSet, ProjectKind,
    ProjectOrigin, ProjectView,
};

impl<PKind: ProjectKind> ProjectView<PKind> {
    /// Creates a new project view from a directory.
    pub fn new_from_directory<Pa: AsRef<Path>>(path: Pa) -> Self {
        let local_path = path.as_ref().to_path_buf();
        let origin = ProjectOrigin {
            local_path: local_path.clone(),
        };
        let mut modules = ModuleSet::new();

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
        let modules = ModuleSet::new();

        ProjectView { modules, origin }
    }
}

impl<P: ProjectKind> std::fmt::Debug for ProjectView<P>
where
    P: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "project ")?;
        self.modules.fmt(f)
    }
}

impl<P: ProjectKind> ModuleSet<P> {
    pub fn new() -> Self {
        ModuleSet {
            entries: Vec::new(),
            forward_index: SequenceTrie::new(),
            backward_index: SequenceTrie::new(),
        }
    }

    pub fn insert(&mut self, new_module: ModuleEntry<P>, path: ModulePath) {
        self.forward_index.insert(path.0.iter(), self.entries.len());
        self.backward_index.insert(path.0.iter().rev(), self.entries.len());
        self.entries.push(new_module);
    }

    /// Retrieves a module given its full module path.
    pub fn module_at(&self, key: ModulePath) -> Option<&ModuleEntry<P>> {
        self.forward_index.get(key.0.iter()).and_then(|index| self.module_at_index(*index))
    }

    /// Retrives the module slotted at a specific index.
    fn module_at_index(&self, index: usize) -> Option<&ModuleEntry<P>> {
        self.entries.get(index)
    }

    /// Iterates through all the existing module paths in this set.
    ///
    /// TODO: Make this an iterator.
    pub fn iter_paths(&self) -> Vec<(ModulePath, usize)> {
        self.forward_index
            .iter()
            .map(|(list, index)| {
                (
                    ModulePath::from(list.into_iter().cloned().collect::<Vec<_>>()),
                    *index,
                )
            })
            .collect()
    }

    /// Returns all possible module paths for a module that could be refferred to as `suffix`.
    pub fn possible_paths_for_module_name(
        &self,
        suffix: ModulePath,
    ) -> Option<&SequenceTrie<ModuleName, usize>>
    {
        self.backward_index.get_node(suffix.0.iter().rev())
    }

    /// Returns true if this module set already contains a module spawned
    /// from a specific path on disk. This is useful for skipping modules manifested
    /// by discovery on the file system that were already indexed by other means.
    pub fn contains_module_from_file_path<Pa: AsRef<Path>>(&self, path: Pa) -> bool {
        // TODO: Maybe the implementation here can be a bit better?
        // TODO: Consider parallelism.
        self.entries.iter().any(|m| {
            if let ModuleOrigin::File(existing_entry_path) = &m.origin {
                path.as_ref().canonicalize().unwrap() == existing_entry_path.canonicalize().unwrap()
            } else {
                false
            }
        })
    }
}

impl<P: ProjectKind> Default for ModuleSet<P> {
    fn default() -> Self {
        Self::new()
    }
}

impl<P: ProjectKind> std::fmt::Debug for ModuleSet<P>
where
    P: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_set().entries(self.entries.iter()).finish()
    }
}

impl<P: ProjectKind> std::fmt::Debug for ModuleEntry<P>
where
    P: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "mod '{}' at `{:?}`", self.name, self.internal_path)?;
        //self.content.fmt(f)
        Ok(())
    }
}

impl ModulePath {
    pub fn joined(&self, other: &Self) -> Self {
        ModulePath(self.0.iter().chain(other.0.iter()).cloned().collect())
    }
}

impl From<&[ModuleName]> for ModulePath {
    fn from(value: &[ModuleName]) -> Self {
        ModulePath(value.to_vec())
    }
}

impl From<Vec<ModuleName>> for ModulePath {
    fn from(value: Vec<ModuleName>) -> Self {
        ModulePath(value)
    }
}

impl<const N: usize> From<[ModuleName; N]> for ModulePath {
    fn from(value: [ModuleName; N]) -> Self {
        ModulePath(value.to_vec())
    }
}

impl<const N: usize> From<[&str; N]> for ModulePath {
    fn from(value: [&str; N]) -> Self {
        ModulePath(value.into_iter().map(String::from).collect())
    }
}

impl Debug for ModulePath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.join("::"))
    }
}


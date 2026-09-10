use std::{collections::HashMap, fmt::Debug, path::Path};

use sequence_trie::SequenceTrie;

use super::{
    Item, ItemSet, ModuleEntry, ModuleName, ModuleOrigin, ModulePath, ModuleSet, ProjectKind,
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
            backward_index: HashMap::new(),
        }
    }

    pub fn insert(&mut self, new_module: ModuleEntry<P>, path: ModulePath) {
        self.forward_index.insert(path.0.iter(), self.entries.len());
        if let Some(last_item) = path.0.last() {
            self.backward_index
                .entry(last_item.clone())
                .or_default()
                .push(path.0.to_vec());
        }
        self.entries.push(new_module);
    }

    /// Iterates through all the existing module paths in this set.
    /// 
    /// TODO: Make this an iterator.
    pub fn iter_paths(
        &self,
    ) -> Vec<ModulePath> {
        self.forward_index
            .keys()
            .map(|list| ModulePath::from(list.into_iter().cloned().collect::<Vec<_>>())).collect()
    }

    /// Returns all possible module paths for a module named `module_name`.
    pub fn possible_paths_for_module_name(&self, module_name: &String) -> Vec<ModulePath> {
        self.backward_index
            .get(module_name)
            .map(|v| v.iter().map(|x| ModulePath::from(x.clone())).collect())
            .unwrap_or_default()
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
        write!(f, "mod '{}' ", self.name)?;
        self.items.fmt(f)
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

impl Debug for ModulePath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.join("::"))
    }
}

impl<P: ProjectKind> ItemSet<P> {
    pub fn new() -> Self {
        ItemSet {
            entries: Vec::new(),
        }
    }

    pub fn insert(&mut self, module_item: Item<P>) {
        self.entries.push(module_item);
    }
}

impl<P: ProjectKind> Default for ItemSet<P> {
    fn default() -> Self {
        Self::new()
    }
}

impl<P: ProjectKind> std::fmt::Debug for ItemSet<P>
where
    P: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_set().entries(self.entries.iter()).finish()
    }
}

impl<P: ProjectKind> std::fmt::Debug for Item<P>
where
    P: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "item {:?}", self.item)
    }
}

use std::{collections::HashMap, fmt::Debug, path::Path};

use sequence_trie::SequenceTrie;
use tower_lsp::lsp_types::{self, Position};

use super::{ModuleEntry, ModuleName, ModuleOrigin, ModulePath, ModuleSet, PositionInText};

impl ModuleSet {
    pub fn new() -> Self {
        ModuleSet {
            entries: Vec::new(),
            forward_index: SequenceTrie::new(),
            backward_index: SequenceTrie::new(),
            path_index: HashMap::new(),
        }
    }

    pub fn insert(&mut self, new_module: ModuleEntry) {
        let index_in_arena = self.entries.len();
        self.forward_index
            .insert(new_module.internal_path.0.iter(), index_in_arena);
        self.backward_index
            .insert(new_module.internal_path.0.iter().rev(), index_in_arena);
        if let ModuleOrigin::File(path_buf) = &new_module.origin {
            let absolute_path = path_buf
                .clone()
                .canonicalize()
                .expect("Path isn't real? :O");
            tracing::info!(
                "Inserting module {} at {}",
                new_module.name,
                absolute_path.display()
            );
            self.path_index.insert(absolute_path, index_in_arena);
        }
        self.entries.push(new_module);
    }

    /// Retrieves a module given its full module path.
    pub fn module_at(&self, key: ModulePath) -> Option<&ModuleEntry> {
        self.forward_index
            .get(key.0.iter())
            .and_then(|index| self.module_at_index(*index))
    }

    /// Retrieves the module given its file path.
    ///
    /// TODO: Maybe use URI instead of file path?
    pub fn module_at_file_path<Pa>(&self, path: Pa) -> Option<&ModuleEntry>
    where
        Pa: AsRef<Path>,
    {
        self.path_index
            .get(path.as_ref())
            .and_then(|index| self.module_at_index(*index))
    }

    /// Retrieves the module slotted at a specific index.
    fn module_at_index(&self, index: usize) -> Option<&ModuleEntry> {
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
    ) -> Option<&SequenceTrie<ModuleName, usize>> {
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

impl Default for ModuleSet {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Debug for ModuleSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_set().entries(self.entries.iter()).finish()
    }
}

impl std::fmt::Debug for ModuleEntry {
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

impl From<lsp_types::Position> for PositionInText {
    fn from(value: lsp_types::Position) -> Self {
        PositionInText {
            line: value.line,
            column: value.character,
        }
    }
}

impl From<PositionInText> for lsp_types::Position {
    fn from(value: PositionInText) -> Self {
        Position {
            line: value.line,
            character: value.column,
        }
    }
}

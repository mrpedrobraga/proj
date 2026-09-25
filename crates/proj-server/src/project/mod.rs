use sequence_trie::SequenceTrie;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

use crate::server::HoverInfo;

pub mod implementations;
pub mod manifest;

/// A trait that describes specific information about a kind of project,
/// for example, the items it can have, how to read it from files, etc.
pub trait Project: Send + Sync {
    /// Creates a new project view from a directory.
    fn new_from_directory<Pa: AsRef<Path>>(path: Pa) -> Self
    where
        Self: Sized;

    /// Updates this project view by looking at this new directory.
    ///
    /// This is faster than creating a new view from scratch if little has changed
    /// on disk since the current version, but way slower if the entire project structure changed.
    ///
    /// TODO: Perhaps use immutable `ProjectView`s instead of taking `&mut self`?
    fn update_from_directory<Pa: AsRef<Path>>(path: Pa) -> Self
    where
        Self: Sized;

    fn load_root_module(&mut self, directory_path: PathBuf)
    where
        Self: Sized;

    fn discover_other_modules(&mut self, directory_path: PathBuf)
    where
        Self: Sized;

    /// Returns the module associated with the given file path if one exists.
    fn module_at_file_path(&self, file_path: &Path) -> Option<&ModuleEntry>;
}

/// Where a project was sourced from.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectOrigin {
    local_path: PathBuf,
}

/// A set of modules in a project.
pub struct ModuleSet {
    /// A topogically sorted list of all modules in the set.
    /// Sibling ordering is not guaranteed but it should match
    /// item order for modules manifested from another module
    /// followed by iteration order for modules manifested from the file system.
    pub entries: Vec<ModuleEntry>,
    /// Allows quick iteration of module paths matching a prefix.
    forward_index: SequenceTrie<ModuleName, usize>,
    /// Allows quick iteration of module paths matching a suffix.
    backward_index: SequenceTrie<ModuleName, usize>,
    /// Allows quickly finding a module given its file path.
    path_index: HashMap<PathBuf, usize>,
}

/// An entry in the project view describing a module in the project.
pub struct ModuleEntry {
    pub name: String,
    /// Path of the module within the project.
    /// Example `::root::foo::bar`
    pub internal_path: ModulePath,
    pub origin: ModuleOrigin,
    pub content: Box<dyn ModuleContent>,
}

#[macro_export]
macro_rules! modpath {
    () => { ModulePath::from([]) };
    ( $name:ident ) => { $crate::project::ModulePath::from([ stringify!($name) ]) };
    ( $name:ident $(:: $frag:ident)* ) => { $crate::project::ModulePath::from([ stringify!($name) $(, stringify!($frag))* ]) };
}

/// A reference to a module in a project.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct ModuleRef(usize);

/// A path representing a module in a project.
///
#[derive(Hash, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct ModulePath(pub Vec<ModuleName>);

/// Type representing the name of a module in paths.
///
/// TODO: Not use `String`.
pub type ModuleName = String;

/// A reference to an item in a specific module;
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct ModuleItemRef(usize);

/// A reference to an item anywhere in a project;
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct ProjectItemRef(ModuleRef, usize);

/// Where a module was sourced from.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ModuleOrigin {
    /// The module was sourced from a standalone file or a directory index.
    /// For example, in the markdown example, every `.md` file is its own module.
    ///
    /// Example `~/Projects/my_project/main.rs`
    ///
    /// TODO: Maybe use URI?
    File(PathBuf),
    /// The module was sourced from an item inside another module.
    /// Think a `mod` block in Rust.
    Item(ProjectItemRef),
}

/// A "Kind" of module content—each can have different content internally and different ways of
/// interacting or generating such content.
pub trait ModuleContent: Send + Sync {
    /// Returns hover information corresponding with a particular location in the source text
    /// if this module was generated from a file or equivalent.
    ///
    /// In this case, it makes sense to keep something like a concrete syntax tree
    /// which maps `PositionInText`s to items.
    fn hover_information_at(&self, position_in_source_text: PositionInText) -> Option<HoverInfo>;

    fn nth_line(&self, line_index: usize) -> Option<String>;
}

/// Position of something in a text document.
///
/// This is preferrable to an `usize` "character index" when editing large texts.
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Copy, Clone, Default, Deserialize, Serialize)]
pub struct PositionInText {
    /// Line position in a document (zero-based).
    pub line: u32,
    /// Column position in a document (zero-based).
    ///
    /// In usage, if this is bigger than the length of the line in the source text,
    /// the consumer will use the line length instead :-)
    pub column: u32,
}

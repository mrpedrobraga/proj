use sequence_trie::SequenceTrie;
use serde::{Deserialize, Serialize};
use std::{path::PathBuf};

pub mod implementations;
pub mod manifest;

/// A trait that describes specific information about a kind of project,
/// for example, the items it can have, how to read it from files, etc.
pub trait ProjectKind {
    /// Type for the items inside a module.
    ///
    /// TODO: Create a trait to use as a bound here.
    type ModuleContent: std::fmt::Debug + Clone + Serialize + for<'de> Deserialize<'de>;

    /// Loads the root module for the project.
    ///
    /// TODO: Handle failure cases.
    fn load_root_module(modules: &mut ModuleSet<Self>, directory_path: PathBuf)
    where
        Self: Sized;

    /// Performs discovery for other standalone modules.
    ///
    /// TODO: Handle failure cases.
    fn discover_other_modules(modules: &mut ModuleSet<Self>, directory_path: PathBuf)
    where
        Self: Sized;
}

#[derive(Clone)]
pub struct ProjectView<P: ProjectKind> {
    pub modules: ModuleSet<P>,
    pub origin: ProjectOrigin,
}

/// Where a project was sourced from.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectOrigin {
    local_path: PathBuf,
}

/// A set of modules in a project.
#[derive(Clone)]
pub struct ModuleSet<P: ProjectKind> {
    /// A topogically sorted list of all modules in the set.
    /// Sibling ordering is not guaranteed but it should match
    /// item order for modules manifested from another module
    /// followed by iteration order for modules manifested from the file system.
    pub entries: Vec<ModuleEntry<P>>,
    /// Allows quick iteration of module paths matching a prefix.
    forward_index: SequenceTrie<ModuleName, usize>,
    /// Allows quick iteration of module paths matching a suffix.
    backward_index: SequenceTrie<ModuleName, usize>,
}

/// An entry in the project view describing a module in the project.
#[derive(Clone, Serialize, Deserialize)]
pub struct ModuleEntry<P: ProjectKind> {
    pub name: String,
    pub internal_path: ModulePath,
    pub origin: ModuleOrigin,
    pub content: P::ModuleContent,
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
    File(PathBuf),
    /// The module was sourced from an item inside another module.
    /// Think a `mod` block in Rust.
    Item(ProjectItemRef),
}

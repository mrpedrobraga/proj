use std::path::PathBuf;

use serde::{Deserialize, Serialize};

pub mod manifest;

pub mod implementations;

/// A trait that describes specific information about a kind of project,
/// for example, the items it can have, how to read it from files, etc.
pub trait ProjectKind {
  /// Type for the items inside a module.
  /// 
  /// TODO: Create a trait to use as a bound here.
  type Item: std::fmt::Debug + Clone + Serialize + for <'de> Deserialize<'de>;
}

#[derive(Debug)]
pub struct ProjectView<P: ProjectKind> {
    /// A topogically sorted list of all modules in the project.
    /// Sibling ordering is not guaranteed but it should match
    /// iteration order for modules manifested from the file system,
    /// and item order for modules manifested from another module.
    pub modules: Vec<ModuleEntry<P>>,
    pub origin: ProjectOrigin,
}

/// Where a project was sourced from.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectOrigin {
    local_path: PathBuf,
}

/// An entry in the project view describing a module in the project.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModuleEntry<P: ProjectKind> {
    pub name: String,
    pub origin: ModuleOrigin,
    pub items: Vec<ModuleItem<P>>,
}

/// A reference to a module in a project.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct ModuleRef(usize);

/// An item within a module.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModuleItem<P: ProjectKind> {
  item: P::Item
}

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

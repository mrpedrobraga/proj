#![doc = include_str!("../README.md")]

use self::project::manifest::ProjectManifester;
use thiserror::Error;

pub mod cli;
pub mod project;
pub mod repl;
pub mod server;

/// The main trait of this library.
/// Implement this for your own struct to create your own package manager.
pub trait ProjectManager {
    type Manifester: ProjectManifester;
}

#[derive(Error, Debug)]
pub enum ProjError {
    #[error("an io error occurred")]
    Io(#[from] std::io::Error),
}

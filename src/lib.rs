#![doc = include_str!("../README.md")]

use thiserror::Error;
use self::project::manifest::ProjectManifester;

pub mod cli;
pub mod server;
pub mod repl;
pub mod project;

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
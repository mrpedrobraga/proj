#![doc = include_str!("../README.md")]

use std::{
    marker::PhantomData,
    path::{Path, PathBuf},
};

use thiserror::Error;

/// The main trait of this library.
/// Implement this for your own struct to create your own package manager.
pub trait ProjectManager {
    type Manifester: ProjectManifester;
}

/// Handy alias so we don't have to specify the error type all the time...
pub type Result<T> = core::result::Result<T, ProjError>;

#[derive(Error, Debug)]
pub enum ProjError {
    #[error("an io error occurred")]
    Io(#[from] std::io::Error),
}

/// Trait for a component that manages the "existence" of a project
/// in a folder: creating, checking validity, getting information, etc.
pub trait ProjectManifester {
    /// Returns whether a directory contains a valid project.
    fn directory_contains_project(&self, path: &(impl AsRef<Path> + ?Sized)) -> Result<bool>;
}

/// A simple manifester that checks for the presence of a manifest file.
pub struct SimpleManifester<ManifestFile> {
    pub manifest_path: PathBuf,
    _marker: PhantomData<ManifestFile>,
}

impl<ManifestFile> SimpleManifester<ManifestFile> {
    pub fn new(manifest_path: PathBuf) -> Self {
        Self { manifest_path, _marker: PhantomData }
    }
}

impl<ManifestFile> ProjectManifester for SimpleManifester<ManifestFile> {
    /// Returns whether a directory contains a valid project.
    ///
    /// It checks for the presence of a manifest, returning `Ok(true)` or `Ok(false)`.
    /// This function may fail with an IO error.
    fn directory_contains_project(&self, path: &(impl AsRef<Path> + ?Sized)) -> Result<bool> {
        let manifest_path = path.as_ref().join(&self.manifest_path);
        let file_exists = std::fs::exists(manifest_path)?;
        Ok(file_exists)
    }
}

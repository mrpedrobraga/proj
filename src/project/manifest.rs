use std::{
    marker::PhantomData,
    path::{Path},
};

use crate::ProjError;

/// Handy alias so we don't have to specify the error type all the time...
pub type Result<T> = core::result::Result<T, ProjError>;

/// Trait for a component that manages the "existence" of a project
/// in a folder: creating, checking validity, getting information, etc.
pub trait ProjectManifester {
    /// Returns whether a directory contains a valid project.
    fn directory_contains_project(&self, path: &(impl AsRef<Path> + ?Sized)) -> Result<bool>;

    /// Returns a path in array form from a string.
    fn parse_module_path_from_str(&self, string_path: &str) -> Result<Vec<String>> {
        Ok(string_path.split("::").map(str::to_string).collect())
    }
}

/// A simple manifester that checks for the presence of a manifest file.
pub struct SimpleManifester<ManifestFile> {
    _marker: PhantomData<ManifestFile>,
}

impl<ManifestFile> SimpleManifester<ManifestFile> {
    pub fn new() -> Self {
        Self { _marker: PhantomData }
    }
}

impl<ManifestFile> Default for SimpleManifester<ManifestFile> {
    fn default() -> Self {
        Self::new()
    }
}

impl<ManifestFile> ProjectManifester for SimpleManifester<ManifestFile> {
    /// Returns whether a directory contains a valid project.
    ///
    /// It checks for the presence of a manifest, returning `Ok(true)` or `Ok(false)`.
    /// This function may fail with an IO error.
    fn directory_contains_project(&self, path: &(impl AsRef<Path> + ?Sized)) -> Result<bool> {
        let manifest_path = path.as_ref().join("README.md");
        let file_exists = std::fs::exists(manifest_path)?;
        Ok(file_exists)
    }
}

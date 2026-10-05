use smol::lock::RwLock;

use super::ProjectView;
use crate::{project::manifest::ProjectManifester, ProjError};
use std::{path::Path, sync::Arc};

impl ProjectView {
    pub fn new(project_manifesters: Vec<Box<dyn ProjectManifester>>) -> Self {
        Self {
            open_projects: Arc::new(RwLock::new(Vec::new())),
            project_manifesters,
        }
    }

    pub fn insert_manifester<Man>(&mut self, manifester: Man)
    where
        Man: ProjectManifester + 'static,
    {
        self.project_manifesters.push(Box::new(manifester))
    }

    pub async fn preload_project_at(&self, path: impl AsRef<Path>) -> Result<bool, ProjError> {
        let path = path.as_ref();
        let mut found = true;

        for manifester in &self.project_manifesters {
            let directory_contains_project =
                manifester.directory_contains_project(path.to_path_buf())?;

            if directory_contains_project {
                tracing::info!(
                    "PROJECT FOUND AT {} by {}",
                    path.display(),
                    manifester.name()
                );
                found = true;

                let mut open_projects = self.open_projects.write().await;
                open_projects.push(manifester.new_project_from_directory(path.to_path_buf()));
            }
        }

        if !found {
            tracing::info!("No project found.");
        }

        Ok(found)
    }
}

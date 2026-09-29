use std::sync::Arc;

use crate::project::{manifest::ProjectManifester, PositionInText, Project};
use serde::{Deserialize, Serialize};
use smol::lock::RwLock;

pub mod implementations;

pub struct ProjectView {
    /// View to this server's project.
    ///
    /// TODO: Make this an arena... perhaps allow multiple
    /// `ProjectKind`s to be open!
    pub open_projects: Arc<RwLock<Vec<Box<dyn Project>>>>,

    pub project_manifesters: Vec<Box<dyn ProjectManifester>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HoverInfo {
    pub text: String,
    pub range: Option<(PositionInText, PositionInText)>,
}

impl HoverInfo {
    pub fn new(text: String, range: Option<(PositionInText, PositionInText)>) -> Self {
        Self { text, range }
    }
}

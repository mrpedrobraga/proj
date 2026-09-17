use serde::{Deserialize, Serialize};

use crate::project::{PositionInText, Project};

pub struct ProjectServer {
    /// View to this server's project.
    ///
    /// TODO: Make this an arena... perhaps allow multiple
    /// `ProjectKind`s to be open!
    pub open_projects: Vec<Box<dyn Project>>,
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

use crate::project::{ProjectKind, ProjectView};

pub struct ProjectServer<P: ProjectKind> {
    pub view: Option<ProjectView<P>>
}
use crate::project::{ProjectKind, ProjectView};

#[derive(Debug)]
pub struct ProjectServer<P: ProjectKind> {
    /// View to this server's project.
    ///
    /// TODO: Make this a graph, allowing multiple
    /// project views to be open at the same time.
    /// This is necessary for dependencies.
    pub view: Option<ProjectView<P>>
}
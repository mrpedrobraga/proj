use crate::{project::ProjectKind, server::ProjectServer};

pub struct ProjectRepl<P: ProjectKind> {
    pub server: ProjectServer<P>
}
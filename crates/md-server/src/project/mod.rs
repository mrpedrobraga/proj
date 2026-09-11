 use serde::{Deserialize, Serialize};

pub mod implementations;

pub const MANIFEST_PATH: &str = "README.md";

#[derive(Debug)]
pub struct MarkdownProject {
    pub root_node: (),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarkdownContent {}


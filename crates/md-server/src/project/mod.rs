use proj_server::project::ModuleSet;
use serde::{Deserialize, Serialize};

pub mod implementations;

pub const MANIFEST_PATH: &str = "README.md";

#[derive(Debug)]
pub struct MarkdownProject {
    pub modules: ModuleSet,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarkdownContent {
    lines_which_are_headings: Vec<usize>,
    text: String,
}

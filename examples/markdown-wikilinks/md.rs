use proj::project::{ProjectKind};
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct MarkdownProject;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MarkdownModuleItem {
    Paragraph(String)
}

impl ProjectKind for MarkdownProject {
    type Item = MarkdownModuleItem;
}
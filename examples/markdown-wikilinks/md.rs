use proj::project::{ModuleEntry, ModuleItem, ModuleOrigin, ProjectKind};
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct MarkdownProject;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MarkdownModuleItem {
    Paragraph(String),
}

impl ProjectKind for MarkdownProject {
    type Item = MarkdownModuleItem;

    fn load_root_module(
        modules: &mut Vec<proj::project::ModuleEntry<Self>>,
        directory_path: std::path::PathBuf,
    ) where
        Self: Sized,
    {
        let root_module_path = directory_path.join("README.md");
        let mut root_module = ModuleEntry {
            name: "root".to_owned(),
            origin: ModuleOrigin::File(root_module_path.clone()),
            items: vec![],
        };
        
        let root_module_raw_source = std::fs::read_to_string(root_module_path).unwrap();
        for line in root_module_raw_source.lines().filter(|l| !l.is_empty()) {
            root_module.items.push(ModuleItem {
                item: MarkdownModuleItem::Paragraph(line.to_owned()),
            });
        }
        
        modules.push(root_module);
    }
}

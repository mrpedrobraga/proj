use proj::project::{
    Item, ItemSet, ModuleEntry,
    ModuleOrigin::{self, File},
    ModulePath, ModuleSet, ProjectKind,
};
use serde::{Deserialize, Serialize};
use walkdir::WalkDir;

#[derive(Debug)]
pub struct MarkdownProject;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MarkdownModuleItem {
    Paragraph(String),
}

impl ProjectKind for MarkdownProject {
    type Item = MarkdownModuleItem;

    fn load_root_module(modules: &mut ModuleSet<Self>, directory_path: std::path::PathBuf)
    where
        Self: Sized,
    {
        let root_module_path = directory_path.join("README.md");
        let mut root_module = ModuleEntry {
            name: "README.md".to_string(),
            origin: ModuleOrigin::File(root_module_path.clone()),
            items: ItemSet::new(),
        };

        let root_module_raw_source = std::fs::read_to_string(root_module_path).unwrap();
        parse_markdown(&mut root_module, &root_module_raw_source);
        modules.insert(root_module, ModulePath::from(["root".to_string()]));
    }

    fn discover_other_modules(modules: &mut ModuleSet<Self>, directory_path: std::path::PathBuf)
    where
        Self: Sized,
    {
        for entry in WalkDir::new(&directory_path) {
            let entry = entry.unwrap();
            let entry_path = entry.path();
            if entry.file_type().is_file()
                && (entry_path.extension().and_then(std::ffi::OsStr::to_str) == Some("md"))
            {
                let relative_path = entry_path.strip_prefix(&directory_path).unwrap();
                let internal_path_components = std::iter::once("file".to_string())
                    .chain(
                        relative_path.with_extension("")
                            .components()
                            .map(|x| x.as_os_str().to_str().unwrap().to_string()),
                    )
                    .collect::<Vec<_>>();
                println!("Markdown file at {}", relative_path.display());

                // TODO: Replace this ridiculous check with something else.
                // Perhaps instead of a `Vec<ModuleEntry>` I create a type named `ModuleSet`
                // that has proper a proper API for this.
                if modules.contains_module_from_file_path(entry_path) {
                    println!("Skipping file as it has already been indexed.");
                    continue;
                };

                let mut module = ModuleEntry {
                    name: relative_path
                        .to_str()
                        .or(entry.file_name().to_str())
                        .unwrap()
                        .to_string(),
                    origin: File(entry_path.to_path_buf()),
                    items: ItemSet::new(),
                };

                parse_markdown(&mut module, &std::fs::read_to_string(entry_path).unwrap());
                modules.insert(module, ModulePath::from(internal_path_components.as_ref()));
            }
        }
    }
}

fn parse_markdown(empty_module: &mut ModuleEntry<MarkdownProject>, markdown_source: &str) {
    for line in markdown_source.lines().filter(|l| !l.is_empty()) {
        empty_module.items.insert(Item {
            item: MarkdownModuleItem::Paragraph(line.to_owned()),
        });
    }
}

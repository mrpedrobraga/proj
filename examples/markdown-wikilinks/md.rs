use markdown::ParseOptions;
use proj::project::{
    ModuleEntry,
    ModuleOrigin::{self, File},
    ModulePath, ModuleSet, ProjectKind,
};
use serde::{Deserialize, Serialize};
use walkdir::WalkDir;

#[derive(Debug)]
pub struct MarkdownProject;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarkdownModuleContent {
    root_node: markdown::mdast::Node
}

impl ProjectKind for MarkdownProject {
    type ModuleContent = MarkdownModuleContent;

    fn load_root_module(modules: &mut ModuleSet<Self>, directory_path: std::path::PathBuf)
    where
        Self: Sized,
    {
        let root_module_path = directory_path.join("README.md");
        let root_module_raw_source = std::fs::read_to_string(&root_module_path).unwrap();
        let content = parse_markdown(&root_module_raw_source);
        let root_module = ModuleEntry {
            name: "README.md".to_string(),
            internal_path: ModulePath::from(["root"]),
            origin: ModuleOrigin::File(root_module_path.clone()),
            content
        };
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
                let internal_path_components = relative_path
                    .with_extension("")
                    .components()
                    .map(|x| x.as_os_str().to_str().unwrap().to_string())
                    .collect::<Vec<_>>();
                let internal_path = ModulePath::from(internal_path_components.as_ref());
                if modules.contains_module_from_file_path(entry_path) {
                    continue;
                };
                let content = parse_markdown(&std::fs::read_to_string(entry_path).unwrap());

                let module = ModuleEntry {
                    name: entry_path.file_stem().unwrap().to_str().unwrap().to_string(),
                    internal_path: internal_path.clone(),
                    origin: File(entry_path.to_path_buf()),
                    content
                };

                modules.insert(module, internal_path);
            }
        }
    }
}

fn parse_markdown(markdown_source: &str) -> MarkdownModuleContent {
    let options = ParseOptions::default();
    let root_mdast_node = markdown::to_mdast(markdown_source, &options).unwrap();
    MarkdownModuleContent { root_node: root_mdast_node }
}

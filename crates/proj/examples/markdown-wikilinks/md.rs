use proj::project::{
    ModuleContentKind, ModuleEntry,
    ModuleOrigin::{self, File},
    ModulePath, ModuleSet, ProjectKind,
};
use serde::{Deserialize, Serialize};
use walkdir::WalkDir;

#[derive(Debug)]
pub struct MarkdownProject;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarkdownModuleContent {
    pub root_node: (),
}

impl ModuleContentKind for MarkdownModuleContent {}

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
            content,
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
                    name: entry_path
                        .file_stem()
                        .unwrap()
                        .to_str()
                        .unwrap()
                        .to_string(),
                    internal_path: internal_path.clone(),
                    origin: File(entry_path.to_path_buf()),
                    content,
                };

                modules.insert(module, internal_path);
            }
        }
    }
}

fn parse_markdown(markdown_source: &str) -> MarkdownModuleContent {
    let arena = comrak::Arena::new();
    let options = comrak::Options {
        extension: comrak::options::Extension::builder().wikilinks_title_after_pipe(true).build(),
        parse: comrak::options::Parse::builder().build(),
        render: comrak::options::Render::default(),
    };
    let root_node = comrak::parse_document(&arena, markdown_source, &options);

    for node in root_node.descendants() {
        println!("{:#?}", node.collect_text());
    }

    MarkdownModuleContent { root_node: () }
}

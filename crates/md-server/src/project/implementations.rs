use proj::{
    modpath,
    project::{ModuleContentKind, ModuleEntry, ModulePath, ProjectKind},
};

use super::{MANIFEST_PATH, MarkdownContent, MarkdownProject};

impl ModuleContentKind for MarkdownContent {}

impl ProjectKind for MarkdownProject {
    type ModuleContent = MarkdownContent;

    fn load_root_module(
        modules: &mut proj::project::ModuleSet<Self>,
        directory_path: std::path::PathBuf,
    ) where
        Self: Sized,
    {
        let root_module_file_path = directory_path.join(MANIFEST_PATH);
        let root_module_source =
            std::fs::read_to_string(&root_module_file_path).expect("Failed to load main module.");
        let content = parse_markdown(&root_module_source);

        let root_module_path = modpath!(root);
        let root_module = ModuleEntry {
            name: MANIFEST_PATH.to_string(),
            internal_path: root_module_path.clone(),
            origin: proj::project::ModuleOrigin::File(root_module_file_path),
            content,
        };

        modules.insert(root_module, root_module_path);
    }

    /// Spawns non declared modules from the file system.
    ///
    /// TODO: Offload the directory walking to `proj` and create a trait
    /// for deciding whether a file/directory should be included,
    /// how to index a file, how to get a file's content.
    fn discover_other_modules(
        modules: &mut proj::project::ModuleSet<Self>,
        directory_path: std::path::PathBuf,
    ) where
        Self: Sized,
    {
        for entry in walkdir::WalkDir::new(&directory_path) {
            let entry = entry.expect("Failed to get entry from file system.");
            let entry_path = entry.path();

            // Checks if the current file is a markdown file!
            if entry.file_type().is_file()
                && (entry_path.extension().and_then(|e| e.to_str()) == Some("md"))
            {
                let raw_markdown =
                    std::fs::read_to_string(entry_path).expect("Failed to read content of file.");
                let content = parse_markdown(&raw_markdown);

                let relative_path = entry_path
                    .strip_prefix(&directory_path)
                    .expect("Failed to strip prefix?");

                let internal_path_components = relative_path
                    .with_extension("")
                    .components()
                    .map(|com| com.as_os_str().to_str().unwrap().to_string())
                    .collect::<Vec<_>>();
                let internal_path = ModulePath::from(internal_path_components);

                // TODO: A better way of getting the name of a module :-)
                let module_name = entry_path
                    .file_stem()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .to_string();

                let module = ModuleEntry {
                    name: module_name,
                    internal_path: internal_path.clone(),
                    origin: proj::project::ModuleOrigin::File(entry_path.to_path_buf()),
                    content,
                };

                modules.insert(module, internal_path);
            }
        }
    }
}

fn parse_markdown(markdown_source: &str) -> MarkdownContent {
    let arena = comrak::Arena::new();
    let options = comrak::Options {
        extension: comrak::options::Extension::builder()
            .wikilinks_title_after_pipe(true)
            .build(),
        parse: comrak::options::Parse::builder().build(),
        render: comrak::options::Render::default(),
    };
    let root_node = comrak::parse_document(&arena, markdown_source, &options);

    for node in root_node.descendants() {
        println!("{:#?}", node.collect_text());
    }

    MarkdownContent {}
}

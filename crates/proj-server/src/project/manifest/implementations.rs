use ::globwalker::GlobWalkerBuilder;

use super::{DynError, IncludeRule, IncludeSource, ModuleInclude, ProjectLayout};
use crate::project::{ModuleContent, ModuleEntry, ModuleOrigin, ModulePath, ModuleSet};
use ::std::{
    collections::VecDeque,
    path::{Path, PathBuf},
    sync::Arc,
};

impl IncludeRule {
    /// Creates a new include rule with this glob and parser.
    pub fn new<Parse>(glob_pattern: String, parse: Parse) -> Self
    where
        Parse: Fn(&IncludeSource) -> core::result::Result<ModuleInclude, DynError> + 'static,
    {
        Self {
            glob_pattern,
            parse: Arc::new(parse),
        }
    }
}

impl<'walk> IncludeSource<'walk> {
    /// Helper function to quickly construct a `ModuleOrigin::File` from this source.s
    pub fn as_module_origin(&self) -> ModuleOrigin {
        ModuleOrigin::File(self.path.to_path_buf())
    }

    /// Returns the last component in the path (file name if file, directory name if directory) if one exists.
    pub fn file_name(&self) -> Option<String> {
        self.path
            .with_extension("")
            .file_name()
            .map(|base_name| base_name.to_string_lossy().to_string())
    }
}

impl ModuleInclude {
    /// Creates a new module include with this name and content.
    ///
    /// The internal path of this module will contain this name preceded
    /// by the names of the parent modules that generated this include.
    pub fn new(name: String, content: Arc<dyn ModuleContent>) -> Self {
        Self {
            name,
            content,
            children_rules: Vec::new(),
        }
    }

    /// Tells the directory visit to also include any modules
    /// matched by [rules]
    pub fn also_include(mut self, rules: Vec<IncludeRule>) -> Self {
        self.children_rules.extend(rules);
        self
    }
}

impl ProjectLayout {
    /// Creates an empty project layout.
    pub fn new() -> Self {
        Self { rules: Vec::new() }
    }

    /// Adds an include rule to the layout.
    ///
    /// These modules will be matched by `rule.glob` and parsed by `rule.parse`
    /// and included in the resulting [ModuleSet].
    pub fn include(mut self, rule: IncludeRule) -> Self {
        self.rules.push(rule);
        self
    }

    /// Gathers modules by walking down the given directory and producing a new `ModuleSet`!
    pub fn gather(&self, root_path: &Path) -> ModuleSet {
        let mut modules = ModuleSet::new();

        struct Job {
            rule: IncludeRule,
            base_path: PathBuf,
            parent_module_path: Vec<String>,
        }

        let mut queue: VecDeque<_> = self
            .rules
            .iter()
            .cloned()
            .map(|rule| Job {
                rule,
                base_path: root_path.to_path_buf(),
                parent_module_path: Vec::new(),
            })
            .collect();

        while let Some(job) = queue.pop_front() {
            // TODO: Allow configuring, say, maximum depth.
            let Ok(walker) = GlobWalkerBuilder::new(&job.base_path, &job.rule.glob_pattern).build()
            else {
                tracing::error!("Failed to parse job pattern `{:?}`", job.rule.glob_pattern);
                continue;
            };

            // Skip files we can't open (for permission reasons).
            let entries = walker.into_iter().filter_map(Result::ok);

            for entry in entries {
                let entry_path = entry.path();

                let Ok(raw) = std::fs::read_to_string(entry_path) else {
                    tracing::error!("Can not read `{}`.", entry_path.display());
                    continue;
                };

                let source = IncludeSource {
                    path: entry_path,
                    file_content: &raw,
                    parent_module_path: job.parent_module_path.as_slice(),
                };

                let Ok(parsed) = (job.rule.parse)(&source) else {
                    // TODO: Include the module anyways in an "errored" state.
                    tracing::warn!("Could not parse `{}`.", entry_path.display());
                    continue;
                };

                let this_module_path: Vec<_> = job
                    .parent_module_path
                    .iter()
                    .cloned()
                    .chain(std::iter::once(parsed.name.clone()))
                    .collect();

                // If this entry's path is `foo/bar/baz.txt`,
                // the children's rules are relative to `foo/bar/`.
                //
                // To get the behaviour you see in rust where a module named `foo.rs`
                // has its children under `foo/` simply give the children rules the `foo/` prefix.
                let child_base_path = entry_path.parent().unwrap_or(&job.base_path).to_path_buf();

                for child_rule in parsed.children_rules {
                    queue.push_back(Job {
                        rule: child_rule,
                        base_path: child_base_path.clone(),
                        parent_module_path: this_module_path.clone(),
                    });
                }

                tracing::info!(
                    "Found a module {} at {}.",
                    this_module_path.join("::"),
                    entry_path.display()
                );

                modules.insert(ModuleEntry {
                    name: parsed.name,
                    internal_path: ModulePath(this_module_path),
                    origin: source.as_module_origin(),
                    content: parsed.content,
                });
            }
        }

        modules
    }
}

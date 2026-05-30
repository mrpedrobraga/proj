# proj

A library for creating modern package managers, inspired by `cargo`.

While working on my projects, I realised that a lot of them required some sense of creating and managing a project represented by a directory on disk. So I created this library, and now I don't have to recreate this functionality from scratch every time.

`proj` is not a package manager itself. It's a library that declares a scaffolding—a collection of traits that you can implement for your own types to create a package manager. `proj` handles three things:

- Project Manifestation — identification of a project on a directory, usually given a manifest (like `package.json` or `Cargo.toml`);
- Project structure — finding out which modules exist, their hierarchy, and dependencies between modules;
- Project Tools — general purpose tools for interacting with projects via code, the CLI or a language server for project creation, refactoring, linting, packaging, and more;

### Using this Library
Implement the library's traits for your own struct. There is an example of a package manager for a toy language in the git repository.

```rust
/*
   The traits kinda look like this.
   In typical Rust fashion, you implement the required items,
   and get access to the given items.
*/

trait ProjectManager {
	type Manifester: ProjectManifest;
	// ...
}

trait ProjectManifest {
	type Manifest: Serialize + Deserialize;
	fn new(directory_path: &Path, info: Manifest) -> proj::Result<()>;
	fn info(directory_path: &Path) -> proj::Result<Self::Manifest>;
	// ...
}
```

### Project Tools
`proj` does not provide any binaries for general purpose use, so imagine that instead of "proj," it's the name of your package manager:

Also, these tools are provided for the CLI, but can also be accessed through code or via an LS.

```bash
# -- Project manifest --
proj new [<template>]
proj info
proj build
proj run
proj dev-serve  # Starts development server (LSP)
proj bundle     # Bundles this project in an entirely self-contained bundle for maximum portability.

# -- Tasks, for user defined routines --
proj task run <task>

# -- Dependency management, for module organization --
proj deps add <dependency>
proj deps remove <dependency>
proj deps list [--graph]
proj deps clean
proj deps upgrade [...<dependency>]

# -- Development and refactoring --
proj mod new <relative_path> [--simulate]
proj mod where <path>
proj mod move <original_path> <new_path> [--simulate]
proj lint check
proj lint apply [--simulate]
proj style apply
```
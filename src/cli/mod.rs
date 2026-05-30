//! # CLI
//! 
//! This module contains a scaffolding that allows a user
//! to interact with your project manager from the CLI.

use std::path::PathBuf;

#[derive(clap::Parser)]
#[command(version, about)]
pub enum CliArgs {
    /// Creates a new project on the CWD.
    New,
    /// Prints project information.
    Info,
    /// Builds the current project.
    Build,
    /// Runs the current project.
    Run,
    /// Starts a development server for the current project.
    DevServe,
    /// Bundles the project, allowing it to be shared in an entirely portable format.
    Bundle,

    #[command(subcommand)]
    Task(CliArgsTask),
    #[command(subcommand)]
    Deps(CliArgsDeps),
    #[command(subcommand)]
    Mod(CliArgsMod),
    #[command(subcommand)]
    Lint(CliArgsLint),
    #[command(subcommand)]
    Style(CliArgsStyle),
}

#[derive(clap::Subcommand)]
pub enum CliArgsTask {
    /// Runs a task.
    Run { #[arg(long)] task_name: String }
}

#[derive(clap::Subcommand)]
pub enum CliArgsDeps {
    /// Adds a new dependency
    Add {
        dependency_name: String,
        dependency_source: String,
    },
    /// Removes an existing dependency
    Remove {
        dependency_name: String,
    },
    /// Lists all existing dependencies
    List {
        #[arg(long)]
        graph: bool
    },
    /// Cleans dependency cache.
    /// Next time this project is built,
    /// all dependencies will be re-fetched.
    Clean,
    Upgrade,
}

#[derive(clap::Subcommand)]
pub enum CliArgsMod {
    /// Creates a new module on the given path.
    /// This is an "in project" path, not a file system path.
    /// 
    /// Example `proj mod new root::utils`;
    New {
        path: PathBuf
    },
    /// Prints the location of a module in the file system given its path.
    Where {
        mod_path: String
    },
    /// Moves a module from one path to another.
    /// These are "in project" paths, not file system paths.
    /// 
    /// Tip: You can use this subcommand to rename your modules, too.
    /// All references to this module should be updated.
    /// 
    /// Example: `proj mod move root::utils root::math::utils`;
    Move {
        from: String,
        to: String
    },
}

#[derive(clap::Subcommand)]
pub enum CliArgsLint {
    /// Links the project.
    Check,
    /// Applies recommendations on the project.
    Apply {
        /// Does not apply recommendations,
        /// but shows them so you can apply them yourself.
        #[arg(long)]
        simulate: bool
    }
}

#[derive(clap::Subcommand)]
pub enum CliArgsStyle {
    /// Formats the project.
    Apply
}
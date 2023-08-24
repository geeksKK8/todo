use std::path::PathBuf;

use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: TodoArgs,
}

#[derive(Subcommand)]
pub enum TodoArgs {
    #[command(about = "Add a new TODO")]
    Add(AddArgs),
    #[command(about = "Get a TODO")]
    Get(GetArgs),
    #[command(about = "Remove a TODO")]
    Remove(RemoveArgs),
    #[command(about = "Mark a TODO as done")]
    Done(DoneArgs),
    #[command(about = "Mark a TODO as undone")]
    Undone(UndoneArgs),
}

#[derive(Args)]
pub struct AddArgs {
    pub file_path: PathBuf,
    pub title: String,
    pub description: String,
}

#[derive(Args)]
pub struct GetArgs {
    pub file_path: PathBuf,
    pub id: Option<u32>,
}

#[derive(Args)]
pub struct RemoveArgs {
    pub file_path: PathBuf,
    pub id: u32,
}

#[derive(Args)]
pub struct DoneArgs {
    pub file_path: PathBuf,
    pub id: u32,
}

#[derive(Args)]
pub struct UndoneArgs {
    pub file_path: PathBuf,
    pub id: u32,
}

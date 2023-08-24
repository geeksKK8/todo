use clap::Parser;

mod args;
mod commands;
mod todo;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;
fn main() -> Result<()> {
    let args = args::Cli::parse();
    match args.command {
        args::TodoArgs::Add(add_args) => {
            commands::add(add_args)?;
        }
        args::TodoArgs::Get(get_args) => {
            commands::get(get_args)?;
        }
        args::TodoArgs::Remove(remove_args) => {
            commands::remove(remove_args)?;
        }
        args::TodoArgs::Done(done_args) => {
            commands::done(done_args)?;
        }
        args::TodoArgs::Undone(undone_args) => {
            commands::undone(undone_args)?;
        }
    }
    Ok(())
}

mod args;
mod path;

use library_core::{self, core_error};
use clap::Args;

pub fn initate_db(args: args::Args) -> Result<(), std::io::Error> {
    let path = path::get_path(args.database)?;
    let repo = library_core::Repository::new(&path);
    Ok(())
}

mod args;
mod path;
mod user_actions;
mod helpers;

use user_actions::UserAction; 
use std::io;

use library_core::{self, core_error};
use clap::Args;


pub fn initate_programm(args: args::Args) -> Result<usize, std::io::Error> {
    let path = path::get_path(args.database)?;
    let repo = library_core::Repository::new(&path);

    loop {
        let action = match user_actions::get_user_action() {
            Ok(act) => act,
            Err(err) => {
                println!("There seems to be a problem: {}", err);
                continue;
            }
        };
        if action==UserAction::Quit {
            break;
        }
    }
    Ok(0)
}



mod path;
pub mod args;
mod user_actions;
mod helpers;
mod action_handler;
mod repl_helpers;

use rustyline::{Editor, history::DefaultHistory};

use user_actions::UserAction; 
use crate::repl_helpers::{CommandCompleter, ReplHelper};
use library_core::{self, core_error};


pub fn initate_programm(args: args::Args) -> Result<usize, std::io::Error> {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    let path = path::get_path(args.database)?;
    println!("Connecting to: {:?}", path);
    let repo = library_core::Repository::new(&path);

    let helper = ReplHelper {
        completer: CommandCompleter {
            commands: vec![
                "help", "list-lernsets", "add-lernset",
                "learn-lernset", "list-learnitems", "add-learnitems", "quit",
            ],
        },
    };

    let mut r1: Editor<ReplHelper, DefaultHistory>  = Editor::new().expect("Failed to create init line editor");

    loop {
        let action = match user_actions::get_user_action(&mut r1) {
            Ok(act) => act,
            Err(err) => {
                println!("There seems to be a problem: {}", err);
                continue;
            }
        };
        if action==UserAction::Quit {
            println!("Exiting program.");
            break;
        }
        else {
            match action_handler::handle_action(action, &repo) {
                Ok(_) => (),
                Err(err) => println!("There seems to be a problem: {:?}", err)
            }
        }
    }
    Ok(0)
}


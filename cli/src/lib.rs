mod args;
mod path;

use std::io;

use library_core::{self, core_error};
use clap::Args;

#[derive(Debug, Copy, Clone)]
enum user_action {
    help,
    list_lernsets,
    add_lernset,
    learn_lernset,
    list_learnitems(usize),
    add_learnitems(usize),
    quit
}

const ALL: [user_action; 7] = [
    user_action::help,
    user_action::list_lernsets,
    user_action::add_lernset,
    user_action::learn_lernset,
    user_action::list_learnitems(5),
    user_action::add_learnitems(5),
    user_action::quit
];

impl user_action {
    pub fn list_actions()  {
        for action in ALL {
            println!("{action:?}");
        }
    }
}

pub fn initate_programm(args: args::Args) -> Result<(), std::io::Error> {
    let path = path::get_path(args.database)?;
    let repo = library_core::Repository::new(&path);
    Ok(())
}

fn get_user_action() -> Result<user_action, std::io::Error> {
    loop {
        println!("What would you like to do? (help for all options)");
        
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        let mut inputs = input.split_whitespace();
        
        match input.trim().to_lowercase().as_str() {
            "help" => return Ok(user_action::help),
        }

    }
}


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

fn get_user_action() -> Result<user_action, String> {
    loop {
        println!("What would you like to do? (help for all options)");
        
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => (),
            Err(err) => return Err(err.to_string())
        };


        let mut inputs = input.split_whitespace();
        
        match inputs.next() {
            Some(input) => match input {
                "help" => return Ok(user_action::help),
                "list_lernsets" => return Ok(user_action::list_lernsets),
                "add_lernset" => return Ok(user_action::add_lernset),
                "learn_lernset" => return Ok(user_action::learn_lernset),
                "list_learnitems" => {
                    let second_input = match convert_str_usize(inputs.next()) {
                        Ok(input) => input,
                        Err(err) => return Err(err),
                    };
                    return Ok(user_action::list_learnitems(second_input))
                }
                "add_learnitems" => {
                    let second_input = match convert_str_usize(inputs.next()) {
                        Ok(input) => input,
                        Err(err) => return Err(err),
                    };
                    return Ok(user_action::add_learnitems(second_input))
                },
                "quit" => return Ok(user_action::quit),
                _ => return Err("Please enter a possible command. Type help for possible commands.".to_string())
            },
            None => return Err("Please input something valid. Type help for possible commands".to_string())
        }
    }
}

fn convert_str_usize(input: Option<&str>) -> Result<usize, String> {
    Ok(match input {
        Some(num) => match num.parse() {
            Ok(num_usize) => num_usize,
            Err(err) => return Err(err.to_string())
        },
        None => return Err("please enter the lernsetId".to_string())
    })
}

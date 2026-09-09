use std::io;
use crate::{helpers, repl_helpers::ReplHelper};
use rustyline::{Editor, history::DefaultHistory};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum UserAction {
    Help,
    ListLernsets,
    AddLernset,
    DeleteLernset(usize),
    LearnLernset(usize),
    ListLearnitems(usize),
    AddLearnitems(usize),
    DeleteLearnitem(usize),
    Quit
}

pub const ALL: [UserAction; 9] = [
    UserAction::Help,
    UserAction::ListLernsets,
    UserAction::AddLernset,
    UserAction::DeleteLernset(5),
    UserAction::LearnLernset(5),
    UserAction::ListLearnitems(5),
    UserAction::AddLearnitems(5),
    UserAction::DeleteLearnitem(5),
    UserAction::Quit
];

impl UserAction {
    pub fn list_actions()  {
        println!("help");
        println!("list-lernsets");
        println!("add-lernset");
        println!("delete-lernset LERNSET_ID");
        println!("learn-lernset LERNSET_ID");
        println!("list-learnitems LERNSET_ID");
        println!("add-learnitems LERNSET_ID");
        println!("delete-learnitem LEARNITEM_ID");
        println!("quit");
    }
}

pub fn get_user_action(r1: &mut Editor<ReplHelper, DefaultHistory>) -> Result<UserAction, String> {
    loop {
        let readline = r1.readline("What would you like to do? (help for all options) \n > ");

        let input = match readline {
            Ok(line) => {
                let _ = r1.add_history_entry(line.as_str());
                line
            },
            Err(err) => return Err(err.to_string())
        };

        let mut inputs = input.split_whitespace();
        
        match inputs.next() {
            Some(input) => match input {
                "help" => return Ok(UserAction::Help),
                "list-lernsets" => return Ok(UserAction::ListLernsets),
                "add-lernset" => return Ok(UserAction::AddLernset),
                "delete-lernset" => {
                    let n = helpers::convert_str_usize(inputs.next())?;
                    return Ok(UserAction::DeleteLernset(n))
                }
                "learn-lernset" => {
                    let n = helpers::convert_str_usize(inputs.next())?;
                    return Ok(UserAction::LearnLernset(n))
                }
                "list-learnitems" => {
                    let n = helpers::convert_str_usize(inputs.next())?;
                    return Ok(UserAction::ListLearnitems(n))
                }
                "add-learnitems" => {
                    let n = helpers::convert_str_usize(inputs.next())?;
                    return Ok(UserAction::AddLearnitems(n))
                },
                "delte-learnitem" => {
                    let n = helpers::convert_str_usize(inputs.next())?;
                    return Ok(UserAction::DeleteLearnitem(n))
                }
                "quit" => return Ok(UserAction::Quit),
                _ => return Err("Please enter a possible command. Type help for possible commands.".to_string())
            },
            None => return Err("Please input something valid. Type help for possible commands".to_string())
        }
    }
}

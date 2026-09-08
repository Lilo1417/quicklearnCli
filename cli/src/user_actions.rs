use std::io;
use crate::helpers;

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

pub fn get_user_action() -> Result<UserAction, String> {
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
                "help" => return Ok(UserAction::Help),
                "list-lernsets" => return Ok(UserAction::ListLernsets),
                "add-lernset" => return Ok(UserAction::AddLernset),
                "delete-lernset" => {
                    let second_input = match helpers::convert_str_usize(inputs.next()) {
                        Ok(input) => input,
                        Err(err) => return Err(err),
                    };
                    return Ok(UserAction::DeleteLernset(second_input))
                }
                "learn-lernset" => {
                    let second_input = match helpers::convert_str_usize(inputs.next()) {
                        Ok(input) => input,
                        Err(err) => return Err(err),
                    };
                    return Ok(UserAction::LearnLernset(second_input))
                }
                "list-learnitems" => {
                    let second_input = match helpers::convert_str_usize(inputs.next()) {
                        Ok(input) => input,
                        Err(err) => return Err(err),
                    };
                    return Ok(UserAction::ListLearnitems(second_input))
                }
                "add-learnitems" => {
                    let second_input = match helpers::convert_str_usize(inputs.next()) {
                        Ok(input) => input,
                        Err(err) => return Err(err),
                    };
                    return Ok(UserAction::AddLearnitems(second_input))
                },
                "delte-learnitem" => {
                    let second_input = match helpers::convert_str_usize(inputs.next()) {
                        Ok(input) => input,
                        Err(err) => return Err(err),
                    };
                    return Ok(UserAction::DeleteLearnitem(second_input))
                }
                "quit" => return Ok(UserAction::Quit),
                _ => return Err("Please enter a possible command. Type help for possible commands.".to_string())
            },
            None => return Err("Please input something valid. Type help for possible commands".to_string())
        }
    }
}

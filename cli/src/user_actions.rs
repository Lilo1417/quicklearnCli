use std::io;
use crate::helpers;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum UserAction {
    Help,
    ListLernsets,
    AddLernset,
    LearnLernset,
    ListLearnitems(usize),
    AddLearnitems(usize),
    Quit
}

pub const ALL: [UserAction; 7] = [
    UserAction::Help,
    UserAction::ListLernsets,
    UserAction::AddLernset,
    UserAction::LearnLernset,
    UserAction::ListLearnitems(5),
    UserAction::AddLearnitems(5),
    UserAction::Quit
];

impl UserAction {
    pub fn list_actions()  {
        for action in ALL {
            println!("{action:?}");
        }
    }
}

pub fn get_user_action() -> Result<UserAction, String> {
    loop {
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
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
                "list_lernsets" => return Ok(UserAction::ListLernsets),
                "add_lernset" => return Ok(UserAction::AddLernset),
                "learn_lernset" => return Ok(UserAction::LearnLernset),
                "list_learnitems" => {
                    let second_input = match helpers::convert_str_usize(inputs.next()) {
                        Ok(input) => input,
                        Err(err) => return Err(err),
                    };
                    return Ok(UserAction::ListLearnitems(second_input))
                }
                "add_learnitems" => {
                    let second_input = match helpers::convert_str_usize(inputs.next()) {
                        Ok(input) => input,
                        Err(err) => return Err(err),
                    };
                    return Ok(UserAction::AddLearnitems(second_input))
                },
                "quit" => return Ok(UserAction::Quit),
                _ => return Err("Please enter a possible command. Type help for possible commands.".to_string())
            },
            None => return Err("Please input something valid. Type help for possible commands".to_string())
        }
    }
}

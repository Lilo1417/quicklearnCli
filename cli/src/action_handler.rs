use std::io;

use crate::user_actions::{self, UserAction};
use library_core::{self, LernsetRepository};

pub fn handle_action(ua: UserAction, repo: &library_core::Repository) -> Result<usize, std::io::Error> {
    match ua {
        UserAction::Help => UserAction::list_actions(),
        UserAction::Quit => return Ok(0),
        UserAction::AddLernset => return add_lernset(repo),
        UserAction::ListLernsets => return list_lernsets(repo),
        UserAction::LearnLernset(_) => (),
        UserAction::AddLearnitems(_) => (),
        UserAction::ListLearnitems(_) => (),
    }
    Ok(0)
}

fn add_lernset(repo: &library_core::Repository) -> Result<usize, std::io::Error> {
    let name = loop {
        println!("How would you like to name you lernset?");
        let mut lernset_name = String::new();
        match io::stdin().read_line(&mut lernset_name) {
            Ok(_) => break lernset_name,
            Err(err) => {
                println!("Please Input valid name. There was the following error: {}", err.to_string());
                continue;
            }
        }
    };
    match repo.sqlite_lernset.create(&name) {
        Ok(_) => println!("Lernset successflully created with name {}", name),
        Err(err) => println!("Something went wrong: {:?}", err)
    };
    Ok(0)
}

fn list_lernsets(repo: &library_core::Repository) -> Result<usize, std::io::Error> {
    let lernsets = match repo.sqlite_lernset.list(repo) {
        Ok(lernsets) => lernsets,
        Err(err) => {
            println!("There was a problem when fetching the lernsets: {:?}", err);
            return Err(std::io::Error);
        }
    };
    Ok(0)
}

use std::io::{self, Read};

use crate::user_actions::{self, UserAction};
use library_core::{self, LearnitemRepository, LernsetRepository};

pub fn handle_action(ua: UserAction, repo: &library_core::Repository) -> Result<usize, library_core::core_error::CoreError> {
    match ua {
        UserAction::Help => UserAction::list_actions(),
        UserAction::Quit => return Ok(0),
        UserAction::AddLernset => return add_lernset(repo),
        UserAction::ListLernsets => return list_lernsets(repo),
        UserAction::LearnLernset(id) => return learn_lernset(repo, id),
        UserAction::AddLearnitems(id) => return add_learnitems(repo, id),
        UserAction::ListLearnitems(id) => (),
    }
    Ok(0)
}

fn add_lernset(repo: &library_core::Repository) -> Result<usize, library_core::core_error::CoreError> {
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

fn list_lernsets(repo: &library_core::Repository) -> Result<usize, library_core::core_error::CoreError> {
    let lernsets = match repo.sqlite_lernset.list() {
        Ok(lernsets) => lernsets,
        Err(err) => {
            println!("There was a problem when fetching the lernsets: {:?}", err);
            return Err(err);
        }
    }; 
    println!("NAME \t ID");
    for lernset in &lernsets {
        println!("{} \t {}", lernset.name.trim(), lernset.lernset_id)
    }
    Ok(0)
}

fn learn_lernset(repo: &library_core::Repository, id: usize) -> Result<usize, library_core::core_error::CoreError> {
    todo!()
}

enum add_learn_opt {
    single,
    multiple,
}

fn add_learnitems(repo: &library_core::Repository, id: usize) -> Result<usize, library_core::core_error::CoreError> {
    let option = loop {
        println!("Would you like to add learnitems one after another[1] or multiple[2]?");
        let mut opt = String::new();
        match io::stdin().read_line(&mut opt) {
            Ok(_) => match opt.as_str().trim() {
                "1" => break add_learn_opt::single,
                "2" => break add_learn_opt::multiple,
                _ => continue
            }
            Err(err) => {
                println!("Please Input valid name. There was the following error: {}", err.to_string());
                continue;
            }
        }
    };

    match option {
        add_learn_opt::single => return add_single_learnitem(repo, id),
        add_learn_opt::multiple => return add_multiple_learnitems(repo, id)
    }
}

fn add_single_learnitem(repo: &library_core::Repository, id: usize) -> Result<usize, library_core::core_error::CoreError> {
    loop {
        let first_meaning = loop {
            let mut str = String::new();
            println!("Enter the origininal meaning of the learnitem: ");
            match io::stdin().read_line(&mut str) {
                Ok(_) => break str,
                Err(err) => {
                    println!("Please Input valid string. There was the following error: {}", err.to_string());
                    continue;
                }
            }
        };
        let second_meaning = loop {
            let mut str = String::new();
            println!("Enter the translated meaning of the learnitem: ");
            match io::stdin().read_line(&mut str) {
                Ok(_) => break str,
                Err(err) => {
                    println!("Please Input valid string. There was the following error: {}", err.to_string());
                    continue;
                }
            }
        };

        match repo.sqlite_learnitem.create(id, first_meaning, second_meaning, library_core::Learnstate::NotStarted) {
            Ok(_) => (),
            Err(err) => return Err(err)
        };

        let mut another = String::new();
        loop {
            println!("Would you like to add anotherone? [y/n]");
            match io::stdin().read_line(&mut another) {
                Ok(_) => match another.as_str().trim() {
                    "y" => break,
                    "n" => return Ok(0),
                    _ => {
                        println!("Please enter y or n.");
                        continue;
                    }
                }
                Err(err) => {
                    println!("There seems to have been a problem while reading you rinput: {}", err.to_string())
                }
            }
        }
    }
}

fn add_multiple_learnitems(repo: &library_core::Repository, id: usize) -> Result<usize, library_core::core_error::CoreError> {
    let input = loop {
        let mut input = String::new();
        println!("Please input your string of meanings with the format 'meaning_one meaning_two; '. To signal your finished press Ctrl+D on linux/macos or Ctrl+Z on Windows.");
        match io::stdin().read_to_string(&mut input) {
            Ok(_) => break input,
            Err(err) => println!("There was a problem while reading you rinput: {}", err.to_string())
        }
    };
    let learnitems = input.trim().split(";");

    let mut count = 0;
    for li in learnitems {
        if li.trim().is_empty() {
            continue;
        }

        let mut meanings = li.split_whitespace();
        let origin_meaning = match meanings.next() {
            Some(mean) => mean,
            None => { 
                println!("Please make sure to enter two meanings for every word and only split with the ; after both meanings.");
                return Err(library_core::core_error::CoreError::Storage("Please make sure to get the formating correct".to_string()));
            }
        };
        let trans_meaning = match meanings.next() {
            Some(mean) => mean,
            None => { 
                println!("Please make sure to enter two meanings for every word and only split with the ; after both meanings.");
                return Err(library_core::core_error::CoreError::Storage("Please make sure to get the formating correct".to_string()));
            }
        };
        match repo.sqlite_learnitem.create(id, origin_meaning.to_string(), trans_meaning.to_string(), library_core::Learnstate::NotStarted) {
            Ok(_) => (),
            Err(err) => return Err(err)
        }
        count+=1;
    }
    println!("Finished adding {} learnitems.", count);
    Ok(0)
}


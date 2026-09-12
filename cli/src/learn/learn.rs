use library_core::{Learnitem, Learnstate, core_error};
use rustyline::{DefaultEditor, error::ReadlineError};

pub fn learn(all_learnitems: &mut Vec<Learnitem>) -> Result<(), core_error::CoreError> {

    loop {
        let cur_indices = get_learnitems(all_learnitems)?;
        for &idx in &cur_indices {
            if learn_learnitem(&all_learnitems[idx])? {
                return Ok(())
            };
        }
    }
}
type Exit = bool;

fn learn_learnitem(li: &Learnitem) -> Result<Exit, core_error::CoreError> {
    let mut rl = DefaultEditor::new().expect("Something went horribly wrong when gnerating rl");

    let prompt = format!(" \x1b[1m[{} \x1b[0m (Ctrl+D to go back)\n > ", li.origin_meaning.trim());
    let input = match rl.readline(&prompt) {
            Ok(line) => line,
            Err(ReadlineError::Eof) => return Ok(true),
            Err(err) => return Err(core_error::CoreError::Read(err))
    };

    todo!("create learning logic");
    Ok(false)
}
fn get_learnitems(all_learnitems: &mut Vec<Learnitem>) -> Result<Vec<usize>, core_error::CoreError> {
    let learning_indeces: Vec<usize> = all_learnitems.iter()
        .enumerate()
        .filter(|(_, l)| matches!(&l.learnstate, Learnstate::Learning(_)))
        .map(|(i, _)| i)
        .collect();
    let a_learnitems = learning_indeces.len();
    let mut cur_indices = learning_indeces;

    if a_learnitems < 10 {
        let not_started_indices: Vec<usize> = all_learnitems.iter()
            .enumerate()
            .filter(|(_, l)| matches!(&l.learnstate, Learnstate::NotStarted))
            .map(|(i, _)| i)
            .collect();

        for &idx in not_started_indices.iter().take(10-a_learnitems) {
            all_learnitems[idx].update_learnstate()?;
            cur_indices.push(idx);
        }
    }

    Ok(cur_indices)
}


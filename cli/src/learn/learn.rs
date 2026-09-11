use library_core::{Learnitem, Learnstate, core_error};
use rustyline::error::ReadlineError::Io;

pub fn learn(mut all_learnitems: Vec<Learnitem>) -> Result<(), core_error::CoreError> {
    let learning_indeces: Vec<usize> = all_learnitems.iter()
        .enumerate()
        .filter(|(_, l)| matches!(l.learnstate, Learnstate::Learning(_)))
        .map(|(i, _)| i)
        .collect();
    let a_learnitems = learning_indeces.len();
    let mut cur_indices = learning_indeces;

    if a_learnitems < 10 {
        let not_started_indices: Vec<usize> = all_learnitems.iter()
            .enumerate()
            .filter(|(_, l)| matches!(l.learnstate, Learnstate::NotStarted))
            .map(|(i, _)| i)
            .collect();

        for &idx in not_started_indices.iter().take(10-a_learnitems) {
            all_learnitems[idx].update_learnstate()?;
            cur_indices.push(idx);
        }
    }

    for &idx in &cur_indices {
        println!("{:?}", all_learnitems[idx]);
    }

    Ok(())
}

use std::vec;

use library_core::{Learnitem, Learnstate};
pub fn learn(learnitems: Vec<Learnitem>) {
    let mut cur_learnitems = learnitems.iter().filter(|l| matches!(l.learnstate, Learnstate::Learning(_)));


}

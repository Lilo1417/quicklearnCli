type RemLearns= usize;

#[derive(Debug)]
pub enum Learnstate {
    Finished,
    Learning(RemLearns),
    NotStarted
}

#[derive(Debug)]
pub struct Learnitem {
    learnitem_id: usize,
    lernset_id: usize,
    origin_meaning: String,
    trans_meaning: String,
    learnstate: Learnstate,
}
